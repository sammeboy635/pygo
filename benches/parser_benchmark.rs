use pygo::parser::{Parser, parse_expression}; // Replace 'my_crate_name' with the name of your crate
use pygo::standard_library::standard_library;
use criterion::{criterion_group, criterion_main, Criterion};


fn parser_benchmark(c: &mut Criterion) {
	let tokens = vec![
        "test2".to_string(), "=".to_string(), "3".to_string(), "+".to_string(), "(".to_string(), "9".to_string(),
        "*".to_string(), "(".to_string(), "5".to_string(), "+".to_string(), "8".to_string(), ")".to_string(),
        ")".to_string(), "*".to_string(), "3".to_string(), "/".to_string(), "5".to_string()
    ];
    // let tokens: Vec<String> = vec![
    //     "test2", "=", "3", "+", "(", "9", "*", "(", "5", "+", "8", ")", ")", "*", "3", "/", "5"
    // ];

    c.bench_function("parse_expression", |b| {
        b.iter(|| {
            let mut parser = Parser::new(tokens.clone(),standard_library());
            let mut instructions = vec![];
            parse_expression(&mut parser, &mut instructions);
        })
    });
}

criterion_group!(benches, parser_benchmark);
criterion_main!(benches);