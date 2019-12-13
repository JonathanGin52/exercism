pub fn nth(n: u32) -> u32 {
    let is_prime = |num| !(2..=(num as f32).sqrt() as u32).any(|i| num % i == 0);

    match (2..).filter(|&x| is_prime(x)).nth(n as usize) {
        Some(num) => num,
        _ => unreachable!(),
    }
}
