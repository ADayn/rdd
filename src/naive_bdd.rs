use std::collections::HashMap;
use std::collections::BTreeMap;
use std::collections::HashSet;

use crate::expr::*;

pub type NodeIdx = usize;

#[derive(Debug)]
pub struct Bdd {
	// list of functions
	pub f: usize,
	// list of nodes
	pub nodes: Vec<BddNode>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BddNode {
	Internal {
		label: usize,
		t_arc: NodeIdx,
		e_arc: NodeIdx,
		// e_complement: bool
	},
	Terminal(bool),
}

use BddNode::*;

pub fn from(e: &Expr, var_ord: &[usize]) -> Bdd {
	let mut bdd = Bdd {
		f      : std::usize::MAX - 1,
		nodes  : Vec::new(),
	};
	let mut cof_asgn: Env = vec![false; var_ord.len()];
	bdd.f = from_rec(e, var_ord, &mut cof_asgn, &mut bdd.nodes, &mut HashMap::new());
	bdd
}

fn from_rec(e: &Expr, rem_support: &[usize], cof_asgn: &mut Env, nodes: &mut Vec<BddNode>, indices: &mut HashMap<BddNode, NodeIdx>) -> NodeIdx {
	if rem_support.is_empty() {
		// No more cofactors to check, eval and create node.
		let b = eval(e, cof_asgn);
		match indices.get(&Terminal(b)) {
			Some(i) => *i,
			None => {
				// Terminal has not been created as node yet, make it
				let i = nodes.len();
				nodes.push(Terminal(b));
				indices.insert(Terminal(b), i);
				i
			},
		}
	} else {
		// Calculate positive and negative cofactors for current var and recurse
		let x = rem_support[0];
		cof_asgn[x] = false;
		let neg_cof_node = from_rec(e, &rem_support[1..], cof_asgn, nodes, indices);
		cof_asgn[x] = true;
		let pos_cof_node = from_rec(e, &rem_support[1..], cof_asgn, nodes, indices);
		if neg_cof_node == pos_cof_node {
			// Both arcs point to same thing, no need for a node
			neg_cof_node
		} else {
			let new_node = Internal {
				label: x,
				t_arc: pos_cof_node,
				e_arc: neg_cof_node
			};
			// If exists use current node, else make new node.
			match indices.get(&new_node) {
				Some(i) => *i,
				None => {
					// Terminal has not been created as node yet, make it
					let i = nodes.len();
					nodes.push(new_node.clone());
					indices.insert(new_node, i);
					i
				},
			}
		}
	}
}

pub fn from_btree_mem(e: &Expr, var_ord: &[usize]) -> Bdd {
	let mut bdd = Bdd {
		f      : std::usize::MAX - 1,
		nodes  : Vec::new(),
	};
	let mut cof_asgn: Env = vec![false; var_ord.len()];
	bdd.f = from_btree_mem_rec(e, var_ord, &mut cof_asgn, &mut bdd.nodes, &mut BTreeMap::new());
	bdd
}

fn from_btree_mem_rec(e: &Expr, rem_support: &[usize], cof_asgn: &mut Env, nodes: &mut Vec<BddNode>, indices: &mut BTreeMap<BddNode, NodeIdx>) -> NodeIdx {
	if rem_support.is_empty() {
		// No more cofactors to check, eval and create node.
		let b = eval(e, cof_asgn);
		match indices.get(&Terminal(b)) {
			Some(i) => *i,
			None => {
				// Terminal has not been created as node yet, make it
				let i = nodes.len();
				nodes.push(Terminal(b));
				indices.insert(Terminal(b), i);
				i
			},
		}
	} else {
		// Calculate positive and negative cofactors for current var and recurse
		let x = rem_support[0];
		cof_asgn[x] = false;
		let neg_cof_node = from_btree_mem_rec(e, &rem_support[1..], cof_asgn, nodes, indices);
		cof_asgn[x] = true;
		let pos_cof_node = from_btree_mem_rec(e, &rem_support[1..], cof_asgn, nodes, indices);
		if neg_cof_node == pos_cof_node {
			// Both arcs point to same thing, no need for a node
			neg_cof_node
		} else {
			let new_node = Internal {
				label: x,
				t_arc: pos_cof_node,
				e_arc: neg_cof_node
			};
			// If exists use current node, else make new node.
			match indices.get(&new_node) {
				Some(i) => *i,
				None => {
					// Terminal has not been created as node yet, make it
					let i = nodes.len();
					nodes.push(new_node.clone());
					indices.insert(new_node, i);
					i
				},
			}
		}
	}
}

impl Bdd {

	pub fn eval(&self, env: &Env) -> bool {
		let mut next_idx = self.f;
		while let Internal { label, t_arc, e_arc, } = &self.nodes[next_idx] {
			next_idx = if env[*label] { *t_arc } else { *e_arc };
		}
		match &self.nodes[next_idx] { // must be terminal
			Terminal(b) => *b,
			_ => unreachable!()
		}
	}

	// pub fn textual_repr(&self) -> String {
	// 	"69".to_string()
	// }
}
