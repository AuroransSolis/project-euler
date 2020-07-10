use std::collections::HashMap;

const MAX: u64 = 20;

fn main() {
    // Find all primes in [0, MAX].
    let mut primes = vec![2];
    for n in (1..).map(|n| 2 * n + 1).take_while(|&n| n <= MAX) {
        if !primes
            .iter()
            .copied()
            .take_while(|&prime| prime * prime <= n)
            .any(|prime| n % prime == 0)
        {
            primes.push(n);
        }
    }
    // Find the maximum power of each prime used by any of the numbers in [0, MAX].
    let mut primes_and_powers = HashMap::new();
    for n in 4..=MAX {
        prime_factorization(&primes, &mut primes_and_powers, n);
    }
    // Insert 1 for all primes in primes that aren't in primes_and_powers.
    primes
        .into_iter()
        .for_each(|prime| {
            let _ = primes_and_powers.entry(prime).or_insert(1);
        });
    // The lowest common multiple of all the numbers in [1, MAX] will be the product of each of the
    // primes in primes_and_powers raised to its corresponding power in the hashmap.
    let lcm = primes_and_powers
        .into_iter()
        .map(|(prime, power)| prime.pow(power))
        .product::<u64>();
    println!("p5: {}", lcm);
}

fn prime_factorization(primes: &[u64], primes_and_powers: &mut HashMap<u64, u32>, mut n: u64) {
    // For any prime p, it will have a power greater than 1 in the factorization of one of the
    // numbers in [1, MAX] if p * p is less than or equal to MAX. Every other prime will just have a
    // power of 1.
    for prime in primes
        .iter()
        .copied()
        .take_while(|prime| prime * prime < MAX)
    {
        if prime <= n {
            // Find the power of the prime, and if that power is greater than the one stored in
            // primes_and_powers, make that the new value for the prime key.
            let mut power = 0;
            while n % prime == 0 {
                power += 1;
                n /= prime;
            }
            if power > 0 {
                let p = primes_and_powers.entry(prime).or_insert(power);
                *p = power.max(*p);
            }
        } else {
            break;
        }
    }
}
