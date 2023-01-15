use ascii_renderer::prelude::*;

#[derive(Debug)]
struct MyLogic {
    pub renderer: Renderer,
}

impl Logic for MyLogic {
    fn process(&mut self, screen_buf: &mut CharBuffer, delta: f32) -> ProcessReturn {
        screen_buf.fill(' ');

        self.renderer.draw(screen_buf);

        self.renderer.meshs.first_mut().unwrap().rotation.y += delta;

        ProcessReturn::Continue
    }
}

fn main() {
    let my_obj = AsciiObj::load("face.obj").unwrap();
    let mut my_meshes: Vec<Mesh> = my_obj.into();
    my_meshes.iter_mut().for_each(|x| {
        x.scale = vec3!(0.01, 0.01, 0.01);
        x.rotation = vec3!(std::f32::consts::PI, 0.0, 0.0);
        x.recenter();
    });
    let mut runner = Runner::new(
        50,
        50,
        25,
        MyLogic {
            renderer: Renderer {
                meshs: my_meshes,
                camera: Camera {
                    position: vec3!(0.0, 0.0, -3.0),
                    rotation: vec3!(0.0, 0.0, 0.0),
                    fov: vec2!(0.8, 0.8),
                },
            },
        },
    );
    runner.run(true);
}
