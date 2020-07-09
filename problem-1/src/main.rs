const MAX: u32 = 1000;

fn main() {
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
