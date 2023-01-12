use super::char_buffer::CharBuffer;
use std::time::Instant;

///The enum returned by the process fn of a logic class. If End is returned, the runner will cease, otherwise it will continue.
pub enum ProcessReturn {
    Continue,
    End,
}

///The trait used to define the behaviour of a runner.
pub trait Logic {
    ///This method runs every frame, is passed the the char buffer (which is maintained from frame to frame, so remember to clear it), delta (the time in secods since the last frame), and returns a ProcessReturn. After being running process every single frame, the runner will print the buffer to the screen. If ProcessReturn::Continue is returned the runner will then continue to the next frame, otherwise it will stop.
    fn process(&mut self, screen_buf: &mut CharBuffer, delta: f32) -> ProcessReturn;
}

///The struct that runs every thing. When ran, every single frame it will run the process method from it's logic (which will mutate the CharBuffer), print the char buffer to the screen, and if process returned continue it will wait for the next frame.
/// # Example
/// ```
/// struct MyLogic;
/// impl Logic for MyLogic {
///     fn process(&mut self, screen_buf: &mut CharBuffer, delta: f32) -> ProcessReturn {
///         let fps_string = delta.to_string();
///         let chars = fps_string.chars();
///         screen_buf.set_char(1, 1, chars.next().unwrap()).unwrap();
///         screen_buf.set_char(2, 1, chars.nest().unwrap()).unwrap();
///     }
/// }
/// let mut my_runner = Runner::new(3, 3, 25, MyLogic);   //Should print the real fps to the screen every frame.
/// my_runner.run(true);
/// ```
pub struct Runner<L: Logic> {
    pub buf: CharBuffer,
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
    pub fn step(&mut self, clear_screen: bool) -> ProcessReturn {
        //! Steps the runner one frame. If the time since the last frame (or the creation of the runner) is less than 1.0 / fps_cap, it will instead wait until enough time has passed and then return continue, so typically it is better to run this twice to ensure a frame will actually render.
        let time_elapsed = self.last_timpoint.elapsed().as_secs_f32();
        if time_elapsed < (1.0 / self.fps_cap as f32) {
            std::thread::sleep(std::time::Duration::from_secs_f32(
                (1.0 / self.fps_cap as f32) - time_elapsed,
            ));
            return ProcessReturn::Continue;
        }
        self.last_timpoint = Instant::now();

        let ret = self.logic.process(&mut self.buf, time_elapsed);
        if clear_screen {
            clear_screen::clear();
        }
        println!("{buf}", buf = self.buf);
        ret
    }
    pub fn run(&mut self, clear_screen: bool) {
        //! Runs the runner. If clear_screen is true, it will attempt to clear the terminal every frame. Otherwise, it will just print out every frame normally. It requires &mut self as the char buffer and logic will likely mutate every frame.
        loop {
            if let ProcessReturn::End = self.step(clear_screen) {
                break;
            }
        }
    }
}
