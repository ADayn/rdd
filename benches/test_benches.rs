#[macro_use]
extern crate criterion;

use criterion::{Criterion, BenchmarkId};
use rdd::expr;
use rdd::expr_rc;
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

fn clone_box_vs_rc(c: &mut Criterion) {
    mk_group!(group, c, "clone_box_vs_rc", 10);
    for bits in [1, 2, 3, 4, 6, 8, 10, 15, 20, 25].iter() {
        group.bench_with_input(BenchmarkId::new("Clone Box", bits), &bits, |b, &bits| {
            let (comp, _, _) = expr::gen::comparator(*bits);
            b.iter(|| {
                comp.clone();
            });
        });
        group.bench_with_input(BenchmarkId::new("Clone Rc", bits), &bits, |b, &bits| {
            let (comp, _, _) = expr::gen::comparator(*bits);
            let comp_rc = expr_rc::to_expr_rc(&comp);
            b.iter(|| {
                comp_rc.clone();
            });
        });
    }
    group.finish();
}

fn eval_box_vs_rc(c: &mut Criterion) {
    mk_group!(group, c, "eval_box_vs_rc", 10);
    for bits in [1, 2, 3, 4, 6, 8, 10, 15, 20, 25].iter() {
        let env = vec![false; *bits * 2];
        group.bench_with_input(BenchmarkId::new("Eval Box", bits), &bits, |b, &bits| {
            let (comp, _, _) = expr::gen::comparator(*bits);
            b.iter(|| {
                expr::eval(&comp, &env);
            });
        });
        group.bench_with_input(BenchmarkId::new("Eval Rc", bits), &bits, |b, &bits| {
            let (comp, _, _) = expr::gen::comparator(*bits);
            let comp_rc = expr_rc::to_expr_rc(&comp);
            b.iter(|| {
                expr_rc::eval_rc(&comp_rc, &env);
            });
        });
    }
    group.finish();
}

fn simplify_box_vs_rc_deep(c: &mut Criterion) {
    mk_group!(group, c, "simplify_box_vs_rc_deep", 10);
    for bits in [1, 2, 3, 4, 6, 8, 10, 15, 20, 25].iter() {
        let env = vec![false; *bits * 2];
        group.bench_with_input(BenchmarkId::new("Simplify Box (low simplification)", bits), &bits, |b, &bits| {
            let (comp, _, _) = expr::gen::comparator(*bits);
            b.iter(|| {
                let e_1 = neg_arc_bdd::subst_and_simplify(&comp, 0, false);
                let e_1 = neg_arc_bdd::subst_and_simplify(&comp, 1, false);

            });
        });
        group.bench_with_input(BenchmarkId::new("Simplify Box (high simplification)", bits), &bits, |b, &bits| {
            let (comp, _, _) = expr::gen::comparator(*bits);
            b.iter(|| {
                let e_1 = neg_arc_bdd::subst_and_simplify(&comp, 0, false);
                let e_1 = neg_arc_bdd::subst_and_simplify(&comp, 1, true);
            });
        });
        group.bench_with_input(BenchmarkId::new("Simplify Rc (low simplification)", bits), &bits, |b, &bits| {
            let (comp, _, _) = expr::gen::comparator(*bits);
            let comp_rc = expr_rc::to_expr_rc(&comp);
            b.iter(|| {
                let e_1 = neg_arc_bdd::subst_and_simplify_rc(&comp_rc, 0, false);
                let e_1 = neg_arc_bdd::subst_and_simplify_rc(&comp_rc, 1, false);

            });
        });
        group.bench_with_input(BenchmarkId::new("Simplify Rc (high simplification)", bits), &bits, |b, &bits| {
            let (comp, _, _) = expr::gen::comparator(*bits);
            let comp_rc = expr_rc::to_expr_rc(&comp);
            b.iter(|| {
                let e_1 = neg_arc_bdd::subst_and_simplify_rc(&comp_rc, 0, false);
                let e_1 = neg_arc_bdd::subst_and_simplify_rc(&comp_rc, 1, true);
            });
        });
    }
    group.finish();
}

