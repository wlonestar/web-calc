use crate::calculator;

pub type Parser = calculator::ExprParser;

#[cfg(test)]
mod tests {

  use super::*;
  use crate::{
    ast::{Expr, OpCode},
    CalculatorError,
  };
  use lalrpop_util::ParseError;

  #[test]
  fn test_integer() {
    let parser = Parser::new();
    let expr = parser.parse("123").unwrap();
    assert_eq!(expr, Box::new(Expr::IntegerLiteral(123)));
    let expr = parser.parse("0123").unwrap();
    assert_eq!(expr, Box::new(Expr::IntegerLiteral(0o123)));
    let expr = parser.parse("0x123").unwrap();
    assert_eq!(expr, Box::new(Expr::IntegerLiteral(0x123)));
  }

  #[test]
  fn test_floating() {
    let parser = Parser::new();
    let expr = parser.parse("1e+1").unwrap();
    assert_eq!(expr, Box::new(Expr::FloatingLiteral(1e+1)));
    let expr = parser.parse(".1e-3").unwrap();
    assert_eq!(expr, Box::new(Expr::FloatingLiteral(0.1e-3)));
    let expr = parser.parse("12.").unwrap();
    assert_eq!(expr, Box::new(Expr::FloatingLiteral(12.)));
  }

  #[test]
  fn test_binary_expr() {
    let expr = Parser::new().parse("22 * 44 + 66").unwrap();
    assert_eq!(
      expr,
      Box::new(Expr::BinaryOperator(
        Box::new(Expr::BinaryOperator(
          Box::new(Expr::IntegerLiteral(22)),
          OpCode::Mul,
          Box::new(Expr::IntegerLiteral(44))
        )),
        OpCode::Add,
        Box::new(Expr::IntegerLiteral(66))
      ))
    )
  }

  #[test]
  fn test_unary_expr() {
    let expr = Parser::new().parse("-1 || 2 && 3").unwrap();
    assert_eq!(
      expr,
      Box::new(Expr::BinaryOperator(
        Box::new(Expr::UnaryOperator(
          OpCode::Sub,
          Box::new(Expr::IntegerLiteral(1)),
        )),
        OpCode::OrOp,
        Box::new(Expr::BinaryOperator(
          Box::new(Expr::IntegerLiteral(2)),
          OpCode::AndOp,
          Box::new(Expr::IntegerLiteral(3))
        ))
      ))
    );
  }

  #[test]
  fn test_error() {
    let expr = calculator::ExprParser::new().parse("18446744073709551616");
    assert!(expr.is_err());
    assert_eq!(
      expr.unwrap_err(),
      ParseError::User {
        error: CalculatorError::InputTooBig
      }
    );
  }
}
