use embedded_graphics::{pixelcolor::Rgb565, prelude::*, primitives::{Circle, Ellipse, PrimitiveStyle}};

pub const WIDTH: u32 = 240;
pub const HEIGHT: u32 = 240;

pub fn draw_face<D>(display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565>,
{
    let side_margin = 60;
    Ellipse::new(Point::new(side_margin, 20), Size::new(30, 120))
        .into_styled(PrimitiveStyle::with_fill(Rgb565::BLACK))
        .draw(display)?;

    Ellipse::new(Point::new(WIDTH as i32 - 30 - side_margin, 20), Size::new(30, 120))
        .into_styled(PrimitiveStyle::with_fill(Rgb565::BLACK))
        .draw(display)?;

    Ellipse::new(Point::new(side_margin + 5, 20 + 10), Size::new(20, 60))
        .into_styled(PrimitiveStyle::with_fill(Rgb565::WHITE))
        .draw(display)?;

    Ellipse::new(Point::new(WIDTH as i32 - 30 - side_margin + 5, 20 + 10), Size::new(20, 60))
        .into_styled(PrimitiveStyle::with_fill(Rgb565::WHITE))
        .draw(display)?;

    Circle::new(Point::new( (WIDTH as i32 / 2) - 5, HEIGHT as i32 - 60), 10)
        .into_styled(PrimitiveStyle::with_fill(Rgb565::MAGENTA))
        .draw(display)?;

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
