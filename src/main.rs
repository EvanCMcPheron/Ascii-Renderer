use ascii_renderer::*;
use char_buffer::CharBuffer;
use runner::{Logic, ProcessReturn, Runner};
use line::draw_line;


#[derive(Debug)]
struct MyLogic{
    center: (usize, usize),
    radius: f32,
    theta: f32,
}


impl Logic for MyLogic {
    fn process(&mut self, screen_buf: &mut CharBuffer, delta: f32) -> ProcessReturn {

        screen_buf.fill(' ');

        self.theta -= -5.0 * delta;

        let end_point = (
            (self.center.0 as f32 + (self.radius * self.theta.cos())) as usize,
            (self.center.1 as f32 + (self.radius * self.theta.sin())) as usize,
        );

        draw_line('+', screen_buf, self.center, end_point);

        ProcessReturn::Continue
    }
}


fn main() {
    let mut runner = Runner::new(100, 100, 20, MyLogic{center: (50, 50), radius: 40.0, theta: 0.0});
    runner.run();
}
