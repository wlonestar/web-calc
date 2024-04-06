use crate::{ast::*, parser::Parser};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalculateError {
  DivisionZero,
  NotBinaryOperator,
  NotUnaryOperator,
  NonBinaryOperator,
  NonUnaryOperator,
  UnknownExpression,
  ParsingError,
}

fn calc(expr: &Expr) -> Result<f64, CalculateError> {
  match expr {
    Expr::IntegerLiteral(n) => Ok(*n as f64),
    Expr::FloatingLiteral(n) => Ok(*n as f64),
    Expr::BinaryOperator(ref lhs, op, ref rhs) => {
      let l = calc(&lhs);
      let r = calc(&rhs);
      if let (Ok(l), Ok(r)) = (l, r) {
        match op {
          OpCode::OrOp => {
            if l != 0. || r != 0. {
              Ok(1 as f64)
            } else {
              Ok(0 as f64)
            }
          }
          OpCode::AndOp => {
            if l != 0. && r != 0. {
              Ok(1 as f64)
            } else {
              Ok(0 as f64)
            }
          }
          OpCode::EqOp => {
            if l == r {
              Ok(1 as f64)
            } else {
              Ok(0 as f64)
            }
          }
          OpCode::NeOp => {
            if l != r {
              Ok(1 as f64)
            } else {
              Ok(0 as f64)
            }
          }
          OpCode::LTOp => {
            if l < r {
              Ok(1 as f64)
            } else {
              Ok(0 as f64)
            }
          }
          OpCode::LEOp => {
            if l <= r {
              Ok(1 as f64)
            } else {
              Ok(0 as f64)
            }
          }
          OpCode::GTOp => {
            if l > r {
              Ok(1 as f64)
            } else {
              Ok(0 as f64)
            }
          }
          OpCode::GEOp => {
            if l >= r {
              Ok(1 as f64)
            } else {
              Ok(0 as f64)
            }
          }
          OpCode::Add => Ok(l + r),
          OpCode::Sub => Ok(l - r),
          OpCode::Mul => Ok(l * r),
          OpCode::Div => {
            if r == 0. {
              Err(CalculateError::DivisionZero)
            } else {
              Ok(l / r)
            }
          }
          OpCode::Mod => Ok(l % r),
          _ => Err(CalculateError::NotBinaryOperator),
        }
      } else {
        Err(CalculateError::NonBinaryOperator)
      }
    }
    Expr::UnaryOperator(op, ref expr) => {
      let expr = calc(&expr);
      if let Ok(expr) = expr {
        match op {
          OpCode::Sub => Ok(-expr),
          OpCode::Not => Ok(!(expr as i64) as f64),
          _ => Err(CalculateError::NotUnaryOperator),
        }
      } else {
        Err(CalculateError::NonUnaryOperator)
      }
    }
    Expr::Error => Err(CalculateError::UnknownExpression),
  }
}

pub fn evaluate(input: &str) -> Result<f64, CalculateError> {
  let parser = Parser::new();
  let expr = parser.parse(input);
  if expr.is_err() {
    Err(CalculateError::ParsingError)
  } else {
    let expr = expr.unwrap();
    calc(&expr)
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_integer() {
    let input = "12";
    let result = evaluate(input).unwrap();
    assert_eq!(result, 12.);
  }

  #[test]
  fn test_unary() {
    assert_eq!(evaluate("-1").unwrap(), -1.);
  }

  #[test]
  fn test_binary() {
    assert_eq!(evaluate("1+2*3").unwrap(), 7.);
  }

  #[test]
  fn test_error() {
    let result = evaluate("1/0");
    assert!(result.is_err());
  }
}
