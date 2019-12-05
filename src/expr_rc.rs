use crate::expr::*;
use std::rc::Rc;
use Expr::*;
use BOp::*;

#[derive(Clone, Debug)]
pub enum ExprRc {
    LitRc(bool),
    VarRc(usize),
    NotRc(Rc<ExprRc>),
    BinaryRc(Rc<ExprRc>, BOp, Rc<ExprRc>),
}

use ExprRc::*;

pub fn not_rc(e: ExprRc) -> ExprRc {
	NotRc(Rc::new(e))
}

pub fn bin_rc(e_1: ExprRc, bop: BOp, e_2: ExprRc) -> ExprRc {
	BinaryRc(
		Rc::new(e_1),
		bop,
		Rc::new(e_2),
	)
}

pub fn to_expr_rc(e: &Expr) -> ExprRc {
	match e {
		Lit(b) => LitRc(*b),
		Var(x) => VarRc(*x),
		Not(e1) => not_rc(to_expr_rc(e1)),
		Binary(e1, bop, e2) => bin_rc(to_expr_rc(e1), *bop, to_expr_rc(e2)),
	}
}

pub fn eval_rc(e: &ExprRc, env: &Env) -> bool {
	match e {
		LitRc(b) => *b,
		VarRc(x) => env[*x],
		NotRc(e1) => !eval_rc(&e1, env),
		BinaryRc(e1, And, e2) => eval_rc(&e1, env) && eval_rc(&e2, env),
		BinaryRc(e1, Or , e2) => eval_rc(&e1, env) || eval_rc(&e2, env),
		// Binary(e1, XOr , e2) => eval(&e1, env) ^ eval(&e2, env),
	}
}
