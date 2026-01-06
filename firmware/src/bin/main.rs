#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use embedded_hal_bus::spi::ExclusiveDevice;
use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    gpio::{Level, Output, OutputConfig},
    main,
    spi::{
        Mode,
        master::{Config, Spi},
    },
    time::Rate,
};
use mipidsi::Builder;
use mipidsi::interface::SpiInterface;
use mipidsi::models::ST7789;
use mipidsi::options::ColorInversion;

use lebron_core::{App, HEIGHT, WIDTH};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    // generator version: 1.1.0

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let cs = peripherals.GPIO10;
    let clk = peripherals.GPIO12;
    let mosi = peripherals.GPIO11;
    let dc = peripherals.GPIO8;

    let spi_bus = Spi::new(
        peripherals.SPI2,
        Config::default()
            .with_frequency(Rate::from_mhz(80))
            .with_mode(Mode::_0),
    )
    .unwrap()
    .with_sck(clk)
    .with_mosi(mosi);

    let cs_output = Output::new(cs, Level::High, OutputConfig::default());
    let dc_output = Output::new(dc, Level::High, OutputConfig::default());

    let spi_device = ExclusiveDevice::new_no_delay(spi_bus, cs_output).unwrap();

    let mut buffer = [0_u8; 8192];

    let di = SpiInterface::new(spi_device, dc_output, &mut buffer);

    let mut delay = Delay::new();

    let mut display = Builder::new(ST7789, di)
        .invert_colors(ColorInversion::Inverted)
        .display_size(WIDTH as u16, HEIGHT as u16)
        .init(&mut delay)
        .unwrap();

    let mut app = App::new();

    loop {
        app.update();
        app.draw(&mut display).unwrap();
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v~1.0/examples
}
