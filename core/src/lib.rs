#![no_std]
use embedded_graphics::prelude::*;
use embedded_graphics::pixelcolor::Rgb565;
use fastrand::Rng;
use tinytga::Tga;

use crate::animation::Animation;

mod animation;

pub const WIDTH: u32 = 240;
pub const HEIGHT: u32 = 240;
pub const TARGET_FPS: u64 = 30;
pub const FRAME_BUDGET: u64 = 1_000_000 / TARGET_FPS;

const STARTUP_ANIM: &[u8] = include_bytes!("../../assets/startup.tga");
const BLINK_ANIM: &[u8] = include_bytes!("../../assets/idle.tga");
const IDLE_TO_EEPY_ANIM: &[u8] = include_bytes!("../../assets/idle_to_eepy.tga");
const EEPY_ANIM: &[u8] = include_bytes!("../../assets/eepy.tga");

const IDLE_IMAGE: &[u8] = include_bytes!("../../assets/face.tga");

const EEPY_TIMEOUT: u32 = 200;

pub enum State {
    Startup,

    Idle,
    Blink,

    IdleToEepy,
    Eepy,
}

pub struct App {
    state: crate::State,
    dice: Rng,
    eepy_timer: u32,

    startup_anim: Animation<Tga<'static, Rgb565>>,
    blink_anim: Animation<Tga<'static, Rgb565>>,
    idle_to_eepy_anim: Animation<Tga<'static, Rgb565>>,
    eepy_anim: Animation<Tga<'static, Rgb565>>,

    idle_image: Tga<'static, Rgb565>,
}

impl App {
    pub fn new() -> Self {
        let startup = Tga::<Rgb565>::from_slice(STARTUP_ANIM).unwrap();
        let startup_anim = animation::Animation::new(startup, 27);

        let blink = Tga::<Rgb565>::from_slice(BLINK_ANIM).unwrap();
        let blink_anim = animation::Animation::new(blink, 9);

        let idle_to_eepy = Tga::<Rgb565>::from_slice(IDLE_TO_EEPY_ANIM).unwrap();
        let idle_to_eepy_anim = animation::Animation::new(idle_to_eepy, 5);

        let eepy = Tga::<Rgb565>::from_slice(EEPY_ANIM).unwrap();
        let eepy_anim = animation::Animation::new(eepy, 16);

        let idle_image = Tga::<Rgb565>::from_slice(IDLE_IMAGE).unwrap();

        Self {
            state: State::Startup,
            dice: Rng::with_seed(12345),
            eepy_timer: 0,

            startup_anim,
            blink_anim,
            idle_to_eepy_anim,
            eepy_anim,

            idle_image
        }
    }

    pub fn update(&mut self) {
        match self.state {
            State::Startup => {
                if self.startup_anim.tick() {
                    self.state = State::Idle;
                    self.eepy_timer = EEPY_TIMEOUT;
                }
            },
            State::Blink => {
                if self.blink_anim.tick() {
                    self.state = State::Idle;
                }
            },
            State::IdleToEepy => {
                if self.idle_to_eepy_anim.tick() {
                    self.state = State::Eepy;
                }
            },
            State::Eepy => {
                self.eepy_anim.tick();
            }
            State::Idle => {
                if self.dice.f32() < 0.01 {
                    self.state = State::Blink;
                }

                self.eepy_timer -= 1;
                if self.eepy_timer == 0 {
                    self.state = State::IdleToEepy;
                }
            },
        }
    }

    pub fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        match self.state {
            State::Startup => self.startup_anim.draw(display)?,
            State::Blink => self.blink_anim.draw(display)?,
            State::IdleToEepy => self.idle_to_eepy_anim.draw(display)?,
            State::Eepy => self.eepy_anim.draw(display)?,
            State::Idle => self.idle_image.draw(display)?,
        }

        Ok(())
    }
}
