#[macro_use]
extern crate criterion;

use criterion::{Criterion, BenchmarkId};
use rdd::expr;
use rdd::naive_bdd;
use rdd::neg_arc_bdd;

// naive vs 
fn no_attr_vs_comp(c: &mut Criterion) {
    // c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
    let mut group = c.benchmark_group("no_attr_vs_comp");
    for bits in [2, 3, 4, 5, 6].iter() {
        group.bench_with_input(BenchmarkId::new("No Attr (Degenerate order)", bits), &bits, |b, &bits| {
            let (comp, ord_bad, _) = expr::gen::comparator(*bits);
            b.iter(|| {
            	naive_bdd::from(&comp, &ord_bad)
            });
        });
        group.bench_with_input(BenchmarkId::new("No Attr (Ideal order)", bits), &bits, |b, &bits| {
            let (comp, _, ord_good) = expr::gen::comparator(*bits);
            b.iter(|| {
            	naive_bdd::from(&comp, &ord_good)
            });
        });
        group.bench_with_input(BenchmarkId::new("Complement Attr (Degenerate order)", bits), &bits, |b, &bits| {
            let (comp, ord_bad, _) = expr::gen::comparator(*bits);
            b.iter(|| {
            	neg_arc_bdd::from(&comp, &ord_bad)
            });
        });
        group.bench_with_input(BenchmarkId::new("Complement Attr (Ideal order)", bits), &bits, |b, &bits| {
            let (comp, _, ord_good) = expr::gen::comparator(*bits);
            b.iter(|| {
            	neg_arc_bdd::from(&comp, &ord_good)
            });
        });
    }
    group.finish();
}

fn naive_vs_support_vs_simplifiedsupport(c: &mut Criterion) {
    // c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
    let mut group = c.benchmark_group("naive_vs_support_vs_simplifiedsupport");
    for bits in [2, 3, 4, 5, 6].iter() {
        group.bench_with_input(BenchmarkId::new("Naive", bits), &bits, |b, &bits| {
            let (comp, ord_bad, _) = expr::gen::comparator(*bits);
            b.iter(|| {
            	neg_arc_bdd::from(&comp, &ord_bad)
            });
        });
        group.bench_with_input(BenchmarkId::new("Support", bits), &bits, |b, &bits| {
            let (comp, ord_bad, _) = expr::gen::comparator(*bits);
            b.iter(|| {
            	neg_arc_bdd::from_support(&comp, &ord_bad)
            });
        });
        group.bench_with_input(BenchmarkId::new("Support Simplified", bits), &bits, |b, &bits| {
            let (comp, ord_bad, _) = expr::gen::comparator(*bits);
            b.iter(|| {
            	neg_arc_bdd::from_support_simplified(&comp, &ord_bad)
            });
        });
    }
    group.finish();
}

fn hash_vs_btree_vs_vec_coff(c: &mut Criterion) {
    // c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
    let mut group = c.benchmark_group("hash_vs_btree_vs_vec_coff");
    for bits in [2, 3, 4, 5, 6].iter() {
        group.bench_with_input(BenchmarkId::new("Hash", bits), &bits, |b, &bits| {
            let (comp, ord_bad, _) = expr::gen::comparator(*bits);
            b.iter(|| {
            	neg_arc_bdd::from_support(&comp, &ord_bad)
            });
        });
        group.bench_with_input(BenchmarkId::new("BTree", bits), &bits, |b, &bits| {
            let (comp, ord_bad, _) = expr::gen::comparator(*bits);
            b.iter(|| {
            	neg_arc_bdd::from_support_btree(&comp, &ord_bad)
            });
        });
        group.bench_with_input(BenchmarkId::new("Vec", bits), &bits, |b, &bits| {
            let (comp, ord_bad, _) = expr::gen::comparator(*bits);
            b.iter(|| {
            	neg_arc_bdd::from_support_no_hash(&comp, &ord_bad)
            });
        });
    }
    group.finish();
}

fn hash_vs_btree_memoization(c: &mut Criterion) {
    // c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
    let mut group = c.benchmark_group("hash_vs_btree_memoization");
    for bits in [2, 3, 4, 5, 6].iter() {
        group.bench_with_input(BenchmarkId::new("Hash", bits), &bits, |b, &bits| {
            let (comp, ord_bad, _) = expr::gen::comparator(*bits);
            b.iter(|| {
            	naive_bdd::from(&comp, &ord_bad)
            });
        });
        group.bench_with_input(BenchmarkId::new("BTree", bits), &bits, |b, &bits| {
            let (comp, ord_bad, _) = expr::gen::comparator(*bits);
            b.iter(|| {
            	naive_bdd::from_btree_mem(&comp, &ord_bad)
            });
        });
    }
    group.finish();
}

criterion_group!(benches, no_attr_vs_comp, naive_vs_support_vs_simplifiedsupport, hash_vs_btree_vs_vec_coff);
criterion_main!(benches);
