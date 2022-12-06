use day03::*;

use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};

fn parse_benchmark(c: &mut Criterion) {
    c.bench_function(
        "Day03::parse", 
        |b| b.iter(|| parse(include_str!("../src/input.txt")))
    );
}

fn part1_benchmark(c: &mut Criterion) {
    let parsed = black_box(parse(include_str!("../src/input.txt")));
    c.bench_function(
        "Day03::part1", 
        |b| b.iter(|| part_1(&parsed))
    );
}

fn part2_benchmark(c: &mut Criterion) {
    let parsed = black_box(parse(include_str!("../src/input.txt")));
    c.bench_function(
        "Day03::part2", 
        |b| b.iter(|| part_2(&parsed))
    );
}

fn all_benchmark(c: &mut Criterion) {
    c.bench_function(
        "Day03::all", 
        |b| b.iter(|| {
            let parsed = parse(include_str!("../src/input.txt"));
            let _ = part_1(&parsed);
            let _ = part_2(&parsed);
        })
    );
}

criterion_group!(benches, parse_benchmark, part1_benchmark, part2_benchmark, all_benchmark);
criterion_main!(benches);