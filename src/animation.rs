use embedded_graphics::prelude::*;
use embedded_graphics::{image::{Image, ImageDrawable, ImageDrawableExt}, primitives::Rectangle};

use crate::{WIDTH, HEIGHT};

pub struct Animation<T>
    where T: ImageDrawable
{
    atlas: T,
    frame_count: u32,
    current_frame_index: u32,
}

impl<T> Animation<T>
    where T: ImageDrawable,
{
    pub fn new(atlas: T, frame_count: u32) -> Self {
        Self {
            atlas,
            frame_count,
            current_frame_index: 0,
        }
    }

    pub fn tick(&mut self) -> bool {
        self.current_frame_index += 1;
        if self.current_frame_index >= self.frame_count {
            self.current_frame_index = 0;
            return true;
        }
        false
    }

    pub fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = T::Color>,
    {
        let y = (self.current_frame_index as i32) * (HEIGHT as i32);

        let area = Rectangle::new(
            Point::new(0, y),
            Size::new(WIDTH, HEIGHT)
        );

        let frame_view = self.atlas.sub_image(&area);

        Image::new(&frame_view, Point::zero()).draw(display)
    }
}
