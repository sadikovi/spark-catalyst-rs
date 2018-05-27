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

use expr::api::*;
use types::DataType;

#[derive(Clone, PartialEq)]
pub enum Literal {
  Boolean(Option<bool>),
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
      Literal::Boolean(value) => value.is_none(),
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

impl fmt::Display for Literal {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    if self.is_null() {
      write!(f, "null")
    } else {
      match self {
        Literal::Boolean(value) => write!(f, "{}", value.unwrap()),
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

impl OutputDataType for Literal {
  fn output_datatype(&self) -> &DataType {
    match self {
      Literal::Boolean(_) => &DataType::BooleanType,
      Literal::Byte(_) => &DataType::ByteType,
      Literal::Short(_) => &DataType::ShortType,
      Literal::Integer(_) => &DataType::IntegerType,
      Literal::Long(_) => &DataType::LongType,
      Literal::Float(_) => &DataType::FloatType,
      Literal::Double(_) => &DataType::DoubleType,
      Literal::String(_) => &DataType::StringType
    }
  }
}

impl ResolveExpression for Literal {
  fn resolve(&self) -> bool {
    true
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

  fn pretty_name(&self) -> String {
    "literal".to_owned()
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

  /// Converts current expression into an expression tree.
  fn as_tree(&self) -> ExpressionTreeNode {
    ExpressionTreeNode::new(self.clone_as_expr(), vec![])
  }

  fn as_any_ref(&self) -> &any::Any {
    self
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_literal_is_null() {
    assert_eq!(Literal::Boolean(Some(true)).is_null(), false);
    assert_eq!(Literal::Byte(Some(1)).is_null(), false);
    assert_eq!(Literal::Short(Some(1)).is_null(), false);
    assert_eq!(Literal::Integer(Some(1)).is_null(), false);
    assert_eq!(Literal::Long(Some(1)).is_null(), false);
    assert_eq!(Literal::Float(Some(1.2)).is_null(), false);
    assert_eq!(Literal::Double(Some(1.2)).is_null(), false);
    assert_eq!(Literal::String(Some("abc".to_string())).is_null(), false);

    assert_eq!(Literal::Boolean(None).is_null(), true);
    assert_eq!(Literal::Byte(None).is_null(), true);
    assert_eq!(Literal::Short(None).is_null(), true);
    assert_eq!(Literal::Integer(None).is_null(), true);
    assert_eq!(Literal::Long(None).is_null(), true);
    assert_eq!(Literal::Float(None).is_null(), true);
    assert_eq!(Literal::Double(None).is_null(), true);
    assert_eq!(Literal::String(None).is_null(), true);
  }

  #[test]
  fn test_literal_display() {
    assert_eq!(Literal::Boolean(Some(true)).to_string(), "true");
    assert_eq!(Literal::Byte(Some(1)).to_string(), "1");
    assert_eq!(Literal::Short(Some(1)).to_string(), "1");
    assert_eq!(Literal::Integer(Some(1)).to_string(), "1");
    assert_eq!(Literal::Long(Some(1)).to_string(), "1");
    assert_eq!(Literal::Float(Some(1.0)).to_string(), "1.0");
    assert_eq!(Literal::Double(Some(1.0)).to_string(), "1.0");
    assert_eq!(Literal::String(Some(String::from("abc"))).to_string(), "\"abc\"");

    assert_eq!(Literal::Boolean(None).to_string(), "null");
    assert_eq!(Literal::Byte(None).to_string(), "null");
    assert_eq!(Literal::Short(None).to_string(), "null");
    assert_eq!(Literal::Integer(None).to_string(), "null");
    assert_eq!(Literal::Long(None).to_string(), "null");
    assert_eq!(Literal::Float(None).to_string(), "null");
    assert_eq!(Literal::Double(None).to_string(), "null");
    assert_eq!(Literal::String(None).to_string(), "null");
  }

  #[test]
  fn test_literal_datatype() {
    assert_eq!(Literal::Boolean(None).data_type(), &DataType::BooleanType);
    assert_eq!(Literal::Byte(None).data_type(), &DataType::ByteType);
    assert_eq!(Literal::Short(None).data_type(), &DataType::ShortType);
    assert_eq!(Literal::Integer(None).data_type(), &DataType::IntegerType);
    assert_eq!(Literal::Long(None).data_type(), &DataType::LongType);
    assert_eq!(Literal::Float(None).data_type(), &DataType::FloatType);
    assert_eq!(Literal::Double(None).data_type(), &DataType::DoubleType);
    assert_eq!(Literal::String(None).data_type(), &DataType::StringType);
  }

  #[test]
  fn test_literal_eq_as_expr() {
    let a = Literal::Integer(Some(1));
    assert_eq!(a.eq_as_expr(&Literal::Integer(Some(1)).clone_as_expr()), true);
    assert_eq!(a.eq_as_expr(&Literal::Integer(Some(2)).clone_as_expr()), false);
    assert_eq!(
      a.eq_as_expr(&Literal::String(Some("abc".to_string())).clone_as_expr()),
      false
    );
    assert_eq!(a.eq_as_expr(&Literal::Integer(None).clone_as_expr()), false);
    assert_eq!(a.eq_as_expr(&Literal::Byte(Some(1)).clone_as_expr()), false);
  }
}
