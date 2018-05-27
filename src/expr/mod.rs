// Copyright 2017 sadikovi
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Module defines base expression trait and implementations.

pub mod api;
#[macro_use]
pub mod common;
pub mod literal;
pub mod arithmetic;
pub mod logical;
pub mod reference;

// Literals

/// Macro to generate literals.
#[macro_export]
macro_rules! lit {
  ($value:expr, bool) => {{
    Box::new(::expr::literal::Literal::Boolean($value))
  }};
  ($value:expr, i8) => {{
    Box::new(::expr::literal::Literal::Byte($value))
  }};
  ($value:expr, i16) => {{
    Box::new(::expr::literal::Literal::Short($value))
  }};
  ($value:expr, i32) => {{
    Box::new(::expr::literal::Literal::Integer($value))
  }};
  ($value:expr, f32) => {{
    Box::new(::expr::literal::Literal::Float($value))
  }};
  ($value:expr, f64) => {{
    Box::new(::expr::literal::Literal::Double($value))
  }};
  ($value:expr, str) => {{
    Box::new(::expr::literal::Literal::String($value))
  }};
}

// Arithmetic expressions

/// Macro for generating `Add` expression.
#[macro_export]
macro_rules! add {
  ($left:expr, $right:expr) => {{
    Box::new(::expr::arithmetic::Add::new($left, $right))
  }}
}

/// Macro for generating `Subtract` expression.
#[macro_export]
macro_rules! sub {
  ($left:expr, $right:expr) => {{
    Box::new(::expr::arithmetic::Subtract::new($left, $right))
  }}
}

/// Macro for generating `Multiply` expression.
#[macro_export]
macro_rules! mul {
  ($left:expr, $right:expr) => {{
    Box::new(::expr::arithmetic::Multiply::new($left, $right))
  }}
}

/// Macro for generating `Divide` expression.
#[macro_export]
macro_rules! div {
  ($left:expr, $right:expr) => {{
    Box::new(::expr::arithmetic::Divide::new($left, $right))
  }}
}

// Logical expressions

/// Macro for generating `GreaterThan` expression.
#[macro_export]
macro_rules! gt {
  ($left:expr, $right:expr) => {{
    Box::new(::expr::logical::GreaterThan::new($left, $right))
  }}
}

/// Macro for generating `LessThan` expression.
#[macro_export]
macro_rules! lt {
  ($left:expr, $right:expr) => {{
    Box::new(::expr::logical::LessThan::new($left, $right))
  }}
}

/// Macro for generating `GreaterThanOrEqual` expression.
#[macro_export]
macro_rules! gteq {
  ($left:expr, $right:expr) => {{
    Box::new(::expr::logical::GreaterThanOrEqual::new($left, $right))
  }}
}

/// Macro for generating `LessThanOrEqual` expression.
#[macro_export]
macro_rules! lteq {
  ($left:expr, $right:expr) => {{
    Box::new(::expr::logical::LessThanOrEqual::new($left, $right))
  }}
}

/// Macro for generating `Equals` expression.
#[macro_export]
macro_rules! eq {
  ($left:expr, $right:expr) => {{
    Box::new(::expr::logical::Equals::new($left, $right))
  }}
}

/// Macro for generating `And` expression.
#[macro_export]
macro_rules! and {
  ($left:expr, $right:expr) => {{
    Box::new(::expr::logical::And::new($left, $right))
  }}
}

/// Macro for generating `Or` expression.
#[macro_export]
macro_rules! or {
  ($left:expr, $right:expr) => {{
    Box::new(::expr::logical::Or::new($left, $right))
  }}
}

/// Macro for generating `Not` expression.
#[macro_export]
macro_rules! not {
  ($child:expr) => {{
    Box::new(::expr::logical::Not::new($child))
  }}
}

#[cfg(test)]
mod tests {
  use super::*;
  use self::api::Expression;

  #[test]
  fn test_arithmetic_expression_tree() {
    let t = add![
      lit![Some(1), i8],
      lit![Some(2), i8]
    ];
    assert_eq!(t.pretty_name(), "add");
    assert_eq!(t.to_string(), "(1 + 2)");

    let t = sub![
      lit![Some(1), i8],
      lit![Some(2), i8]
    ];
    assert_eq!(t.pretty_name(), "subtract");
    assert_eq!(t.to_string(), "(1 - 2)");

    let t = mul![
      lit![Some(1), i8],
      lit![Some(2), i8]
    ];
    assert_eq!(t.pretty_name(), "multiply");
    assert_eq!(t.to_string(), "(1 * 2)");

    let t = div![
      lit![Some(1), i8],
      lit![Some(2), i8]
    ];
    assert_eq!(t.pretty_name(), "divide");
    assert_eq!(t.to_string(), "(1 / 2)");
  }

