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

use expr::api::OutputDataType;
use types::DataType;

binary_expression![GreaterThan, ">", "greater than",
  impl OutputDataType for GreaterThan {
    fn output_datatype(&self) -> &DataType {
      &DataType::BooleanType
    }
  }
];

binary_expression![LessThan, "<", "less than",
  impl OutputDataType for LessThan {
    fn output_datatype(&self) -> &DataType {
      &DataType::BooleanType
    }
  }
];

binary_expression![GreaterThanOrEqual, ">=", "greater than or equal",
  impl OutputDataType for GreaterThanOrEqual {
    fn output_datatype(&self) -> &DataType {
      &DataType::BooleanType
    }
  }
];

binary_expression![LessThanOrEqual, "<=", "less than or equal",
  impl OutputDataType for LessThanOrEqual {
    fn output_datatype(&self) -> &DataType {
      &DataType::BooleanType
    }
  }
];

binary_expression![Equals, "==", "equals",
  impl OutputDataType for Equals {
    fn output_datatype(&self) -> &DataType {
      &DataType::BooleanType
    }
  }
];

binary_expression![And, "&&", "and",
  impl OutputDataType for And {
    fn output_datatype(&self) -> &DataType {
      &DataType::BooleanType
    }
  }
];

binary_expression![Or, "||", "or",
  impl OutputDataType for Or {
    fn output_datatype(&self) -> &DataType {
      &DataType::BooleanType
    }
  }
];

unary_expression![Not, "!", "not",
  impl OutputDataType for Not {
    fn output_datatype(&self) -> &DataType {
      &DataType::BooleanType
    }
  }
];
