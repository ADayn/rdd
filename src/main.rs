use std::collections::HashMap;
use std::collections::HashSet;

/*
 * Expressions
 */
#[derive(Clone, Debug)]
enum BOp {
    And,
    Or
}

#[derive(Clone, Debug)]
enum Expr {
    Lit(bool),
    Var(String),
    Not(Box<Expr>),
    Binary(Box<Expr>, BOp, Box<Expr>),
}

/*
 * BDDs
 */
type NodeIdx = usize;

#[derive(Debug)]
struct BDD {
	// list of functions
	f: usize,
	// list of nodes
	nodes: Vec<BDD_Node>,
	// order of vars
	// indices: HashMap<BDD_Node, NodeIdx>
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum BDD_Node {
	Internal {
		label: String,
		t_arc: NodeIdx,
		e_arc: NodeIdx,
		// e_complement: bool
	},
	Terminal(bool),
}

fn from(e: &Expr, var_ord: &[String]) -> BDD {
	let mut bdd = BDD {
		f    : 69,
		nodes: Vec::new(),
	};
	let mut cof_asgn: Env = HashMap::new();
	bdd.f = from_rec(e, var_ord, &mut cof_asgn, &mut bdd);
	bdd
}

fn from_rec(e: &Expr, rem_support: &[String], cof_asgn: &mut Env, bdd: &mut BDD) -> NodeIdx {
	if rem_support.is_empty() {
		// No more cofactors to check, eval and create node.
		let b = eval(e, cof_asgn);
		match bdd.nodes.iter().position(|x| x.clone() == Terminal(b)) {
			Some(i) => i,
			None => {
				// Terminal has not been created as node yet, make it
				bdd.nodes.push(Terminal(b));
				bdd.nodes.len() - 1
			},
		}
	} else {
		// Calculate positive and negative cofactors for current var and recurse
		let x = &rem_support[0];
		cof_asgn.insert(x.clone(), false);
		let neg_cof_node = from_rec(e, &rem_support[1..], cof_asgn, bdd);
		cof_asgn.insert(x.clone(), true);
		let pos_cof_node = from_rec(e, &rem_support[1..], cof_asgn, bdd);
		let new_node = Internal {
			label: x.clone(),
			t_arc: pos_cof_node,
			e_arc: neg_cof_node
		};
		match bdd.nodes.iter().position(|x| x.clone() == new_node) {
			Some(i) => i,
			None => {
				// Terminal has not been created as node yet, make it
				bdd.nodes.push(new_node);
				bdd.nodes.len() - 1
			},
		}
	}
}

type Env = HashMap<String, bool>;

use crate::Expr::*;
use crate::BDD_Node::*;
use crate::BOp::*;

fn main() {
	let x1 = "x1".to_string();

	fn t() -> Expr { Lit(true) };
	fn f() -> Expr { Lit(false) };
	let vx1: Expr = Var(x1.clone());
	let e: Expr = Binary(Box::new(t()), And, Box::new(vx1));
	// let h: Expr = Binary(Box::new(g), Or, Box::new(f()));

	let mut env: Env = HashMap::new();
	env.insert(x1.clone(), true);

    println!("Hello, world!");
    println!("Evaluating: {:?}", &e);
    println!("Free vars: {:?}", free_vars(&e));
    println!("Result: {:?}", eval(&e, &env));
    println!("BDD: {:?}", from(&e, &[x1.clone()]));
}

fn free_vars(e: &Expr) -> HashSet<String> {
	let mut var_set = HashSet::new();
	find_vars_rec(e, &mut var_set);
	var_set
}

fn find_vars_rec(e: &Expr, found_vars: &mut HashSet<String>) {
	match e {
		Lit(_) => {}
		Var(x) => {
			found_vars.insert(x.clone());
		}
		Not(e_1) => {
			find_vars_rec(&e_1, found_vars);
		}
		Binary(e_1, _, e_2) => {
			find_vars_rec(&e_1, found_vars);
			find_vars_rec(&e_2, found_vars);
		},
	}
}

fn eval(e: &Expr, env: &Env) -> bool {
	match e {
		Lit(b) => *b,
		Var(x) => match env.get(x) {
			Some(&b) => b,
			_ => panic!(),
		},
		Not(e) => !eval(&e, env),
		Binary(e1, And, e2) => eval(&e1, env) && eval(&e2, env),
		Binary(e1, Or , e2) => eval(&e1, env) || eval(&e2, env),
	}
}