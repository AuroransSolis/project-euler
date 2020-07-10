const CHECK: u64 = 600_851_475_143;

fn main() {
    let mut num = CHECK;
    let mut primes = vec![2];
    let mut largest = 0;
    for candidate in (1..).map(|n| n * 2 + 1) {
        if candidate > num {
            break;
        } else if !primes
            .iter()
            .take_while(|&prime| prime * prime < candidate)
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
