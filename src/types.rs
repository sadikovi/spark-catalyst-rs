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

//! Module for defining all supported data types.
//! This represents a subset of Spark SQL types.

use std::fmt;

/// The collection of all data types supported by the optimizer.
#[derive(Clone, Debug, PartialEq)]
pub enum DataType {
  BooleanType,
  ByteType,
  ShortType,
  IntegerType,
  LongType,
  FloatType,
  DoubleType,
  StringType,
  StructType(Vec<StructField>)
}

impl DataType {
  /// Creates new `StructType` with a list of fields.
  pub fn struct_type(fields: Vec<StructField>) -> DataType {
    DataType::StructType(fields)
  }

  /// Adds a `StructField` to this type.
  /// Panics if current field is not `StructType`.
  pub fn add(mut self, field: StructField) -> DataType {
    match self {
      DataType::StructType(ref mut fields) => fields.push(field),
      _ => panic!("Not a StructType")
    }
    self
  }

  /// Adds `StructField` as name and data type.
  pub fn add_field(self, name: &str, data_type: DataType) -> DataType {
    self.add(StructField::new(name.to_owned(), data_type))
  }

  /// Adds `StructField` as name, data type, and nullable flag.
  pub fn add_field_n(self, name: &str, data_type: DataType, nullable: bool) -> DataType {
    self.add(StructField::new(name.to_owned(), data_type).with_nullable(nullable))
  }

  /// Default size in bytes of a value of this data type, used for size estimation.
  pub fn default_size(&self) -> usize {
    match self {
      DataType::BooleanType => 1,
      DataType::ByteType => 1,
      DataType::ShortType => 2,
      DataType::IntegerType => 4,
      DataType::LongType => 8,
      DataType::FloatType => 4,
      DataType::DoubleType => 8,
      DataType::StringType => 20,
      DataType::StructType(ref fields) => {
        fields.iter().map(|field| field.data_type().default_size()).sum()
      }
    }
  }

  /// Returns number of fields in this struct type.
  /// Panics if field is not `StructType`.
  pub fn num_fields(&self) -> usize {
    match self {
      DataType::StructType(ref fields) => fields.len(),
      _ => panic!("Not a StructType")
    }
  }

  /// Returns `true` if type is `StructType`, `false` otherwise.
  pub fn is_struct(&self) -> bool {
    match self {
      DataType::StructType(_) => true,
      _ => false
    }
  }

  /// Returns `true` if this type used to represent everything that is not null, UDTs,
  /// arrays, structs, and maps.
  pub fn is_atomic(&self) -> bool {
    match self {
      DataType::BooleanType |
      DataType::ByteType |
      DataType::ShortType |
      DataType::IntegerType |
      DataType::LongType |
      DataType::FloatType |
      DataType::DoubleType |
      DataType::StringType => true,
      _ => false
    }
  }

  /// Returns string representation of schema tree.
  pub fn tree_string(&self) -> String {
    match self {
      DataType::StructType(_) => {
        let mut buf = vec![];
        buf.push("root".to_string());
        self.print_tree(" |", &mut buf);
        buf.join("\n")
      },
      _ => panic!("Not a StructType")
    }
  }

  /// Internal method to print tree.
  fn print_tree(&self, prefix: &str, buf: &mut Vec<String>) {
    match self {
      DataType::StructType(ref fields) => {
        for field in fields {
          field.print_tree(prefix, buf);
        }
      },
      _ => {
        // no-op operation
      }
    }
  }

  /// Internal method to extract short type name.
  fn type_name(&self) -> &str {
    match self {
      DataType::BooleanType => "bool",
      DataType::ByteType => "byte",
      DataType::ShortType => "short",
      DataType::IntegerType => "int",
      DataType::LongType => "long",
      DataType::FloatType => "float",
      DataType::DoubleType => "double",
      DataType::StringType => "string",
      DataType::StructType(_) => "struct"
    }
  }
}

impl fmt::Display for DataType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      DataType::StructType(ref fields) => {
        let num_fields = fields.len();
        write!(f, "struct<")?;
        for (idx, field) in fields.iter().enumerate() {
          write!(f, "{}:{}", field.name(), field.data_type())?;
          if idx < num_fields - 1 {
            write!(f, ",")?;
          }
        }
        write!(f, ">")
      },
      _ => write!(f, "{}", self.type_name())
    }
  }
}

/// A field inside a StructType.
///
/// Contains:
/// - `name`, the name of this field.
/// - `data_type`, the data type of this field.
/// - `nullable`, indicates if values of this type field can be `null` values.
#[derive(Clone, Debug, PartialEq)]
pub struct StructField {
  name: String,
  data_type: DataType,
  nullable: bool,
}

impl StructField {
  /// Creates new struct field with name and type.
  /// Assumes that field is nullable by default.
  pub fn new(name: String, data_type: DataType) -> Self {
    Self {
      name: name,
      data_type: data_type,
      nullable: true
    }
  }

  /// Returns name of this field.
  pub fn name(&self) -> &str {
    &self.name
  }

  /// Returns type of this field.
  pub fn data_type(&self) -> &DataType {
    &self.data_type
  }

  /// Returns `true` if field is nullable, `false` otherwise.
  pub fn is_nullable(&self) -> bool {
    self.nullable
  }

  /// Marks current field as either nullable or non-nullable.
  pub fn with_nullable(mut self, is_nullable: bool) -> Self {
    self.nullable = is_nullable;
    self
  }

