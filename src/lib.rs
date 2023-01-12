pub mod char_buffer;
pub mod line;
pub mod rendering;
pub mod runner;

pub use rendering::{Vector2, Vector3};

pub mod prelude {
    pub use super::char_buffer::CharBuffer;
    pub use super::line::Line;
    pub use super::rendering::{Camera, Polygon, Renderer};
    pub use super::runner::{Logic, Runner};
    pub use super::{vec2, vec3, Vector2, Vector3};
}

pub fn create_cube() -> rendering::Polygon {
    //!Generates a 2 x 2 x 2 cube for testing and sampling

    let mut cube = rendering::Polygon::default();
    //Top Square
    cube.insert_vertex(0, vec3!(1.0, 1.0, 1.0));
    cube.insert_vertex(1, vec3!(-1.0, 1.0, 1.0));
    cube.insert_vertex(2, vec3!(-1.0, -1.0, 1.0));
    cube.insert_vertex(3, vec3!(1.0, -1.0, 1.0));

    cube.add_edge((0, 1));
    cube.add_edge((1, 2));
    cube.add_edge((2, 3));
    cube.add_edge((3, 0));

    //Bottom Square
    cube.insert_vertex(4, vec3!(1.0, 1.0, -1.0));
    cube.insert_vertex(5, vec3!(-1.0, 1.0, -1.0));
    cube.insert_vertex(6, vec3!(-1.0, -1.0, -1.0));
    cube.insert_vertex(7, vec3!(1.0, -1.0, -1.0));

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
