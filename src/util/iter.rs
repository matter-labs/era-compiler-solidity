//!
//! Functions related to iteration.
//!

pub fn prefixes<T>(slice: &[T]) -> impl DoubleEndedIterator<Item = &[T]> {
    (0..=slice.len()).map(move |len| &slice[..len])
}