  /// Prints tree string.
  fn print_tree(&self, prefix: &str, buf: &mut Vec<String>) {
    buf.push(format!("{}- {}: {} (nullable = {})",
      prefix, self.name, self.data_type.type_name(), self.nullable));
    self.data_type.print_tree(&format!("   {}", prefix), buf);
  }
}

impl fmt::Display for StructField {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "StructField({}, {}, {})", self.name, self.data_type, self.nullable)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_datatype_is_atomic() {
    assert_eq!(DataType::BooleanType.is_atomic(), true);
    assert_eq!(DataType::ByteType.is_atomic(), true);
    assert_eq!(DataType::ShortType.is_atomic(), true);
    assert_eq!(DataType::IntegerType.is_atomic(), true);
    assert_eq!(DataType::LongType.is_atomic(), true);
    assert_eq!(DataType::FloatType.is_atomic(), true);
    assert_eq!(DataType::DoubleType.is_atomic(), true);
    assert_eq!(DataType::StringType.is_atomic(), true);
    assert_eq!(DataType::struct_type(vec![]).is_atomic(), false);
  }

  #[test]
  fn test_datatype_is_struct() {
    assert_eq!(DataType::BooleanType.is_struct(), false);
    assert_eq!(DataType::ByteType.is_struct(), false);
    assert_eq!(DataType::ShortType.is_struct(), false);
    assert_eq!(DataType::IntegerType.is_struct(), false);
    assert_eq!(DataType::LongType.is_struct(), false);
    assert_eq!(DataType::FloatType.is_struct(), false);
    assert_eq!(DataType::DoubleType.is_struct(), false);
    assert_eq!(DataType::StringType.is_struct(), false);
    assert_eq!(DataType::struct_type(vec![]).is_struct(), true);
  }

  #[test]
  fn test_datatype_type_name() {
    assert_eq!(DataType::BooleanType.type_name(), "bool");
    assert_eq!(DataType::ByteType.type_name(), "byte");
    assert_eq!(DataType::ShortType.type_name(), "short");
    assert_eq!(DataType::IntegerType.type_name(), "int");
    assert_eq!(DataType::LongType.type_name(), "long");
    assert_eq!(DataType::FloatType.type_name(), "float");
    assert_eq!(DataType::DoubleType.type_name(), "double");
    assert_eq!(DataType::StringType.type_name(), "string");
    assert_eq!(DataType::struct_type(vec![]).type_name(), "struct");
  }

  #[test]
  fn test_datatype_struct() {
    let schema = DataType::struct_type(vec![])
      .add_field("a", DataType::IntegerType)
      .add_field_n("b", DataType::IntegerType, false)
      .add(StructField::new("c".to_owned(), DataType::IntegerType));

    assert_eq!(
      schema,
      DataType::struct_type(vec![
        StructField::new("a".to_owned(), DataType::IntegerType).with_nullable(true),
        StructField::new("b".to_owned(), DataType::IntegerType).with_nullable(false),
        StructField::new("c".to_owned(), DataType::IntegerType).with_nullable(true)
      ])
    );
  }

  #[test]
  fn test_datatype_struct_num_fields() {
    let schema = DataType::struct_type(vec![]);
    assert_eq!(schema.num_fields(), 0);

    let schema = schema
      .add_field("a", DataType::IntegerType)
      .add_field("b", DataType::StringType);
    assert_eq!(schema.num_fields(), 2);
  }

  #[test]
  fn test_datatype_display() {
    let schema = DataType::struct_type(vec![])
      .add_field("a", DataType::IntegerType)
      .add_field("b", DataType::struct_type(vec![])
        .add_field("c", DataType::StringType)
        .add_field("d", DataType::DoubleType)
      );

    assert_eq!(
      format!("{}", schema),
      "struct<a:int,b:struct<c:string,d:double>>"
    );
  }

  #[test]
  fn test_datatype_tree_string() {
    let schema = DataType::struct_type(vec![])
      .add_field("a", DataType::IntegerType)
      .add_field("b", DataType::DoubleType)
      .add_field("c", DataType::StringType)
      .add_field("d", DataType::struct_type(vec![])
        .add_field("x", DataType::ByteType)
        .add_field("y", DataType::ShortType)
        .add_field_n("z", DataType::BooleanType, false)
      )
      .add_field_n("e", DataType::FloatType, false)
      .add_field_n("f", DataType::LongType, false);

    let expected_tree = vec![
      "root",
      " |- a: int (nullable = true)",
      " |- b: double (nullable = true)",
      " |- c: string (nullable = true)",
      " |- d: struct (nullable = true)",
      "    |- x: byte (nullable = true)",
      "    |- y: short (nullable = true)",
      "    |- z: bool (nullable = false)",
      " |- e: float (nullable = false)",
      " |- f: long (nullable = false)"
    ].join("\n");

    assert_eq!(
      schema.tree_string(),
      expected_tree
    )
  }

  #[test]
  fn test_structfield() {
    let field = StructField::new("field_name".to_owned(), DataType::IntegerType);
    assert_eq!(field.name(), "field_name");
    assert_eq!(field.data_type(), &DataType::IntegerType);
    assert_eq!(field.is_nullable(), true);

    let field = field.with_nullable(false);
    assert_eq!(field.is_nullable(), false);
  }
}
