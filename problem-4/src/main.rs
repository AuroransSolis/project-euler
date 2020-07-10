fn main() {
    let highest_palindrome = (10..91)
        .rev()
        .map(|m| 11 * m)
        .flat_map(|m| {
            (100..1000)
                .rev()
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
