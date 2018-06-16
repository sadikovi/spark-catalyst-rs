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

use trees::TreeNode;
use types::DataType;

/// A generic expression.
///
/// Each implementation should provide all closure fields.
pub struct Expression {
  // Unique node name.
  name: String,

  // List of children for the node.
  children: Vec<Expression>,

  // Pretty string for expression.
  display_func: Box<Fn(&Expression) -> String>,

  // Whether or not this expression is foldable.
  foldable_func: Box<Fn(&Expression) -> bool>,

  // Whether or not this expression is deterministic.
  deterministic_func: Box<Fn(&Expression) -> bool>,

  // Whether or not this expression is nullable.
  nullable_func: Box<Fn(&Expression) -> bool>,

  // Whether or not this expression is resolved.
  resolved_func: Box<Fn(&Expression) -> bool>,

  // Resulting data type for this expression.
  datatype_func: Box<Fn(&Expression) -> &DataType>,

  // Clone function for the expression.
  clone_func: Box<Fn(&Expression) -> Expression>,

  // Partial equality function for the expression.
  eq_func: Box<Fn(&Expression, &Expression) -> bool>
}

impl Expression {
  /// Returns a string with pretty print for the expression and all its children.
  pub fn pretty_string(&self) -> String {
    (self.display_func)(self)
  }

  /// Returns `true` when an expression is a candidate for static evaluation before the
  /// query is executed.
  ///
  /// The following conditions are used to determine suitability for constant folding:
  /// - binary expression is foldable if its both left and right child are foldable.
  /// - literal is foldable.
  /// - `coalesce` is foldable if all of its children are foldable.
  /// - `not`, `is_null`, or `is_not_null` is foldable if its child is foldable.
  /// - `cast` or `minus` is foldable if its child is foldable.
  pub fn foldable(&self) -> bool {
    (self.foldable_func)(self)
  }

  /// Returns `true` when the current expression always return the same result for fixed
  /// inputs from children.
  ///
  /// An expression should be considered as non-deterministic if:
  /// - it relies on some mutable internal state, or
  /// - it relies on some implicit input that is not part of the children expression list.
  /// - it has non-deterministic child or children.
  /// - it assumes the input satisfies some certain condition via the child operator.
  pub fn deterministic(&self) -> bool {
    (self.deterministic_func)(self)
  }

  /// Returns `true` when the current expression is nullable.
  ///
  /// Some expressions rely on nullability of their children to determine the nullability
  /// of the current expression.
  pub fn nullable(&self) -> bool {
    (self.nullable_func)(self)
  }

  /// Returns `true` if this expression and all its children have been resolved to a
  /// specific schema and input data types checking passed, and `false` if it still
  /// contains any unresolved placeholders or has data types mismatch.
  pub fn resolved(&self) -> bool {
    (self.resolved_func)(self)
  }

  /// Returns the data type of the result of evaluating this expression.
  ///
  /// It is invalid to query the dataType of an unresolved expression
  /// (i.e., when `resolved` == `false`).
  pub fn data_type(&self) -> &DataType {
    (self.datatype_func)(self)
  }

  /// Returns list of children for this expression.
  pub fn children(&self) -> &[Expression] {
    &self.children[..]
  }
}

impl TreeNode<Expression> for Expression {
  fn node_name(&self) -> String {
    format!("{}", self.name)
  }

  fn verbose_string(&self) -> String {
    format!("{}", self.pretty_string())
  }

  fn get(&self) -> &Expression {
    &self
  }

  fn num_children(&self) -> usize {
    self.children.len()
  }

  fn get_child(&self, pos: usize) -> Option<&Expression> {
    self.children.get(pos)
  }

  fn set_child(&mut self, pos: usize, child: Expression) {
    self.children[pos] = child;
  }

  fn clone_tree(&self) -> Expression {
    self.clone()
  }

  fn equals(&self, other: &Expression) -> bool {
    self.eq(other)
  }
}

impl Clone for Expression {
  fn clone(&self) -> Self {
    (self.clone_func)(self)
  }
}

impl PartialEq for Expression {
  fn eq(&self, other: &Expression) -> bool {
    (self.eq_func)(self, other)
  }
}

/// Expression builder.
/// Used to build templates for other expressions.
pub struct ExpressionBuilder {
  expression: Expression
}

