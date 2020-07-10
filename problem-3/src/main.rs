const CHECK: u64 = 600_851_475_143;

fn main() {
    let mut num = CHECK;
    // This holds a collection of prime numbers we've found so far in factoring the number. This can
    // be used to check the primality of factor candidates.
    let mut primes = vec![2];
    let mut largest = 0;
    // Only check the odd numbers greater than or equal to 3.
    for candidate in (1..).map(|n| n * 2 + 1) {
        // Early exit. num will get whittled down over time as we factor it, so we can close up shop
        // if we've got numbers greater than or equal to it. If this exit isn't taken, check all
        // primes in primes less than the square root of the candidate for a factor. If none are
        // found, push to primes and check whether num is divisible by it. If it is divisible,
        // divide num by it as many times as possible, and set largest to this new prime factor.
        // When the loop is finally broken, largest will contain the largest prime factor.
        if candidate > num {
            break;
        } else if !primes
            .iter()
            .take_while(|&prime| prime * prime <= candidate)
            .any(|&prime| candidate % prime == 0)
        {
            primes.push(candidate);
            if num % candidate == 0 {
                while num % candidate == 0 {
                    num /= candidate;
                }
                largest = candidate;
            }
        }
    }
    println!("p3: {}", largest);
}
