use ascii_renderer::*;
use char_buffer::CharBuffer;
use runner::{Logic, ProcessReturn, Runner};
use rendering::{Renderer, Polygon, Camera, Vector3, Vector2};
use v2;
use v3;

#[derive(Debug)]
struct MyLogic{
    pub renderer: Renderer,
    pub time_offset: f32,
}

impl Logic for MyLogic {
    fn process(&mut self, screen_buf: &mut CharBuffer, delta: f32) -> ProcessReturn {
        screen_buf.fill(' ');
        
        self.time_offset += delta;

        self.renderer.draw(screen_buf);
        self.renderer.polygons[0].rotation.x += delta * 0.8;
        self.renderer.polygons[0].rotation.y += delta * 1.0;
        self.renderer.polygons[0].rotation.z += delta * 1.2;

        self.renderer.polygons[0].scale.x = 1.0 + (self.time_offset * 2.0).sin() * 0.5;
        self.renderer.polygons[0].scale.y = 1.0 + (self.time_offset * 3.0).sin() * 0.5;
        self.renderer.polygons[0].scale.z = 1.0 + (self.time_offset * 5.0).sin() * 0.5;

        ProcessReturn::Continue
    }
}

fn main() {

    let cube = crate::create_cube();

    let mut runner = Runner::new(
        50,
        50,
        25,
        MyLogic {
            renderer: Renderer {
                polygons: vec![cube],
                camera: Camera { 
                    position: v3!(0.0, 0.0, -7.0),
                    rotation: v3!(0.0, 0.0, 0.0),
                    fov: v2!(0.8, 0.8)
                }
            },
            time_offset: 0.0,
        },
    );
    runner.run(true);
}




