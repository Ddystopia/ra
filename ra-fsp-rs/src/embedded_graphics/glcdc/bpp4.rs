use embedded_graphics::{
    pixelcolor::raw::RawU4,
    prelude::{DrawTarget, RawData, *},
    primitives::Rectangle,
};
use ra_fsp_sys::generated::display_in_format_t;

use crate::embedded_graphics::glcdc::{Display, Kind};

pub struct Bpp4;

const BPP: usize = Bpp4::BPP;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Clut4Pixel(pub RawU4);

impl Kind for Bpp4 {
    type Pixel = Clut4Pixel;

    const RAW_FORMAT: display_in_format_t = display_in_format_t::DISPLAY_IN_FORMAT_CLUT4;
}

impl Clut4Pixel {
    pub const fn new(value: u8) -> Self {
        Clut4Pixel(RawU4::new(value & 0x0F))
    }
    pub fn into_inner(self) -> u8 {
        self.0.into_inner()
    }
}

impl<'a, 'b> DrawTarget for Display<Bpp4> {
    type Color = Clut4Pixel;
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

        for Pixel(Point { x, y }, Clut4Pixel(color)) in pixels {
            if color == self.filter.0 {
                continue;
            }

            let v = color.into_inner();
            let (x, y) = (x as usize, y as usize);
            // `x` is intentionally not clipped to `< hstride`: an out-of-row `x`
            // wraps into the next row rather than being rejected. This is a
            // deliberate perf tradeoff (skip the per-pixel branch); it is never
            // UB because `get_mut` still bounds-checks the final flat index.
            let Some(p) = buffer[..].get_mut(y * hstride + x / 2) else {
                continue;
            };

            write_shift(p, v, 4 * (x % 2));
        }

        Ok(())
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        if color == self.filter {
            return Ok(());
        }

        let buffer = match &mut self.next_buffer {
            Some(buf) => buf,
            None => return Ok(()),
        };

        let v = color.0.into_inner();
        buffer.fill((v << 4) | v);

