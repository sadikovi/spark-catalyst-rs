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

use expr::api::{Expression, binary};

/// Adds left and right expressions.
pub fn add(left: Expression, right: Expression) -> Expression {
  binary("ADD".to_owned(), "+".to_owned(), left, right)
    .clone(Box::new(|exp| {
      add(exp.children()[0].clone(), exp.children()[1].clone())
    }))
    .build()
}

/// Subtracts right expression from left expression.
pub fn sub(left: Expression, right: Expression) -> Expression {
  binary("SUB".to_owned(), "-".to_owned(), left, right)
    .clone(Box::new(|exp| {
      sub(exp.children()[0].clone(), exp.children()[1].clone())
    }))
    .build()
}
