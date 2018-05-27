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

//! Common macros and functions for expressions.

macro_rules! binary_expression {
  ($struct_name:ident, $symbol:expr, $pretty_name:expr, $($specs:item),*) => {
    pub struct $struct_name {
      left: Box<::expr::api::Expression>,
      right: Box<::expr::api::Expression>
    }

    impl $struct_name {
      pub fn new(
        left: Box<::expr::api::Expression>,
        right: Box<::expr::api::Expression>
      ) -> Self
      {
        Self { left: left, right: right }
      }
    }

    // List of specialised trait implementations.
    $(
      $specs
    )*

    impl ::std::fmt::Display for $struct_name {
      fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "({} {} {})", self.left, $symbol, self.right)
      }
    }

    impl ::expr::api::Expression for $struct_name {
      fn foldable(&self) -> bool {
        self.left.foldable() && self.right.foldable()
      }

      fn deterministic(&self) -> bool {
        self.left.deterministic() && self.right.deterministic()
      }

      fn nullable(&self) -> bool {
        self.left.nullable() || self.right.nullable()
      }

      fn pretty_name(&self) -> String {
        $pretty_name.to_owned()
      }

      fn clone_as_expr(&self) -> Box<::expr::api::Expression> {
        Box::new(self.clone())
      }

      fn eq_as_expr(&self, other: &Box<::expr::api::Expression>) -> bool {
        match Box::new(other.as_any_ref()).downcast_ref::<Self>() {
          Some(expr) => {
            self.left.eq_as_expr(&expr.left) && self.right.eq_as_expr(&expr.right)
          },
          None => false
        }
      }

      fn as_tree(&self) -> ::expr::api::ExpressionTreeNode {
        let left_node = self.left.as_tree();
        let right_node = self.right.as_tree();
        ::expr::api::ExpressionTreeNode::new(
          self.clone_as_expr(),
          vec![left_node, right_node]
        )
      }

      fn as_any_ref(&self) -> &::std::any::Any {
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
  }
}

macro_rules! unary_expression {
  ($struct_name:ident, $symbol:expr, $pretty_name:expr, $($specs:item),*) => {
    pub struct $struct_name {
      child: Box<::expr::api::Expression>
    }

    impl $struct_name {
      pub fn new(child: Box<::expr::api::Expression>) -> Self {
        Self { child: child }
      }
    }

    // List of specialised trait implementations.
    $(
      $specs
    )*

    impl ::std::fmt::Display for $struct_name {
      fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}({})", $symbol, self.child)
      }
    }

    impl ::expr::api::Expression for $struct_name {
      fn foldable(&self) -> bool {
        self.child.foldable()
      }

      fn deterministic(&self) -> bool {
        self.child.deterministic()
      }

      fn nullable(&self) -> bool {
        self.child.nullable()
      }

      fn pretty_name(&self) -> String {
        $pretty_name.to_owned()
      }

      fn clone_as_expr(&self) -> Box<::expr::api::Expression> {
        Box::new(self.clone())
      }

      fn eq_as_expr(&self, other: &Box<::expr::api::Expression>) -> bool {
        match Box::new(other.as_any_ref()).downcast_ref::<Self>() {
          Some(expr) => {
            self.child.eq_as_expr(&expr.child)
          },
          None => false
        }
      }

      fn as_tree(&self) -> ::expr::api::ExpressionTreeNode {
        let child_node = self.child.as_tree();
        ::expr::api::ExpressionTreeNode::new(self.clone_as_expr(), vec![child_node])
      }

      fn as_any_ref(&self) -> &::std::any::Any {
        self
      }
    }

    impl Clone for $struct_name {
      fn clone(&self) -> Self {
        let child_expr = self.child.clone_as_expr();
        Self::new(child_expr)
      }
    }

    impl PartialEq for $struct_name {
      fn eq(&self, other: &$struct_name) -> bool {
        self.child.eq_as_expr(&other.child)
      }
    }
  }
}
