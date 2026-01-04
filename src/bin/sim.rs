use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
};
use embedded_graphics_simulator::{SimulatorDisplay, SimulatorEvent, Window, OutputSettings};
use lebron::WIDTH;
use lebron::HEIGHT;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(WIDTH, HEIGHT));

    let mut window = Window::new("Lebron", &OutputSettings::default());

    let mut app = lebron::App::new();

    'running: loop {
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                _ => {},
            }
        }


        app.update();
        app.draw(&mut display)?;
    }

    Ok(())
}
