#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use esp_hal::{
    clock::CpuClock, delay::Delay, main, time::Instant
};
use lebron_firmware::display::new_display;

use lebron_core::{App, FRAME_BUDGET};

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

    let mut display_buffer = [0_u8; 8192];
    let mut display = new_display(
        peripherals.GPIO10,
        peripherals.GPIO16,
        peripherals.GPIO15,
        peripherals.GPIO8,
        peripherals.SPI2,
        &mut display_buffer
    );

    let mut app = App::new();

    let delay = Delay::new();
    loop {
        let frame_start = Instant::now();
        app.update();
        let _ = app.draw(&mut display);

        let elapsed = frame_start.elapsed().as_micros();
        if elapsed < FRAME_BUDGET {
            let wait = FRAME_BUDGET - elapsed;
            delay.delay_micros(wait as u32);
        }
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v~1.0/examples
}
