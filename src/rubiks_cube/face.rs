use crate::rubiks_cube::*;

/// | Face | Color  | Reason |
/// | ---- | ------ | ------ |
/// | 0    | White  | Up     |
/// | 1    | Blue   | Left   |
/// | 2    | Orange | Front  |
/// | 3    | Red    | Back   |
/// | 4    | Green  | Right  |
/// | 5    | Yellow | Down   |
#[derive(Debug, Clone, Copy)]
pub enum Face {
    Up = 0,
    Left = 1,
    Front = 2,
    Back = 3,
    Right = 4,
    Down = 5,
}
impl Face {
    /// | Face | Color  | Reason |
    /// | ---- | ------ | ------ |
    /// | 0    | White  | Up     |
    /// | 1    | Blue   | Left   |
    /// | 2    | Orange | Front  |
    /// | 3    | Red    | Back   |
    /// | 4    | Green  | Right  |
    /// | 5    | Yellow | Down   |
    pub const fn color(&self) -> Color {
        Self::COLORS[*self as usize]
    }
    pub const COLORS: [Color; FACES_COUNT] = [White, Orange, Blue, Red, Green, Yellow];
}
