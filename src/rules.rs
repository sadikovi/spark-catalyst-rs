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

//! Rule execution and strategy.
//! Also provides batches of rules that can be run once or until a fixed point.

use errors::CatalystError;

/// An execution strategy for rules that indicates the maximum number of executions.
/// If the execution reaches fix point (i.e. converge) before max iterations, it will stop.
pub enum Strategy {
    Once,
    FixedPoint(u16),
}

impl Strategy {
    /// Return number of iterations associated with strategy.
    pub fn num_iterations(&self) -> u16 {
        match *self {
            Strategy::Once => 1,
            Strategy::FixedPoint(iterations) => iterations,
        }
    }
}

/// Execution rule.
pub trait Rule {
    /// Rule name.
    fn name(&self) -> String;

    /// Transform plan A into a new plan according to the rule.
    fn apply<A>(&self, plan: &A) -> A;
}

/// Batch of rules.
/// Contains name, associated strategy and list of rules to run.
pub struct Batch<R: Rule> {
    name: String,
    strategy: Strategy,
    rules: Vec<R>
}

impl<R: Rule> Batch<R> {
    /// Return new batch of rules.
    pub fn new(name: String, strategy: Strategy, rules: Vec<R>) -> Self {
        Self { name: name, strategy: strategy, rules: rules }
    }

    /// Name of this batch.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Strategy for this batch.
    pub fn strategy(&self) -> &Strategy {
        &self.strategy
    }

    /// Number of rules in this batch.
    pub fn num_rules(&self) -> usize {
        self.rules.len()
    }

    /// Get rule reference for the index.
    /// None is returned if index is out of bound.
    pub fn get_rule(&self, idx: usize) -> Option<&R> {
        self.rules.get(idx)
    }
}

/// Abstract rule executor for batches of rules.
pub trait RuleExecutor<A: Clone + PartialEq, R: Rule> {
    /// Sequence of rule batches.
    fn batches() -> Vec<Batch<R>>;

    /// Defines a check function that checks for structural integrity of the plan after the
    /// execution of each rule. For example, we can check whether a plan is still resolved after
    /// each rule in optimizer, so we can catch rules that return invalid plans. The check function
    /// returns `false` if the given plan doesn't pass the structural integrity check.
    fn is_plan_integral(plan: &A) -> bool;

    /// Executes the batches of rules defined by the subclass. The batches are executed serially
    /// using the defined execution strategy. Within each batch, rules are also executed serially.
    fn execute(plan: &A) -> Result<A, CatalystError> {
        // current plan for update
        let mut current_plan = plan.clone();

        for batch in Self::batches() {
            let mut iteration = 1;
            let mut do_continue = true;
            // initial batch plan
            let batch_start_plan = current_plan.clone();
            // last plan after applying batch
            let mut last_plan = current_plan.clone();

            while do_continue {
                for i in 0..batch.num_rules() {
                    let rule = batch.get_rule(i).unwrap();
                    current_plan = rule.apply(&current_plan);

                    if !Self::is_plan_integral(&current_plan) {
                        return tree_err!("After applying rule {} in batch {}, the structural integrity of
                            the plan is broken", rule.name(), batch.name());
                    }
                }
                iteration += 1;
                if iteration > batch.strategy.num_iterations() {
                    if batch.strategy.num_iterations() > 1 {
                        debug!("Max iterations {} reached for batch {}",
                            iteration - 1, batch.name());
                    }
                    // for rules that run more than once
                    do_continue = false;
                }

                if current_plan == last_plan {
                    // if current plan does not change anymore for fixed point
                    debug!("Fixed point reached for batch {} after {} iterations",
                        batch.name(), iteration - 1);
                    do_continue = false;
                } else {
                    last_plan = current_plan.clone();
                }
            }

            if batch_start_plan != current_plan {
                // TODO: improve log message to show plan difference
                debug!("Batch {} updated current plan", batch.name());
            } else {
                debug!("Batch {} has no effect", batch.name());
            }
        }
        Ok(current_plan)
    }
}
