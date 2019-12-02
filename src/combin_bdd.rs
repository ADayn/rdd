use std::collections::BTreeMap;
use crate::expr::*;
use crate::neg_arc_bdd::*;
use Expr::*;
use BOp::*;

/////////////////////////
pub fn from_combinatorial(e: &Expr, var_ord: &[usize]) -> Bdd {
	let mut bdd = Bdd {
		f: func(0, false),
		nodes: Vec::new(),
	};
	// let mut cof_asgn = vec![false; var_ord.len()];
	bdd.f = from_combinatorial_rec(e, var_ord, &mut bdd.nodes, &mut BTreeMap::new(), &mut BTreeMap::new());
	bdd
}

fn from_combinatorial_rec(e: &Expr,
						  var_ord: &[usize],
	                      nodes: &mut Vec<InternalNode>,
	                      unique_table: &mut BTreeMap<InternalNode, NodeIdx>,
	                      computed_table: &mut BTreeMap<(FunctionNode, FunctionNode), FunctionNode>) -> FunctionNode {
	match e {
		Lit(b) => func(term, !b),
		Var(x) => unique_insert(*x, func(term, false), func(term, true), nodes, unique_table),
		Not(e1) => {
			let mut f = from_combinatorial_rec(e1, var_ord, nodes, unique_table, computed_table);
			f.complement = !f.complement;
			f
		},
		Binary(e1, And, e2) => {
			let f1 = from_combinatorial_rec(e1, var_ord, nodes, unique_table, computed_table);
			let f2 = from_combinatorial_rec(e2, var_ord, nodes, unique_table, computed_table);
			bdd_and(f1, f2, var_ord, nodes, unique_table, computed_table)
		},
		Binary(e1, Or , e2) => {
			// apply and using demorgan
			let mut f1 = from_combinatorial_rec(e1, var_ord, nodes, unique_table, computed_table);
			f1.complement = !f1.complement;
			let mut f2 = from_combinatorial_rec(e2, var_ord, nodes, unique_table, computed_table);
			f2.complement = !f2.complement;
			let mut f = bdd_and(f1, f2, var_ord, nodes, unique_table, computed_table);
			f.complement = !f.complement;
			f
		},
	}
}

// fn get_or_make(n: IdentNode)

fn bdd_and(f: FunctionNode,
	       g: FunctionNode,
	       rem_support: &[usize],
	       nodes: &mut Vec<InternalNode>,
	       unique_table: &mut BTreeMap<InternalNode, NodeIdx>,
	       computed_table: &mut BTreeMap<(FunctionNode, FunctionNode), FunctionNode>) -> FunctionNode {
	// if (terminal case) return result
	if f.head == g.head {
		// f == g || f == !g
		if f.complement == g.complement { f } else { func(term, true) }
	} else if f.head == term {
		// f == 0 || f == 1
		if f.complement { f } else { g }
	} else if g.head == term {
		// g == 0 || g == 1
		if g.complement { g } else { f }
	}

	else {
		// if (computed table has entry ({f, g}, r)) return r;
		let entry_key = if f < g { (f, g) } else { (g, f) };
		match computed_table.get(&entry_key) {
			Some(r) => *r,
			None    => {
		
				let coff = |f: FunctionNode, x: usize| -> (FunctionNode, FunctionNode) {
					let f_node = &nodes[f.head];
					if f_node.label == x {
						(func(f_node.t_arc, f.complement), func(f_node.e_arc, f_node.e_complement ^ f.complement))
					} else {
						(f, f)
					}
				};

				// let x be the top variable of {f,g};
				let x = rem_support[0];
				let (f_x, f_nx) = coff(f, x);
				let (g_x, g_nx) = coff(g, x);
				
				// t = AND(fx, gx);
				let t = bdd_and(f_x, g_x, &rem_support[1..], nodes, unique_table, computed_table);
				
				// e = AND(f¬x, g¬x);
				let e = bdd_and(f_nx, g_nx, &rem_support[1..], nodes, unique_table, computed_table);

				// r = findOrAddUniqueTable(x, t, e);
				let r = unique_insert(x, t, e, nodes, unique_table);

				// insertComputedTable({f, g}, r);
				computed_table.insert(entry_key, r);

				// return r;
				r
			}
 		}

	}
}

// fn from_rec_old(e: &Expr, rem_support: &[usize], cof_asgn: &mut Env, nodes: &mut Vec<InternalNode>, indices: &mut HashMap<InternalNode, NodeIdx>) -> FunctionNode {
// 	if rem_support.is_empty() {
// 		// No more cofactors to check, eval and create node.
// 		// unit terminal is true, so false representation requires complementation
// 		func(term, !eval(e, cof_asgn))
// 	} else {
// 		let x = rem_support[0];
// 		// Calculate positive and negative cofactors for current var and recurse
// 		cof_asgn[x] = true;
// 		let pos_cof = from_rec(e, &rem_support[1..], cof_asgn, nodes, indices);
// 		cof_asgn[x] = false;
// 		let neg_cof = from_rec(e, &rem_support[1..], cof_asgn, nodes, indices);
// 		if neg_cof == pos_cof {
// 			// Both arcs point to same thing, no need for a node
// 			neg_cof
// 		} else {
// 			let (complement, e_complement) =
// 				if pos_cof.complement {
// 					// (move pos_cof negation to this func, flip since whole func is negated)
// 					(true, !neg_cof.complement)
// 				} else {
// 					(false, neg_cof.complement)
// 				};
// 			let node = InternalNode {
// 				label: x,
// 				t_arc: pos_cof.head,
// 				e_arc: neg_cof.head,
// 				e_complement: e_complement
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
// 			}, complement)
// 		}
// 	}
// }

fn unique_insert(x: usize, pos_cof: FunctionNode, neg_cof: FunctionNode, nodes: &mut Vec<InternalNode>, table: &mut BTreeMap<InternalNode, NodeIdx>) -> FunctionNode {
	if pos_cof == neg_cof {
		// Both arcs point to same thing, no need for a node
		pos_cof
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
		func(match table.get(&node) {
			Some(i) => *i,
			None => {
				// node has not been created as node yet, make it
				let i = nodes.len();
				table.insert(node.clone(), i);
				nodes.push(node);
				i
			},
		}, complement)
	}
}
