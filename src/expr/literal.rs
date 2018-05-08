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

//! Literal expressions.

use std::any;
use std::fmt;

use expr::api::{Expression, ExpressionTreeNode};
use types::DataType;

#[derive(Clone, PartialEq)]
pub enum Literal {
  Byte(Option<i8>),
  Short(Option<i16>),
  Integer(Option<i32>),
  Long(Option<i64>),
  Float(Option<f32>),
  Double(Option<f64>),
  String(Option<String>)
}

impl Literal {
  pub fn is_null(&self) -> bool {
    match self {
      Literal::Byte(value) => value.is_none(),
      Literal::Short(value) => value.is_none(),
      Literal::Integer(value) => value.is_none(),
      Literal::Long(value) => value.is_none(),
      Literal::Float(value) => value.is_none(),
      Literal::Double(value) => value.is_none(),
      Literal::String(value) => value.is_none()
    }
  }
}

/// Trait provides a method overloading for creating literals from different values.
pub trait LiteralConvert<T> {
  fn lit(value: T) -> Self;
}

macro_rules! lit {
  ($source_type:ident, $literal:ident) => {
    impl LiteralConvert<$source_type> for Literal {
      fn lit(value: $source_type) -> Literal {
        Literal::$literal(Some(value))
      }
    }
  };
}

lit![i8, Byte];
lit![i16, Short];
lit![i32, Integer];
lit![i64, Long];
lit![f32, Float];
lit![f64, Double];
lit![String, String];

impl<'a> LiteralConvert<&'a str> for Literal {
  fn lit(value: &str) -> Literal {
    Literal::String(Some(value.to_string()))
  }
}

impl fmt::Display for Literal {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    if self.is_null() {
      write!(f, "null")
    } else {
      match self {
        Literal::Byte(value) => write!(f, "{}", value.unwrap()),
        Literal::Short(value) => write!(f, "{}", value.unwrap()),
        Literal::Integer(value) => write!(f, "{}", value.unwrap()),
        Literal::Long(value) => write!(f, "{}", value.unwrap()),
        Literal::Float(value) => write!(f, "{:?}", value.unwrap()),
        Literal::Double(value) => write!(f, "{:?}", value.unwrap()),
        Literal::String(value) => write!(f, "\"{}\"", value.as_ref().unwrap())
      }
    }
  }
}

impl Expression for Literal {
  fn foldable(&self) -> bool {
    true
  }

  fn deterministic(&self) -> bool {
    true
  }

  fn nullable(&self) -> bool {
    self.is_null()
  }

  fn resolved(&self) -> bool {
    true
  }

  fn data_type(&self) -> &DataType {
    match self {
      Literal::Byte(_) => &DataType::ByteType,
      Literal::Short(_) => &DataType::ShortType,
      Literal::Integer(_) => &DataType::IntegerType,
      Literal::Long(_) => &DataType::LongType,
      Literal::Float(_) => &DataType::FloatType,
      Literal::Double(_) => &DataType::DoubleType,
      Literal::String(_) => &DataType::StringType
    }
  }

  fn pretty_name(&self) -> &str {
    "literal"
  }

  fn clone_as_expr(&self) -> Box<Expression> {
    Box::new(self.clone())
  }

  fn eq_as_expr(&self, other: &Box<Expression>) -> bool {
    match Box::new(other.as_any_ref()).downcast_ref::<Literal>() {
      Some(literal) => self.eq(literal),
      None => false
    }
  }

  /// Converts current expression into an expression tree.
  fn as_tree(self) -> ExpressionTreeNode {
    ExpressionTreeNode::new(Box::new(self), vec![])
  }

  fn as_any_ref(&self) -> &any::Any {
    self
  }
}
