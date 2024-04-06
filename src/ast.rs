use std::fmt::{Debug, Error, Formatter};

#[derive(PartialEq)]
pub enum OpCode {
  OrOp,    // ||
  AndOp,   // &&
  EqOp,    // ==
  NeOp,    // !=
  LTOp,    // <
  LEOp,    // <=
  GTOp,    // >
  GEOp,    // >=
  Add,     // +
  Sub,     // -
  Mul,     // *
  Div,     // /
  Mod,     // %
  Not,     // !
}

impl Debug for OpCode {
  fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
    match self {
      OpCode::OrOp => write!(fmt, "||"),
      OpCode::AndOp => write!(fmt, "&&"),
      OpCode::EqOp => write!(fmt, "=="),
      OpCode::NeOp => write!(fmt, "!="),
      OpCode::LTOp => write!(fmt, "<"),
      OpCode::LEOp => write!(fmt, "<="),
      OpCode::GTOp => write!(fmt, ">"),
      OpCode::GEOp => write!(fmt, ">="),
      OpCode::Add => write!(fmt, "+"),
      OpCode::Sub => write!(fmt, "-"),
      OpCode::Mul => write!(fmt, "*"),
      OpCode::Div => write!(fmt, "/"),
      OpCode::Mod => write!(fmt, "%"),
      OpCode::Not => write!(fmt, "!"),
    }
  }
}

#[derive(PartialEq)]
pub enum Expr {
  IntegerLiteral(u64),
  FloatingLiteral(f64),
  BinaryOperator(Box<Expr>, OpCode, Box<Expr>),
  UnaryOperator(OpCode, Box<Expr>),
  Error,
}

impl Debug for Expr {
  fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
    use self::Expr::*;

    fn fmt_helper(
      expr: &Expr,
      fmt: &mut Formatter,
      level: usize,
    ) -> Result<(), std::fmt::Error> {
      match expr {
        IntegerLiteral(n) => {
          writeln!(fmt, "{}IntegerLiteral {}", "  ".repeat(level), n)
        }
        FloatingLiteral(n) => {
          writeln!(fmt, "{}FloatingLiteral {}", "  ".repeat(level), n)
        }
        BinaryOperator(lhs, op, rhs) => {
          writeln!(fmt, "{}BinaryOperator '{:?}'", "  ".repeat(level), op)?;
          fmt_helper(lhs, fmt, level + 1)?;
          fmt_helper(rhs, fmt, level + 1)
        }
        UnaryOperator(op, expr) => {
          writeln!(
            fmt,
            "{}UaryOperator 'prefix' '{:?}'",
            "  ".repeat(level),
            op
          )?;
          fmt_helper(expr, fmt, level + 1)
        }
        Error => write!(fmt, "error"),
      }
    }

    fmt_helper(self, fmt, 0)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let expr = Box::new(Expr::BinaryOperator(
      Box::new(Expr::BinaryOperator(
        Box::new(Expr::IntegerLiteral(22)),
        OpCode::Mul,
        Box::new(Expr::IntegerLiteral(44)),
      )),
      OpCode::Add,
      Box::new(Expr::UnaryOperator(
        OpCode::Sub,
        Box::new(Expr::IntegerLiteral(66)),
      )),
    ));

    println!("{:?}", expr);
  }
}