fn simplify_box_vs_rc_shallow(c: &mut Criterion) {
    mk_group!(group, c, "simplify_box_vs_rc_shallow", 10);
    for bits in [1, 2, 3, 4, 6, 8, 10, 15, 20, 25].iter() {
        let env = vec![false; *bits * 2];
        group.bench_with_input(BenchmarkId::new("Simplify Box (low simplification)", bits), &bits, |b, &bits| {
            let (comp, _, _) = expr::gen::comparator(*bits);
            b.iter(|| {
                let e_1 = neg_arc_bdd::subst_and_simplify(&comp, (*bits - 1) * 2, false);
                let e_1 = neg_arc_bdd::subst_and_simplify(&comp, (*bits - 1) * 2 + 1, false);

            });
        });
        group.bench_with_input(BenchmarkId::new("Simplify Box (high simplification)", bits), &bits, |b, &bits| {
            let (comp, _, _) = expr::gen::comparator(*bits);
            b.iter(|| {
                let e_1 = neg_arc_bdd::subst_and_simplify(&comp, (*bits - 1) * 2, false);
                let e_1 = neg_arc_bdd::subst_and_simplify(&comp, (*bits - 1) * 2 + 1, true);
            });
        });
        group.bench_with_input(BenchmarkId::new("Simplify Rc (low simplification)", bits), &bits, |b, &bits| {
            let (comp, _, _) = expr::gen::comparator(*bits);
            let comp_rc = expr_rc::to_expr_rc(&comp);
            b.iter(|| {
                let e_1 = neg_arc_bdd::subst_and_simplify_rc(&comp_rc, (*bits - 1) * 2, false);
                let e_1 = neg_arc_bdd::subst_and_simplify_rc(&comp_rc, (*bits - 1) * 2 + 1, false);

            });
        });
        group.bench_with_input(BenchmarkId::new("Simplify Rc (high simplification, shallow expr)", bits), &bits, |b, &bits| {
            let (comp, _, _) = expr::gen::comparator(*bits);
            let comp_rc = expr_rc::to_expr_rc(&comp);
            b.iter(|| {
                let e_1 = neg_arc_bdd::subst_and_simplify_rc(&comp_rc, (*bits - 1) * 2, false);
                let e_1 = neg_arc_bdd::subst_and_simplify_rc(&comp_rc, (*bits - 1) * 2 + 1, true);
            });
        });
    }
    group.finish();
}

