#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use embedded_hal_compat::ReverseCompat;
use esp_hal::{
    clock::CpuClock, delay::Delay, i2c::master::{Config as I2cConfig, I2c}, main, time::{Instant, Rate}, uart::{Uart, Config as UartConfig}
};
use esp_println::{print, println};
use lebron_firmware::display::new_display;

use lebron_core::{App, FRAME_BUDGET};
use lis3dh::Lis3dh;
use lis3dh::accelerometer::Accelerometer;
use log::{debug, error, info};

use embedded_io::Write;

pub struct AudioPlayer<'a> {
    uart: Uart<'a, esp_hal::Blocking>,
}

impl<'a> AudioPlayer<'a>
{
    pub fn new(uart: Uart<'a, esp_hal::Blocking>) -> Self {
        Self { uart }
    }

    pub fn reset(&mut self) {
        self.send_cmd(0x0C, 0);
    }

    fn send_cmd(&mut self, command: u8, param: u16) {
        let mut buffer = [0u8; 10];
        buffer[0] = 0x7E; // Start
        buffer[1] = 0xFF; // Ver
        buffer[2] = 0x06; // Len
        buffer[3] = command;
        buffer[4] = 0x00; // Feedback
        buffer[5] = (param >> 8) as u8; // Param High
        buffer[6] = (param & 0xFF) as u8; // Param Low
        buffer[9] = 0xEF; // End

        // Checksum
        let sum: u16 = (buffer[1] as u16).wrapping_add(buffer[2] as u16)
            .wrapping_add(buffer[3] as u16).wrapping_add(buffer[4] as u16)
            .wrapping_add(buffer[5] as u16).wrapping_add(buffer[6] as u16);
        let checksum = 0u16.wrapping_sub(sum);
        buffer[7] = (checksum >> 8) as u8;
        buffer[8] = (checksum & 0xFF) as u8;

        info!("sending audio command {:X?}", buffer);

        let _ = self.uart.write(&buffer);
    }

    pub fn listen_debug(&mut self) {
        println!("--- SNIFFER START ---");
        println!("Waiting for data from DFPlayer... (If this hangs, RX is dead)");

        let mut buffer = [0u8; 1];

        for i in 0..10 {
            match self.uart.read(&mut buffer) {
                Ok(_) => {
                    print!("{:02X} ", buffer[0]);
                },
                Err(e) => {
                    println!("Error: {:?}", e);
                    break;
                }
            }
        }
        println!("\n--- SNIFFER END ---");
    }

    pub fn play_track(&mut self, track: u16) {
        self.send_cmd(0x03, track);
    }

    pub fn set_volume(&mut self, vol: u16) {
        self.send_cmd(0x06, vol); // 0-30
    }
}

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

    print!("initializing...");
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    println!("DONE");

    print!("setting up display...");
    let mut display_buffer = [0_u8; 8192];
    let mut display = new_display(
        peripherals.GPIO10,
        peripherals.GPIO16,
        peripherals.GPIO15,
        peripherals.GPIO8,
        peripherals.SPI2,
        &mut display_buffer
    );
    println!("DONE");

    print!("instantiating I2C...");
    let i2c = I2c::new(
        peripherals.I2C0,
        I2cConfig::default()
            .with_frequency(Rate::from_khz(400)),
    )
        .unwrap()
        .with_sda(peripherals.GPIO5)
        .with_scl(peripherals.GPIO6);
    println!("DONE");

    print!("connecting accelerometer...");
    let i2c_legacy = i2c.reverse();
    let mut accelerometer = Lis3dh::new_i2c(i2c_legacy, lis3dh::SlaveAddr::Alternate).unwrap();
    accelerometer.set_range(lis3dh::Range::G8).unwrap();
    println!("DONE");

    print!("constructing app state...");
    let mut app = App::new();
    println!("DONE");

    print!("establishing uart connection to audio player...");
    let uart = Uart::new(peripherals.UART1, UartConfig::default().with_baudrate(9600))
        .unwrap()
        .with_tx(peripherals.GPIO17)
        .with_rx(peripherals.GPIO18);

    let mut audio_player = AudioPlayer::new(uart);
    println!("DONE");

    let delay = Delay::new();
    info!("Waiting for module to wake up");
    delay.delay_millis(2000);
    // audio_player.listen_debug();

    info!("Setting Volume");
    audio_player.set_volume(30); // Set volume (0 to 30)
    delay.delay_millis(100);

    info!("Playing Track 1");
    audio_player.play_track(1); // Plays /mp3/0001.mp3
    delay.delay_millis(100);

    println!("entering main loop");
    let mut frame_count: u32 = 0;
    loop {
        let frame_start = Instant::now();

        let accel_data = accelerometer.accel_norm();
        if frame_count % 30 == 0 {
            match &accel_data {
                Ok(data) => debug!("Accel: X={:.2} Y={:.2} Z={:.2}", data.x, data.y, data.z),
                Err(e) => error!("Accel Error: {:?}", e),
            }
        }

        app.update(accel_data.ok().map(|d| (d.x, d.y, d.z)));
        let _ = app.draw(&mut display);

        let elapsed = frame_start.elapsed().as_micros();
        if elapsed < FRAME_BUDGET {
            let wait = FRAME_BUDGET - elapsed;
            delay.delay_micros(wait as u32);
        }

        frame_count = frame_count.wrapping_add(1);
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v~1.0/examples
}
