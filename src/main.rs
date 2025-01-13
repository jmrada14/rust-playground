extern crate rand;
extern crate rayon;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::time::Instant;
pub mod array_utils;

/// Sorts multiple arrays based on the ordering of elements in a reference array.
///
/// # Type Parameters
/// * `T` - Type of elements in the first array, must implement `Ord` and `Sync`
/// * `U` - Type of elements in other arrays, must implement `Sync`, `Copy`, and `Send`
///
/// # Arguments
/// * `first_array` - The reference array whose ordering will determine the sort
/// * `arrays` - A slice of vectors to be reordered based on `first_array`
///
/// # Returns
/// Returns a new Vec containing the reordered arrays
///
fn sort_arrays_by_first_array<T: Ord + Sync, U: Sync + Copy + Send>(
    first_array: &[T],
    arrays: &[Vec<U>],
) -> Vec<Vec<U>> {
    let len = first_array.len();
    let mut indices: Vec<usize> = (0..len).collect();
    indices.par_sort_unstable_by(|&a, &b| first_array[a].cmp(&first_array[b]));
    arrays
        .par_iter()
        .map(|array| indices.iter().map(|&idx| array[idx]).collect())
        .collect()
}
const ARRAY_SIZE: usize = 1_000_000;
const ARRAY_COUNT: usize = 1000;
fn main() {
    println!("Generating arrays...");
    let start = Instant::now();
    let int_arrays: Vec<Vec<i32>> = (0..ARRAY_COUNT)
        .into_par_iter()
        .map(|_| array_utils::generate_int_array(ARRAY_SIZE))
        .collect();
    let duration = start.elapsed();
    println!("Time taken to generate  arrays: {:?}", duration);

    println!("Sorting integer arrays...");
    let start_int_sort = Instant::now();
    let _sorted_int_arrays = sort_arrays_by_first_array(&int_arrays[0], &int_arrays);
    let duration_int_sort = start_int_sort.elapsed();
    println!("Time taken to sort integer arrays: {:?}", duration_int_sort);
}
