use super::char_buffer::CharBuffer;
use super::line::Line;
use std::collections::HashMap;

/// Slightly more concise way of declaring a Vector3
#[macro_export]
macro_rules! vec3 {
    ($x: expr, $y: expr, $z: expr) => {
        Vector3::new($x, $y, $z)
    };
    ($x: expr, $y: expr, $z: expr,) => {
        Vector3::new($x, $y, $z)
    };
}

/// Slightly more concise way of declaring a Vector2
#[macro_export]
macro_rules! vec2 {
    ($x: expr, $y: expr) => {
        Vector2::new($x, $y)
    };
    ($x: expr, $y: expr,) => {
        Vector2::new($x, $y)
    };
}

/// Used for rendering meshs to a CharBuffer.
#[derive(Debug, Clone)]
pub struct Renderer {
    pub meshs: Vec<Mesh>,
    pub camera: Camera,
}

impl Renderer {
    ///Draws all the meshs to the CharBuffer
    /// # Example
    /// ```
    /// let buf = CharBuffer::new(30, 30);  //Make sure to use a char buffer that has dimensions proportional to the camera's FOV, otherwise everything will be stretched oddly...
    /// let renderer = Renderer {
    ///     meshs: vec![create_cube()],
    ///     camera: Camera {
    ///         position: vec3!(0.0, 0.0, -10.0),
    ///         rotation: vec3!(0.0, 0.0, 0.0),
    ///         fov: vec2!(0.7, 0.7);   //FOV is in radians
    ///     },
    /// };
    /// renderer.draw(&mut buf);
    /// println!("{buf}");
    /// ```
    pub fn draw(&self, buffer: &mut CharBuffer) {
        for mesh in self.meshs.iter() {
            self.draw_mesh(mesh, buffer);
        }
    }
    /// Draws an individual mesh.
    pub fn draw_mesh(&self, mesh: &Mesh, buffer: &mut CharBuffer) {
        let point_map: HashMap<usize, Vector2> = mesh
            .get_global_verticies()
            .iter()
            .map(|(&k, &v)| {
                let mut pnt = self.camera.map_point_uv(v);
                pnt.x *= buffer.dimensions.0 as f32;
                pnt.y *= buffer.dimensions.1 as f32;
                (k, pnt)
            })
            .fold(HashMap::new(), |mut accum, (k, v)| {
                accum.insert(k, v);
                accum
            });

        let lines: Vec<Line> = mesh
            .edges
            .iter()
            .map(|&point_indexs| Line {
                char: mesh.char,
                points: (
                    (*point_map.get(&point_indexs.0).unwrap()).into(),
                    (*point_map.get(&point_indexs.1).unwrap()).into(),
                ),
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
    /// Maps a global 3d point to the screen. The output is a UV point, meaning the top left of the screen is (0.0, 0.0) and the bottom right is (1.0, 1.0)
    pub fn map_point_uv(&self, point: Vector3) -> Vector2 {
        //Maps a three dimensional GLOBAL point to UV point dictating its location on screen
        //EX: (0.0, 0.0) is top left of screen and (1.0, 1.0) is bottom right of screen
        let relative = (point - self.position).rotate(self.rotation);

        let thetas = vec2!(
            vec2!(relative.z, relative.x).to_polar().y,
            vec2!(relative.z, relative.y).to_polar().y
        );

        vec2!(thetas.x / self.fov.x + 0.5, thetas.y / self.fov.y + 0.5)
    }
}

/// A struct containing all the data for a mesh. Rotation, as with everything in this crate, is in radians, with each value determining the amount that the mesh should be rotated around the given axis.
/// Note that vertices are stored on a hashmap, not a vector.
#[derive(Debug, Clone)]
pub struct Mesh {
    vertices: HashMap<usize, Vector3>,
    edges: Vec<(usize, usize)>,
    pub rotation: Vector3,
    pub position: Vector3,
    pub scale: Vector3,
    pub char: char,
}

impl Mesh {
    pub fn insert_vertex(&mut self, index: usize, vertex: Vector3) -> Option<Vector3> {
        self.vertices.insert(index, vertex)
    }
    pub fn get_vertex(&mut self, index: usize) -> Option<Vector3> {
        self.vertices.get(&index).map(|&x| x)
    }
    pub fn insert_vertices(
        &mut self,
        vertices: Vec<(usize, Vector3)>,
    ) -> Vec<(usize, Option<Vector3>)> {
        vertices
            .iter()
            .map(|(index, vertex)| (*index, self.insert_vertex(*index, *vertex)))
            .collect()
    }
    pub fn remove_vertex(&mut self, index: usize) -> Option<Vector3> {
        self.vertices.remove(&index)
    }
    pub fn get_verticies(&self) -> &HashMap<usize, Vector3> {
        &self.vertices
    }
    pub fn get_verticies_mut(&mut self) -> &mut HashMap<usize, Vector3> {
        &mut self.vertices
    }
    pub fn add_edge(&mut self, edge: (usize, usize)) {
        self.edges.push(edge)
    }
    pub fn add_edges(&mut self, edges: Vec<(usize, usize)>) {
        for edge in edges {
            self.edges.push(edge);
        }
    }
    pub fn remove_edge(&mut self, edge: (usize, usize)) -> Option<(usize, usize)> {
        let i = self.edges.iter().enumerate().find(|(_, &x)| x == edge)?.0;
        Some(self.edges.remove(i))
    }
    pub fn get_edges(&self) -> &Vec<(usize, usize)> {
        &self.edges
    }
    pub fn get_edges_mut(&mut self) -> &mut Vec<(usize, usize)> {
        &mut self.edges
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

impl std::default::Default for Mesh {
    fn default() -> Self {
        Self {
            vertices: HashMap::new(),
            edges: vec![],
            rotation: vec3!(0.0, 0.0, 0.0),
            position: vec3!(0.0, 0.0, 0.0),
            scale: vec3!(1.0, 1.0, 1.0),
            char: '+',
        }
    }
}

/// A struct used for storing 3d points, rotation vectors, etc. It is easiest to create using vec3!(x, y, z)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub fn rotate(self, rotation_vec: Vector3) -> Self {
        //Rotate around x
        let mut ret = {
            let z_y = vec2!(self.z, self.y).rotate(rotation_vec.x);
            vec3!(self.x, z_y.y, z_y.x)
        };

        //Rotate around y
        ret = {
            let x_z = vec2!(ret.x, ret.z).rotate(rotation_vec.y);
            vec3!(x_z.x, ret.y, x_z.y)
        };

        //Rotate around z
        ret = {
            let x_y = vec2!(ret.x, ret.y).rotate(rotation_vec.z);
            vec3!(x_y.x, x_y.y, ret.z)
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
        (self.x, self.y, self.z)
    }
}

impl std::ops::AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = vec3!(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z);
    }
}

impl std::ops::SubAssign for Vector3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl std::ops::Mul<f32> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: f32) -> Self::Output {
        vec3!(self.x * rhs, self.y * rhs, self.z * rhs,)
    }
}