fn no_attr_vs_comp(c: &mut Criterion) {
    mk_group!(group, c, "no_attr_vs_comp", 10);
    for bits in [2, 4, 6, 8, 10, 12].iter() {
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
        group.bench_with_input(BenchmarkId::new("Complement (Degenerate order)", bits), &bits, |b, &bits| {
            let (comp, ord_bad, _) = expr::gen::comparator(*bits);
            b.iter(|| {
            	neg_arc_bdd::from(&comp, &ord_bad)
            });
        });
        group.bench_with_input(BenchmarkId::new("Support w/ Hash (Degenerate order)", bits), &bits, |b, &bits| {
            let (comp, ord_bad, _) = expr::gen::comparator(*bits);
            b.iter(|| {
            	neg_arc_bdd::from_support(&comp, &ord_bad)
            });
        });
        group.bench_with_input(BenchmarkId::new("Support w/ BTree (Degenerate order)", bits), &bits, |b, &bits| {
            let (comp, ord_bad, _) = expr::gen::comparator(*bits);
            b.iter(|| {
            	neg_arc_bdd::from_support_btree(&comp, &ord_bad)
            });
        });
        group.bench_with_input(BenchmarkId::new("Support w/ Vec (Degenerate order)", bits), &bits, |b, &bits| {
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
    for bits in [4, 6, 8, 10, 12, 14].iter() {
        group.bench_with_input(BenchmarkId::new("Support w/ Vec (Degenerate order)", bits), &bits, |b, &bits| {
            let (comp, ord_bad, _) = expr::gen::comparator(*bits);
            b.iter(|| {
                neg_arc_bdd::from_support_vec(&comp, &ord_bad)
            });
        });
        group.bench_with_input(BenchmarkId::new("Support w/ Vec (Ideal order)", bits), &bits, |b, &bits| {
            let (comp, _, ord_good) = expr::gen::comparator(*bits);
            b.iter(|| {
                neg_arc_bdd::from_support_vec(&comp, &ord_good)
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

fn supportsimplified_box_vs_rc(c: &mut Criterion) {
    mk_group!(group, c, "supportsimplified_box_vs_rc", 10);
    for bits in [4, 6, 8, 10, 12, 14].iter() {
        group.bench_with_input(BenchmarkId::new("Box (Degenerate order)", bits), &bits, |b, &bits| {
            let (comp, ord_bad, _) = expr::gen::comparator(*bits);
            b.iter(|| {
                neg_arc_bdd::from_support_simplified(&comp, &ord_bad)
            });
        });
        group.bench_with_input(BenchmarkId::new("Box (Ideal order)", bits), &bits, |b, &bits| {
            let (comp, _, ord_good) = expr::gen::comparator(*bits);
            b.iter(|| {
                neg_arc_bdd::from_support_simplified(&comp, &ord_good)
            });
        });
        group.bench_with_input(BenchmarkId::new("Rc (Degenerate order)", bits), &bits, |b, &bits| {
            let (comp, ord_bad, _) = expr::gen::comparator(*bits);
            let comp_rc = expr_rc::to_expr_rc(&comp);
            b.iter(|| {
                neg_arc_bdd::from_support_simplified_rc(&comp_rc, &ord_bad)
            });
        });
        group.bench_with_input(BenchmarkId::new("Rc (Ideal order)", bits), &bits, |b, &bits| {
            let (comp, _, ord_good) = expr::gen::comparator(*bits);
            let comp_rc = expr_rc::to_expr_rc(&comp);
            b.iter(|| {
                neg_arc_bdd::from_support_simplified_rc(&comp_rc, &ord_good)
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

fn combinatorial_btree_vs_hash(c: &mut Criterion) {
    mk_group!(group, c, "combinatorial_btree_vs_hash", 10);
    for bits in [4, 6, 8, 10, 12, 14, 16, 18].iter() {
        group.bench_with_input(BenchmarkId::new("BTree (Degenerate order)", bits), &bits, |b, &bits| {
            let (comp, ord_bad, _) = expr::gen::comparator(*bits);
            b.iter(|| {
            	combin_bdd::from_combinatorial(&comp, &ord_bad)
            });
        });
        group.bench_with_input(BenchmarkId::new("BTree (Ideal order)", bits), &bits, |b, &bits| {
            let (comp, _, ord_good) = expr::gen::comparator(*bits);
            b.iter(|| {
            	combin_bdd::from_combinatorial(&comp, &ord_good)
            });
        });
        group.bench_with_input(BenchmarkId::new("Hash (Degenerate order)", bits), &bits, |b, &bits| {
            let (comp, ord_bad, _) = expr::gen::comparator(*bits);
            b.iter(|| {
            	combin_bdd::from_combinatorial_hash(&comp, &ord_bad)
            });
        });
        group.bench_with_input(BenchmarkId::new("Hash (Ideal order)", bits), &bits, |b, &bits| {
            let (comp, _, ord_good) = expr::gen::comparator(*bits);
            b.iter(|| {
            	combin_bdd::from_combinatorial_hash(&comp, &ord_good)
            });
        });
    }
    group.finish();
}

fn all_degenerate(c: &mut Criterion) {
    mk_group!(group, c, "all_degenerate", 10);
    for bits in [4, 6, 8, 10, 12].iter() {
        group.bench_with_input(BenchmarkId::new("No Attr (Degenerate order)", bits), &bits, |b, &bits| {
            let (comp, ord_bad, _) = expr::gen::comparator(*bits);
            b.iter(|| {
            	naive_bdd::from(&comp, &ord_bad)
            });
        });
        group.bench_with_input(BenchmarkId::new("Complement (Degenerate order)", bits), &bits, |b, &bits| {
            let (comp, ord_bad, _) = expr::gen::comparator(*bits);
            b.iter(|| {
            	neg_arc_bdd::from(&comp, &ord_bad)
            });
        });
        group.bench_with_input(BenchmarkId::new("Support w/ Vec (Degenerate order)", bits), &bits, |b, &bits| {
            let (comp, ord_bad, _) = expr::gen::comparator(*bits);
            b.iter(|| {
            	neg_arc_bdd::from_support_vec(&comp, &ord_bad)
            });
        });
        group.bench_with_input(BenchmarkId::new("Support Simplified (Degenerate order)", bits), &bits, |b, &bits| {
            let (comp, ord_bad, _) = expr::gen::comparator(*bits);
            b.iter(|| {
            	neg_arc_bdd::from_support_simplified(&comp, &ord_bad)
            });
        });
        group.bench_with_input(BenchmarkId::new("Combinatorial (Degenerate order)", bits), &bits, |b, &bits| {
            let (comp, ord_bad, _) = expr::gen::comparator(*bits);
            b.iter(|| {
            	combin_bdd::from_combinatorial_hash(&comp, &ord_bad)
            });
        });
    }
    group.finish();
}

fn all_ideal(c: &mut Criterion) {
    mk_group!(group, c, "all_ideal", 10);
    for bits in [4, 6, 8, 10, 12].iter() {
        group.bench_with_input(BenchmarkId::new("No Attr (Ideal order)", bits), &bits, |b, &bits| {
            let (comp, _, ord_good) = expr::gen::comparator(*bits);
            b.iter(|| {
            	naive_bdd::from(&comp, &ord_good)
            });
        });
        group.bench_with_input(BenchmarkId::new("Complement (Ideal order)", bits), &bits, |b, &bits| {
            let (comp, _, ord_good) = expr::gen::comparator(*bits);
            b.iter(|| {
            	neg_arc_bdd::from(&comp, &ord_good)
            });
        });
        group.bench_with_input(BenchmarkId::new("Support w/ Vec (Ideal order)", bits), &bits, |b, &bits| {
            let (comp, _, ord_good) = expr::gen::comparator(*bits);
            b.iter(|| {
            	neg_arc_bdd::from_support_vec(&comp, &ord_good)
            });
        });
        group.bench_with_input(BenchmarkId::new("Support Simplified (Ideal order)", bits), &bits, |b, &bits| {
            let (comp, _, ord_good) = expr::gen::comparator(*bits);
            b.iter(|| {
            	neg_arc_bdd::from_support_simplified(&comp, &ord_good)
            });
        });
        group.bench_with_input(BenchmarkId::new("Combinatorial (Ideal order)", bits), &bits, |b, &bits| {
            let (comp, _, ord_good) = expr::gen::comparator(*bits);
            b.iter(|| {
            	combin_bdd::from_combinatorial_hash(&comp, &ord_good)
            });
        });
    }
    group.finish();
}
fn all_for_key(c: &mut Criterion) {
    mk_group!(group, c, "all_for_key", 10);
    for bits in [2, 3, 4, ].iter() {
        group.bench_with_input(BenchmarkId::new("No Attr", bits), &bits, |b, &bits| {
            let (comp, _, ord_good) = expr::gen::comparator(*bits);
            b.iter(|| {
            	naive_bdd::from(&comp, &ord_good)
            });
        });
        group.bench_with_input(BenchmarkId::new("Complement", bits), &bits, |b, &bits| {
            let (comp, _, ord_good) = expr::gen::comparator(*bits);
            b.iter(|| {
            	neg_arc_bdd::from(&comp, &ord_good)
            });
        });
        group.bench_with_input(BenchmarkId::new("Support w/ Vec", bits), &bits, |b, &bits| {
            let (comp, _, ord_good) = expr::gen::comparator(*bits);
            b.iter(|| {
            	neg_arc_bdd::from_support_vec(&comp, &ord_good)
            });
        });
        group.bench_with_input(BenchmarkId::new("Support Simplified", bits), &bits, |b, &bits| {
            let (comp, _, ord_good) = expr::gen::comparator(*bits);
            b.iter(|| {
            	neg_arc_bdd::from_support_simplified(&comp, &ord_good)
            });
        });
        group.bench_with_input(BenchmarkId::new("Combinatorial", bits), &bits, |b, &bits| {
            let (comp, _, ord_good) = expr::gen::comparator(*bits);
            b.iter(|| {
            	combin_bdd::from_combinatorial_hash(&comp, &ord_good)
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

criterion_group!(benches, clone_box_vs_rc,
                          eval_box_vs_rc,
                          simplify_box_vs_rc_deep,
                          simplify_box_vs_rc_shallow,
                          no_attr_vs_comp,
	                      comp_vs_support,
	                      hash_vs_btree_vs_vec_coff,
	                      support_vs_supportsimplified,
                          supportsimplified_box_vs_rc,
	                      supportsimplified_vs_combinatorial,
	                      combinatorial_btree_vs_hash,
	                      all_degenerate,
	                      all_ideal,
	                      all_for_key);
criterion_main!(benches);
