use core::{any::TypeId, mem::zeroed, pin::Pin, ptr};

use pin_init::{InPlaceWrite, PinInit, pin_data, pin_init_from_closure};
use ra_fsp_sys::generated::{
    self as raw, //-
    self as api,
    R_GLCDC_BASE,
    R_GLCDC_Close,
    R_GLCDC_Open,
    R_GLCDC_Type,
    display_api_t,
    display_cfg_t,
    display_color_t,
    display_ctrl_t,
    display_instance_t,
    e_display_in_format,
    e_display_state,
    e_fsp_err,
    fsp_err_t,
    glcdc_extended_cfg_t,
    glcdc_instance_ctrl_t,
    st_display_color__bindgen_ty_1,
    st_display_color__bindgen_ty_1__bindgen_ty_1,
};

use crate::{
    Block, Result,
    display_api::*,
    fsp_try_unsafe, pac,
    state_markers::{Closed, Opened},
    unsafe_pinned::UnsafePinned,
    utils,
};

pub fn example() {
    const HEIGHT: usize = 480;
    const WIDTH: usize = 800;
    const BUFFER_SIZE: usize = HEIGHT * HSTRIDE;
    const HSTRIDE: usize = layer_hstride(
        HEIGHT as u16,
        WIDTH as u16,
        e_display_in_format::DISPLAY_IN_FORMAT_32BITS_ARGB8888,
    )
    .unwrap() as usize;

    static F: static_cell::ConstStaticCell<FrameBuffer<BUFFER_SIZE>> =
        static_cell::ConstStaticCell::new(FrameBuffer::new());

    let conf = DisplayConf {
        input_buffers: [Some(F.take().to_mut()), None],
        input: [
            Some(Input {
                hsize: WIDTH as u16,
                vsize: HEIGHT as u16,
                format: e_display_in_format::DISPLAY_IN_FORMAT_32BITS_ARGB8888,
                line_descending_enable: false,
                lines_repeat_enable: false,
                lines_repeat_times: 0,
            }),
            None,
        ],
        output: Output {
            htiming: raw::display_timing_t {
                total_cyc: 1056,
                display_cyc: 800,
                back_porch: 256,
                sync_width: 128,
                sync_polarity: raw::e_display_signal_polarity::DISPLAY_SIGNAL_POLARITY_LOACTIVE,
            },
            vtiming: raw::display_timing_t {
                total_cyc: 628,
                display_cyc: 480,
                back_porch: 148,
                sync_width: 4,
                sync_polarity: raw::e_display_signal_polarity::DISPLAY_SIGNAL_POLARITY_LOACTIVE,
            },
            format: raw::e_display_out_format::DISPLAY_OUT_FORMAT_24BITS_RGB888,
            endian: raw::e_display_endian::DISPLAY_ENDIAN_LITTLE,
            color_order: raw::e_display_color_order::DISPLAY_COLOR_ORDER_RGB,
            data_enable_polarity: raw::e_display_signal_polarity::DISPLAY_SIGNAL_POLARITY_HIACTIVE,
            sync_edge: raw::e_display_sync_edge::DISPLAY_SIGNAL_SYNC_EDGE_RISING,
            bg_color: display_color(0, 0, 0, 0),
            brightness: raw::display_brightness_t {
                enable: false,
                r: 512,
                g: 512,
                b: 512,
            },
            contrast: raw::display_contrast_t {
                enable: false,
                r: 128,
                g: 128,
                b: 128,
            },
            dithering_on: false,
        },
        layer: [
            raw::display_layer_t {
                coordinate: raw::display_coordinate_t { x: 0, y: 0 },
                bg_color: display_color(255, 255, 255, 255),
                fade_control: raw::e_display_fade_control::DISPLAY_FADE_CONTROL_NONE,
                fade_speed: 0,
            },
            raw::display_layer_t {
                coordinate: raw::display_coordinate_t { x: 0, y: 0 },
                bg_color: display_color(255, 255, 255, 255),
                fade_control: raw::e_display_fade_control::DISPLAY_FADE_CONTROL_NONE,
                fade_speed: 0,
            },
        ],
        line_detect: Some(crate::Irq {
            int: pac::Interrupt::IEL15,
            prio: Some(10),
        }),
        underflow_1: None,
        underflow_2: None,
        callback: Some({
            extern "C" fn glcdc_callback(_p_args: &mut raw::display_callback_args_t) {
                unimplemented!()
            }
            glcdc_callback
        }),
        extend: GlcdcExtendedConfig {
            tcon_hsync: raw::e_glcdc_tcon_pin::GLCDC_TCON_PIN_0,
            tcon_vsync: raw::e_glcdc_tcon_pin::GLCDC_TCON_PIN_1,
            tcon_de: raw::e_glcdc_tcon_pin::GLCDC_TCON_PIN_2,
            correction_proc_order: raw::e_glcdc_correction_proc_order::GLCDC_CORRECTION_PROC_ORDER_BRIGHTNESS_CONTRAST2GAMMA,
            clksrc: raw::e_glcdc_clk_src::GLCDC_CLK_SRC_INTERNAL,
            clock_div_ratio: raw::e_glcdc_panel_clk_div::GLCDC_PANEL_CLK_DIVISOR_32,
            dithering_mode: raw::e_glcdc_dithering_mode::GLCDC_DITHERING_MODE_TRUNCATE,
            dithering_pattern_A: raw::e_glcdc_dithering_pattern::GLCDC_DITHERING_PATTERN_11,
            dithering_pattern_B: raw::e_glcdc_dithering_pattern::GLCDC_DITHERING_PATTERN_11,
            dithering_pattern_C: raw::e_glcdc_dithering_pattern::GLCDC_DITHERING_PATTERN_11,
            dithering_pattern_D: raw::e_glcdc_dithering_pattern::GLCDC_DITHERING_PATTERN_11,
        },
    };

    let glcdc_regs = unsafe { pac::GLCDC::steal() };
    let initializer = Glcdc::<Opened>::open(glcdc_regs, conf);
    // pin_init::stack_try_pin_init!(let glcdc = initializer);
    static PLACE: static_cell::StaticCell<Glcdc<Opened>> = static_cell::StaticCell::new();
    let glcdc = PLACE.uninit().write_pin_init(initializer);
    // let glcdc = <This<_> as pin_init::InPlaceWrite<_>>::write_pin_init(uninit, initializer);

    let mut glcdc = glcdc.expect("Failed to open");
    glcdc.as_mut().start().expect("Failed to start");
}