impl ExpressionBuilder {
  /// Creates new expression builder.
  pub fn new(name: String) -> Self {
    Self {
      expression: Expression {
        name: name,
        children: vec![],
        display_func: Box::new(|_| unimplemented!()),
        foldable_func: Box::new(|_| unimplemented!()),
        deterministic_func: Box::new(|_| unimplemented!()),
        nullable_func: Box::new(|_| unimplemented!()),
        resolved_func: Box::new(|_| unimplemented!()),
        datatype_func: Box::new(|_| unimplemented!()),
        clone_func: Box::new(|_| unimplemented!()),
        eq_func: Box::new(|_, _| unimplemented!())
      }
    }
  }

  /// Sets children for the expression.
  pub fn children(mut self, value: Vec<Expression>) -> Self {
    self.expression.children = value;
    self
  }

  /// Sets display function.
  pub fn display(mut self, func: Box<Fn(&Expression) -> String>) -> Self {
    self.expression.display_func = func;
    self
  }

  /// Sets foldable function.
  pub fn foldable(mut self, func: Box<Fn(&Expression) -> bool>) -> Self {
    self.expression.foldable_func = func;
    self
  }

  /// Sets deterministic function.
  pub fn deterministic(mut self, func: Box<Fn(&Expression) -> bool>) -> Self {
    self.expression.deterministic_func = func;
    self
  }

  /// Sets nullable function.
  pub fn nullable(mut self, func: Box<Fn(&Expression) -> bool>) -> Self {
    self.expression.nullable_func = func;
    self
  }

  /// Sets resolved function.
  pub fn resolved(mut self, func: Box<Fn(&Expression) -> bool>) -> Self {
    self.expression.resolved_func = func;
    self
  }

  /// Sets data type function.
  pub fn datatype(mut self, func: Box<Fn(&Expression) -> &DataType>) -> Self {
    self.expression.datatype_func = func;
    self
  }

  /// Sets clone function.
  pub fn clone(mut self, func: Box<Fn(&Expression) -> Expression>) -> Self {
    self.expression.clone_func = func;
    self
  }

  /// Sets equality function.
  pub fn eq(mut self, func: Box<Fn(&Expression, &Expression) -> bool>) -> Self {
    self.expression.eq_func = func;
    self
  }

  /// Returns expression.
  pub fn build(self) -> Expression {
    self.expression
  }
}

/// Represents binary expression node.
pub fn binary(
  name: String,
  symbol: String,
  left: Expression,
  right: Expression
) -> ExpressionBuilder
{
  ExpressionBuilder::new(name)
    .children(vec![left, right])
    .display(Box::new(move |exp| {
      format!("({} {} {})",
        exp.children[0].pretty_string(),
        symbol,
        exp.children[1].pretty_string()
      )
    }))
    .foldable(Box::new(|exp| {
      exp.children[0].foldable() && exp.children[1].foldable()
    }))
    .deterministic(Box::new(|exp| {
      exp.children[0].deterministic() && exp.children[1].deterministic()
    }))
    .nullable(Box::new(|exp| {
      exp.children[0].nullable() || exp.children[1].nullable()
    }))
    .resolved(Box::new(|exp| {
      exp.children[0].resolved() && exp.children[1].resolved()
    }))
    .datatype(Box::new(|exp| {
      exp.children[0].data_type()
    }))
    .clone(Box::new(|_| {
      unimplemented!()
    }))
    .eq(Box::new(|a, b| {
      a.name == b.name &&
        a.children.len() == b.children.len() &&
        a.children[0].eq(&b.children[0]) &&
        a.children[1].eq(&b.children[1])
    }))
}

// Represents unary expression node.
pub fn unary(name: String, symbol: String, child: Expression) -> ExpressionBuilder {
  ExpressionBuilder::new(name)
    .children(vec![child])
    .display(Box::new(move |exp| {
      format!("({}{})", symbol, exp.children[0].pretty_string())
    }))
    .foldable(Box::new(|exp| {
      exp.children[0].foldable()
    }))
    .deterministic(Box::new(|exp| {
      exp.children[0].deterministic()
    }))
    .nullable(Box::new(|exp| {
      exp.children[0].nullable()
    }))
    .resolved(Box::new(|exp| {
      exp.children[0].resolved()
    }))
    .datatype(Box::new(|exp| {
      exp.children[0].data_type()
    }))
    .clone(Box::new(|_| {
      unimplemented!()
    }))
    .eq(Box::new(|a, b| {
      a.name == b.name &&
        a.children.len() == b.children.len() &&
        a.children[0].eq(&b.children[0])
    }))
}
