const MAX: usize = 1000;

fn main() {
    let sum = // Explanation:
        // Given that lcm(3, 5) = 15, for any k, 3k mod 15 will have a residue of 0, 3, 6, 9, or 12.
        // Similarly, for any k, 5k mod 15 will have a residue of 0, 5, or 10. As such, we can iterate
        // over multiples of 15 less than the maximum value (as per the problem statement), add the
        // possible residues (0, 3, 5, 6, 9, 10, and 12), and add them to the sum if they're less than
        // the maximum value. This avoids any division or modulus, so it should be pretty quick.
        (0..)
            .map(|n| n * 15)
            .take_while(|&m| m < MAX)
            .map(|m| {
                [0, 3, 5, 6, 9, 10, 12]
                    .iter()
                    .map(|&s| m + s)
                    .take_while(|&t| t < MAX)
                    .sum::<usize>()
            })
            .sum::<usize>();
    println!("p1 (iter): {}", sum);
    println!("p1 (func): {}", get_sum(MAX));
}

fn get_sum(max: usize) -> usize {
    let mul = max / 15;
    let res_max = max % 15;
    [0, 3, 5, 6, 9, 10, 12]
        .iter()
        .copied()
        .map(|residue| {
            if residue < res_max {
                arithmetic_sequence_sum(residue, 15, mul + 1)
            } else {
                arithmetic_sequence_sum(residue, 15, mul)
            }
        })
        .sum::<usize>()
}

#[inline]
fn arithmetic_sequence_sum(start: usize, diff: usize, n: usize) -> usize {
    (n * (2 * start + (n - 1) * diff)) / 2
}
