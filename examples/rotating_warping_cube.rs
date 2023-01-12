use ascii_renderer::prelude::*;


struct MyLogic {
    pub renderer: Renderer,
    pub time_offset: f32,
}

impl Logic for MyLogic {
    fn process(&mut self, screen_buf: &mut CharBuffer, delta: f32) -> ProcessReturn {
        screen_buf.fill(' ');

        self.time_offset += delta;  //Keeps track of time

        self.renderer.draw(screen_buf);
        self.renderer.polygons[0].rotation.x += delta * 0.8;    //Rotates the cube
        self.renderer.polygons[0].rotation.y += delta * 1.0;
        self.renderer.polygons[0].rotation.z += delta * 1.2;

        self.renderer.polygons[0].scale.x = 1.0 + (self.time_offset * 2.0).sin() * 0.5;     //Scales the cube according to sin(time)
        self.renderer.polygons[0].scale.y = 1.0 + (self.time_offset * 3.0).sin() * 0.5;
        self.renderer.polygons[0].scale.z = 1.0 + (self.time_offset * 5.0).sin() * 0.5;

        ProcessReturn::Continue
    }
}

fn main() {
    let mut runner = Runner::new(
        50,
        50,
        25,
        MyLogic {
            renderer: Renderer {
                polygons: vec![ascii_renderer::create_cube()],
                camera: Camera {
                    position: vec3!(0.0, 0.0, -7.0),
                    rotation: vec3!(0.0, 0.0, 0.0),
                    fov: vec2!(0.8, 0.8),
                },
            },
            time_offset: 0.0,
        },
    );
    runner.run(true);
}