#[repr(C)] // `#[repr(C)]` is for `Glcdc::<Closed>::open`
#[pin_data(PinnedDrop)]
pub struct Glcdc<State: 'static> {
    regs: pac::GLCDC,
    c_ext_cfg: core::mem::MaybeUninit<UnsafePinned<glcdc_extended_cfg_t>>,
    ctrl: UnsafePinned<glcdc_instance_ctrl_t>,
    cfg: UnsafePinned<display_cfg_t>,
    inst: UnsafePinned<display_instance_t>, // points to cfg and ctrl
    _marker: core::marker::PhantomData<State>,
}

#[allow(non_snake_case)]
#[derive(Default)]
pub struct GlcdcExtendedConfig {
    pub tcon_hsync: raw::glcdc_tcon_pin_t,
    pub tcon_vsync: raw::glcdc_tcon_pin_t,
    pub tcon_de: raw::glcdc_tcon_pin_t,
    pub correction_proc_order: raw::glcdc_correction_proc_order_t,
    pub clksrc: raw::glcdc_clk_src_t,
    pub clock_div_ratio: raw::glcdc_panel_clk_div_t,
    pub dithering_mode: raw::glcdc_dithering_mode_t,
    pub dithering_pattern_A: raw::glcdc_dithering_pattern_t,
    pub dithering_pattern_B: raw::glcdc_dithering_pattern_t,
    pub dithering_pattern_C: raw::glcdc_dithering_pattern_t,
    pub dithering_pattern_D: raw::glcdc_dithering_pattern_t,
    // unsupported
    // pub phy_layer: *mut ::core::ffi::c_void,
}

