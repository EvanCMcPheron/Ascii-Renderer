use super::prelude::*;
pub use obj::ObjError;
use obj::{Obj, Object};
use std::{collections::HashMap, hash::Hash, ops::Deref};

#[derive(Debug, Clone)]
pub struct AsciiObj(Obj);

impl AsciiObj {
    pub fn save(&self, path: impl AsRef<std::path::Path>) -> Result<(), ObjError> {
        self.0.save(path)
    }
    pub fn load(path: impl AsRef<std::path::Path>) -> Result<Self, ObjError> {
        Obj::load(path).map(|x| AsciiObj(x))
    }
}

impl std::convert::Into<Vec<Mesh>> for AsciiObj {
    fn into(self) -> Vec<Mesh> {
        #[inline]
        fn simple_polygon_to_edges(polygon: obj::SimplePolygon) -> Vec<(usize, usize)> {
            let last_vertex_index = polygon.0.last().unwrap().0;
            polygon
                .0
                .iter()
                .map(|x| x.0)
                .enumerate()
                .fold(vec![], |mut accum, (i, v_index)| {
                    if i == 0 {
                        accum.push((last_vertex_index, v_index))
                    } else {
                        let prev = accum.last().unwrap().1;
                        accum.push((prev, v_index));
                    }
                    accum
                })
        }
        #[inline]
        fn edges_contains_index(edges: &Vec<(usize, usize)>, value: usize) -> bool {
            edges.iter().fold(false, |accum, (index1, index2)| {
                if *index1 == value || *index2 == value {
                    true
                } else {
                    accum
                }
            })
        }

        self.0
            .data
            .objects
            .iter()
            .map(|object| {
                //Object to mesh
                let edges: Vec<(usize, usize)> = object
                    .groups
                    .iter()
                    .map(|group| group.polys.iter())
                    .flatten()
                    .map(|x| x.to_owned())
                    .map(simple_polygon_to_edges)
                    .flatten()
                    .collect();

                let positions: HashMap<usize, Vector3> = self
                    .0
                    .data
                    .position
                    .iter()
                    .map(Vector3::from)
                    .enumerate()
                    .filter(|(i, _)| edges_contains_index(&edges, *i))
                    .fold(HashMap::new(), |mut accum, (i, pos)| {
                        accum.insert(i, pos);
                        accum
                    });

                let mut mesh = Mesh::default();
                *mesh.get_edges_mut() = edges;
                *mesh.get_verticies_mut() = positions;
                mesh
            })
            .collect()
    }
}

impl std::convert::From<&[f32; 3]> for Vector3 {
    fn from(value: &[f32; 3]) -> Self {
        vec3!(value[0], value[1], value[2],)
    }
}
