use embedded_graphics::{pixelcolor::Rgb565, prelude::DrawTarget};
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_hal::{delay::Delay, gpio::{Level, Output, OutputConfig, OutputPin, interconnect::PeripheralOutput}, spi::{Mode, master::{Config, Instance, Spi}}, time::Rate};
use lebron_core::{HEIGHT, WIDTH};
use mipidsi::{Builder, interface::SpiInterface, models::ST7789, options::ColorInversion};

pub fn new_display(
    // fixme: this shouldn't take specific gpios but just a general PeripheralOutput, but traits
    // are hard :c
    cs: impl PeripheralOutput<'static> + OutputPin + 'static,
    clk: impl PeripheralOutput<'static> + OutputPin + 'static,
    mosi: impl PeripheralOutput<'static> + OutputPin + 'static,
    dc: impl PeripheralOutput<'static> + OutputPin + 'static,
    spi: impl Instance + 'static,
    buffer: &mut [u8]) -> impl DrawTarget<Color = Rgb565>
{
    let spi_bus = Spi::new(
        spi,
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

    let di = SpiInterface::new(spi_device, dc_output, buffer);

    let mut delay = Delay::new();

    Builder::new(ST7789, di)
        .invert_colors(ColorInversion::Inverted)
        .display_size(WIDTH as u16, HEIGHT as u16)
        .init(&mut delay)
        .unwrap()
}
