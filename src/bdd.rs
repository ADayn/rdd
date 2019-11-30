use std::collections::HashMap;
use std::collections::HashSet;

use crate::expr::*;

pub type NodeIdx = usize;

#[derive(Debug)]
pub struct Bdd {
	// list of functions
	pub f: usize,
	// list of nodes
	pub nodes: Vec<BddNode>,
	// order of vars
	// indices: HashMap<BDD_Node, NodeIdx>
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BddNode {
	Internal {
		label: String,
		t_arc: NodeIdx,
		e_arc: NodeIdx,
		// e_complement: bool
	},
	Terminal(bool),
}

use BddNode::*;

pub fn from(e: &Expr, var_ord: &[String]) -> Bdd {
	let mut bdd = Bdd {
		f    : 69,
		nodes: Vec::new(),
	};
	let mut cof_asgn: Env = HashMap::new();
	bdd.f = from_rec(e, var_ord, &mut cof_asgn, &mut bdd);
	bdd
}

fn from_rec(e: &Expr, rem_support: &[String], cof_asgn: &mut Env, bdd: &mut Bdd) -> NodeIdx {
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
		if neg_cof_node == pos_cof_node {
			// Both arcs point to same thing, no need for a node
			neg_cof_node
		} else {
			let new_node = Internal {
				label: x.clone(),
				t_arc: pos_cof_node,
				e_arc: neg_cof_node
			};
			// If exists use current node, else make new node.
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
}