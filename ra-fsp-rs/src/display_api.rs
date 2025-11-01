use core::{ffi::c_void, mem::MaybeUninit, pin::Pin, ptr};

use crate::{Block, Result, fsp_try_unsafe, state_markers, utils::Irq};

use ra_fsp_sys::generated::{
    display_api_t, // -
    display_brightness_t,
    display_callback_args_t,
    display_cfg_t,
    display_clut_cfg_t,
    display_color_order_t,
    display_color_t,
    display_colorkeying_layer_t,
    display_contrast_t,
    display_correction_t,
    display_endian_t,
    display_frame_layer_t,
    display_in_format_t,
    display_input_cfg_t,
    display_instance_t,
    display_layer_t,
    display_out_format_t,
    display_output_cfg_t,
    display_runtime_cfg_t,
    display_signal_polarity_t,
    display_status_t,
    display_sync_edge_t,
    display_timing_t,
    e_display_in_format,
};

const DISPLAY_INPUT_CFG_OFF: display_input_cfg_t = display_input_cfg_t {
    p_base: ptr::null_mut(),
    hsize: 0,
    vsize: 0,
    hstride: 0,
    format: e_display_in_format::DISPLAY_IN_FORMAT_32BITS_ARGB8888,
    line_descending_enable: false,
    lines_repeat_enable: false,
    lines_repeat_times: 0,
};

