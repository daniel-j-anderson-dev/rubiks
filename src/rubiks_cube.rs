pub mod color;
pub mod face;
pub mod rotation;

pub use self::{color::*, face::*, rotation::*};

use crate::tensor::*;

pub const RANK: usize = 3;
pub const FACES_COUNT: usize = 6;
pub const ROW_COUNT: usize = 3;
pub const COLUMN_COUNT: usize = 3;
pub const SHAPE: [usize; RANK] = [COLUMN_COUNT, ROW_COUNT, COLUMN_COUNT];

#[derive(Debug, Clone, Copy)]
pub struct RubiksCube {
    faces: Tensor3<Color, { FACES_COUNT }, { ROW_COUNT }, { COLUMN_COUNT }>,
}
/// Constructors
impl RubiksCube {
    pub fn new() -> Self {
        Self {
            faces: tensor3_from_fn(|face_index, _, _| Face::COLORS[face_index]),
        }
    }
}
/// Accessors
impl RubiksCube {
    pub fn face(&self, face: Face) -> &[[Color; COLUMN_COUNT]; ROW_COUNT] {
        &self.faces[face as usize]
    }
    pub fn face_mut(&mut self, face: Face) -> &mut [[Color; COLUMN_COUNT]; ROW_COUNT] {
        &mut self.faces[face as usize]
    }
}
/// Mutators
impl RubiksCube {
    pub fn rotate(&mut self, rotation: Rotation) {
        todo!()
    }
}
