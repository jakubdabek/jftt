#![allow(dead_code)]

extern crate num;

use num::integer::{div_floor, mod_floor};
use num::pow;
use std::convert::TryInto;
use std::fmt::{Debug, Error, Formatter};

pub enum Expr {
    BinOp(Box<Expr>, BinaryOpcode, Box<Expr>),
    UnOp(UnaryOpcode, Box<Expr>),
    Num(i32),
    Error,
}

pub trait ExprVisitor: Sized {
    fn visit_binop(&mut self, l: &Expr, _: BinaryOpcode, r: &Expr) {
        l.walk(self);
        r.walk(self);
    }
    fn visit_unop(&mut self, _: UnaryOpcode, e: &Expr) {
        e.walk(self);
    }
    fn visit_num(&mut self, _: i32) {}
    fn visit_error(&mut self) {}
}

impl Expr {
    pub fn walk<V: ExprVisitor>(&self, visitor: &mut V) {
        use self::Expr::*;

        match *self {
            BinOp(ref l, op, ref r) => visitor.visit_binop(l, op, r),
            UnOp(op, ref e) => visitor.visit_unop(op, e),
            Num(val) => visitor.visit_num(val),
            Error => visitor.visit_error(),
        }
    }
}

pub struct ExprEvaluator {
    stack: Vec<Result<i32, String>>,
}

impl ExprEvaluator {
    pub fn new() -> ExprEvaluator {
        ExprEvaluator { stack: vec![] }
    }

    pub fn value(mut self) -> Result<i32, String> {
        self.stack.pop().ok_or("Empty stack!!!".to_string())?
    }
}

impl ExprVisitor for ExprEvaluator {
    fn visit_binop(&mut self, l: &Expr, op: BinaryOpcode, r: &Expr) {
        l.walk(self);
        let lval = self.stack.pop().unwrap();
        r.walk(self);
        let rval = self.stack.pop().unwrap();

        let get_result = || op.execute(lval?, rval?);

        self.stack.push(get_result());
    }

    fn visit_unop(&mut self, op: UnaryOpcode, e: &Expr) {
        e.walk(self);
        let val = self.stack.pop().unwrap();

        let get_result = || op.execute(val?);

        self.stack.push(get_result());
    }

    fn visit_num(&mut self, val: i32) {
        self.stack.push(Ok(val));
    }

    fn visit_error(&mut self) {
        self.stack.push(Err("???".to_string()));
    }
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Num(n) => write!(fmt, "{:?}", n),
            BinOp(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            UnOp(op, ref a) => write!(fmt, "({:?} {:?})", op, a),
            Error => write!(fmt, "error"),
        }
    }
}

#[derive(Copy, Clone)]
pub enum BinaryOpcode {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
    Exponentiation,
}

impl BinaryOpcode {
    fn execute(&self, a: i32, b: i32) -> Result<i32, String> {
        use self::BinaryOpcode::*;

        match *self {
            Addition => Ok(a + b),
            Subtraction => Ok(a - b),
            Multiplication => Ok(a * b),
            Division => {
                if b != 0 {
                    Ok(div_floor(a, b))
                } else {
                    Err("Division by zero!".to_string())
                }
            }
            Modulo => {
                if b != 0 {
                    Ok(mod_floor(a, b))
                } else {
                    Err("Division by zero!".to_string())
                }
            }
            Exponentiation => {
                if b >= 0 {
                    Ok(pow(a, b.try_into().unwrap()))
                } else {
                    Err("Negative exponent!".to_string())
                }
            }
        }
    }
}

impl Debug for BinaryOpcode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        use self::BinaryOpcode::*;
        write!(
            f,
            "{}",
            match *self {
                Addition => "+",
                Subtraction => "-",
                Multiplication => "*",
                Division => "/",
                Modulo => "%",
                Exponentiation => "^",
            }
        )
    }
}

#[derive(Copy, Clone)]
pub enum UnaryOpcode {
    Negation,
}

impl UnaryOpcode {
    fn execute(&self, val: i32) -> Result<i32, String> {
        use self::UnaryOpcode::*;

        match *self {
            Negation => Ok(-val),
        }
    }
}

impl Debug for UnaryOpcode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        use self::UnaryOpcode::*;
        write!(
            f,
            "{}",
            match *self {
                Negation => "-",
            }
        )
    }
}