const API: display_api_t = display_api_t {
    open: Some(api::R_GLCDC_Open),
    close: Some(api::R_GLCDC_Close),
    start: Some(api::R_GLCDC_Start),
    stop: Some(api::R_GLCDC_Stop),
    layerChange: Some(api::R_GLCDC_LayerChange),
    bufferChange: Some(api::R_GLCDC_BufferChange),
    correction: Some(api::R_GLCDC_ColorCorrection),
    clut: Some(api::R_GLCDC_ClutUpdate),
    clutEdit: Some(api::R_GLCDC_ClutEdit),
    colorKeySet: Some(api::R_GLCDC_ColorKeySet),
    statusGet: Some(api::R_GLCDC_StatusGet),
};

unsafe impl<S> Block for Glcdc<S> {
    type Config = display_cfg_t;
    type Instance = display_instance_t;
    type Api = display_api_t;
    type State = S;
    type Context = ();

    const API: &'static Self::Api = &API;

    fn ctrl(&self) -> *mut core::ffi::c_void {
        self.ctrl.get().cast()
    }

    fn instance(&self) -> &Self::Instance {
        unsafe { &*self.inst.get() }
    }
}

unsafe impl<S: 'static> Send for Glcdc<S> {}
unsafe impl<S: 'static> Sync for Glcdc<S> {}

impl Glcdc<Closed> {
    pub fn new(regs: pac::GLCDC) -> Self {
        const {
            assert!(
                e_display_state::DISPLAY_STATE_CLOSED as u32 == 0,
                "`DISPLAY_STATE_CLOSED` must be zero"
            );
        }
        Self {
            regs,
            c_ext_cfg: core::mem::MaybeUninit::zeroed(),
            ctrl: UnsafePinned::new(unsafe { zeroed() }),
            cfg: UnsafePinned::new(unsafe { zeroed() }),
            inst: UnsafePinned::new(unsafe { zeroed() }),
            _marker: core::marker::PhantomData,
        }
    }
    pub fn open(
        self: Pin<&mut Self>,
        cfg: DisplayConf<GlcdcExtendedConfig>,
    ) -> Result<Pin<&mut Glcdc<Opened>>> {
        unsafe {
            let this = ptr::from_mut(self.get_unchecked_mut());
            let regs = ptr::read(&(*this).regs);

            if (*(*this).ctrl.get()).state != e_display_state::DISPLAY_STATE_CLOSED {
                return Err(e_fsp_err::FSP_ERR_ALREADY_OPEN);
            }

            let this = this.cast::<Glcdc<Opened>>();
            init_open(this, regs, cfg)?;
            Ok(Pin::new_unchecked(&mut *this))
        }
    }
}

impl Glcdc<Opened> {
    pub fn open(
        glcdc: pac::GLCDC,
        cfg: DisplayConf<GlcdcExtendedConfig>,
    ) -> impl PinInit<Glcdc<Opened>, fsp_err_t> {
        unsafe { pin_init_from_closure(|slot| init_open(slot, glcdc, cfg)) }
    }
}

#[pin_init::pinned_drop]
impl<S: 'static> PinnedDrop for Glcdc<S> {
    fn drop(mut self: Pin<&mut Self>) {
        if TypeId::of::<Closed>() == TypeId::of::<S>() {
            return;
        }

        loop {
            match fsp_try_unsafe!({
                let this = self.as_mut().get_unchecked_mut();
                R_GLCDC_Close(this.ctrl.get().cast::<display_ctrl_t>())
            }) {
                // wait until display is not being updated
                Err(e_fsp_err::FSP_ERR_INVALID_UPDATE_TIMING) => continue,
                Ok(()) => return,
                Err(e) => panic!("Error closing GLCDC: {e:0X}"),
            }
        }
    }
}

