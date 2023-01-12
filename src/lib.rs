pub mod char_buffer;
pub mod line;
pub mod runner;
pub mod rendering;


pub use rendering::{Vector2, Vector3};


pub fn create_cube() -> rendering::Polygon {
    
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
