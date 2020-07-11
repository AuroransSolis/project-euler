const MAX: u64 = 100;

fn main() {
    // Using formulae for the sum of squares and the sum of a range.
    let sum_of_squares = MAX * (MAX + 1) * (MAX * 2 + 1) / 6;
    let square_of_sums = (MAX * (MAX + 1) / 2).pow(2);
    println!("p6: {}", square_of_sums - sum_of_squares);
}
