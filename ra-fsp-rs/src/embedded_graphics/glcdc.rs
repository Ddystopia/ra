use core::{
    mem::MaybeUninit,
    pin::Pin,
    sync::atomic::{AtomicU32, Ordering},
};

use diatomic_waker::{DiatomicWaker, WakeSinkRef, WakeSourceRef};
use embedded_graphics::prelude::*;
use ra_fsp_sys::generated::{
    display_event_t, display_frame_layer_t, display_in_format_t, e_display_event,
    e_fsp_err::FSP_ERR_INVALID_UPDATE_TIMING,
};

use crate::{
    Callback,
    display_api::{FrameBufferMut, bpp},
    glcdc::Glcdc,
    state_markers::Opened,
};

pub trait Kind {
    const RAW_FORMAT: display_in_format_t;
    const BPP: usize = bpp(Self::RAW_FORMAT) as usize;

    #[cfg(feature = "buoyant")]
    type Pixel: PixelColor
        + From<<Self::Pixel as PixelColor>::Raw>
        + buoyant::primitives::Interpolate
        + buoyant::color::AlphaColor;
 
    #[cfg(not(feature = "buoyant"))]
    type Pixel: PixelColor + From<<Self::Pixel as PixelColor>::Raw>;
}

mod bpp4;
mod bpp8;

/// Chroma-key filtering in the draw targets. With the `glcdc-chroma-filter`
/// feature off this folds to `false` and every filter branch compiles out,
/// so writes of the filter color reach the framebuffer like any other.
const FILTERING: bool = cfg!(feature = "glcdc-chroma-filter");

pub use bpp4::{Bpp4, Clut4Pixel};
pub use bpp8::{Bpp8, Clut8Pixel};

pub struct Display<K: Kind> {
    next_buffer: Option<FrameBufferMut>,
    size: Size,
    hstride_bytes: usize,
    filter: K::Pixel,
    layer: display_frame_layer_t,
    synchronizer_rx: WakeSinkRef<'static>,
    // counter: &'static AtomicU32,
}

pub struct GlcdcSynchronizer(DiatomicWaker, AtomicU32, MaybeUninit<GcldcDisplayCallback>);

struct GcldcDisplayCallback {
    source: WakeSourceRef<'static>,
    counter: &'static AtomicU32,
}

// todo: make generic over DisplayApi. Needs to create an approach for callbacks
impl<K: Kind> Display<K> {
    pub fn new(
        mut glcdc: Pin<&mut Glcdc<Opened>>,
        synchronizer: &'static mut GlcdcSynchronizer,
        layer: display_frame_layer_t,
        next_buffer: FrameBufferMut,
        filter: K::Pixel,
    ) -> crate::Result<Self> {
        let input = glcdc.cfg().input[layer as usize];
        let synchronizer_rx = synchronizer.0.sink_ref();
        let source = synchronizer_rx.source_ref();
        let callback = synchronizer.2.write(GcldcDisplayCallback {
            source,
            counter: &synchronizer.1,
        });
        assert_eq!(bpp(input.format) as usize, K::BPP);
        // fixme: it will overwrite existing callback from another layer
        glcdc.as_mut().set_callback(callback)?;
        Ok(Self {
            next_buffer: Some(next_buffer),
            size: Size::new(input.hsize as u32, input.vsize as u32),
            hstride_bytes: input.hstride as usize * K::BPP / 8,
            filter,
            layer,
            synchronizer_rx,
            // counter: &synchronizer.1,
        })
    }

    pub async fn flush_debounce(
        &mut self,
        mut glcdc: Pin<&mut Glcdc<Opened>>,
        mut debounce: impl AsyncFnMut(usize),
    ) -> crate::Result<()> {
        let mut i = 0;
        loop {
            match self.flush(glcdc.as_mut()).await {
                Ok(()) => return Ok(()),
                Err(e) if e == FSP_ERR_INVALID_UPDATE_TIMING => {
                    debounce(i).await;
                    i += 1;
                }
                Err(e) => return Err(e),
            }
        }
    }

    pub async fn flush(&mut self, mut glcdc: Pin<&mut Glcdc<Opened>>) -> crate::Result<()> {
        let Some(buffer) = self.next_buffer.take() else {
            // fixme: maybe return false?
            return Ok(());
        };

        match glcdc.as_mut().change_buffer(self.layer, buffer) {
            Ok(()) => (),
            Err((e, buffer)) => {
                self.next_buffer = Some(buffer);
                return Err(e);
            }
        }

        let next_buffer = self
            .synchronizer_rx
            .wait_until(|| glcdc.as_mut().take_buffer(self.layer))
            .await;

        self.next_buffer = Some(next_buffer);

        Ok(())
    }
}

impl GlcdcSynchronizer {
    pub const fn new() -> Self {
        Self(
            DiatomicWaker::new(),
            AtomicU32::new(0),
            MaybeUninit::uninit(),
        )
    }
}

impl Callback<display_event_t> for GcldcDisplayCallback {
    fn call(context: &Self, event: display_event_t) {
        if event == e_display_event::DISPLAY_EVENT_LINE_DETECTION {
            context.counter.fetch_add(1, Ordering::Relaxed);
            context.source.notify();
        }
    }
}

impl<K: Kind> OriginDimensions for Display<K> {
    fn size(&self) -> Size {
        self.size
    }
}
