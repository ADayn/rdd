use std::collections::HashMap;

#[derive(Debug)]
enum BOp {
    And,
    Or
}

#[derive(Debug)]
enum Expr {
    Lit(bool),
    Var(String),
    Not(Box<Expr>),
    Binary(Box<Expr>, BOp, Box<Expr>),
}

type Env = HashMap<String, bool>;

use crate::Expr::*;
use crate::BOp::*;

// fn op(bop: BOp) -> fn(bool, bool) -> bool {
// 	match bop {
// 		And => |x, y| -> x && y,
// 		Or  => |x, y| -> x || y,
// 	}
// }

fn free_vars(e: Expr, env: &Env) -> bool {
	match e {
		Lit(b) => b,
		Var(x) => match env.get(&x) {
			Some(&b) => b,
			_ => panic!(),
		},
		Not(e) => !eval(*e, env),
		Binary(e1, And, e2) => eval(*e1, env) && eval(*e2, env),
		Binary(e1, Or , e2) => eval(*e1, env) || eval(*e2, env),
		_ => panic!(),
	}
}

fn eval(e: Expr, env: &Env) -> bool {
	match e {
		Lit(b) => b,
		Var(x) => match env.get(&x) {
			Some(&b) => b,
			_ => panic!(),
		},
		Not(e) => !eval(*e, env),
		Binary(e1, And, e2) => eval(*e1, env) && eval(*e2, env),
		Binary(e1, Or , e2) => eval(*e1, env) || eval(*e2, env),
		_ => panic!(),
	}
}

fn main() {
	let x1 = "x1".to_string();

	let t: Expr = Lit(true);
	let f: Expr = Lit(false);
	let vx1: Expr = Var(x1.clone());
	let e: Expr = Binary(Box::new(f), And, Box::new(vx1));

	let empty: Env = HashMap::new();
	let mut env: Env = HashMap::new();
	env.insert(x1.clone(), true);

    println!("Hello, world!");
    println!("Evaluating: {:?}", e);
    println!("Result: {:?}", eval(e, &env));
}
