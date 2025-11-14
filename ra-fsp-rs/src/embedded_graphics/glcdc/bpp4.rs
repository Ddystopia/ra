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

        let bounding_box = self.bounding_box();
        let area = area.intersection(&bounding_box);
        let buffer = match &mut self.next_buffer {
            Some(buf) => buf,
            None => return Ok(()),
        };
        let v = color.0.into_inner();
        let fill = (v << 4) | v;

        if area == bounding_box {
            buffer.fill(fill);
            return Ok(());
        }

        let hstride = self.hstride_bytes;
        let tl = area.top_left;
        let Some(br) = area.bottom_right() else {
            return Ok(());
        };

        for y in tl.y..=br.y {
            let row = &mut buffer[(y as usize) * hstride..][..hstride];

            let (mut sx, mut ex) = (tl.x as usize, br.x as usize);

            if sx % 2 == 1 {
                // x starts on high nibble
                let p = &mut row[sx * BPP / 8];
                write_shift(p, v, 4);
                // starting byte done, skip it
                sx += 1;
            }

            if ex % 2 == 0 {
                // x ends on low nibble
                let p = &mut row[ex * BPP / 8];
                write_shift(p, v, 0);
                // ending byte done, skip it
                ex -= 1;
            }

            if let Some(slice) = row.get_mut(sx * BPP / 8..=ex * BPP / 8) {
                slice.fill(fill);
            }
        }

        Ok(())
    }

    fn fill_contiguous<I>(&mut self, area: &Rectangle, colors: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Self::Color>,
    {
        let bounding_box = self.bounding_box();
        let area = area.intersection(&bounding_box);
        if area.is_zero_sized() {
            return Ok(());
        }
        let buffer = match &mut self.next_buffer {
            Some(buf) => buf,
            None => return Ok(()),
        };

        let mut colors = colors.into_iter().map(|c| c.0.into_inner());
        let filter = self.filter.0.into_inner();
        let hstride = self.hstride_bytes;
        let tl = area.top_left;
        let Some(br) = area.bottom_right() else {
            return Ok(());
        };

        if tl.x == br.x && tl.y == br.y {
            let shift = 4 * (tl.x % 2) as u8;
            let i = tl.x as usize * BPP / 8;
            let Some(v) = colors.next() else {
                return Ok(());
            };
            if v == filter {
                return Ok(());
            }
            let p = &mut buffer[(tl.y as usize) * hstride..][i];
            write_shift(p, v, shift as usize);
            return Ok(());
        }

        if tl.x == br.x {
            let i = tl.x as usize * BPP / 8;
            let shift = 4 * (tl.x % 2) as u8;
            for y in tl.y..=br.y {
                let Some(v) = colors.next() else { break };
                if v == filter {
                    continue;
                }
                let p = &mut buffer[(y as usize) * hstride..][i];
                write_shift(p, v, shift as usize);
            }
            return Ok(());
        }

        assert!(tl.x < br.x);

        // everything after that is 2ms in total

        for y in tl.y..=br.y {
            let row_start_at = (y as usize) * hstride;
            let row = &mut buffer[row_start_at..][..hstride];

            let mut sx = tl.x as usize;
            let mut ex = br.x as usize;

            // ensure sx is even
            if sx % 2 == 1 {
                let Some(v) = colors.next() else { break };
                if v != filter {
                    write_left(&mut row[sx * BPP / 8], v);
                }
                sx += 1;
            }

            let mut last = None;

            // ensure ex is odd
            if ex % 2 == 0 {
                last = Some(ex);
                ex -= 1;
            }

            let start = sx * BPP / 8;
            let len_bytes = (ex - sx + 1) * BPP / 8;
            fill_by_two(&mut row[start..][..len_bytes], &mut colors, filter);

            // let mut start = sx * BPP / 8;
            // let mut len_bytes = (ex - sx + 1) * BPP / 8;
            // if len_bytes % 16 != 0 {
            //     let cut = len_bytes % 16;
            //     fill_by_two(&mut row[start..][..cut], &mut colors, filter);
            //     len_bytes -= cut;
            //     start += cut;
            // }
            //
            // if len_bytes != 0 {
            //     let start = row_start_at + start;
            //     let u32_buf = &mut buffer.as_u32_mut()[start / 4..][..len_bytes / 4];
            //     fill_by_eigth(u32_buf, &mut colors, filter);
            // }

            if let Some(ex) = last {
                let row = &mut buffer[(y as usize) * hstride..][..hstride];
                let Some(v) = colors.next() else { break };
                if v != filter {
                    write_right(&mut row[ex * BPP / 8], v);
                }
            }
        }
        Ok(())
    }
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

impl buoyant::primitives::Interpolate for Clut4Pixel {
    fn interpolate(from: Self, to: Self, amount: u8) -> Self {
        if amount == 0 { from } else { to }
    }
}

impl buoyant::color::AlphaColor for Clut4Pixel {}
