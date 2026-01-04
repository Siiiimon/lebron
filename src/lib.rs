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
const IDLETOEEPY: &[u8] = include_bytes!("../assets/idle_to_eepy.bmp");
const EEPY: &[u8] = include_bytes!("../assets/eepy.bmp");

const IDLE_IMAGE: &[u8] = include_bytes!("../assets/face.bmp");

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

    startup_anim: Animation<Bmp<'static, Rgb565>>,
    blink_anim: Animation<Bmp<'static, Rgb565>>,
    idle_to_eepy_anim: Animation<Bmp<'static, Rgb565>>,
    eepy_anim: Animation<Bmp<'static, Rgb565>>,

    idle_image: Bmp<'static, Rgb565>,
}

impl App {
    pub fn new() -> Self {
        let startup = Bmp::<Rgb565>::from_slice(STARTUP_ANIM).unwrap();
        let startup_anim = animation::Animation::new(startup, 27);

        let blink = Bmp::<Rgb565>::from_slice(BLINK_ANIM).unwrap();
        let blink_anim = animation::Animation::new(blink, 9);

        let idle_to_eepy = Bmp::<Rgb565>::from_slice(IDLETOEEPY).unwrap();
        let idle_to_eepy_anim = animation::Animation::new(idle_to_eepy, 4);

        let eepy = Bmp::<Rgb565>::from_slice(EEPY).unwrap();
        let eepy_anim = animation::Animation::new(eepy, 5);

        let idle_image = Bmp::<Rgb565>::from_slice(IDLE_IMAGE).unwrap();

        Self {
            state: State::Startup,
            dice: Rng::new(),

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

                if self.dice.f32() < 0.001 {
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
