use ascii_renderer::*;
use char_buffer::CharBuffer;
use line::Line;
use runner::{Logic, ProcessReturn, Runner};

#[derive(Debug)]
struct MyLogic {
    center: (f32, f32),
    radius: f32,
    theta: f32,
}

impl Logic for MyLogic {
    fn process(&mut self, screen_buf: &mut CharBuffer, delta: f32) -> ProcessReturn {
        screen_buf.fill(' ');

        self.theta += (2.0 * std::f32::consts::PI) * delta;

        for i in 0..200 {
            let t = ((2.0 * std::f32::consts::PI) / 200.0) * i as f32;
            screen_buf.set_char((self.center.0 + (self.radius + 2.0) * t.cos()) as usize, (self.center.1 + (self.radius + 2.0) * t.sin()) as usize, '=');
        }

        let line = Line {
            char: '+',
            points: (
                self.center,
                (
                    self.radius * self.theta.cos() + self.center.0,
                    self.radius * self.theta.sin() + self.center.1,
                ),
            ),
        };

        screen_buf.draw_line(line);

        ProcessReturn::Continue
    }
}

fn main() {
    let mut runner = Runner::new(
        30,
        30,
        20,
        MyLogic {
            center: (15.0, 15.0),
            radius: 10.0,
            theta: 3.0,
        },
    );
    runner.run(true);
}
