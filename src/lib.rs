use embedded_graphics::{image::Image, pixelcolor::Rgb565, prelude::*};
use tinybmp::Bmp;

pub const WIDTH: u32 = 240;
pub const HEIGHT: u32 = 240;

const FACE_DATA: &[u8] = include_bytes!("../assets/face.bmp");

pub fn draw_face<D>(display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565>,
{
    let bmp = Bmp::<Rgb565>::from_slice(FACE_DATA).unwrap();

    Image::new(&bmp, Point::new(0, 0)).draw(display)?;

    Ok(())
}

pub fn draw<D>(display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565>,
{
    // #EC91B4
    display.clear(Rgb565::new(0xEC, 0x91, 0xB4))?;
    draw_face(display)?;
    Ok(())
}
