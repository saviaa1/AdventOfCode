use day05::*;

use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};

fn parse_benchmark(c: &mut Criterion) {
    c.bench_function(
        "Day05::parse", 
        |b| b.iter(|| parse(include_str!("../src/input.txt")))
    );
}

fn parse_large_benchmark(c: &mut Criterion) {
    c.bench_function(
        "Day05::parse_60mb", 
        |b| b.iter(|| parse(include_str!("../src/large_input.txt")))
    );
}


fn part1_benchmark(c: &mut Criterion) {
    let parsed = black_box(parse(include_str!("../src/input.txt")));
    c.bench_function(
        "Day05::part1", 
        |b| b.iter(|| part_1(&parsed))
    );
}

fn part1_large_benchmark(c: &mut Criterion) {
    let parsed = black_box(parse(include_str!("../src/large_input.txt")));
    c.bench_function(
        "Day05::part1_60mb", 
        |b| b.iter(|| part_1(&parsed))
    );
}

fn part2_benchmark(c: &mut Criterion) {
    let parsed = black_box(parse(include_str!("../src/input.txt")));
    c.bench_function(
        "Day05::part2", 
        |b| b.iter(|| part_2(&parsed))
    );
}

fn all_benchmark(c: &mut Criterion) {
    c.bench_function(
        "Day05::all", 
        |b| b.iter(|| {
            let parsed = parse(include_str!("../src/input.txt"));
            let _ = part_1(&parsed);
            let _ = part_2(&parsed);
        })
    );
}

criterion_group!(benches, parse_benchmark, parse_large_benchmark, part1_benchmark, part1_large_benchmark, part2_benchmark, all_benchmark);
criterion_main!(benches);