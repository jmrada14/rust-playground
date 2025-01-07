extern crate rand;
extern crate rayon;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::iter;

pub fn generate_string_array(size: usize) -> Vec<String> {
    let mut rng = thread_rng();
    (0..size)
        .map(|_| {
            iter::repeat(())
                .map(|_| rng.sample(Alphanumeric))
                .take(10)
                .map(char::from)
                .collect()
        })
        .collect()
}

pub fn generate_int_array(size: usize) -> Vec<i32> {
    let mut rng = thread_rng();
    (0..size).map(|_| rng.gen_range(0..10000)).collect()
}
