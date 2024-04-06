use interpreter::evaluate;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalculatorError {
  InputTooBig,
}

mod calculator;

pub mod ast;
pub mod interpreter;
pub mod parser;

#[wasm_bindgen]
pub fn calculate(input: &str) -> Option<f64> {
  evaluate(&input).ok().or(None)
}
