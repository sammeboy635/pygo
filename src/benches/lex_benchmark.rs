// benches/lex_benchmark.rs
use pygo::Interpreter::lexer::lex;
use criterion::{criterion_group, criterion_main, Criterion};


fn lex_benchmark(c: &mut Criterion) {
    let input = "test2 = 3 + (9 * (5 + 8)) * 3 / 5".to_string();

    c.bench_function("lex", |b| b.iter(|| lex(&input)));
}


criterion_group!(benches, lex_benchmark);
criterion_main!(benches);