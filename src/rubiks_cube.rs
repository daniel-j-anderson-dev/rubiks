use crate::*;

pub const FACES_COUNT: usize = 6;
pub const ROW_COUNT: usize = 3;
pub const COLUMN_COUNT: usize = 3;
pub const SHAPE: [usize; 3] = [COLUMN_COUNT, ROW_COUNT, COLUMN_COUNT];

#[derive(Debug, Clone, Copy)]
pub struct RubiksCube {
    /// | Face | Color  | Reason |
    /// | ---- | ------ | ------ |
    /// | 0    | White  | Up     |
    /// | 1    | Blue   | Left   |
    /// | 2    | Orange | Front  |
    /// | 3    | Red    | Back   |
    /// | 4    | Green  | Right  |
    /// | 5    | Yellow | Down   |
    faces: Tensor3<Color, { FACES_COUNT }, { ROW_COUNT }, { COLUMN_COUNT }>,
}
impl RubiksCube {
    pub fn new() -> Self {
        let colors = [White, Orange, Blue, Red, Green, Yellow];
        Self {
            faces: tensor3_from_fn(|face_index, _, _| colors[face_index]),
        }
    }
}
