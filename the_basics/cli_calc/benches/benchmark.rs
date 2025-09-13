use criterion::{black_box, Criterion, criterion_group, criterion_main};

fn add_macro(a: f64, b: f64) -> f64 {
    macro_rules! add { ($x:expr, $y:expr) => { $x as f64 + $y as f64 }; }
    add!(a, b)
}

fn add_func(a: f64, b: f64) -> f64 {
    a + b
}

fn bench_add(c: &mut Criterion) {
    c.bench_function("macro add", |b| b.iter(|| add_macro(black_box(5.0), black_box(6.0))));
    c.bench_function("function add", |b| b.iter(|| add_func(black_box(5.0), black_box(6.0))));
}

// use criterion::{criterion_group, criterion_main, Criterion};

fn bench_addition(c: &mut Criterion) {
    c.bench_function("addition", |b| {
        b.iter(|| {
            let x = 1 + 2;
            criterion::black_box(x);
        })
    });
}

criterion_group!(benches, bench_addition);
//criterion_main!(benches);
//criterion_group!(benches, bench_add);
criterion_main!(benches);

