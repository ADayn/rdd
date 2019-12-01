use std::collections::HashMap;
use std::collections::HashSet;

// Performace iprovements:
// String -> &str
// vec search -> hashtable
// Add complement arcs
// sat how many / find one
// Find optimal ordering

use rdd::expr::*;
use Expr::*;
use BOp::*;

use rdd::naive_bdd;
use rdd::neg_arc_bdd;

macro_rules! run_bdd {
	( $n:expr, $from:expr, $print:expr ) => {
		let (comp, comp_var_ord_bad, comp_var_ord_good) = gen::comparator($n);
		if $print {
			println!("e: {:?}\ngood: {:?}\nbad:{:?}", comp, comp_var_ord_good, comp_var_ord_bad);
		}
		println!("\n***** Making good bdd...");
		let good = $from(&comp, &comp_var_ord_good);
		if $print {
			println!("head: {:?}\nbdd: {}", good.f, good.textual_repr());
		}
		println!("\n***** Making bad bdd...");
		let bad = $from(&comp, &comp_var_ord_bad);
		if $print {
			println!("head: {:?}\nbdd: {}", bad.f, bad.textual_repr());
		}
		println!("\n***** {} - good bdd size: {:?}, bad bdd size: {:?}", $n, good.nodes.len(), bad.nodes.len());
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
	println!("\n\nNeg Arc:");
	run_bdd!(2, neg_arc_bdd::from, true);
}
