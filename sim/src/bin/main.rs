use std::thread;
use std::time::Duration;

use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
};
use embedded_graphics_simulator::{SimulatorDisplay, SimulatorEvent, Window, OutputSettings};
use lebron_core::App;
use lebron_core::WIDTH;
use lebron_core::HEIGHT;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(WIDTH, HEIGHT));

    let mut window = Window::new("Lebron", &OutputSettings::default());

    let mut app = App::new();

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

        thread::sleep(Duration::from_millis(33));
    }

    Ok(())
}
