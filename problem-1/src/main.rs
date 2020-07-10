const MAX: u32 = 1000;

fn main() {
    // Explanation:
    // Given that lcm(3, 5) = 15, for any k, 3k mod 15 will have a residue of 0, 3, 6, 9, or 12.
    // Similarly, for any k, 5k mod 15 will have a residue of 0, 5, or 10. As such, we can iterate
    // over multiples of 15 less than the maximum value (as per the problem statement), add the
    // possible residues (0, 3, 5, 6, 9, 10, and 12), and add them to the sum if they're less than
    // the maximum value. This avoids any division or modulus, so it should be pretty quick.
    let sum = (0u32..)
        .map(|n| n * 15)
        .take_while(|&m| m < MAX)
        .map(|m| {
            [0, 3, 5, 6, 9, 10, 12]
                .iter()
                .map(|&s| m + s)
                .take_while(|&t| t < MAX)
                .sum::<u32>()
        })
        .sum::<u32>();
    println!("p1: {}", sum);
}
