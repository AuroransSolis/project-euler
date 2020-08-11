use criterion::{black_box, criterion_group, criterion_main, Criterion};

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

const MAXES: [usize; 5] = [1000, 10_000, 100_000, 1_000_000, 10_000_000];

fn bench_solution(c: &mut Criterion) {
    for max in MAXES.iter().copied() {
        c.bench_function(format!("problem 1 (max: {})", max).as_str(), |b| {
            b.iter(|| black_box(get_sum(max)))
        });
    }
}

criterion_group! {
    name = problem_1_bench;
    config = Criterion::default();
    targets = bench_solution
}

criterion_main!(problem_1_bench);
