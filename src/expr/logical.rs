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

binary_expression![GreaterThan, ">", "greater than"];
binary_expression![LessThan, "<", "less than"];
binary_expression![GreaterThanOrEqual, ">=", "greater than or equal"];
binary_expression![LessThanOrEqual, "<=", "less than or equal"];
binary_expression![Equals, "==", "equals"];

binary_expression![And, "&&", "and"];
binary_expression![Or, "||", "or"];

unary_expression![Not, "!", "not"];
