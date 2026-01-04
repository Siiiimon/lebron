use std::thread;
use std::time::Duration;

use linux_embedded_hal::{SpidevDevice, Delay};
use mipidsi::interface::SpiInterface;
use mipidsi::{Builder, models::ST7789};
use rppal::gpio::Gpio;

fn main() {
    let spi = SpidevDevice::open("/dev/spidev0.1").unwrap();

    let gpio = Gpio::new().unwrap();

    let dc = gpio.get(9).unwrap().into_output();

    println!("SUCCESS: Hardware is ready for the screen driver.");

    let mut bl = gpio.get(19).unwrap().into_output();
    bl.set_high();

    let mut buffer = [0_u8; 512];

    // Create a DisplayInterface from SPI and DC pin, with no manual CS control
    let di = SpiInterface::new(spi, dc, &mut buffer);

    // Create the ST7789 display driver from the display interface and optional RST pin
    let mut delay = Delay;
    let mut display = Builder::new(ST7789, di)
        .invert_colors(mipidsi::options::ColorInversion::Inverted)
        .display_size(240, 240)
        .init(&mut delay).unwrap();

    // draw something
    loop {
        lebron::draw(&mut display).unwrap();
        thread::sleep(Duration::from_millis(500));
    }
}
