/// A 3-dimensional array with it's shape listed in descending order form `OUTER_MOST` to `INNER_MOST`  
pub type Tensor3<T, const OUTER_DIM: usize, const MIDDLE_DIM: usize, const INNER_DIM: usize> =
    [[[T; INNER_DIM]; MIDDLE_DIM]; OUTER_DIM];

pub fn tensor3_from_fn<T, const H: usize, const W: usize, const D: usize>(
    mut f: impl FnMut(usize, usize, usize) -> T,
) -> Tensor3<T, H, W, D> {
    core::array::from_fn(|i| core::array::from_fn(|j| core::array::from_fn(|k| f(i, j, k))))
}
