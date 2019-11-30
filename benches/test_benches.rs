#[macro_use]
extern crate criterion;

use criterion::{Criterion, BenchmarkId};
use rdd::expr;
use rdd::naive_bdd;

fn naive_bdd_comparator(c: &mut Criterion) {
    // c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
    let mut group = c.benchmark_group("naive_bdd_comparator");
    for bits in [1, 2, 3, 4, 5].iter() {
        group.bench_with_input(BenchmarkId::new("Degenerate order", bits), &bits, |b, &bits| {
            let (comp, ord_bad, _) = expr::gen::comparator(*bits);
            b.iter(|| {
            	naive_bdd::from(&comp, &ord_bad)
            });
        });
        group.bench_with_input(BenchmarkId::new("Efficient order", bits), &bits, |b, &bits| {
            let (comp, _, ord_good) = expr::gen::comparator(*bits);
            b.iter(|| {
            	naive_bdd::from(&comp, &ord_good)
            });
        });
    }
    group.finish();
}

criterion_group!(benches, naive_bdd_comparator);
criterion_main!(benches);