#[repr(align(64))]
pub struct FrameBuffer<const SIZE: usize>(pub [u8; SIZE]);
/// Invariant: the slice and its length are 64-byte aligned
pub struct FrameBufferMut(pub(crate) &'static mut [u8]);

#[derive(Clone, Copy, Debug)]
pub struct Input {
    pub hsize: u16,
    pub vsize: u16,
    pub format: display_in_format_t,
    pub line_descending_enable: bool,
    pub lines_repeat_enable: bool,
    pub lines_repeat_times: u16,
}

pub struct Output {
    pub htiming: display_timing_t,
    pub vtiming: display_timing_t,
    pub format: display_out_format_t,
    pub endian: display_endian_t,
    pub color_order: display_color_order_t,
    pub data_enable_polarity: display_signal_polarity_t,
    pub sync_edge: display_sync_edge_t,
    pub bg_color: display_color_t,
    pub brightness: display_brightness_t,
    pub contrast: display_contrast_t,
    // gamma correction is unimplemented
    // pub p_gamma_correction: *mut display_gamma_correction_t,
    pub dithering_on: bool,
}

#[derive(Default)]
pub struct DisplayConf<Extended> {
    pub(crate) input_buffers: [Option<FrameBufferMut>; 2],
    pub input: [Option<Input>; 2],
    pub output: Output,
    pub layer: [display_layer_t; 2],
    pub line_detect: Option<Irq>,
    pub underflow_1: Option<Irq>,
    pub underflow_2: Option<Irq>,
    pub callback: Option<extern "C" fn(p_args: &mut display_callback_args_t)>,
    pub extend: Extended,
}

pub const fn bpp(format: e_display_in_format) -> u8 {
    match format {
        e_display_in_format::DISPLAY_IN_FORMAT_32BITS_ARGB8888 => 32,
        e_display_in_format::DISPLAY_IN_FORMAT_32BITS_RGB888 => 32,
        e_display_in_format::DISPLAY_IN_FORMAT_16BITS_RGB565 => 16,
        e_display_in_format::DISPLAY_IN_FORMAT_16BITS_ARGB1555 => 16,
        e_display_in_format::DISPLAY_IN_FORMAT_16BITS_ARGB4444 => 16,
        e_display_in_format::DISPLAY_IN_FORMAT_CLUT8 => 8,
        e_display_in_format::DISPLAY_IN_FORMAT_CLUT4 => 4,
        e_display_in_format::DISPLAY_IN_FORMAT_CLUT1 => 1,
    }
}

impl<const SIZE: usize> FrameBuffer<SIZE> {
    pub const fn new() -> Self {
        Self([0u8; SIZE])
    }
    pub fn to_mut(&'static mut self) -> FrameBufferMut {
        const {
            assert!(
                SIZE % 64 == 0,
                "Display Frame Buffer must have 64-byte aligned size"
            )
        }

        FrameBufferMut(&mut self.0)
    }
}

impl FrameBufferMut {
    pub unsafe fn from_raw_parts(ptr: *mut u8, size: usize) -> Self {
        unsafe { FrameBufferMut(core::slice::from_raw_parts_mut(ptr, size)) }
    }
    pub fn to_raw_parts(self) -> (*mut u8, usize) {
        let mut this = core::mem::ManuallyDrop::new(self);
        let len = this.len();
        let ptr = this.as_mut_ptr() as *mut u8;

        (ptr, len)
    }
}
impl core::ops::Deref for FrameBufferMut {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
impl core::ops::DerefMut for FrameBufferMut {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

pub const fn layer_hstride(vsize: u16, hsize: u16, format: e_display_in_format) -> Option<u32> {
    let 16..=1016 = vsize else { return None };
    let 16..=1020 = hsize else { return None };

    let bits = hsize as u32 * (bpp(format) as u32);
    Some(bits.div_ceil(64 * 8) * 64)
}

pub enum DisplayConfError {
    InvalidDimensions(u16, u16),
    UnmatchedBufferSize { extected: usize, found: usize },
}

impl<Extended> DisplayConf<Extended> {
    pub fn c_conf(&mut self) -> core::result::Result<display_cfg_t, DisplayConfError> {
        let [b1, b2] = core::mem::take(&mut self.input_buffers);
        let [i1, i2] = self.input;

        let input = [b1.zip(i1), b2.zip(i2)].map(|i| {
            i.map_or(Ok(DISPLAY_INPUT_CFG_OFF), |(b, i)| {
                let (p_base, buf_len) = b.to_raw_parts();
                let p_base = p_base.cast::<u32>();

                assert!(p_base.addr() & (64 - 1) == 0, "Must be 64-byte aligned");
                assert!(p_base.is_aligned());

                let (vsize, hsize, format) = (i.vsize, i.hsize, i.format);
                let Some(hstride) = layer_hstride(vsize, hsize, format) else {
                    return Err(DisplayConfError::InvalidDimensions(vsize, hsize));
                };
                let required_buf_len = hstride as usize * vsize as usize;

                // To be able to return it back, we will need to calculate the size.
                if buf_len != required_buf_len {
                    return Err(DisplayConfError::UnmatchedBufferSize {
                        extected: required_buf_len,
                        found: buf_len,
                    });
                }

                Ok(display_input_cfg_t {
                    p_base,
                    hstride,
                    vsize,
                    hsize,
                    format,
                    line_descending_enable: i.line_descending_enable,
                    lines_repeat_enable: i.lines_repeat_enable,
                    lines_repeat_times: i.lines_repeat_times,
                })
            })
        });

        let [i1, i2] = match input {
            [Ok(i1), Ok(i2)] => [i1, i2],
            [Err(e), _] | [_, Err(e)] => return Err(e),
        };

        Ok(display_cfg_t {
            input: [i1, i2],
            output: display_output_cfg_t {
                htiming: self.output.htiming,
                vtiming: self.output.vtiming,
                format: self.output.format,
                endian: self.output.endian,
                color_order: self.output.color_order,
                data_enable_polarity: self.output.data_enable_polarity,
                sync_edge: self.output.sync_edge,
                bg_color: self.output.bg_color,
                brightness: self.output.brightness,
                contrast: self.output.contrast,
                p_gamma_correction: ptr::null_mut(),
                dithering_on: self.output.dithering_on,
            },
            layer: self.layer,
            line_detect_irq: Irq::extract_irq(self.line_detect),
            underflow_1_irq: Irq::extract_irq(self.underflow_1),
            underflow_2_irq: Irq::extract_irq(self.underflow_2),
            line_detect_ipl: Irq::extract_ipl(self.underflow_2),
            underflow_1_ipl: Irq::extract_ipl(self.underflow_2),
            underflow_2_ipl: Irq::extract_ipl(self.underflow_2),
            p_callback: self.callback.map(|cb| crate::utils::cast_callback(cb)),
            p_context: ptr::null_mut(),
            p_extend: ptr::null(),
        })
    }
}

impl core::fmt::Display for DisplayConfError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DisplayConfError::InvalidDimensions(vsize, hsize) => {
                write!(
                    f,
                    "Invalid dimensions: vsize={vsize}, hsize={hsize}, expected vsize in 16..=1016 and hsize in 16..=1020"
                )
            }
            DisplayConfError::UnmatchedBufferSize { extected, found } => {
                write!(
                    f,
                    "Unmatched buffer size: expected={extected}, found={found}",
                )
            }
        }
    }
}

impl Default for Output {
    fn default() -> Self {
        // SAFETY: all-zero output config is valid
        unsafe { core::mem::zeroed() }
    }
}

