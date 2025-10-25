pub type Tensor3<T, const OUTER_MOST: usize, const MIDDLE: usize, const INNER_MOST: usize> =
    [[[T; INNER_MOST]; MIDDLE]; OUTER_MOST];

pub fn tensor3_from_fn<T, const H: usize, const W: usize, const D: usize>(
    mut f: impl FnMut(usize, usize, usize) -> T,
) -> Tensor3<T, H, W, D> {
    core::array::from_fn(|i| core::array::from_fn(|j| core::array::from_fn(|k| f(i, j, k))))
}
