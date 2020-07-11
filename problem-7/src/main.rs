const AMT: usize = 10_001;

fn main() {
    let mut primes = Vec::with_capacity(AMT);
    primes.push(2);
    // Only check odd numbers since the only even prime is 2
    for n in (1..).map(|n| n * 2 + 1) {
        // Break when we have the number of primes we want
        if primes.len() == AMT {
            break;
        } else if !primes
            .iter()
            .copied()
            .take_while(|prime| prime * prime <= n)
            .any(|prime| n % prime == 0)
        {
            primes.push(n);
        }
    }
    println!("p7: {}", primes[primes.len() - 1]);
}
