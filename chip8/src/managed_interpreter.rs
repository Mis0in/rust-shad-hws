use crate::{
    data::Word,
    error::Result,
    image::Image,
    interpreter::{Interpreter, SCREEN_HEIGHT, SCREEN_WIDTH},
    platform::{Key, Platform, Point, Sprite},
};

use core::time::Duration;
////////////////////////////////////////////////////////////////////////////////

pub struct FrameBuffer([[bool; SCREEN_WIDTH]; SCREEN_HEIGHT]);

impl Default for FrameBuffer {
    fn default() -> Self {
        Self([[false; SCREEN_WIDTH]; SCREEN_HEIGHT])
    }
}

impl FrameBuffer {
    pub fn iter_rows(&self) -> impl Iterator<Item = &[bool; SCREEN_WIDTH]> {
        self.0.iter()
    }

    pub fn get(&self, i: usize, j: usize) -> bool {
        self.0[i][j]
    }

    pub fn change(&mut self, x: usize, y: usize) {
        self.0[y][x] ^= true;
    }
}

////////////////////////////////////////////////////////////////////////////////

pub trait RandomNumberGenerator: FnMut() -> Word {}

impl<R: FnMut() -> Word> RandomNumberGenerator for R {}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
struct ManagedPlatform<R: RandomNumberGenerator> {
    rand: R,
    frame_buffer: FrameBuffer,
    delay_timer: Word,
    sound_timer: Word,
    keys: [bool; 16],
    last_key: Option<Key>,
}

impl<R: RandomNumberGenerator> Platform for ManagedPlatform<R> {
    //I hope it's working
    fn draw_sprite(&mut self, pos: Point, sprite: Sprite) -> bool {
        let mut collision = false;
        let mut pos = pos;
        pos.x %= SCREEN_WIDTH as u8;
        pos.y %= SCREEN_HEIGHT as u8;

        sprite.iter_pixels().for_each(|pixel| {
            let dpoint = pos + pixel;

            if dpoint.x < SCREEN_WIDTH as u8 && dpoint.y < SCREEN_HEIGHT as u8 {
                let x = dpoint.x as usize;
                let y = dpoint.y as usize;

                if self.frame_buffer.get(y, x) {
                    collision = true;
                }

                self.frame_buffer.change(x, y);
            }
        });
        collision
    }

    fn clear_screen(&mut self) {
        self.frame_buffer = FrameBuffer::default();
    }

    fn get_delay_timer(&self) -> Word {
        self.delay_timer
    }

    fn set_delay_timer(&mut self, value: Word) {
        self.delay_timer = value;
    }

    fn set_sound_timer(&mut self, value: Word) {
        self.sound_timer = value;
    }

    fn is_key_down(&self, key: Key) -> bool {
        self.keys[key.as_usize()]
    }

    fn consume_key_press(&mut self) -> Option<Key> {
        let key = self.last_key;
        self.last_key = None;
        key
    }

    fn get_random_word(&mut self) -> Word {
        (self.rand)()
    }
}

impl<R: RandomNumberGenerator> ManagedPlatform<R> {
    fn new(rand: R) -> Self {
        Self {
            rand,
            frame_buffer: Default::default(),
            delay_timer: 0,
            sound_timer: 0,
            keys: [false; 16],
            last_key: None,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct ManagedInterpreter<R: RandomNumberGenerator> {
    inner: Interpreter<ManagedPlatform<R>>,
    counter: u32,
}

impl<R: RandomNumberGenerator> ManagedInterpreter<R> {
    pub const DEFAULT_OPERATION_DURATION: Duration = Duration::from_millis(2);
    pub const DEFAULT_DELAY_TICK_DURATION: Duration = Duration::from_nanos(16666667);
    pub const DEFAULT_SOUND_TICK_DURATION: Duration = Duration::from_nanos(16666667);

    pub fn new(image: impl Image, rand: R) -> Self {
        Self::new_with_durations(
            image,
            rand,
            Self::DEFAULT_OPERATION_DURATION,
            Self::DEFAULT_DELAY_TICK_DURATION,
            Self::DEFAULT_SOUND_TICK_DURATION,
        )
    }

    pub fn new_with_durations(
        image: impl Image,
        rand: R,
        _operation_duration: Duration,
        _delay_tick_duration: Duration,
        _sound_tick_duration: Duration,
    ) -> Self {
        Self {
            inner: Interpreter::new(image, ManagedPlatform::new(rand)),
            counter: 0,
        }
    }

    pub fn simulate_one_instruction(&mut self) -> Result<()> {
        self.counter += 1;
        if (self.counter % 8 == 0 && self.counter % 48 != 0) || (self.counter % 50 == 0) {
            let timer = self.inner.platform().get_delay_timer();
            if timer != 0 {
                self.inner.platform_mut().set_delay_timer(timer - 1);
            }
        }
        self.inner.run_next_instruction()
    }

    pub fn simulate_duration(&mut self, duration: Duration) -> Result<()> {
        let duration_millis = duration.as_millis();
        let timer_duration_millis = Self::DEFAULT_OPERATION_DURATION.as_millis();
        let ops_count: u128 = duration_millis / timer_duration_millis;
        for _ in 0..ops_count {
            self.simulate_one_instruction()?
        }
        Ok(())
    }

    pub fn frame_buffer(&self) -> &FrameBuffer {
        &self.inner.platform().frame_buffer
    }

    pub fn set_key_down(&mut self, key: Key, is_down: bool) {
        if is_down {
            self.inner.platform_mut().last_key = Some(key);
        }
        self.inner.platform_mut().keys[key.as_usize()] = is_down;
    }
}
