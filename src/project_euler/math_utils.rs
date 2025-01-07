pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

pub fn lcm_range(n: u64) -> u64 {
    (1..=n).fold(1, |acc, x| lcm(acc, x))
}
