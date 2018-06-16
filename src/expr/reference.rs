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

//! Contains bound references representing columns.
/*
use std::fmt;
use std::any;

use expr::api::*;
use types::DataType;

/// Column reference, which can be either bound or unbound depending on the provided
/// data type.
#[derive(Clone, Debug, PartialEq)]
pub struct Reference {
  name: String,
  data_type: Option<DataType>,
  nullable: bool
}

impl Reference {
  pub fn new(name: String, data_type: Option<DataType>, nullable: bool) -> Self {
    Self {
      name: name,
      data_type: data_type,
      nullable: nullable
    }
  }
}

impl fmt::Display for Reference {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self.data_type {
      Some(_) => write!(f, "{}#", self.name),
      None => write!(f, "'{}", self.name)
    }
  }
}

impl OutputDataType for Reference {
  fn output_datatype(&self) -> &DataType {
    match self.data_type {
      Some(ref dt) => dt,
      None => panic!("Cannot extract data type from unresolved reference, \
        resolve attributes first")
    }
  }
}

impl ResolveExpression for Reference {
  fn resolve(&self) -> bool {
    self.data_type.is_some()
  }
}

impl Expression for Reference {
  fn foldable(&self) -> bool {
    false
  }

  fn deterministic(&self) -> bool {
    // Reference is assumed to be deterministic, since it comes from the relation.
    true
  }

  fn nullable(&self) -> bool {
    self.nullable
  }

  /// Returns a user-facing string representation of this expression's name.
  fn pretty_name(&self) -> String {
    format!("{:?}", self)
  }

  fn clone_as_expr(&self) -> Box<Expression> {
    Box::new(self.clone())
  }

  fn eq_as_expr(&self, other: &Box<Expression>) -> bool {
    match Box::new(other.as_any_ref()).downcast_ref::<Self>() {
      Some(literal) => self.eq(literal),
      None => false
    }
  }

  fn as_tree(&self) -> ExpressionTreeNode {
    ExpressionTreeNode::new(self.clone_as_expr(), vec![])
  }

  fn as_any_ref(&self) -> &any::Any {
    self
  }
}
*/
