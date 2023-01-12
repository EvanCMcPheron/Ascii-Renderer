use ascii_renderer::*;
use char_buffer::CharBuffer;
use line::Line;
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
        self.renderer.polygons[0].rotation.x += delta * 0.2;
        self.renderer.polygons[0].rotation.y += delta * 0.4;

        self.renderer.camera.position.z = -10.0 + 2.0 * (self.time_offset * 2.0).sin();

        ProcessReturn::Continue
    }
}

fn main() {

    let cube = create_cube();

    let mut runner = Runner::new(
        50,
        50,
        25,
        MyLogic {
            renderer: Renderer {
                polygons: vec![cube],
                camera: Camera { 
                    position: v3!(0.0, 0.0, -10.0),
                    rotation: v3!(0.0, 0.0, 0.0),
                    fov: v2!(0.5, 0.5)
                }
            },
            time_offset: 0.0,
        },
    );
    runner.run(true);
}


fn create_cube() -> Polygon {
    
    let mut cube = Polygon::default();
    //Top Square
    cube.insert_vertex(0, v3!(1.0, 1.0, 1.0));
    cube.insert_vertex(1, v3!(-1.0, 1.0, 1.0));
    cube.insert_vertex(2, v3!(-1.0, -1.0, 1.0));
    cube.insert_vertex(3, v3!(1.0, -1.0, 1.0));

    cube.add_edge((0, 1));
    cube.add_edge((1, 2));
    cube.add_edge((2, 3));
    cube.add_edge((3, 0));
    
    //Bottom Square
    cube.insert_vertex(4, v3!(1.0, 1.0, -1.0));
    cube.insert_vertex(5, v3!(-1.0, 1.0, -1.0));
    cube.insert_vertex(6, v3!(-1.0, -1.0, -1.0));
    cube.insert_vertex(7, v3!(1.0, -1.0, -1.0));

    cube.add_edge((4, 5));
    cube.add_edge((5, 6));
    cube.add_edge((6, 7));
    cube.add_edge((7, 4));

    //Connecting the squares
    cube.add_edge((0, 4));
    cube.add_edge((1, 5));
    cube.add_edge((2, 6));
    cube.add_edge((3, 7));

    cube
}