pub trait DisplayApi {
    /// start the display
    fn start(self: Pin<&mut Self>) -> Result<()>;
    /// stop the display
    fn stop(self: Pin<&mut Self>) -> Result<()>;
    /// change layer parameters at runtime
    ///
    /// `cfg` - new layer configuration
    /// `layer` - layer to change
    fn layer_change(
        self: Pin<&mut Self>,
        cfg: &display_runtime_cfg_t,
        layer: display_frame_layer_t,
    ) -> Result<()>;
    /// change layer framebuffer pointer
    ///
    /// `framebuffer` - 64-byte aligned framebuffer pointer
    /// `layer` - layer to change
    fn buffer_change(
        self: Pin<&mut Self>,
        framebuffer: *mut u8,
        layer: display_frame_layer_t,
    ) -> Result<()>;
    /// set color correction
    fn correction(self: Pin<&mut Self>, correction: &display_correction_t) -> Result<()>;
    /// set CLUT for the display device
    ///
    /// `clut` - configuration structure
    /// `layer` - layer to affect
    fn clut(
        self: Pin<&mut Self>,
        clut: &display_clut_cfg_t,
        layer: display_frame_layer_t,
    ) -> Result<()>;
    /// overwrite one CLUT entry in the display device
    ///
    /// `layer` - layer to affect
    /// `index` - CLUT element index
    /// `color` - desired color code
    fn clut_edit(
        self: Pin<&mut Self>,
        layer: display_frame_layer_t,
        index: u8,
        color: u32,
    ) -> Result<()>;
    /// configure color keying
    ///
    /// `cfg` - color keying configuration
    /// `layer` - layer to affect
    fn color_key_set(
        self: Pin<&mut Self>,
        cfg: display_colorkeying_layer_t,
        layer: display_frame_layer_t,
    ) -> Result<()>;
    /// get display device status
    fn status_get(self: Pin<&mut Self>) -> Result<display_status_t>;
}

impl<T> DisplayApi for T
where
    T: Block<Instance = display_instance_t>,
    T: Block<Api = display_api_t>,
    T: Block<State = state_markers::Opened>,
{
    fn start(self: Pin<&mut Self>) -> Result<()> {
        let ctrl = self.ctrl().cast::<c_void>();
        fsp_try_unsafe!((T::API.start.unwrap())(ctrl))
    }

    fn stop(self: Pin<&mut Self>) -> Result<()> {
        let ctrl = self.ctrl().cast::<c_void>();
        fsp_try_unsafe!((T::API.stop.unwrap())(ctrl))
    }

    fn layer_change(
        self: Pin<&mut Self>,
        // is this lifetime ok?
        cfg: &display_runtime_cfg_t,
        layer: display_frame_layer_t,
    ) -> Result<()> {
        let ctrl = self.ctrl();
        fsp_try_unsafe!((T::API.layerChange.unwrap())(ctrl, cfg, layer))
    }

    fn buffer_change(
        self: Pin<&mut Self>,
        framebuffer: *mut u8,
        layer: display_frame_layer_t,
    ) -> Result<()> {
        let ctrl = self.ctrl();
        fsp_try_unsafe!((T::API.bufferChange.unwrap())(ctrl, framebuffer, layer))
    }

    fn correction(self: Pin<&mut Self>, correction: &display_correction_t) -> Result<()> {
        let ctrl = self.ctrl();
        fsp_try_unsafe!((T::API.correction.unwrap())(ctrl, correction))
    }

    fn clut(
        self: Pin<&mut Self>,
        clut: &display_clut_cfg_t,
        layer: display_frame_layer_t,
    ) -> Result<()> {
        let ctrl = self.ctrl();
        fsp_try_unsafe!((T::API.clut.unwrap())(ctrl, clut, layer))
    }

    fn clut_edit(
        self: Pin<&mut Self>,
        layer: display_frame_layer_t,
        index: u8,
        color: u32,
    ) -> Result<()> {
        let ctrl = self.ctrl();
        fsp_try_unsafe!((T::API.clutEdit.unwrap())(ctrl, layer, index, color))
    }

    fn color_key_set(
        self: Pin<&mut Self>,
        cfg: display_colorkeying_layer_t,
        layer: display_frame_layer_t,
    ) -> Result<()> {
        let ctrl = self.ctrl();
        fsp_try_unsafe!((T::API.colorKeySet.unwrap())(ctrl, cfg, layer))
    }

    fn status_get(self: Pin<&mut Self>) -> Result<display_status_t> {
        let ctrl = self.ctrl();
        let mut status = MaybeUninit::zeroed();
        fsp_try_unsafe!((T::API.statusGet.unwrap())(ctrl, status.as_mut_ptr()))?;
        Ok(unsafe { status.assume_init() })
    }
}
