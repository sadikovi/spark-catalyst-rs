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

//! List of various catalyst errors used in library.

/// Enum lists all errors used in rule execution and analysis.
#[derive(Debug)]
pub enum CatalystError {
  /// `Tree` error is raised when plan is not integral/tree is invalid.
  Tree(String)
}

macro_rules! tree_err {
  ($fmt:expr) => (Err(CatalystError::Tree($fmt.to_owned())));
  ($fmt:expr, $($args:expr), *) => (Err(CatalystError::Tree(format!($fmt, $($args), *))));
}