  #[test]
  fn test_logical_expression_tree() {
    let t = gt![
      lit![Some(2), i32],
      lit![Some(1), i32]
    ];

    assert_eq!(t.pretty_name(), "greater than");
    assert_eq!(t.to_string(), "(2 > 1)");

    let t = lt![
      lit![Some(2), i32],
      lit![Some(1), i32]
    ];

    assert_eq!(t.pretty_name(), "less than");
    assert_eq!(t.to_string(), "(2 < 1)");

    let t = gteq![
      lit![Some(2), i32],
      lit![Some(1), i32]
    ];

    assert_eq!(t.pretty_name(), "greater than or equal");
    assert_eq!(t.to_string(), "(2 >= 1)");

    let t = lteq![
      lit![Some(2), i32],
      lit![Some(1), i32]
    ];

    assert_eq!(t.pretty_name(), "less than or equal");
    assert_eq!(t.to_string(), "(2 <= 1)");

    let t = eq![
      lit![Some(2), i32],
      lit![Some(2), i32]
    ];

    assert_eq!(t.pretty_name(), "equals");
    assert_eq!(t.to_string(), "(2 == 2)");

    let t = and![
      lit![Some(true), bool],
      lit![Some(false), bool]
    ];

    assert_eq!(t.pretty_name(), "and");
    assert_eq!(t.to_string(), "(true && false)");

    let t = or![
      lit![Some(true), bool],
      lit![Some(false), bool]
    ];

    assert_eq!(t.pretty_name(), "or");
    assert_eq!(t.to_string(), "(true || false)");

    let t = not![
      lit![Some(true), bool]
    ];

    assert_eq!(t.pretty_name(), "not");
    assert_eq!(t.to_string(), "!(true)");
  }

  #[test]
  fn test_arithmetic_expression_tree_resolve() {
    // Resolved expressions

    let t = add![lit![Some(1), i32], lit![Some(2), i32]];
    assert!(t.resolved());

    let t = sub![lit![Some(1), i32], lit![Some(2), i32]];
    assert!(t.resolved());

    let t = mul![lit![Some(1), i32], lit![Some(2), i32]];
    assert!(t.resolved());

    let t = div![lit![Some(1), i32], lit![Some(2), i32]];
    assert!(t.resolved());

    // Unresolved expressions

    let t = add![lit![Some(1), i32], lit![Some(true), bool]];
    assert!(!t.resolved());

    let t = sub![lit![Some(1), i32], lit![Some(true), bool]];
    assert!(!t.resolved());

    let t = mul![lit![Some(true), bool], lit![Some(2), i32]];
    assert!(!t.resolved());

    let t = div![lit![Some(true), bool], lit![Some(2), i32]];
    assert!(!t.resolved());
  }

  #[test]
  fn test_expression_tree_resolve() {
    // Resolved expressions

    let t = and![lit![Some(true), bool], lit![Some(true), bool]];
    assert!(t.resolved());

    let t = or![lit![Some(true), bool], lit![Some(true), bool]];
    assert!(t.resolved());

    let t = gt![lit![Some(2), i32], lit![Some(1), i32]];
    assert!(t.resolved());

    let t = lt![lit![Some(2), i32], lit![Some(1), i32]];
    assert!(t.resolved());

    let t = gteq![lit![Some(2), i32], lit![Some(1), i32]];
    assert!(t.resolved());

    let t = lteq![lit![Some(2), i32], lit![Some(1), i32]];
    assert!(t.resolved());

    let t = not![lit![Some(true), bool]];
    assert!(t.resolved());

    // Unresolved expressions

    let t = and![lit![Some(1), i32], lit![Some(true), bool]];
    assert!(!t.resolved());

    let t = or![lit![Some(2), i32], lit![Some(true), bool]];
    assert!(!t.resolved());

    let t = gt![lit![Some(2), i32], lit![Some(true), bool]];
    assert!(!t.resolved());

    let t = lt![lit![Some(2.0), f32], lit![Some(1), i32]];
    assert!(!t.resolved());

    let t = gteq![lit![Some(2.0), f32], lit![Some(1), i32]];
    assert!(!t.resolved());

    let t = lteq![lit![Some(true), bool], lit![Some(1), i32]];
    assert!(!t.resolved());

    let t = not![lit![Some(1), i32]];
    assert!(!t.resolved());
  }
}
