const MAX: usize = 1000;

fn main() {
    let (a, b, c) = (0..)
        .flat_map(|a| (0..a).map(move |b| (a, b)))
        .map(|(a, b)| (a, b, MAX - a - b))
        .map(|(a, b, c)| {
            let mut abc = [a, b, c];
            abc.sort();
            let [a, b, c] = abc;
            (a, b, c)
        })
        .filter(|(a, b, c)| a.pow(2) + b.pow(2) == c.pow(2))
        .next()
        .unwrap();
    println!("a: {}, b: {}, c: {}", a, b, c);
    let product = a * b * c;
    println!("p9: {}", product);
}
