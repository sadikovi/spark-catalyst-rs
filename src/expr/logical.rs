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

//! Logical expressions.

use expr::api::{OutputDataType, ResolveExpression};
use types::DataType;

binary_expression![GreaterThan, ">", "greater than",
  impl OutputDataType for GreaterThan {
    fn output_datatype(&self) -> &DataType {
      &DataType::BooleanType
    }
  },
  impl ResolveExpression for GreaterThan {
    fn resolve(&self) -> bool {
      self.left.resolved() && self.right.resolved() &&
        self.left.data_type() == self.right.data_type()
    }
  }
];

binary_expression![LessThan, "<", "less than",
  impl OutputDataType for LessThan {
    fn output_datatype(&self) -> &DataType {
      &DataType::BooleanType
    }
  },
  impl ResolveExpression for LessThan {
    fn resolve(&self) -> bool {
      self.left.resolved() && self.right.resolved() &&
        self.left.data_type() == self.right.data_type()
    }
  }
];

binary_expression![GreaterThanOrEqual, ">=", "greater than or equal",
  impl OutputDataType for GreaterThanOrEqual {
    fn output_datatype(&self) -> &DataType {
      &DataType::BooleanType
    }
  },
  impl ResolveExpression for GreaterThanOrEqual {
    fn resolve(&self) -> bool {
      self.left.resolved() && self.right.resolved() &&
        self.left.data_type() == self.right.data_type()
    }
  }
];

binary_expression![LessThanOrEqual, "<=", "less than or equal",
  impl OutputDataType for LessThanOrEqual {
    fn output_datatype(&self) -> &DataType {
      &DataType::BooleanType
    }
  },
  impl ResolveExpression for LessThanOrEqual {
    fn resolve(&self) -> bool {
      self.left.resolved() && self.right.resolved() &&
        self.left.data_type() == self.right.data_type()
    }
  }
];

binary_expression![Equals, "==", "equals",
  impl OutputDataType for Equals {
    fn output_datatype(&self) -> &DataType {
      &DataType::BooleanType
    }
  },
  impl ResolveExpression for Equals {
    fn resolve(&self) -> bool {
      self.left.resolved() && self.right.resolved() &&
        self.left.data_type() == self.right.data_type()
    }
  }
];

binary_expression![And, "&&", "and",
  impl OutputDataType for And {
    fn output_datatype(&self) -> &DataType {
      &DataType::BooleanType
    }
  },
  impl ResolveExpression for And {
    fn resolve(&self) -> bool {
      self.left.resolved() && self.left.data_type() == &DataType::BooleanType &&
        self.right.resolved() && self.right.data_type() == &DataType::BooleanType
    }
  }
];

binary_expression![Or, "||", "or",
  impl OutputDataType for Or {
    fn output_datatype(&self) -> &DataType {
      &DataType::BooleanType
    }
  },
  impl ResolveExpression for Or {
    fn resolve(&self) -> bool {
      self.left.resolved() && self.left.data_type() == &DataType::BooleanType &&
        self.right.resolved() && self.right.data_type() == &DataType::BooleanType
    }
  }
];

unary_expression![Not, "!", "not",
  impl OutputDataType for Not {
    fn output_datatype(&self) -> &DataType {
      &DataType::BooleanType
    }
  },
  impl ResolveExpression for Not {
    fn resolve(&self) -> bool {
      self.child.resolved() && self.child.data_type() == &DataType::BooleanType
    }
  }
];
