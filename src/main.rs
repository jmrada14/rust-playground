extern crate rand;
extern crate rayon;
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
/// # Performance
/// Uses parallel sorting and iteration for improved performance on large datasets.
/// # Benchmark results:
/// Time taken to generate 100 arrays of size 1_000_000: 167.909625ms
/// Time taken to sort integer arrays: 101.931667ms
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_arrays_by_first_array() {
        // Test case 1: Basic integer sorting
        let first = vec![3, 1, 4, 2];
        let arrays = vec![
            vec![3, 1, 4, 2],
            vec![30, 10, 40, 20],
            vec![300, 100, 400, 200],
        ];
        let expected = vec![
            vec![1, 2, 3, 4],
            vec![10, 20, 30, 40],
            vec![100, 200, 300, 400],
        ];
        assert_eq!(sort_arrays_by_first_array(&first, &arrays), expected);

        // Test case 2: Empty arrays
        let empty: Vec<i32> = vec![];
        let empty_arrays: Vec<Vec<i32>> = vec![vec![]];
        assert_eq!(
            sort_arrays_by_first_array(&empty, &empty_arrays),
            vec![vec![]]
        );

        // Test case 3: Single element arrays
        let single = vec![1];
        let single_arrays = vec![vec![1], vec![10]];
        assert_eq!(
            sort_arrays_by_first_array(&single, &single_arrays),
            single_arrays
        );

        // Test case 4: Arrays with duplicate values
        let duplicates = vec![2, 1, 2, 1];
        let dup_arrays = vec![vec![2, 1, 2, 1], vec![20, 10, 20, 10]];
        let expected_dups = vec![vec![1, 1, 2, 2], vec![10, 10, 20, 20]];
        assert_eq!(
            sort_arrays_by_first_array(&duplicates, &dup_arrays),
            expected_dups
        );
    }
}

const ARRAY_SIZE: usize = 1_000_000;
const ARRAY_COUNT: usize = 100;
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