unsafe fn init_open(
    slot: *mut Glcdc<Opened>,
    glcdc: pac::GLCDC,
    mut cfg: DisplayConf<GlcdcExtendedConfig>,
) -> Result<()> {
    unsafe {
        (*slot).regs = glcdc;
        (*slot).ctrl = UnsafePinned::new(zeroed());
        (*(*slot).inst.get()).p_ctrl = (*slot).ctrl.get().cast::<core::ffi::c_void>();
        (*(*slot).inst.get()).p_cfg = (*slot).cfg.get().cast_const();
        (*(*slot).inst.get()).p_api = ptr::from_ref(&API);

        let p_extend = (*slot)
            .c_ext_cfg
            .write(UnsafePinned::new(cfg.extend.c_conf()))
            .get()
            .cast::<core::ffi::c_void>();

        let mut display_cfg = match cfg.c_conf() {
            Ok(c) => c,
            Err(e) => {
                log::error!("GLCDC config error: {e}");
                return Err(e_fsp_err::FSP_ERR_INVALID_ARGUMENT);
            }
        };
        display_cfg.p_extend = p_extend;
        (*slot).cfg = UnsafePinned::new(display_cfg);

        let p_ctrl = UnsafePinned::raw_get(&raw const (*slot).ctrl);
        let p_cfg = UnsafePinned::raw_get(&raw const (*slot).cfg);

        // FSP needs to IRQs to setup contexts, but it will additionally
        // unconditionally set priorities from cfg.
        // Thus we read them and give to FSP. FSP will thus not change them.
        // Critical section is for nothing to change priorities between out
        // read and FSP's write.
        critical_section::with(|_| {
            utils::try_read_priority_into(cfg.line_detect, &raw mut (*p_cfg).line_detect_ipl);
            utils::try_read_priority_into(cfg.underflow_1, &raw mut (*p_cfg).underflow_1_ipl);
            utils::try_read_priority_into(cfg.underflow_2, &raw mut (*p_cfg).underflow_2_ipl);

            fsp_try_unsafe!(R_GLCDC_Open(p_ctrl.cast::<display_ctrl_t>(), p_cfg))
        })
    }
}

impl GlcdcExtendedConfig {
    pub const fn c_conf(&self) -> glcdc_extended_cfg_t {
        glcdc_extended_cfg_t {
            tcon_hsync: self.tcon_hsync,
            tcon_vsync: self.tcon_vsync,
            tcon_de: self.tcon_de,
            correction_proc_order: self.correction_proc_order,
            clksrc: self.clksrc,
            clock_div_ratio: self.clock_div_ratio,
            dithering_mode: self.dithering_mode,
            dithering_pattern_A: self.dithering_pattern_A,
            dithering_pattern_B: self.dithering_pattern_B,
            dithering_pattern_C: self.dithering_pattern_C,
            dithering_pattern_D: self.dithering_pattern_D,
            phy_layer: ptr::null_mut(),
        }
    }
}

impl<S: 'static> Glcdc<S> {
    pub fn regs(&self) -> &pac::GLCDC {
        &self.regs
    }
}

pub const fn regs() -> *mut R_GLCDC_Type {
    R_GLCDC_BASE as *mut R_GLCDC_Type
}

pub const fn display_color(r: u8, g: u8, b: u8, a: u8) -> display_color_t {
    display_color_t {
        __bindgen_anon_1: st_display_color__bindgen_ty_1 {
            byte: st_display_color__bindgen_ty_1__bindgen_ty_1 { b, g, r, a },
        },
    }
}
