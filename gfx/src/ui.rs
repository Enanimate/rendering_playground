use crate::state::Vertex;

pub struct Element {
    pub shape: Vec<[f32; 3]>,
    color: [f32; 4],
}

impl Element {
    pub fn new() -> Element {
        Element { 
            shape: Vec::new(), 
            color: [0.0, 0.0, 0.0, 0.0] 
        }
    }

    pub fn build(self) -> Vec<Vertex> {
        let mut output = Vec::new();
        for vertex in self.shape {
            output.push(Vertex {position: vertex, _padding: [0.0], color: self.color});
        }

        output
    }

    pub fn with_shape(mut self, shape: Vec<[f32; 3]>) -> Self {
        self.shape = shape;
        self
    }

    pub fn with_color(mut self, color: [f32; 4]) -> Self {
        self.color = color;
        self
    }
}