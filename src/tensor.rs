/// A 3-dimensional array with it's shape listed in descending order form `OUTER_MOST` to `INNER_MOST`  
pub type Tensor3<T, const DIM0: usize, const DIM1: usize, const DIM2: usize> =
    [[[T; DIM2]; DIM1]; DIM0];

/// constructs a [Tensor3] from a function `f` that is given an index for each element.
/// # Parameters
/// - `f`
///   - `parameter0`: index into `dimension0` (increases the slowest)
///   - `parameter1`: index into `dimension1`
///   - `parameter2`: index into `dimension1` (increases the fastest)
///   - `returns`: the value of the output [Tensor3] at index `parameter0, parameter1, parameter2`
pub fn tensor3_from_fn<T, const DIM0: usize, const DIM1: usize, const DIM2: usize>(
    mut f: impl FnMut(usize, usize, usize) -> T,
) -> Tensor3<T, DIM0, DIM1, DIM2> {
    core::array::from_fn(|i| core::array::from_fn(|j| core::array::from_fn(|k| f(i, j, k))))
}

/// A 2-dimensional array with it's shape listed in descending order form `OUTER_MOST` to `INNER_MOST`  
pub type Tensor2<T, const DIM0: usize, const DIM1: usize> = [[T; DIM1]; DIM0];

/// constructs a [Tensor3] from a function `f` that is given an index for each element.
/// # Parameters
/// - `f`
///   - `parameter0`: index into `dimension0` (increases the slowest)
///   - `parameter1`: index into `dimension1`
///   - `returns`: the value of the output [Tensor3] at index `parameter0, parameter1`
pub fn tensor2_from_fn<T, const DIM0: usize, const DIM1: usize>(
    mut f: impl FnMut(usize, usize) -> T,
) -> Tensor2<T, DIM0, DIM1> {
    core::array::from_fn(|i| core::array::from_fn(|j| f(i, j)))
}
