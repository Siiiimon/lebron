use linux_embedded_hal::spidev::{SpiModeFlags, SpidevOptions};
use linux_embedded_hal::{SpidevDevice, Delay};
use mipidsi::interface::SpiInterface;
use mipidsi::{Builder, models::ST7789};
use rppal::gpio::Gpio;

fn main() {
    let mut spi = SpidevDevice::open("/dev/spidev0.1").unwrap();

    let spi_options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(60_000_000)
        .mode(SpiModeFlags::SPI_MODE_3)
        .build();

    spi.configure(&spi_options).unwrap();

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

    let mut app = lebron::App::new();

    // draw something
    loop {
        app.update();
        app.draw(&mut display).unwrap();
    }
}
