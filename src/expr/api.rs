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

//! Contains API definitions for creating and manipulating expressions.

use std::any;
use std::fmt;

use trees::TreeNode;
use types::DataType;

/// A base expression trait.
///
/// Do not provide specific implementations on `Expression` trait, use specialised traits
/// instead.
pub trait Expression: fmt::Display + OutputDataType {
  /// Returns `true` when an expression is a candidate for static evaluation before the
  /// query is executed.
  ///
  /// The following conditions are used to determine suitability for constant folding:
  /// - binary expression is foldable if its both left and right child are foldable.
  /// - literal is foldable.
  /// - `coalesce` is foldable if all of its children are foldable.
  /// - `not`, `is_null`, or `is_not_null` is foldable if its child is foldable.
  /// - `cast` or `minus` is foldable if its child is foldable.
  fn foldable(&self) -> bool;

  /// Returns `true` when the current expression always return the same result for fixed
  /// inputs from children.
  ///
  /// An expression should be considered as non-deterministic if:
  /// - it relies on some mutable internal state, or
  /// - it relies on some implicit input that is not part of the children expression list.
  /// - it has non-deterministic child or children.
  /// - it assumes the input satisfies some certain condition via the child operator.
  fn deterministic(&self) -> bool;

  /// Returns `true` when the current expression is nullable.
  ///
  /// Some expressions rely on nullability of their children to determine the nullability
  /// of the current expression.
  fn nullable(&self) -> bool;

  /// Returns `true` if this expression and all its children have been resolved to a
  /// specific schema and input data types checking passed, and `false` if it still
  /// contains any unresolved placeholders or has data types mismatch.
  fn resolved(&self) -> bool;

  /// Returns the data type of the result of evaluating this expression.
  ///
  /// It is invalid to query the dataType of an unresolved expression
  /// (i.e., when `resolved` == `false`).
  fn data_type(&self) -> &DataType {
    self.output_datatype()
  }

  /// Returns a user-facing string representation of this expression's name.
  fn pretty_name(&self) -> &str;

  /// Clones this expression and returns it as `Box<Expression>`.
  /// This method should behave similar to `Clone` trait.
  fn clone_as_expr(&self) -> Box<Expression>;

  /// Partial equality for expression.
  /// This method should behave similar to `PartialEq` trait.
  fn eq_as_expr(&self, other: &Box<Expression>) -> bool;

  /// Converts current expression into an expression tree.
  fn as_tree(&self) -> ExpressionTreeNode;

  /// Converts current expression into `Any` for downcast in equality.
  /// Normally should return `self` for concrete types.
  fn as_any_ref(&self) -> &any::Any;
}

/// Trait for providing the data type of the result of evaluating this expression.
pub trait OutputDataType {
  fn output_datatype(&self) -> &DataType;
}

/// Tree node for expression.
///
/// Should be used for all expressions defined in the optimizer or any external
/// expressions. Provides tree methods for traversal and folding.
pub struct ExpressionTreeNode {
  // Current expression that this tree node is created for.
  expr: Box<Expression>,
  // Child expressions.
  children: Vec<ExpressionTreeNode>
}

impl ExpressionTreeNode {
  /// Creates new expression tree node.
  pub fn new(expr: Box<Expression>, children: Vec<ExpressionTreeNode>) -> Self {
    Self { expr: expr, children: children }
  }
}

impl TreeNode<ExpressionTreeNode> for ExpressionTreeNode {
  fn node_name(&self) -> String {
    self.expr.pretty_name().to_owned()
  }

  fn verbose_string(&self) -> String {
    format!("{}", self.expr)
  }

  fn get(&self) -> &ExpressionTreeNode {
    &self
  }

  fn num_children(&self) -> usize {
    self.children.len()
  }

  fn get_child(&self, idx: usize) -> Option<&ExpressionTreeNode> {
    self.children.get(idx)
  }

  fn set_child(&mut self, idx: usize, child: ExpressionTreeNode) {
    self.children[idx] = child;
  }

  fn clone_tree(&self) -> ExpressionTreeNode {
    self.clone()
  }

  fn equals(&self, other: &ExpressionTreeNode) -> bool {
    self.eq(other)
  }
}

impl Clone for ExpressionTreeNode {
  fn clone(&self) -> Self {
    ExpressionTreeNode::new(self.expr.clone_as_expr(), self.children.clone())
  }
}

impl PartialEq for ExpressionTreeNode {
  fn eq(&self, other: &ExpressionTreeNode) -> bool {
    self.expr.eq_as_expr(&other.expr) && self.children.eq(&other.children)
  }
}
