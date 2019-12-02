use std::collections::HashMap;
use std::collections::BTreeMap;
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

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct InternalNode {
	pub label: usize,
	pub t_arc: NodeIdx,
	pub e_arc: NodeIdx,
	pub e_complement: bool
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FunctionNode {
	pub head: NodeIdx,
	pub complement: bool
}

pub const term: usize = std::usize::MAX;

impl Bdd {

	pub fn eval(&self, env: &Env) -> bool {
		let mut next_node = self.f.head;
		let mut complement = self.f.complement;
		while next_node != term {
			let node = &self.nodes[next_node];
			next_node = if env[node.label] {
				node.t_arc
			} else {
				complement ^= node.e_complement;
				node.e_arc
			};
		}
		// true if complement is false
		!complement
	}

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

	// size of reachable bdd, not including terminal
	pub fn size(&self) -> usize {
		let mut visited = vec![false; self.nodes.len()];
		self.size_rec(self.f.head, &mut visited);
		visited.iter().filter(|&x| *x).count()
	}

	fn size_rec(&self, n: NodeIdx, visited: &mut Vec<bool>) {
		if n != term && !visited[n] {
			let node = &self.nodes[n];
			self.size_rec(node.t_arc, visited);
			self.size_rec(node.e_arc, visited);
			visited[n] = true;
		}
	}
}

pub fn func(head: NodeIdx, complement: bool) -> FunctionNode {
	FunctionNode {
		head: head,
		complement: complement
	}
}

/////////////////////////
pub fn from(e: &Expr, var_ord: &[usize]) -> Bdd {
	fn rec(e: &Expr, rem_support: &[usize], cof_asgn: &mut Env, nodes: &mut Vec<InternalNode>, indices: &mut HashMap<InternalNode, NodeIdx>) -> FunctionNode {
		if rem_support.is_empty() {
			// No more cofactors to check, eval and create node.
			// unit terminal is true, so false representation requires complementation
			func(term, !eval(e, cof_asgn))
		} else {
			let x = rem_support[0];
			// Calculate positive and negative cofactors for current var and recurse
			cof_asgn[x] = true;
			let pos_cof = rec(e, &rem_support[1..], cof_asgn, nodes, indices);
			cof_asgn[x] = false;
			let neg_cof = rec(e, &rem_support[1..], cof_asgn, nodes, indices);
			if neg_cof == pos_cof {
				// Both arcs point to same thing, no need for a node
				neg_cof
			} else {
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
			}
		}
	}

	let mut bdd = Bdd {
		f: func(0, false),
		nodes: Vec::new(),
		// indices: HashMap::new(),
	};
	let mut cof_asgn = vec![false; var_ord.len()];
	bdd.f = rec(e, var_ord, &mut cof_asgn, &mut bdd.nodes, &mut HashMap::new());
	bdd
}

////////////////////////

#[derive(PartialEq)]
enum SupportResult {
	Dependant,
	IndependantVaries,
	IndependantConst(bool)
}
use SupportResult::*;

fn in_support(x: usize, e: &Expr, cof_asgn: &mut PartialEnv) -> bool {
	fn rec(x: usize, e: &Expr, cof_asgn: &mut PartialEnv) -> SupportResult {
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
				match rec(x, e1, cof_asgn) {
					IndependantConst(b) => IndependantConst(!b),
					s => s
				},
			Binary(e1, And, e2) =>
				match (rec(x, e1, cof_asgn), rec(x, e2, cof_asgn)) {
					(IndependantConst(false), _) |
					(_, IndependantConst(false))    => IndependantConst(false),
					(IndependantConst(true), s) |
					(s, IndependantConst(true))     => s,
					(Dependant, _) | (_, Dependant) => Dependant,
					(IndependantVaries, IndependantVaries) => IndependantVaries
				},
			Binary(e1, Or, e2) =>
				match (rec(x, e1, cof_asgn), rec(x, e2, cof_asgn)) {
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

	rec(x, e, cof_asgn) == Dependant
}

pub fn from_support(e: &Expr, var_ord: &[usize]) -> Bdd {
	fn rec(e: &Expr, rem_support: &[usize], cof_asgn: &mut PartialEnv, nodes: &mut Vec<InternalNode>, indices: &mut HashMap<InternalNode, NodeIdx>) -> FunctionNode {
		if rem_support.is_empty() {
			// No more cofactors to check, eval and create node.
			// unit terminal is true, so false representation requires complementation
			func(term, !eval_partial(e, cof_asgn))
		} else {
			let x = rem_support[0];
			if in_support(x, e, cof_asgn) {
				// Calculate positive and negative cofactors for current var and recurse
				cof_asgn.insert(x, true);
				let pos_cof = rec(e, &rem_support[1..], cof_asgn, nodes, indices);
				cof_asgn.insert(x, false);
				let neg_cof = rec(e, &rem_support[1..], cof_asgn, nodes, indices);
				cof_asgn.remove(&x);

				if neg_cof == pos_cof {
					// Both arcs point to same thing, no need for a node
					neg_cof
				} else {
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
				}
			} else {
				cof_asgn.insert(x, false); // doesn't matter, needed for eval
				let rec = rec(e, &rem_support[1..], cof_asgn, nodes, indices);
				cof_asgn.remove(&x);
				rec
			}
		}
	}

	let mut bdd = Bdd {
		f: func(0, false),
		nodes: Vec::new(),
		// indices: HashMap::new(),
	};
	let mut cof_asgn: PartialEnv = HashMap::new();
	bdd.f = rec(e, var_ord, &mut cof_asgn, &mut bdd.nodes, &mut HashMap::new());
	bdd
}

////////////////////////
fn subst_and_simplify(e: &Expr, x: usize, b: bool) -> Expr {
	match e {
		Lit(b2) => Lit(*b2),
		Var(x2) => if x == *x2 {Lit(b)} else {Var(*x2)},
		Not(e1) =>
			match subst_and_simplify(e1, x, b) {
				Lit(b1) => Lit(!b1),
				e_simp => not(e_simp)
			},
		Binary(e1, And, e2) =>
			match subst_and_simplify(e1, x, b) {
				Lit(false) => Lit(false),
				Lit(true)  => subst_and_simplify(e2, x, b),
				e1_simp    =>
					match subst_and_simplify(e2, x, b) {
						Lit(false) => Lit(false),
						Lit(true)  => e1_simp,
						e2_simp    => bin(e1_simp, And, e2_simp)
					}
			},
		Binary(e1, Or, e2) =>
			match subst_and_simplify(e1, x, b) {
				Lit(true)  => Lit(true),
				Lit(false) => subst_and_simplify(e2, x, b),
				e1_simp    =>
					match subst_and_simplify(e2, x, b) {
						Lit(true)  => Lit(true),
						Lit(false) => e1_simp,
						e2_simp    => bin(e1_simp, Or, e2_simp)
					}
			},
	}
}

// Only used after simplification, so if the variable is here it is in the support of the func
fn in_support_simplified(x: usize, e: &Expr) -> bool {
	match e {
		Lit(_) => false,
		Var(x2) => *x2 == x,
		Not(e1) => in_support_simplified(x, e1),
		Binary(e1, _, e2) => in_support_simplified(x, e1) || in_support_simplified(x, e2)
	}
}

pub fn from_support_simplified(e: &Expr, var_ord: &[usize]) -> Bdd {
	fn rec(e: &Expr, rem_support: &[usize], cof_asgn: &mut Env, nodes: &mut Vec<InternalNode>, indices: &mut BTreeMap<InternalNode, NodeIdx>) -> FunctionNode {
		if rem_support.is_empty() {
			// No more cofactors to check, eval and create node.
			// unit terminal is true, so false representation requires complementation
			func(term, !eval(e, cof_asgn))
		} else {
			let x = rem_support[0];
			if in_support_simplified(x, e) {
				// Calculate positive and negative cofactors for current var and recurse
				cof_asgn[x] = true;
				let e_pos = subst_and_simplify(e, x, true);
				let pos_cof = rec(&e_pos, &rem_support[1..], cof_asgn, nodes, indices);
				
				cof_asgn[x] = false;
				let e_neg = subst_and_simplify(e, x, false);
				let neg_cof = rec(&e_neg, &rem_support[1..], cof_asgn, nodes, indices);

				if neg_cof == pos_cof {
					// Both arcs point to same thing, no need for a node
					neg_cof
				} else {
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
				}
			} else {
				// cof_asgn[x] = false; // doesn't matter, needed for eval and set by default
				// no need to simplify again, already factored out
				let rec = rec(e, &rem_support[1..], cof_asgn, nodes, indices);
				rec
			}
		}
	}

	let mut bdd = Bdd {
		f: func(0, false),
		nodes: vec![],
		// indices: HashMap::new(),
	};
	let mut cof_asgn: Env = vec![false; var_ord.len()];
	bdd.f = rec(e, var_ord, &mut cof_asgn, &mut bdd.nodes, &mut BTreeMap::new());
	bdd
}

// pub fn from_support_simplified_vec(e: &Expr, var_ord: &[usize]) -> Bdd {
// 	fn from_support_simplified_vec_rec(e: &Expr, rem_support: &[usize], cof_asgn: &mut Env, nodes: &mut Vec<InternalNode>, indices: &mut BTreeMap<InternalNode, NodeIdx>) -> FunctionNode {
// 		if rem_support.is_empty() {
// 			// No more cofactors to check, eval and create node.
// 			// unit terminal is true, so false representation requires complementation
// 			func(term, !eval(e, cof_asgn))
// 		} else {
// 			let x = rem_support[0];
// 			if in_support_simplified(x, e) {
// 				// Calculate positive and negative cofactors for current var and recurse
// 				cof_asgn[x] = true;
// 				let e_pos = subst_and_simplify(e, x, true);
// 				let pos_cof = from_support_simplified_rec(&e_pos, &rem_support[1..], cof_asgn, nodes, indices);
				
// 				cof_asgn.insert(x, false);
// 				let e_neg = subst_and_simplify(e, x, false);
// 				let neg_cof = from_support_simplified_rec(&e_neg, &rem_support[1..], cof_asgn, nodes, indices);

// 				if neg_cof == pos_cof {
// 					// Both arcs point to same thing, no need for a node
// 					neg_cof
// 				} else {
// 					let (complement, e_complement) =
// 						if pos_cof.complement {
// 							// (move pos_cof negation to this func, flip since whole func is negated)
// 							(true, !neg_cof.complement)
// 						} else {
// 							(false, neg_cof.complement)
// 						};
// 					let node = InternalNode {
// 						label: x,
// 						t_arc: pos_cof.head,
// 						e_arc: neg_cof.head,
// 						e_complement: e_complement
// 					};
// 					func(match indices.get(&node) {
// 						Some(i) => *i,
// 						None => {
// 							// node has not been created as node yet, make it
// 							let i = nodes.len();
// 							indices.insert(node.clone(), i);
// 							nodes.push(node);
// 							i
// 						},
// 					}, complement)
// 				}
// 			} else {
// 				cof_asgn.insert(x, false); // doesn't matter, needed for eval
// 				// no need to simplify again, already factored out
// 				let rec = from_support_simplified_rec(e, &rem_support[1..], cof_asgn, nodes, indices);
// 				cof_asgn.remove(&x);
// 				rec
// 			}
// 		}
// 	}

// 	let mut bdd = Bdd {
// 		f: func(0, false),
// 		nodes: Vec::new(),
// 		// indices: HashMap::new(),
// 	};
// 	let mut cof_asgn: Env = vec![];
// 	bdd.f = from_support_simplified_rec(e, var_ord, &mut cof_asgn, &mut bdd.nodes, &mut BTreeMap::new());
// 	bdd
// }





////////////////////////
pub fn from_support_btree(e: &Expr, var_ord: &[usize]) -> Bdd {
	fn rec(e: &Expr, rem_support: &[usize], cof_asgn: &mut PartialEnvBTree, nodes: &mut Vec<InternalNode>, indices: &mut HashMap<InternalNode, NodeIdx>) -> FunctionNode {
		if rem_support.is_empty() {
			// No more cofactors to check, eval and create node.
			// unit terminal is true, so false representation requires complementation
			func(term, !eval_partialbtree(e, cof_asgn))
		} else {
			let x = rem_support[0];
			if in_support_btree(x, e, cof_asgn) {
				// Calculate positive and negative cofactors for current var and recurse
				cof_asgn.insert(x, true);
				let pos_cof = rec(e, &rem_support[1..], cof_asgn, nodes, indices);
				cof_asgn.insert(x, false);
				let neg_cof = rec(e, &rem_support[1..], cof_asgn, nodes, indices);
				cof_asgn.remove(&x);

				if neg_cof == pos_cof {
					// Both arcs point to same thing, no need for a node
					neg_cof
				} else {
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
				}
			} else {
				cof_asgn.insert(x, false); // doesn't matter, needed for eval
				let rec = rec(e, &rem_support[1..], cof_asgn, nodes, indices);
				cof_asgn.remove(&x);
				rec
			}
		}
	}

	let mut bdd = Bdd {
		f: func(0, false),
		nodes: Vec::new(),
		// indices: HashMap::new(),
	};
	let mut cof_asgn: PartialEnvBTree = BTreeMap::new();
	bdd.f = rec(e, var_ord, &mut cof_asgn, &mut bdd.nodes, &mut HashMap::new());
	bdd
}

fn in_support_btree(x: usize, e: &Expr, cof_asgn: &mut PartialEnvBTree) -> bool {
	fn rec(x: usize, e: &Expr, cof_asgn: &mut PartialEnvBTree) -> SupportResult {
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
				match rec(x, e1, cof_asgn) {
					IndependantConst(b) => IndependantConst(!b),
					s => s
				},
			Binary(e1, And, e2) =>
				match (rec(x, e1, cof_asgn), rec(x, e2, cof_asgn)) {
					(IndependantConst(false), _) |
					(_, IndependantConst(false))    => IndependantConst(false),
					(IndependantConst(true), s) |
					(s, IndependantConst(true))     => s,
					(Dependant, _) | (_, Dependant) => Dependant,
					(IndependantVaries, IndependantVaries) => IndependantVaries
				},
			Binary(e1, Or, e2) =>
				match (rec(x, e1, cof_asgn), rec(x, e2, cof_asgn)) {
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

	rec(x, e, cof_asgn) == Dependant
}

//////////////////////////////////////////////////////////

pub fn from_support_vec(e: &Expr, var_ord: &[usize]) -> Bdd {
	fn rec(e: &Expr, rem_support: &[usize], cof_asgn: &mut Env, cof_valid: &mut Env, nodes: &mut Vec<InternalNode>, indices: &mut HashMap<InternalNode, NodeIdx>) -> FunctionNode {
		if rem_support.is_empty() {
			// No more cofactors to check, eval and create node.
			// unit terminal is true, so false representation requires complementation
			func(term, !eval(e, cof_asgn))
		} else {
			let x = rem_support[0];
			if in_support_vec(x, e, cof_asgn, cof_valid) {
				// Calculate positive and negative cofactors for current var and recurse
				cof_asgn[x] = true;
				cof_valid[x] = true;
				let pos_cof = rec(e, &rem_support[1..], cof_asgn, cof_valid, nodes, indices);
				cof_asgn[x] = false;
				let neg_cof = rec(e, &rem_support[1..], cof_asgn, cof_valid, nodes, indices);
				cof_valid[x] = false;

				if neg_cof == pos_cof {
					// Both arcs point to same thing, no need for a node
					neg_cof
				} else {
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
				}
			} else {
				cof_asgn[x] = false; // doesn't matter, needed for eval
				cof_valid[x] = true;
				let rec = rec(e, &rem_support[1..], cof_asgn, cof_valid, nodes, indices);
				cof_valid[x] = false;
				rec
			}
		}
	}

	let mut bdd = Bdd {
		f: func(0, false),
		nodes: vec![],
		// indices: HashMap::new(),
	};
	let mut cof_asgn: Env = vec![false; var_ord.len()];
	let mut cof_valid: Vec<bool> = vec![false; var_ord.len()];
	bdd.f = rec(e, var_ord, &mut cof_asgn, &mut cof_valid, &mut bdd.nodes, &mut HashMap::new());
	bdd
}

fn in_support_vec(x: usize, e: &Expr, cof_asgn: &mut Env, cof_valid: &mut Env) -> bool {
	fn rec(x: usize, e: &Expr, cof_asgn: &mut Env, cof_valid: &mut Env) -> SupportResult {
		match e {
			Lit(b) => IndependantConst(*b),
			Var(x2) =>
				if x == *x2 {
					Dependant
				} else {
					if cof_valid[*x2] { IndependantConst(cof_asgn[*x2]) }
					else             { IndependantVaries }
				},
			Not(e1) =>
				match rec(x, e1, cof_asgn, cof_valid) {
					IndependantConst(b) => IndependantConst(!b),
					s => s
				},
			Binary(e1, And, e2) =>
				match (rec(x, e1, cof_asgn, cof_valid), rec(x, e2, cof_asgn, cof_valid)) {
					(IndependantConst(false), _) |
					(_, IndependantConst(false))    => IndependantConst(false),
					(IndependantConst(true), s) |
					(s, IndependantConst(true))     => s,
					(Dependant, _) | (_, Dependant) => Dependant,
					(IndependantVaries, IndependantVaries) => IndependantVaries
				},
			Binary(e1, Or, e2) =>
				match (rec(x, e1, cof_asgn, cof_valid), rec(x, e2, cof_asgn, cof_valid)) {
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

	rec(x, e, cof_asgn, cof_valid) == Dependant
}

