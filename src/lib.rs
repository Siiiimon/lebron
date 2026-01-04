use embedded_graphics::prelude::*;
use embedded_graphics::pixelcolor::Rgb565;
use fastrand::Rng;
use tinybmp::Bmp;

use crate::animation::Animation;

mod animation;

pub const WIDTH: u32 = 240;
pub const HEIGHT: u32 = 240;

const STARTUP_ANIM: &[u8] = include_bytes!("../assets/startup.bmp");
const BLINK_ANIM: &[u8] = include_bytes!("../assets/idle.bmp");

const IDLE_IMAGE: &[u8] = include_bytes!("../assets/face.bmp");

pub enum State {
    STARTUP,
    IDLE,
    BLINK,
}

pub struct App {
    state: crate::State,
    dice: Rng,

    startup_anim: Animation<Bmp<'static, Rgb565>>,
    blink_anim: Animation<Bmp<'static, Rgb565>>,

    idle_image: Bmp<'static, Rgb565>,
}

impl App {
    pub fn new() -> Self {
        let startup = Bmp::<Rgb565>::from_slice(STARTUP_ANIM).unwrap();
        let startup_anim = animation::Animation::new(startup, 27);

        let blink = Bmp::<Rgb565>::from_slice(BLINK_ANIM).unwrap();
        let blink_anim = animation::Animation::new(blink, 9);

        let idle_image = Bmp::<Rgb565>::from_slice(IDLE_IMAGE).unwrap();

        Self {
            state: State::STARTUP,
            dice: Rng::new(),

            startup_anim,
            blink_anim,

            idle_image
        }
    }

    pub fn update(&mut self) {
        match self.state {
            State::STARTUP => {
                if self.startup_anim.tick() {
                    self.state = State::IDLE;
                }
            },
            State::BLINK => {
                if self.blink_anim.tick() {
                    self.state = State::IDLE;
                }
            }
            State::IDLE => {
                if self.dice.f32() < 0.01 {
                    self.state = State::BLINK;
                }
            }
        }
    }

    pub fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        match self.state {
            State::STARTUP => self.startup_anim.draw(display)?,
            State::BLINK => self.blink_anim.draw(display)?,
            State::IDLE => self.idle_image.draw(display)?,
        }

        Ok(())
    }
}
