use std::collections::HashMap;
use std::collections::HashSet;

use crate::expr::*;
use Expr::*;
use BOp::*;

pub type NodeIdx = usize;

#[derive(Debug)]
pub struct Bdd {
	pub f: FunctionNode,
	// list of nodes
	pub nodes: Vec<InternalNode>,
	// indices: HashMap<InternalNode, NodeIdx>
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InternalNode {
	label: usize,
	t_arc: NodeIdx,
	e_arc: NodeIdx,
	e_complement: bool
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FunctionNode {
	head: NodeIdx,
	complement: bool
}

const term: usize = std::usize::MAX;

fn func(head: NodeIdx, complement: bool) -> FunctionNode {
	FunctionNode {
		head: head,
		complement: complement
	}
}

pub fn from(e: &Expr, var_ord: &[usize]) -> Bdd {
	let mut bdd = Bdd {
		f: func(0, false),
		nodes: Vec::new(),
		// indices: HashMap::new(),
	};
	let mut cof_asgn: PartialEnv = HashMap::new();
	bdd.f = from_rec(e, var_ord, &mut cof_asgn, &mut bdd.nodes, &mut HashMap::new());
	bdd
}

#[derive(PartialEq)]
enum SupportResult {
	Dependant,
	IndependantVaries,
	IndependantConst(bool)
}
use SupportResult::*;

fn in_support(x: usize, e: &Expr, cof_asgn: &mut PartialEnv) -> bool {
	in_support_rec(x, e, cof_asgn) == Dependant
}

fn in_support_rec(x: usize, e: &Expr, cof_asgn: &mut PartialEnv) -> SupportResult {
	match e {
		Lit(b) => IndependantConst(*b),
		Var(x2) =>
			if x == *x2 {
				Dependant
			} else {
				match cof_asgn.get(x2) {
					Some(b) => IndependantConst(*b),
					None => IndependantVaries
				}
			},
		Not(e1) =>
			match in_support_rec(x, e1, cof_asgn) {
				IndependantConst(b) => IndependantConst(!b),
				s => s
			},
		Binary(e1, And, e2) =>
			match (in_support_rec(x, e1, cof_asgn), in_support_rec(x, e2, cof_asgn)) {
				(IndependantConst(false), _) |
				(_, IndependantConst(false))    => IndependantConst(false),
				(IndependantConst(true), s) |
				(s, IndependantConst(true))     => s,
				(Dependant, _) | (_, Dependant) => Dependant,
				(IndependantVaries, IndependantVaries) => IndependantVaries
			},
		Binary(e1, Or, e2) =>
			match (in_support_rec(x, e1, cof_asgn), in_support_rec(x, e2, cof_asgn)) {
				(IndependantConst(true), _) |
				(_, IndependantConst(true))     => IndependantConst(true),
				(IndependantConst(false), s) |
				(s, IndependantConst(false))    => s,
				(Dependant, _) | (_, Dependant) => Dependant,
				(IndependantVaries, IndependantVaries) => IndependantVaries
			}
		// Binary(e1, XOr , e2) => eval(&e1, env) ^ eval(&e2, env),s
	}
}

fn from_rec(e: &Expr, rem_support: &[usize], cof_asgn: &mut PartialEnv, nodes: &mut Vec<InternalNode>, indices: &mut HashMap<InternalNode, NodeIdx>) -> FunctionNode {
	if rem_support.is_empty() {
		// No more cofactors to check, eval and create node.
		// unit terminal is true, so false representation requires complementation
		func(term, !eval_partial(e, cof_asgn))
	} else {
		let x = rem_support[0];
		if in_support(x, e, cof_asgn) {
			// Calculate positive and negative cofactors for current var and recurse
			cof_asgn.insert(x, true);
			let pos_cof = from_rec(e, &rem_support[1..], cof_asgn, nodes, indices);
			cof_asgn.insert(x, false);
			let neg_cof = from_rec(e, &rem_support[1..], cof_asgn, nodes, indices);
			cof_asgn.remove(&x);

			let (complement, e_complement) =
				if pos_cof.complement {
					// (move pos_cof negation to this func, flip since whole func is negated)
					(true, !neg_cof.complement)
				} else {
					(false, neg_cof.complement)
				};
			let node = InternalNode {
				label: x,
				t_arc: pos_cof.head,
				e_arc: neg_cof.head,
				e_complement: e_complement
			};
			func(match indices.get(&node) {
				Some(i) => *i,
				None => {
					// node has not been created as node yet, make it
					let i = nodes.len();
					indices.insert(node.clone(), i);
					nodes.push(node);
					i
				},
			}, complement) 
		} else {
			cof_asgn.insert(x, false); // doesn't matter, needed for eval
			from_rec(e, &rem_support[1..], cof_asgn, nodes, indices)
		}
	}
}

impl Bdd {
	pub fn textual_repr(&self) -> String {
		format!("f = {:?}", self.textual_repr_rec(self.f.head, &mut vec![false; self.nodes.len()], self.f.complement))
	}

	fn textual_repr_rec(&self, n: NodeIdx, calculated: &mut Vec<bool>, negate: bool) -> String {
		if n == term {
			if negate {"F"} else {"T"}.to_string()
		} else {
			format!("{}{}",
				if negate { "!" } else {""},
				if calculated[n] {
					format!("n{}", n)
				} else {
					let node = &self.nodes[n];
					calculated[n] = true;
					format!("n{} @ x{} := ({}, {})",
						n,
						node.label,
						self.textual_repr_rec(node.t_arc, calculated, false),
						self.textual_repr_rec(node.e_arc, calculated, node.e_complement))
				})
		}
	}
}

// fn from_rec(e: &Expr, var_ord: &[usize], nodes: &mut Vec<InternalNode>, indices: &mut HashMap<InternalNode, NodeIdx>) -> FunctionNode {
// 	let term = std::usize::MAX;
// 	match e {
// 		// unit terminal is true, so false representation requires complementation
// 		Lit(b) => func(term, !b),
// 		Var(x) => {
// 			let node = InternalNode {
// 				label: *x,
// 				t_arc: term,
// 				e_arc: term,
// 				e_complement: true
// 			};
// 			func(match indices.get(&node) {
// 				Some(i) => *i,
// 				None => {
// 					// node has not been created as node yet, make it
// 					let i = nodes.len();
// 					indices.insert(node.clone(), i);
// 					nodes.push(node);
// 					i
// 				},
// 			}, false)
// 		}
// 		Not(e) => {
// 			let mut func = from_rec(e, var_ord, nodes, indices);
// 			func.complement = !func.complement;
// 			func
// 		},
// 		Binary(e1, And, e2) => unimplemented!(),
// 		Binary(e1, Or , e2) => unimplemented!(),
// 		Binary(e1, XOr , e2) => unimplemented!(),
// 	}
// }