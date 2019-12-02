#[macro_use]
extern crate criterion;

use criterion::{Criterion, BenchmarkId};
use rdd::expr;
use rdd::naive_bdd;
use rdd::neg_arc_bdd;
use rdd::combin_bdd;
use std::time::Duration;

macro_rules! mk_group {
	( $group:ident, $c:expr, $label:expr, $samps:expr ) => (
	    let plot_config = criterion::PlotConfiguration::default()
	        .summary_scale(criterion::AxisScale::Logarithmic);
		let mut $group = $c.benchmark_group($label);
    	$group.plot_config(plot_config);
		$group.sample_size($samps);
		$group.warm_up_time(Duration::from_millis(50));
		$group.measurement_time(Duration::from_millis(1000));
		$group.nresamples(50);
	)
}

fn no_attr_vs_comp(c: &mut Criterion) {
    mk_group!(group, c, "no_attr_vs_comp", 10);
    for bits in [2, 4, 6, 8].iter() {
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

fn comp_vs_support(c: &mut Criterion) {
    mk_group!(group, c, "comp_vs_support", 10);
    for bits in [4, 6, 8, 10, 12].iter() {
        group.bench_with_input(BenchmarkId::new("Complement (Degenerate order)", bits), &bits, |b, &bits| {
            let (comp, ord_bad, _) = expr::gen::comparator(*bits);
            b.iter(|| {
            	neg_arc_bdd::from(&comp, &ord_bad)
            });
        });
        group.bench_with_input(BenchmarkId::new("Complement (Ideal order)", bits), &bits, |b, &bits| {
            let (comp, _, ord_good) = expr::gen::comparator(*bits);
            b.iter(|| {
            	neg_arc_bdd::from(&comp, &ord_good)
            });
        });
        group.bench_with_input(BenchmarkId::new("Support (Degenerate order)", bits), &bits, |b, &bits| {
            let (comp, ord_bad, _) = expr::gen::comparator(*bits);
            b.iter(|| {
            	neg_arc_bdd::from_support(&comp, &ord_bad)
            });
        });
        group.bench_with_input(BenchmarkId::new("Support (Ideal order)", bits), &bits, |b, &bits| {
            let (comp, _, ord_good) = expr::gen::comparator(*bits);
            b.iter(|| {
            	neg_arc_bdd::from_support(&comp, &ord_good)
            });
        });
    }
    group.finish();
}

fn hash_vs_btree_vs_vec_coff(c: &mut Criterion) {
    mk_group!(group, c, "hash_vs_btree_vs_vec_coff", 10);
    for bits in [4, 6, 8, 10, 12].iter() {
        group.bench_with_input(BenchmarkId::new("Complement (Degeneratee)", bits), &bits, |b, &bits| {
            let (comp, ord_bad, _) = expr::gen::comparator(*bits);
            b.iter(|| {
            	neg_arc_bdd::from(&comp, &ord_bad)
            });
        });
        group.bench_with_input(BenchmarkId::new("Support w/ Hash (Degeneratee)", bits), &bits, |b, &bits| {
            let (comp, ord_bad, _) = expr::gen::comparator(*bits);
            b.iter(|| {
            	neg_arc_bdd::from_support(&comp, &ord_bad)
            });
        });
        group.bench_with_input(BenchmarkId::new("Support w/ BTree (Degeneratee)", bits), &bits, |b, &bits| {
            let (comp, ord_bad, _) = expr::gen::comparator(*bits);
            b.iter(|| {
            	neg_arc_bdd::from_support_btree(&comp, &ord_bad)
            });
        });
        group.bench_with_input(BenchmarkId::new("Support w/ Vec (Degeneratee)", bits), &bits, |b, &bits| {
            let (comp, ord_bad, _) = expr::gen::comparator(*bits);
            b.iter(|| {
            	neg_arc_bdd::from_support_vec(&comp, &ord_bad)
            });
        });
    }
    group.finish();
}

fn support_vs_supportsimplified(c: &mut Criterion) {
    mk_group!(group, c, "support_vs_supportsimplified", 10);
    for bits in [4, 6, 8, 10, 12].iter() {
        group.bench_with_input(BenchmarkId::new("Support (Degenerate order)", bits), &bits, |b, &bits| {
            let (comp, ord_bad, _) = expr::gen::comparator(*bits);
            b.iter(|| {
            	neg_arc_bdd::from_support(&comp, &ord_bad)
            });
        });
        group.bench_with_input(BenchmarkId::new("Support (Ideal order)", bits), &bits, |b, &bits| {
            let (comp, _, ord_good) = expr::gen::comparator(*bits);
            b.iter(|| {
            	neg_arc_bdd::from_support(&comp, &ord_good)
            });
        });
        group.bench_with_input(BenchmarkId::new("Support Simplified (Degenerate order)", bits), &bits, |b, &bits| {
            let (comp, ord_bad, _) = expr::gen::comparator(*bits);
            b.iter(|| {
            	neg_arc_bdd::from_support_simplified(&comp, &ord_bad)
            });
        });
        group.bench_with_input(BenchmarkId::new("Support Simplified (Ideal order)", bits), &bits, |b, &bits| {
            let (comp, _, ord_good) = expr::gen::comparator(*bits);
            b.iter(|| {
            	neg_arc_bdd::from_support_simplified(&comp, &ord_good)
            });
        });
    }
    group.finish();
}

fn supportsimplified_vs_combinatorial(c: &mut Criterion) {
    mk_group!(group, c, "supportsimplified_vs_combinatorial", 10);
    for bits in [4, 6, 8, 10, 12, 14].iter() {
        group.bench_with_input(BenchmarkId::new("Support Simplified (Degenerate order)", bits), &bits, |b, &bits| {
            let (comp, ord_bad, _) = expr::gen::comparator(*bits);
            b.iter(|| {
            	neg_arc_bdd::from_support_simplified(&comp, &ord_bad)
            });
        });
        group.bench_with_input(BenchmarkId::new("Support Simplified (Ideal order)", bits), &bits, |b, &bits| {
            let (comp, _, ord_good) = expr::gen::comparator(*bits);
            b.iter(|| {
            	neg_arc_bdd::from_support_simplified(&comp, &ord_good)
            });
        });
        group.bench_with_input(BenchmarkId::new("Combinatorial (Degenerate order)", bits), &bits, |b, &bits| {
            let (comp, ord_bad, _) = expr::gen::comparator(*bits);
            b.iter(|| {
            	combin_bdd::from_combinatorial(&comp, &ord_bad)
            });
        });
        group.bench_with_input(BenchmarkId::new("Combinatorial (Ideal order)", bits), &bits, |b, &bits| {
            let (comp, _, ord_good) = expr::gen::comparator(*bits);
            b.iter(|| {
            	combin_bdd::from_combinatorial(&comp, &ord_good)
            });
        });
    }
    group.finish();
}

// fn hash_vs_btree_lookup(c: &mut Criterion) {
//     mk_group!(group, c, "hash_vs_btree_lookup", 10);
//     for bits in [4, 6, 8, 10, 12].iter() {
//         group.bench_with_input(BenchmarkId::new("Hash", bits), &bits, |b, &bits| {
//             let (comp, ord_bad, _) = expr::gen::comparator(*bits);
//             b.iter(|| {
//             	naive_bdd::from(&comp, &ord_bad)
//             });
//         });
//         group.bench_with_input(BenchmarkId::new("BTree", bits), &bits, |b, &bits| {
//             let (comp, ord_bad, _) = expr::gen::comparator(*bits);
//             b.iter(|| {
//             	naive_bdd::from_btree_mem(&comp, &ord_bad)
//             });
//         });
//     }
//     group.finish();
// }

criterion_group!(benches, no_attr_vs_comp,
	                      comp_vs_support,
	                      hash_vs_btree_vs_vec_coff,
	                      support_vs_supportsimplified,
	                      supportsimplified_vs_combinatorial);
criterion_main!(benches);
