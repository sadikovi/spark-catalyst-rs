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

use expr::api::{OutputDataType, ResolveExpression};
use types::DataType;

binary_expression![Add, "+", "add",
  impl OutputDataType for Add {
    fn output_datatype(&self) -> &DataType {
      self.left.data_type()
    }
  },
  impl ResolveExpression for Add {
    fn resolve(&self) -> bool {
      self.left.resolved() && self.right.resolved() &&
        self.left.data_type() == self.right.data_type()
    }
  }
];

binary_expression![Subtract, "-", "subtract",
  impl OutputDataType for Subtract {
    fn output_datatype(&self) -> &DataType {
      self.left.data_type()
    }
  },
  impl ResolveExpression for Subtract {
    fn resolve(&self) -> bool {
      self.left.resolved() && self.right.resolved() &&
        self.left.data_type() == self.right.data_type()
    }
  }
];

binary_expression![Multiply, "*", "multiply",
  impl OutputDataType for Multiply {
    fn output_datatype(&self) -> &DataType {
      self.left.data_type()
    }
  },
  impl ResolveExpression for Multiply {
    fn resolve(&self) -> bool {
      self.left.resolved() && self.right.resolved() &&
        self.left.data_type() == self.right.data_type()
    }
  }
];

binary_expression![Divide, "/", "divide",
  impl OutputDataType for Divide {
    fn output_datatype(&self) -> &DataType {
      self.left.data_type()
    }
  },
  impl ResolveExpression for Divide {
    fn resolve(&self) -> bool {
      self.left.resolved() && self.right.resolved() &&
        self.left.data_type() == self.right.data_type()
    }
  }
];
