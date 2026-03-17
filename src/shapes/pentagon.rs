use crate::objects::face::Face;
use crate::objects::model::Model;
use crate::objects::vertex::Vertex;

pub fn model() -> Model {
    let vertices = vec![
        Vertex::new([-0.0868241, 0.49240386, 0.0], [0.5, 0.0, 0.5]),
        Vertex::new([-0.49513406, 0.06958647, 0.0], [0.5, 0.0, 0.5]),
        Vertex::new([-0.21918549, -0.44939706, 0.0], [0.5, 0.0, 0.5]),
        Vertex::new([0.35966998, -0.3473291, 0.0], [0.5, 0.0, 0.5]),
        Vertex::new([0.44147372, 0.2347359, 0.0], [0.5, 0.0, 0.5]),
    ];

    let faces = vec![Face::new(0, 1, 4), Face::new(1, 2, 4), Face::new(2, 3, 4)];
    Model::new("Pentagon", vertices, faces)
}
