fn main() {
    // All palindromic numbers with an even number of digits are divisible by 11, and since the
    // number we're trying to find is a product of two three digit numbers, it is six digits long,
    // and by virtue of that and it being palindromic it is divisible by 11. Thus we will iterate
    // over multiples of 11 and multiply those with other numbers and check for palindromes.
    // 10 * 11 = 110, smallest three digit multiple of 11
    // 90 * 11 = 990, largest three digit multiple of 11
    let highest_palindrome = (10..91)
        .rev()
        .map(|m| 11 * m)
        .flat_map(|m| {
            // Our other factor will be in [100, 1000)
            (100..1000)
                .rev()
                // Only take six digit numbers
                .take_while(move |n| m * n > 100_000)
                .map(move |n| (m, n))
        })
        .map(|(m, n)| m * n)
        .filter(|p| {
            let s = format!("{}", p);
            let s = s.as_bytes();
            s[0] == s[5] && s[1] == s[4] && s[2] == s[3]
        })
        .max()
        .unwrap();
    println!("p4: {:?}", highest_palindrome);
}
