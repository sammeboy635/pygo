// benches/lex_benchmark.rs
use pygo::lexer::lex;
use pygo::lexer::lex2;
use criterion::{criterion_group, criterion_main, Criterion};


fn lex_benchmark(c: &mut Criterion) {
    let operators = vec!["+", "-", "*", "/", "%", ":", "(", ")"];
    let input = "test2 = 3 + (9 * (5 + 8)) * 3 / 5".to_string();

    c.bench_function("lex", |b| b.iter(|| lex(&input, &operators)));
}

fn lex2_benchmark(c: &mut Criterion) {
    let operators = vec!["+", "-", "*", "/", "%", ":", "(", ")"];
    let input = "test2 = 3 + (9 * (5 + 8)) * 3 / 5".to_string();

    c.bench_function("lex", |b| b.iter(|| lex2(&input, &operators)));
}

criterion_group!(benches, lex_benchmark, lex2_benchmark);
criterion_main!(benches);