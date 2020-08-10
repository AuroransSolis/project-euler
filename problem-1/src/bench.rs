use criterion::{black_box, criterion_group, criterion_main, Criterion};

macro_rules! build_benchmark {
    ($($max:literal),+) => {
        fn bench_solution(c: &mut Criterion) {
            $(
                c.bench_function(format!("problem 1 (max: {})", $max).as_str(), |b| {
                    b.iter(|| black_box(get_sum($max)))
                });
            )*
        }
    }
}

fn get_sum(max: usize) -> usize {
    (0..=max / 15)
        .map(|n| {
        [0, 3, 5, 6, 9, 10, 12]
            .iter()
            .map(|&s| n * 15 + s)
            .take_while(|&t| t < max)
            .sum::<usize>()
        })
        .sum::<usize>()
}

build_benchmark! {
    1000, 10_000, 100_000, 1_000_000, 10_000_000
}

criterion_group! {
    name = problem_1_bench;
    config = Criterion::default();
    targets = bench_solution
}

criterion_main!(problem_1_bench);