use embedded_graphics::{
    pixelcolor::raw::RawU8,
    prelude::{DrawTarget, RawData, *},
    primitives::Rectangle,
};
use ra_fsp_sys::generated::display_in_format_t;

use crate::embedded_graphics::glcdc::{Display, Kind};

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
            if color == self.filter.0 {
                continue;
            }

            let v = color.into_inner();
            let (x, y) = (x as usize, y as usize);
            let Some(p) = buffer[..].get_mut(y * hstride + x) else {
                continue;
            };

            *p = v;
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

        buffer.fill(color.0.into_inner());

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
        let fill = color.0.into_inner();

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

            let (sx, ex) = (tl.x as usize, br.x as usize);

            row[sx..=ex].fill(fill);
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

        for y in tl.y..=br.y {
            let row = &mut buffer[(y as usize) * hstride..][..hstride];
            let (sx, ex) = (tl.x as usize, br.x as usize);
            for x in sx..=ex {
                let Some(v) = colors.next() else { break };
                if v == filter {
                    continue;
                }
                row[x] = v;
            }
        }

        Ok(())
    }
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
