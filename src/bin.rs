// check support
// simplify expr as you go before support check 
// String -> int
// Add complement arcs
// btreemap vs hashmap

// Remaining Performace iprovements:
// vec memo vs hashtable vs bdd
// sat how many / find one
// Find optimal ordering
// slice vs vec dequeue vs linked list
// combinatorial bbdd


use rdd::expr::*;
use Expr::*;
use BOp::*;

use rdd::naive_bdd;
use rdd::neg_arc_bdd;
use rdd::combin_bdd;

macro_rules! run_bdd {
	( $n:expr, $from:expr, $print:expr ) => {
		let (comp, comp_var_ord_bad, comp_var_ord_good) = gen::comparator($n);
		if $print {
			println!("e: {:?}\ngood: {:?}\nbad: {:?}", comp, comp_var_ord_good, comp_var_ord_bad);
		}
		println!("***** Making good bdd...");
		let good = $from(&comp, &comp_var_ord_good);
		if $print {
			println!("bdd: {}", good.textual_repr());
		}
		println!("***** Making bad bdd...");
		let bad = $from(&comp, &comp_var_ord_bad);
		if $print {
			println!("bdd: {}", bad.textual_repr());
		}
		println!("***** {} - good bdd size: {:?}, bad bdd size: {:?}", $n, good.size(), bad.size());
	};
	
}
macro_rules! run_bdd_expr {
	( $e:expr, $order:expr, $from:expr, $print:expr ) => {
		if $print {
			println!("e: {:?}\norder: {:?}", $e, $order);
		}
		println!("***** Making bdd...");
		let bdd = $from($e, $order);
		if $print {
			println!("bdd: {}", bdd.textual_repr());
		}
		println!("***** bdd size: {:?}\n", bdd.size());
	};
	
}

macro_rules! test_bdd {
	( $test:expr, $from:expr, $print:expr ) => {
		let (comp, comp_var_ord_bad, comp_var_ord_good) = gen::comparator($test.len() / 2);
		let env = $test;

		let good_naive_bdd = naive_bdd::from(&comp, &comp_var_ord_good);
		let good_test_bdd = $from(&comp, &comp_var_ord_good);
		let good_naive_eval = good_naive_bdd.eval(&env);
		let good_test_eval = good_test_bdd.eval(&env);
		println!("(good) {} =? {}", good_naive_eval, good_test_eval);
		assert!(good_naive_eval == good_test_eval);

		let bad_naive_bdd = naive_bdd::from(&comp, &comp_var_ord_bad);
		let bad_test_bdd = $from(&comp, &comp_var_ord_bad);
		let bad_naive_eval = bad_naive_bdd.eval(&env);
		let bad_test_eval = bad_test_bdd.eval(&env);
		println!("(bad) {} =? {}", bad_naive_eval, bad_test_eval);
		assert!(bad_naive_eval == bad_test_eval);
	};
}

fn main() {
	// let x1 = "a".to_string();
	// let x2 = "b".to_string();
	// let x3 = "c".to_string();
	// let x4 = "d".to_string();

	// let vx1: Expr = Var(x1.clone());
	// let vx2: Expr = Var(x2.clone());
	// let vx3: Expr = Var(x3.clone());
	// let vx4: Expr = Var(x4.clone());
	// // let e_old: Expr = Binary(Box::new(t()), And, Box::new(vx1));
	// let mut e: Expr = Binary(Box::new(vx1), Or, Box::new(vx2));
	// e = Binary(Box::new(e.clone()), Or, Box::new(e.clone()));
	// e = Binary(Box::new(e.clone()), XOr, Box::new(e.clone()));
	// e = Binary(Box::new(e.clone()), And, Box::new(Binary(Box::new(vx3), Or, Box::new(vx4))));
	// // let h: Expr = Binary(Box::new(g), Or, Box::new(f()));

	// let mut env: Env = HashMap::new();
	// env.insert(x1.clone(), true);

	// println!("Hello, world!");
	// println!("Evaluating: {:?}", &e);
	// println!("Result: {:?}", eval(&e, &env));
	// let bdd = from(&e, &[x1.clone(), x2.clone(), x3.clone(), x4.clone()]);
	// println!("BDD: {:?}", &bdd);

	// for timing tests:
	// run_bdd!(2);
	// thread::sleep(Duration::from_secs(1));
	// run_bdd!(5);
	// thread::sleep(Duration::from_secs(1));
	// println!("Naive:");
	// run_bdd!(2, naive_bdd::from, true);
	// println!("\n\nNeg Arc:");
	// run_bdd!(3, neg_arc_bdd::from, true);



	// run_bdd!(13, neg_arc_bdd::from_support_no_hash, false);
	run_bdd!(20, combin_bdd::from_combinatorial, false);
	// let e = bin(Var(1), Or, Var(0));
	// let ord = &[0, 1];

	// run_bdd_expr!(&e, ord, neg_arc_bdd::from_support_no_hash, true);
	// run_bdd_expr!(&e, ord, combin_bdd::from_combinatorial, true);

	// test_bdd!(vec![true, true,
	//                false, false,
	//                true, true,], combin_bdd::from_combinatorial, true);

	// test_bdd!(vec![true, true,
	//                false, false,
	//                false, false,
	//                true, true,
	//                false, false,], naive_bdd::from_btree_mem, true);

	// test_bdd!(vec![true, false,
	//                false, true,
	//                false, false,
	//                true, true,
	//                false, false,], naive_bdd::from_btree_mem, true);

	// test_bdd!(vec![true, false,
	//                false, true,
	//                false, true,
	//                true, false,
	//                false, true,], naive_bdd::from_btree_mem, true);

	// test_bdd!(vec![true, false,
	//                false, true,
	//                false, false,
	//                true, true,
	//                false, false,
	//                true, false,
	//                false, false,
	//                true, true,
	//                false, false,
	//                true, true,
	//                false, false,], naive_bdd::from_btree_mem, true);

	// test_bdd!(vec![true, true,
	//                false, false,
	//                false, false,
	//                true, true,
	//                false, false,
	//                true, true,
	//                false, true,
	//                false, true,
	//                true, false,
	//                false, false,
	//                false, false,], naive_bdd::from_btree_mem, true);
}