impl std::ops::Div<f32> for Vector3 {
    type Output = Vector3;
    fn div(self, rhs: f32) -> Self::Output {
        vec3!(self.x / rhs, self.y / rhs, self.z / rhs,)
    }
}

impl std::ops::MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl std::ops::DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl std::ops::Neg for Vector3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        vec3!(-self.x, -self.y, -self.z)
    }
}

/// A struct used for storing 2d points, rotation vectors, etc. It is easiest to create using vec2!(x, y)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn to_polar(self) -> Self {
        //! x: radius, y: theta
        vec2!(
            (self.x * self.x + self.y * self.y).sqrt(),
            self.y.atan2(self.x)
        )
    }
    pub fn to_cartesian(self) -> Self {
        vec2!(self.x * self.y.cos(), self.x * self.y.sin())
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
        (self.x, self.y)
    }
}

impl std::ops::AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl std::ops::SubAssign for Vector2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl std::ops::Mul<f32> for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: f32) -> Self::Output {
        vec2!(self.x * rhs, self.y * rhs,)
    }
}

impl std::ops::Div<f32> for Vector2 {
    type Output = Vector2;
    fn div(self, rhs: f32) -> Self::Output {
        vec2!(self.x / rhs, self.y / rhs,)
    }
}

impl std::ops::MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl std::ops::DivAssign<f32> for Vector2 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl std::ops::Neg for Vector2 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        vec2!(-self.x, -self.y,)
    }
}
