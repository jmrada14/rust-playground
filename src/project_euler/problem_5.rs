// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

pub mod math_utils;

fn main() {
    let result = math_utils::lcm_range(20);
    println!("{}", result);
}
