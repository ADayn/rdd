use std::collections::HashMap;
use std::collections::HashSet;

/*
 * Expressions
 */
#[derive(Clone, Debug)]
pub enum BOp {
    And,
    Or,
    XOr
}

#[derive(Clone, Debug)]
pub enum Expr {
    Lit(bool),
    Var(String),
    Not(Box<Expr>),
    Binary(Box<Expr>, BOp, Box<Expr>),
}

pub type Env = HashMap<String, bool>;

use Expr::*;
use BOp::*;

pub fn not(e_1: Expr) -> Expr {
	Not(
		Box::new(e_1),
	)
}

pub fn bin(e_1: Expr, bop: BOp, e_2: Expr) -> Expr {
	Binary(
		Box::new(e_1),
		bop,
		Box::new(e_2),
	)
}

pub fn eval(e: &Expr, env: &Env) -> bool {
	match e {
		Lit(b) => *b,
		Var(x) => match env.get(x) {
			Some(&b) => b,
			_ => panic!(),
		},
		Not(e) => !eval(&e, env),
		Binary(e1, And, e2) => eval(&e1, env) && eval(&e2, env),
		Binary(e1, Or , e2) => eval(&e1, env) || eval(&e2, env),
		Binary(e1, XOr , e2) => eval(&e1, env) ^ eval(&e2, env),
	}
}

// fn free_vars(e: &Expr) -> HashSet<String> {
// 	let mut var_set = HashSet::new();
// 	find_vars_rec(e, &mut var_set);
// 	var_set
// }

// fn find_vars_rec(e: &Expr, found_vars: &mut HashSet<String>) {
// 	match e {
// 		Lit(_) => {}
// 		Var(x) => {
// 			found_vars.insert(x.clone());
// 		}
// 		Not(e_1) => {
// 			find_vars_rec(&e_1, found_vars);
// 		}
// 		Binary(e_1, _, e_2) => {
// 			find_vars_rec(&e_1, found_vars);
// 			find_vars_rec(&e_2, found_vars);
// 		},
// 	}
// }

pub mod gen {
	use crate::expr::*;

	use Expr::*;
	use BOp::*;

	// comparator function for xn..1 > yn..1
	pub fn comparator<'a>(n_bits: u32) -> (Expr, Vec<String>, Vec<String>) {
		let mut var_ord_bad: Vec<String> = vec![];
		let mut var_ord_good: Vec<String> = vec![];
		for i in 1..=n_bits {
			var_ord_bad.push(format!("y{}", i));
		}
		for i in 1..=n_bits {
			var_ord_bad.push(format!("x{}", i));
		}
		for i in 1..=n_bits {
			var_ord_good.push(format!("x{}", i));
			var_ord_good.push(format!("y{}", i));
		}
		(comparator_rec(n_bits), var_ord_bad, var_ord_good)
	}

	fn comparator_rec<'a>(n_bits: u32) -> Expr {
		// (xn && yn || !xn && !yn) && ...rec)
		if n_bits == 0 {
			Lit(true)
		} else {
			let xn = format!("x{}", n_bits);
			let yn = format!("y{}", n_bits);
			bin(
				bin(
					bin(
						not(Var(xn.clone())),
						And,
						not(Var(yn.clone())),
					),
					Or,
					bin(
						Var(xn.clone()),
						And,
						Var(yn.clone()),
					),
				),
				And,
				comparator_rec(n_bits - 1)
			)
		}
	}

	pub fn lt<'a>(n_bits: u32) -> (Expr, Vec<String>, Vec<String>) {
		let mut var_ord_bad: Vec<String> = vec![];
		let mut var_ord_good: Vec<String> = vec![];
		for i in 1..=n_bits {
			var_ord_bad.push(format!("y{}", i));
		}
		for i in 1..=n_bits {
			var_ord_bad.push(format!("x{}", i));
		}
		for i in 1..=n_bits {
			var_ord_good.push(format!("x{}", i));
			var_ord_good.push(format!("y{}", i));
		}
		(lt_rec(n_bits), var_ord_bad, var_ord_good)
	}

	fn lt_rec<'a>(n_bits: u32) -> Expr {
		// xn && !yn || (!xn && !yn && ...rec)
		if n_bits == 0 {
			Lit(true)
		} else {
			let xn = format!("x{}", n_bits);
			let yn = format!("y{}", n_bits);
			bin(
				bin(
					Var(xn.clone()),
					And,
					not(Var(yn.clone())),
				),
				Or,
				bin(
					bin(
						not(Var(xn.clone())),
						And,
						not(Var(yn.clone())),
					),
					And,
					comparator_rec(n_bits - 1),
				)
			)
		}
	}
}
