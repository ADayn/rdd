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

fn func(head: NodeIdx, complement: bool) -> FunctionNode {
	FunctionNode {
		head: head,
		complement: complement
	}
}

pub fn from(e: &Expr, var_ord: &[usize]) -> Bdd {
	let mut bdd = Bdd {
		f: func(std::usize::MAX - 1, false),
		nodes: Vec::new(),
		// indices: HashMap::new(),
	};
	bdd.f = from_rec(e, var_ord, &mut bdd.nodes, &mut HashMap::new());
	bdd
}

fn from_rec(e: &Expr, var_ord: &[usize], nodes: &mut Vec<InternalNode>, indices: &mut HashMap<InternalNode, NodeIdx>) -> FunctionNode {
	let term = std::usize::MAX;
	match e {
		// unit terminal is true, so false representation requires complementation
		Lit(b) => func(term, !b),
		Var(x) => {
			let node = InternalNode {
				label: *x,
				t_arc: term,
				e_arc: term,
				e_complement: true
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
			}, false)
		}
		Not(e) => {
			let mut func = from_rec(e, var_ord, nodes, indices);
			func.complement = !func.complement;
			func
		},
		Binary(e1, And, e2) => unimplemented!(),
		Binary(e1, Or , e2) => unimplemented!(),
		Binary(e1, XOr , e2) => unimplemented!(),
	}
	// if rem_support.is_empty() {
	// 	// No more cofactors to check, eval and create node.
	// 	let b = eval(e, cof_asgn);
	// 	match bdd.nodes.iter().position(|n| n.clone() == Terminal(b)) {
	// 		Some(i) => i,
	// 		None => {
	// 			// Terminal has not been created as node yet, make it
	// 			bdd.nodes.push(Terminal(b));
	// 			bdd.nodes.len() - 1
	// 		},
	// 	}
	// } else {
	// 	// Calculate positive and negative cofactors for current var and recurse
	// 	let x = rem_support[0];
	// 	cof_asgn[x] = false;
	// 	let neg_cof_node = from_rec(e, &rem_support[1..], cof_asgn, bdd);
	// 	cof_asgn[x] = true;
	// 	let pos_cof_node = from_rec(e, &rem_support[1..], cof_asgn, bdd);
	// 	if neg_cof_node == pos_cof_node {
	// 		// Both arcs point to same thing, no need for a node
	// 		neg_cof_node
	// 	} else {
	// 		let new_node = Internal {
	// 			label: x,
	// 			t_arc: pos_cof_node,
	// 			e_arc: neg_cof_node
	// 		};
	// 		// If exists use current node, else make new node.
	// 		match bdd.nodes.iter().position(|n| n.clone() == new_node) {
	// 			Some(i) => i,
	// 			None => {
	// 				// Terminal has not been created as node yet, make it
	// 				bdd.nodes.push(new_node);
	// 				bdd.nodes.len() - 1
	// 			},
	// 		}
	// 	}
	// }
}