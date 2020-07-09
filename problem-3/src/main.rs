const CHECK: u64 = 600_851_475_143;

fn main() {
    let mut num = CHECK;
    let mut primes = vec![2];
    let mut largest = 0;
    for candidate in (1..=(CHECK as f64).sqrt() as u64 / 2).map(|n| n * 2 + 1) {
        if !primes.iter().any(|&prime| candidate % prime == 0) {
            primes.push(candidate);
            if num % candidate == 0 {
                num /= candidate;
                largest = candidate;
            }
        }
    }
    println!("p3: {}", largest);
}
