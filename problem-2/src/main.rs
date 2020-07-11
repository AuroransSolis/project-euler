fn main() {
    let mut total = 0;
    fib(&mut total);
    println!("p2: {}", total);
}

// Entry point to the recursion stack. Starts by adding 2 to the total since only the sum of each
// pair of numbers is checked for being even, so 2 would otherwise be missed.
fn fib(t: &mut u32) {
    *t += 2;
    fib_r(1, 2, t);
}

// Basic Fibonacci series recursion structure. Add the two numbers into a third, check some sort of
// limit, break the recursion stack if the limit is hit, otherwise recurse with the second input
// number and the sum.
fn fib_r(a: u32, b: u32, t: &mut u32) {
    let c = a + b;
    if c >= 4_000_000 {
        return;
    } else {
        if c % 2 == 0 {
            *t += c;
        }
        fib_r(b, c, t);
    }
}
