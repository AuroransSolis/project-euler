const MAX: usize = 2_000_000;

fn main() {
    let mut primes = Vec::with_capacity(((MAX as f64) / (MAX as f64).ln()) as usize);
    primes.push(2);
    for n in (3..MAX).step_by(2) {
        if !primes
            .iter()
            .take_while(|&prime| prime * prime <= n)
            .any(|prime| n % prime == 0)
        {
            primes.push(n);
        }
    }
    println!("number of primes: {}", primes.len());
    let summation = primes.into_iter().sum::<usize>();
    println!("p10: {}", summation);
}
