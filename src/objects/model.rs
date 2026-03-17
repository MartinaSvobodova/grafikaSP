use super::face::Face;
use super::vertex::Vertex;

#[derive(Clone, Debug)]
pub struct Model {
    pub name: String,
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Face>,
}

impl Model {
    pub fn new(name: impl Into<String>, vertices: Vec<Vertex>, faces: Vec<Face>) -> Self {
        Self {
            name: name.into(),
            vertices,
            faces,
        }
    }

    pub fn triangle(name: impl Into<String>, vertices: [Vertex; 3]) -> Self {
        Self {
            name: name.into(),
            vertices: vertices.to_vec(),
            faces: vec![Face::new(0, 1, 2)],
        }
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn face_count(&self) -> usize {
        self.faces.len()
    }

    pub fn indices(&self) -> Vec<u16> {
        let mut out = Vec::with_capacity(self.faces.len() * 3);
        for face in &self.faces {
            out.extend_from_slice(&face.indices);
        }
        out
    }
}
