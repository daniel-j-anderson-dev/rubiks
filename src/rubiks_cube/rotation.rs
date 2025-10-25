use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct Rotation {
    pub face: Face,
    pub is_clockwise: bool,
}

pub fn rotate_face(face: &mut Tensor2<Color, ROW_COUNT, COLUMN_COUNT>, clockwise: bool) {
    for old_row_index in 0..ROW_COUNT {
        for old_column_index in 0..COLUMN_COUNT {
            let old = face.clone();
            let (face_row_index, face_column_index) = if clockwise {
                (old_column_index, 2 - old_row_index)
            } else {
                (2 - old_column_index, old_row_index)
            };
            face[face_row_index][face_column_index] = old[old_row_index][old_column_index];
        }
    }
}
