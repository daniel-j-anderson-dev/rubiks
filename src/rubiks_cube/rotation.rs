use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct Rotation {
    pub face: Face,
    pub is_clockwise: bool,
}

pub fn rotate_index(clockwise: bool, row_index: usize, column_index: usize) -> (usize, usize) {
    if clockwise {
        (column_index, 2 - row_index)
    } else {
        (2 - column_index, row_index)
    }
}

pub fn rotate_face(face: &mut Tensor2<Color, ROW_COUNT, COLUMN_COUNT>, clockwise: bool) {
    for row_index in 0..ROW_COUNT {
        for column_index in 0..COLUMN_COUNT {
            let old = face.clone();
            let (face_row_index, face_column_index) =
                rotate_index(clockwise, row_index, column_index);
            face[face_row_index][face_column_index] = old[row_index][column_index];
        }
    }
}
