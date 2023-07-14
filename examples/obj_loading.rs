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
    let mut my_meshes: Vec<Mesh> = AsciiObj::load("face.obj").unwrap().into();
    my_meshes.iter_mut().for_each(|mesh| {
        // * Scales the obj down. rotates it so that it is rightside up, and recenters it.
        mesh.scale = vec3!(0.01, 0.01, 0.01);
        mesh.rotation = vec3!(std::f32::consts::PI, 0.0, 0.0);
        mesh.recenter(); // * This OBJ is really far from the origin for some reason, so if it is not recentered it
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
