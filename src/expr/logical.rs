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

//! Logical expressions.

use expr::api::{Expression, ExpressionBuilder, binary, unary};
use types::DataType;

/// Returns builder for logical binary expression.
fn logical_binary(
  name: &str,
  symbol: &str,
  left: Expression,
  right: Expression) ->
ExpressionBuilder
{
  binary(name.to_owned(), symbol.to_owned(), left, right)
    .datatype(Box::new(|_| &DataType::BooleanType))
}

/// Returns builder for logical unary expression.
fn logical_unary(name: &str, symbol: &str, child: Expression) -> ExpressionBuilder {
  unary(name.to_owned(), symbol.to_owned(), child)
    .datatype(Box::new(|_| &DataType::BooleanType))
}

/// Left > right.
pub fn gt(left: Expression, right: Expression) -> Expression {
  logical_binary("GREATER_THAN", ">", left, right)
    .clone(Box::new(|exp| {
      gt(exp.children()[0].clone(), exp.children()[1].clone())
    }))
    .build()
}

/// Left >= right.
pub fn ge(left: Expression, right: Expression) -> Expression {
  logical_binary("GREATER_OR_EQUAL", ">=", left, right)
    .clone(Box::new(|exp| {
      ge(exp.children()[0].clone(), exp.children()[1].clone())
    }))
    .build()
}

/// Left < right.
pub fn lt(left: Expression, right: Expression) -> Expression {
  logical_binary("LESS_THAN", "<", left, right)
    .clone(Box::new(|exp| {
      lt(exp.children()[0].clone(), exp.children()[1].clone())
    }))
    .build()
}

/// Left <= right.
pub fn le(left: Expression, right: Expression) -> Expression {
  logical_binary("LESS_OR_EQUAL", "<=", left, right)
    .clone(Box::new(|exp| {
      le(exp.children()[0].clone(), exp.children()[1].clone())
    }))
    .build()
}

/// Left && right.
pub fn and(left: Expression, right: Expression) -> Expression {
  logical_binary("AND", "&&", left, right)
    .clone(Box::new(|exp| {
      and(exp.children()[0].clone(), exp.children()[1].clone())
    }))
    .build()
}

/// Left || right.
pub fn or(left: Expression, right: Expression) -> Expression {
  logical_binary("OR", "||", left, right)
    .clone(Box::new(|exp| {
      or(exp.children()[0].clone(), exp.children()[1].clone())
    }))
    .build()
}

/// Negation
pub fn not(child: Expression) -> Expression {
  logical_unary("NOT", "!", child)
    .clone(Box::new(|exp| {
      not(exp.children()[0].clone())
    }))
    .build()
}

/// Is null
pub fn is_null(child: Expression) -> Expression {
  logical_unary("IS_NULL", "", child)
    .display(Box::new(|exp| {
      format!("({} is null)", exp.children()[0].pretty_string())
    }))
    .clone(Box::new(|exp| {
      is_null(exp.children()[0].clone())
    }))
    .build()
}
