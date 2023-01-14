use ascii_renderer::{prelude::AsciiObj, *};
use char_buffer::CharBuffer;
use rendering::{Camera, Mesh, Renderer, Vector2, Vector3};
use runner::{Logic, ProcessReturn, Runner};
use vec2;
use vec3;

#[derive(Debug)]
struct MyLogic {
    pub renderer: Renderer,
}

impl Logic for MyLogic {
    fn process(&mut self, screen_buf: &mut CharBuffer, delta: f32) -> ProcessReturn {
        screen_buf.fill(' ');

        self.renderer.draw(screen_buf);

        

        ProcessReturn::Continue
    }
}

fn main() {
    let my_obj = AsciiObj::load("face.obj").unwrap();
    let mut my_meshes: Vec<Mesh> = my_obj.into();
    my_meshes
        .iter_mut()
        .for_each(|x| x.scale = vec3!(0.01, 0.01, 0.01));
    let mut runner = Runner::new(
        50,
        50,
        25,
        MyLogic {
            renderer: Renderer {
                meshs: my_meshes,
                camera: Camera {
                    position: vec3!(7.53688964 * 0.01, 6.0, (-0.435878601 * 0.01) + 10.0),
                    rotation: vec3!(0.0, 0.0, 0.0),
                    fov: vec2!(0.8, 0.8),
                },
            },
        },
    );
    runner.run(true);
}
