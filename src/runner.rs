use super::char_buffer::CharBuffer;
use std::{time::Instant, io::Write};


pub enum ProcessReturn {
    Continue,
    End,
}


pub trait Logic {
    fn process(&mut self, screen_buf: &mut CharBuffer, delta: f32) -> ProcessReturn;
}


pub struct Runner<L: Logic>
{
    buf: CharBuffer,
    logic: L,
    pub fps_cap: usize,
    last_timpoint: Instant,
}


impl<L: Logic> Runner<L> {
    pub fn new(width: usize, height: usize, fps_cap: usize, logic: L) -> Self {
        Self {
            buf: CharBuffer::new(width, height),
            logic,
            fps_cap,
            last_timpoint: Instant::now(),
        }
    }
    pub fn step(&mut self) -> ProcessReturn {
        let time_elapsed = self.last_timpoint.elapsed().as_secs_f32();
        if time_elapsed < (1.0 / self.fps_cap as f32) {
            std::thread::sleep(std::time::Duration::from_secs_f32((1.0 / self.fps_cap as f32) - time_elapsed));
            return ProcessReturn::Continue;
        }
        self.last_timpoint = Instant::now();

        let ret = self.logic.process(&mut self.buf, time_elapsed);
        clear_screen::clear();
        println!("{buf}", buf = self.buf);
        ret
    }
    pub fn run(&mut self) {
        loop {
            if let ProcessReturn::End = self.step() {
                break;
            }
        }
    }
}