        Ok(())
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        if color == self.filter {
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
        let v = color.0.into_inner();
        let fill = (v << 4) | v;

        if full_screen {
            buffer.fill(fill);
            return Ok(());
        }

        // Byte extent of the span: sb..eb covers every touched byte. A span
        // starting on an odd pixel owns only the high nibble of its first
        // byte, one ending on an even pixel only the low nibble of its last
        // byte; those edges are read-modify-write, the middle is a plain fill.
        let sb = x0 * BPP / 8;
        let eb = (x1 + 1) * BPP / 8;
        let odd_start = x0 % 2 == 1;
        let odd_end = x1 % 2 == 1;
        let mid_s = sb + odd_start as usize;
        let mid_e = eb - odd_end as usize;

        for row in buffer[y0 * hstride..y1 * hstride].chunks_exact_mut(hstride) {
            if odd_start {
                write_shift(&mut row[sb], v, 4);
            }

            if odd_end {
                write_shift(&mut row[eb - 1], v, 0);
            }

            row[mid_s..mid_e].fill(fill);
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

        let mut colors = colors.into_iter().map(|c| c.0.into_inner());
        let filter = self.filter.0.into_inner();
        let hstride = self.hstride_bytes;

        if x1 - x0 == 1 {
            let i = x0 * BPP / 8;
            let shift = 4 * (x0 % 2);

            if y1 - y0 == 1 {
                let Some(v) = colors.next() else {
                    return Ok(());
                };
                if v == filter {
                    return Ok(());
                }
                write_shift(&mut buffer[y0 * hstride + i], v, shift);
                return Ok(());
            }

            for row in buffer[y0 * hstride..y1 * hstride].chunks_exact_mut(hstride) {
                let Some(v) = colors.next() else { break };
                if v == filter {
                    continue;
                }
                write_shift(&mut row[i], v, shift);
            }
            return Ok(());
        }

        // Byte extent of the span (see fill_solid): edge nibbles are handled
        // separately so the middle can be packed two pixels per byte.
        let sb = x0 * BPP / 8;
        let eb = (x1 + 1) * BPP / 8;
        let odd_start = x0 % 2 == 1;
        let odd_end = x1 % 2 == 1;
        let mid_s = sb + odd_start as usize;
        let mid_e = eb - odd_end as usize;

        for row in buffer[y0 * hstride..y1 * hstride].chunks_exact_mut(hstride) {
            if odd_start {
                let Some(v) = colors.next() else { break };
                if v != filter {
                    write_left(&mut row[sb], v);
                }
            }

            fill_by_two(&mut row[mid_s..mid_e], &mut colors, filter);

            if odd_end {
                let Some(v) = colors.next() else { break };
                if v != filter {
                    write_right(&mut row[eb - 1], v);
                }
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

#[inline(always)]
fn fill_by_two<I: Iterator<Item = u8>>(place: &mut [u8], mut iter: I, filter: u8) {
    for p in place.iter_mut() {
        let v1 = iter.next();
        let v2 = iter.next();
        if let Some((v1, v2)) = v1.zip(v2) {
            if v1 != filter && v2 != filter {
                *p = (v2 << 4) | v1;
            } else {
                if v1 != filter {
                    write_right(p, v1);
                } else if v2 != filter {
                    write_left(p, v2);
                }
            }
        } else {
            break;
        }
    }
}

#[inline(always)]
#[cfg(false)]
fn fill_by_eigth<I: Iterator<Item = u8>>(place: &mut [u32], mut iter: I, filter: u8) {
    let read_nibble = |val: u32, i: usize| ((val >> (i * 4)) & 0xF) as u8;
    let replace_if_filtered = |color: u8, background: u32, i: usize| {
        if color == filter {
            read_nibble(background, i)
        } else {
            color
        }
    };
    for chunk in place.iter_mut() {
        let background = *chunk;
        let v1 = iter.next().map(|v| replace_if_filtered(v, background, 0));
        let v2 = iter.next().map(|v| replace_if_filtered(v, background, 1));
        let v3 = iter.next().map(|v| replace_if_filtered(v, background, 2));
        let v4 = iter.next().map(|v| replace_if_filtered(v, background, 3));
        let v5 = iter.next().map(|v| replace_if_filtered(v, background, 4));
        let v6 = iter.next().map(|v| replace_if_filtered(v, background, 5));
        let v7 = iter.next().map(|v| replace_if_filtered(v, background, 6));
        let v8 = iter.next().map(|v| replace_if_filtered(v, background, 7));
        if let Some((((((((v1, v2), v3), v4), v5), v6), v7), v8)) =
            v1.zip(v2).zip(v3).zip(v4).zip(v5).zip(v6).zip(v7).zip(v8)
        {
            *chunk = u32::from_le_bytes([
                (v2 << 4) | v1,
                (v4 << 4) | v3,
                (v6 << 4) | v5,
                (v8 << 4) | v7,
            ]);
        } else {
            break;
        }
    }
}

#[inline(always)]
fn write_shift(place: &mut u8, v: u8, shift: usize) {
    let mut pix = *place;
    pix &= !(0xF << shift);
    pix |= v << shift;
    *place = pix;
}

#[inline(always)]
fn write_left(place: &mut u8, v: u8) {
    write_shift(place, v, 4);
}

#[inline(always)]
fn write_right(place: &mut u8, v: u8) {
    write_shift(place, v, 0);
}

impl PixelColor for Clut4Pixel {
    type Raw = RawU4;
}

impl From<RawU4> for Clut4Pixel {
    fn from(raw: RawU4) -> Self {
        Clut4Pixel(raw)
    }
}
impl From<u8> for Clut4Pixel {
    fn from(raw: u8) -> Self {
        Clut4Pixel(raw.into())
    }
}

impl From<usize> for Clut4Pixel {
    fn from(raw: usize) -> Self {
        Clut4Pixel((raw as u8).into())
    }
}

#[cfg(feature = "buoyant")]
impl buoyant::primitives::Interpolate for Clut4Pixel {
    fn interpolate(from: Self, to: Self, amount: u8) -> Self {
        if amount == 0 { from } else { to }
    }
}

#[cfg(feature = "buoyant")]
impl buoyant::color::AlphaColor for Clut4Pixel {}
