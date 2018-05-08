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

//! Arithmetic expressions.

use std::any;
use std::fmt;

use expr::api::{Expression, ExpressionTreeNode};
use types::DataType;

macro_rules! binary_expression {
  ($struct_name:ident, $symbol:expr, $pretty_name:expr) => {
    pub struct $struct_name {
      left: Box<Expression>,
      right: Box<Expression>
    }

    impl $struct_name {
      pub fn new(left: Box<Expression>, right: Box<Expression>) -> Self {
        Self { left: left, right: right }
      }
    }

    impl fmt::Display for $struct_name {
      fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {} {})", self.left, $symbol, self.right)
      }
    }

    impl Expression for $struct_name {
      fn foldable(&self) -> bool {
        self.left.foldable() && self.right.foldable()
      }

      fn deterministic(&self) -> bool {
        self.left.deterministic() && self.right.deterministic()
      }

      fn nullable(&self) -> bool {
        self.left.nullable() || self.right.nullable()
      }

      fn resolved(&self) -> bool {
        self.left.resolved() && self.right.resolved()
      }

      fn data_type(&self) -> &DataType {
        self.left.data_type()
      }

      fn pretty_name(&self) -> &str {
        $pretty_name
      }

      fn clone_as_expr(&self) -> Box<Expression> {
        Box::new(self.clone())
      }

      fn eq_as_expr(&self, other: &Box<Expression>) -> bool {
        match Box::new(other.as_any_ref()).downcast_ref::<Self>() {
          Some(expr) => {
            self.left.eq_as_expr(&expr.left) && self.right.eq_as_expr(&expr.right)
          },
          None => false
        }
      }

      /// Converts current expression into an expression tree.
      fn as_tree(&self) -> ExpressionTreeNode {
        let left_node = self.left.as_tree();
        let right_node = self.right.as_tree();
        ExpressionTreeNode::new(self.clone_as_expr(), vec![left_node, right_node])
      }

      fn as_any_ref(&self) -> &any::Any {
        self
      }
    }

    impl Clone for $struct_name {
      fn clone(&self) -> Self {
        let left_expr = self.left.clone_as_expr();
        let right_expr = self.right.clone_as_expr();
        Self::new(left_expr, right_expr)
      }
    }

    impl PartialEq for $struct_name {
      fn eq(&self, other: &$struct_name) -> bool {
        self.left.eq_as_expr(&other.left) && self.right.eq_as_expr(&other.right)
      }
    }
  };
}

binary_expression![Add, "+", "add"];
binary_expression![Subtract, "-", "subtract"];
binary_expression![Multiply, "*", "multiply"];
binary_expression![Divide, "/", "divide"];
