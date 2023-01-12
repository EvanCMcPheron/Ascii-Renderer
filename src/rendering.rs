use std::{collections::HashMap, hash::Hash};
use super::char_buffer::CharBuffer;
use super::line::Line;


#[macro_export]
macro_rules! v3 {
    ($x: expr, $y: expr, $z: expr) => {
        Vector3::new($x, $y, $z)
    };
    ($x: expr, $y: expr, $z: expr,) => {
        Vector3::new($x, $y, $z)
    }
}


#[macro_export]
macro_rules! v2 {
    ($x: expr, $y: expr) => {
        Vector2::new($x, $y)
    };
    ($x: expr, $y: expr,) => {
        Vector2::new($x, $y)
    }
}


#[derive(Debug, Clone)]
pub struct Renderer {
    pub polygons: Vec<Polygon>,
    pub camera: Camera,
}


impl Renderer {
    pub fn draw(&self, buffer: &mut CharBuffer) {
        for polygon in self.polygons.iter() {
            self.draw_polygon(polygon, buffer);
        }
    }
    pub fn draw_polygon(&self, polygon: &Polygon, buffer: &mut CharBuffer) {
        let point_map: HashMap<usize, Vector2> = polygon.get_global_verticies()
            .iter()
            .map(|(&k, &v)| {
                let mut pnt = self.camera.map_point_uv(v);
                pnt.x *= buffer.dimensions.0 as f32;
                pnt.y *= buffer.dimensions.1 as f32;
                (
                    k,
                    pnt
                )
            })
            .fold(HashMap::new(), |mut accum, (k, v)| {
                accum.insert(k, v);
                accum
            });
        
        let lines: Vec<Line> = polygon.edges.iter()
            .map(|&point_indexs| {
                Line {
                    char: polygon.char,
                    points: (
                        (*point_map.get(&point_indexs.0).unwrap()).into(),
                        (*point_map.get(&point_indexs.1).unwrap()).into(),
                    )
                }
            })
            .collect();
        
        buffer.draw_lines(lines);
    }
}


#[derive(Debug, Clone)]
pub struct Camera {
    pub position: Vector3,
    pub rotation: Vector3,
    pub fov: Vector2,
}


impl Camera {
    pub fn map_point_uv(&self, point: Vector3) -> Vector2 {
        //Maps a three dimensional GLOBAL point to UV point dictating its location on screen
        //EX: (0.0, 0.0) is top left of screen and (1.0, 1.0) is bottom right of screen
        let relative = (point - self.position).rotate(self.rotation);

        let thetas = v2!(
            v2!(relative.z, relative.x).to_polar().y,
            v2!(relative.z, relative.y).to_polar().y
        );

        v2!(
            thetas.x / self.fov.x + 0.5,
            thetas.y / self.fov.y + 0.5
        )
    }
}


#[derive(Debug, Clone)]
pub struct Polygon {
    vertices: HashMap<usize, Vector3>,
    edges: Vec<(usize, usize)>,
    pub rotation: Vector3,
    pub position: Vector3,
    pub scale: Vector3,
    pub char: char,
}


impl Polygon {
    pub fn insert_vertex(&mut self, index: usize, vertex: Vector3) -> Option<Vector3> {
        self.vertices.insert(index, vertex)
    }
    pub fn remove_vertex(&mut self, index: usize) -> Option<Vector3> {
        self.vertices.remove(&index)
    }
    pub fn add_edge(&mut self, edge: (usize, usize)) {
        self.edges.push(edge)
    }
    pub fn remove_edge(&mut self, edge: (usize, usize)) -> Option<(usize, usize)> {
        let i = self.edges.iter().enumerate().find(|(_, &x)| x == edge)?.0;
        Some(self.edges.remove(i))
    }
    pub fn get_global_verticies(&self) -> HashMap<usize, Vector3> {
        let mut ret = self.vertices.clone();
        ret.iter_mut().for_each(|(_, item)| {
            item.x *= self.scale.x;
            item.y *= self.scale.y;
            item.z *= self.scale.z;

            *item = item.rotate(self.rotation);

            *item = *item + self.position;
        });
        ret
    }
}


impl std::default::Default for Polygon {
    fn default() -> Self {
        Self {
            vertices: HashMap::new(),
            edges: vec![],
            rotation: v3!(0.0, 0.0, 0.0),
            position: v3!(0.0, 0.0, 0.0),
            scale: v3!(1.0, 1.0, 1.0),
            char: '+',
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}


impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z
        }
    }
    pub fn rotate(self, rotation_vec: Vector3) -> Self {
        //Rotate around x
        let mut ret = {
            let z_y = v2!(self.z, self.y).rotate(rotation_vec.x);
            v3!(self.x, z_y.y, z_y.x)
        };
        
        //Rotate around y
        ret = {
            let x_z = v2!(ret.x, ret.z).rotate(rotation_vec.y);
            v3!(x_z.x, ret.y, x_z.y)
        };

        //Rotate around z
        ret = {
            let x_y = v2!(ret.x, ret.y).rotate(rotation_vec.z);
            v3!(x_y.x, x_y.y, ret.z)
        };
        
        ret
    }
}


impl std::ops::Add for Vector3 {
    type Output = Vector3;
    fn add(self, other: Self) -> Self::Output {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}


impl std::ops::Sub for Vector3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}


impl std::convert::Into<(f32, f32, f32)> for Vector3 {
    fn into(self) -> (f32, f32, f32) {
        (
            self.x,
            self.y,
            self.z,
        )
    }
}


#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}


impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y
        }
    }
    pub fn to_polar(self) -> Self {
        //! x: radius, y: theta
        v2!((self.x * self.x + self.y * self.y).sqrt(), self.y.atan2(self.x))
    }
    pub fn to_cartesian(self) -> Self {
        v2!(self.x * self.y.cos(), self.x * self.y.sin())
    }
    pub fn rotate(self, delta_theta: f32) -> Self {
        let mut polar = self.to_polar();
        polar.y += delta_theta;
        polar.to_cartesian()
    }
}


impl std::ops::Add for Vector2 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}


impl std::ops::Sub for Vector2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}


impl std::convert::Into<(f32, f32)> for Vector2 {
    fn into(self) -> (f32, f32) {
        (
            self.x,
            self.y,
        )
    }
}

