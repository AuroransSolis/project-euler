fn main() {
    let mut total = 0;
    fib(&mut total);
    println!("p2: {}", total);
}

fn fib(t: &mut u32) {
    *t += 2;
    fib_r(1, 2, t);
}

fn fib_r(a: u32, b: u32, t: &mut u32) {
    let c = a + b;
    if c >= 4_000_000 {
        return;
    }
    if c % 2 == 0 {
        *t += c;
    }
    fib_r(b, c, t);
}
