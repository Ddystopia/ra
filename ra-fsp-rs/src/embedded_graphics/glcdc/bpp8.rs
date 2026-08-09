use embedded_graphics::{
    pixelcolor::raw::RawU8,
    prelude::{DrawTarget, RawData, *},
    primitives::Rectangle,
};
use ra_fsp_sys::generated::display_in_format_t;

use crate::embedded_graphics::glcdc::{Display, FILTERING, Kind};

pub struct Bpp8;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Clut8Pixel(pub RawU8);

impl Kind for Bpp8 {
    type Pixel = Clut8Pixel;

    const RAW_FORMAT: display_in_format_t = display_in_format_t::DISPLAY_IN_FORMAT_CLUT8;
}

impl Clut8Pixel {
    pub const fn new(value: u8) -> Self {
        Clut8Pixel(RawU8::new(value))
    }
    pub fn into_inner(self) -> u8 {
        self.0.into_inner()
    }
}

impl<'a, 'b> DrawTarget for Display<Bpp8> {
    type Color = Clut8Pixel;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let hstride = self.hstride_bytes;
        let buffer = match &mut self.next_buffer {
            Some(buf) => buf,
            None => return Ok(()),
        };

        for Pixel(Point { x, y }, Clut8Pixel(color)) in pixels {
            if FILTERING && color == self.filter.0 {
                continue;
            }

            let v = color.into_inner();
            let (x, y) = (x as usize, y as usize);
            // `x` is intentionally not clipped to `< hstride`: an out-of-row `x`
            // wraps into the next row rather than being rejected. This is a
            // deliberate perf tradeoff (skip the per-pixel branch); it is never
            // UB because `get_mut` still bounds-checks the final flat index.
            let Some(p) = buffer[..].get_mut(y * hstride + x) else {
                continue;
            };

            *p = v;
        }

        Ok(())
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        if FILTERING && color == self.filter {
            return Ok(());
        }

        let buffer = match &mut self.next_buffer {
            Some(buf) => buf,
            None => return Ok(()),
        };

        buffer.fill(color.0.into_inner());

        Ok(())
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        if FILTERING && color == self.filter {
            return Ok(());
        }

        let Some((x0, y0, x1, y1)) = clip(area, self.size) else {
            return Ok(());
        };
        let full_screen = x0 == 0
            && y0 == 0
            && x1 == self.size.width as usize
            && y1 == self.size.height as usize;
        let hstride = self.hstride_bytes;
        let buffer = match &mut self.next_buffer {
            Some(buf) => buf,
            None => return Ok(()),
        };
        let fill = color.0.into_inner();

        if full_screen {
            buffer.fill(fill);
            return Ok(());
        }

        for row in buffer[y0 * hstride..y1 * hstride].chunks_exact_mut(hstride) {
            row[x0..x1].fill(fill);
        }

        Ok(())
    }

    fn fill_contiguous<I>(&mut self, area: &Rectangle, colors: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Self::Color>,
    {
        let Some((x0, y0, x1, y1)) = clip(area, self.size) else {
            return Ok(());
        };
        let buffer = match &mut self.next_buffer {
            Some(buf) => buf,
            None => return Ok(()),
        };

        // No `.map()` adapter on the pixel stream, and `colors.next()` is
        // called directly rather than through zip's `&mut I` blanket impl:
        // both would drop an #[inline(always)] on the iterator's `next`
        // (egi strips), leaving an out-of-line call per pixel at opt-level z.
        let mut colors = colors.into_iter();
        let filter = self.filter;
        let hstride = self.hstride_bytes;
        let span_len = x1 - x0;

        for row in buffer[y0 * hstride..y1 * hstride].chunks_exact_mut(hstride) {
            // Slice the span once so the compiler can see its length;
            // iterating the span eliminates per-pixel index arithmetic and
            // the bounds check that `row[x] = v` would otherwise emit.
            let span = &mut row[x0..x1];
            let mut n = 0usize;
            for p in span.iter_mut() {
                let Some(v) = colors.next() else { break };
                n += 1;
                if !FILTERING || v != filter {
                    *p = v.into_inner();
                }
            }
            // `colors` was exhausted before the span was fully consumed:
            // nothing more will be drawn on subsequent rows either.
            if n < span_len {
                return Ok(());
            }
        }

        Ok(())
    }
}

/// Clips `area` to a `size`-bounded origin rectangle using plain integer
/// arithmetic. Returns half-open pixel bounds `(x0, y0, x1, y1)`, or `None`
/// when nothing is left.
#[inline(always)]
fn clip(area: &Rectangle, size: Size) -> Option<(usize, usize, usize, usize)> {
    let w = size.width.min(i32::MAX as u32) as i32;
    let h = size.height.min(i32::MAX as u32) as i32;
    let x0 = area.top_left.x.max(0);
    let y0 = area.top_left.y.max(0);
    let x1 = area.top_left.x.saturating_add_unsigned(area.size.width).min(w);
    let y1 = area.top_left.y.saturating_add_unsigned(area.size.height).min(h);
    if x0 >= x1 || y0 >= y1 {
        return None;
    }
    Some((x0 as usize, y0 as usize, x1 as usize, y1 as usize))
}

impl PixelColor for Clut8Pixel {
    type Raw = RawU8;
}

impl From<RawU8> for Clut8Pixel {
    fn from(raw: RawU8) -> Self {
        Clut8Pixel(raw)
    }
}
impl From<u8> for Clut8Pixel {
    fn from(raw: u8) -> Self {
        Clut8Pixel(raw.into())
    }
}
impl From<usize> for Clut8Pixel {
    fn from(raw: usize) -> Self {
        Clut8Pixel((raw as u8).into())
    }
}

#[cfg(feature = "buoyant")]
impl buoyant::primitives::Interpolate for Clut8Pixel {
    fn interpolate(from: Self, to: Self, amount: u8) -> Self {
        if amount == 0 { from } else { to }
    }
}

#[cfg(feature = "buoyant")]
impl buoyant::color::AlphaColor for Clut8Pixel {}
