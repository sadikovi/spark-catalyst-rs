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

//! Collection of generic methods and traits for working with trees.
//!
//! A library for easily manipulating trees of operators. Operators that extend
//! `TreeNode` are granted the following interface/methods:
//!
//! - Scala collection like methods: `foreach`, `map`, `flat_map`, `collect`, etc.
//! - `transform_up`/`transform_down` - accepts a partial function (one that returns
//!   option) that is used to generate a new tree. When the partial function can be
//!   applied to a given tree segment, that segment is replaced with the result,
//!   otherwise old segment is retained. Depending on "up" or "down" order, partial
//!   function is recursively applied to current node and all children of current node,
//!   or vice versa.
//! - debugging support - pretty printing, tree structure display, etc.

/// Generic `TreeNode` to provide traversal and transform.
pub trait TreeNode<A: TreeNode<A>> {
  /// Returns string label for this node.
  fn node_name(&self) -> String;

  /// One-line description of the node.
  fn verbose_string(&self) -> String;

  /// Returns underlying instance `A`.
  fn get(&self) -> &A;

  /// Number of children for this node.
  fn num_children(&self) -> usize;

  /// Returns child for a specified index.
  ///
  /// If index is out of bound, return `None`, should be in sync with `num_children`.
  fn get_child(&self, idx: usize) -> Option<&A>;

  /// Sets new child at a specified index.
  ///
  /// If index is out of bound, this should be no-op.
  fn set_child(&mut self, idx: usize, child: A);

  /// Clones current tree node recursively.
  ///
  /// Should be similar to `Clone` trait functionality.
  fn clone_tree(&self) -> A;

  /// Equality of two trees, compares recursively.
  ///
  /// Returns `true` if both full trees are equal, otherwise `false`.
  /// Should be similar to `PartialEq` trait functionality.
  fn equals(&self, other: &A) -> bool;

  /// Returns `true` if this node is a leaf node, `false` otherwise.
  fn is_leaf(&self) -> bool { self.num_children() == 0 }

  /// Finds first node that matches predicate function.
  ///
  /// If no such node is found returns `None`.
  fn find<F>(&self, func: &mut F) -> Option<&A> where F: FnMut(&A) -> bool {
    if func(self.get()) {
      return Some(self.get());
    }
    let mut idx = 0;
    while let Some(child) = self.get_child(idx) {
      match child.find(func) {
        res @ Some(_) => return res,
        None => { }, // no-op, continue searching
      }
      idx += 1;
    }
    None
  }

  /// Runs the given function recursively on this node and then on children.
  fn foreach<F>(&self, func: &mut F) where F: FnMut(&A) {
    func(self.get());
    let mut idx = 0;
    while let Some(child) = self.get_child(idx) {
      child.foreach(func);
      idx += 1;
    }
  }

  /// Runs the given function recursively on children and then on this node.
  fn foreach_up<F>(&self, func: &mut F) where F: FnMut(&A) {
    let mut idx = 0;
    while let Some(child) = self.get_child(idx) {
      child.foreach_up(func);
      idx += 1;
    }
    func(self.get());
  }

  /// Internal method to recursively apply map for all nodes.
  fn internal_map<F, R>(&self, func: &mut F, res: &mut Vec<R>) where F: FnMut(&A) -> R {
    self.foreach(&mut |node| { res.push(func(node)) });
  }

  /// Returns vector of `R` instances by applying function to all nodes
  /// in pre-order traversal.
  fn map<F, R>(&self, func: &mut F) -> Vec<R> where F: FnMut(&A) -> R {
    let mut res = Vec::new();
    self.internal_map(func, &mut res);
    res
  }

  /// Internal method to recursively apply `flat_map` for all nodes.
  fn internal_flat_map<F, R>(
    &self,
    func: &mut F,
    res: &mut Vec<R>
  ) where F: FnMut(&A) -> Vec<R>
  {
    self.foreach(&mut |node| { res.append(&mut func(node)) });
  }

  /// Returns vector of `R` instances by applying function to all nodes in pre-order
  /// traversal and collect all returned sequences into a resulting vector.
  fn flat_map<F, R>(&self, func: &mut F) -> Vec<R> where F: FnMut(&A) -> Vec<R> {
    let mut res = Vec::new();
    self.internal_flat_map(func, &mut res);
    res
  }

  /// Returns vector containing the result of applying a partial function to all
  /// elements in this tree on which the function is defined (returns `Some(R)`).
  fn collect<F, R>(
    &self,
    partial_func: &mut F
  ) -> Vec<R> where F: FnMut(&A) -> Option<R>
  {
    let mut res = Vec::new();
    self.foreach(&mut |node| {
      if let Some(result) = partial_func(node) {
        res.push(result);
      }
    });
    res
  }

  /// Return vector containing copies of all leaves in this tree.
  fn collect_leaves(&self) -> Vec<A> {
    self.collect(&mut |node| if node.is_leaf() { Some(node.clone_tree()) } else { None } )
  }

  /// Return copy of this node with modified children by applying `func` to all
  /// immediate children of this node.
  fn map_children<F>(&self, func: &mut F) -> A where F: FnMut(&A) -> A {
    let mut cloned_node = self.get().clone_tree();
    let mut idx = 0;
    while let Some(child) = self.get_child(idx) {
      cloned_node.set_child(idx, func(child));
      idx += 1;
    }
    cloned_node
  }

  /// Returns a copy of this node where `rule` has been recursively applied to it and
  /// all of its children (pre-order). When `rule` does not apply to a given node it
  /// is left unchanged.
  fn transform_down<F>(&self, rule: &mut F) -> A where F: FnMut(&A) -> Option<A> {
    match rule(&self.get()) {
      Some(after_rule) => after_rule.map_children(&mut |node| node.transform_down(rule)),
      None => self.map_children(&mut |node| node.transform_down(rule)),
    }
  }

  /// Return a copy of this node where `rule` has been recursively applied first to all
  /// of its children and then itself (post-order). When `rule` does not apply to a given
  /// node, it is left unchanged.
  fn transform_up<F>(&self, rule: &mut F) -> A where F: FnMut(&A) -> Option<A> {
    let updated_node = self.map_children(&mut |node| node.transform_up(rule));
    match rule(&updated_node) {
      Some(after_rule) => after_rule,
      None => updated_node,
    }
  }

  /// Internal method to generate tree string.
  fn recur_gen_tree(
    &self,
    depth: usize,
    prefix: &str,
    is_last_child: bool,
    buffer: &mut Vec<String>
  )
  {
    let parent_prefix =
      if depth == 0 { "" } else if is_last_child { "+- " } else { "- " };
    // generate prefix for current node
    let curr = format!("{}{}{}", prefix, parent_prefix, self.node_name());
    buffer.push(curr);
    // add child levels
    let depth = depth + 1; // update to child depth
    let mut idx = 0;
    while let Some(child) = self.get_child(idx) {
      let is_last_child = idx == self.num_children() - 1;
      // when node is last child, we separately attach '+' when constructing prefix
      let node_sym = if is_last_child { "" } else { ":" };
      let prefix = format!("{}{}{}", prefix, " ".repeat(parent_prefix.len()), node_sym);
      child.recur_gen_tree(depth, &prefix, is_last_child, buffer);
      idx += 1;
    }
  }

  /// Internal method that returns generated lines of tree string.
  fn internal_tree_lines(&self) -> Vec<String> {
    let mut buffer = Vec::new();
    self.recur_gen_tree(0, "", false, &mut buffer);
    buffer
  }

  /// Return a string representation of the nodes in this tree.
  ///
  /// Tree is traversed in depth-first order, with appropriate offsets for each child
  /// level.
  fn tree_string(&self) -> String {
    self.internal_tree_lines().join("\n")
  }

  /// Return a string representation of the nodes in this tree, where each operator is
  /// numbered.
  ///
  /// The numbers are based on depth-first traversal of the tree with inner children
  /// traversed first before children.
  fn numbered_tree_string(&self) -> String {
    let mut buffer = Vec::new();
    for (i, line) in self.internal_tree_lines().iter().enumerate() {
      buffer.push(format!("{:0width$} {}", i + 1, line, width=2));
    }
    buffer.join("\n")
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  // == Test node ==
  #[derive(Clone, Debug, PartialEq)]
  struct TestNode {
    label: String,
    children: Vec<TestNode>
  }

  impl TestNode {
    fn new(label: String, children: Vec<TestNode>) -> Self {
      Self { label: label, children: children }
    }
  }

  impl TreeNode<TestNode> for TestNode {
    fn node_name(&self) -> String { format!("{}", self.label) }

    fn verbose_string(&self) -> String { format!("({})", self.label) }

    fn get(&self) -> &TestNode { &self }

    fn num_children(&self) -> usize { self.children.len() }

    fn get_child(&self, idx: usize) -> Option<&TestNode> { self.children.get(idx) }

    fn set_child(&mut self, idx: usize, child: TestNode) { self.children[idx] = child; }

    fn clone_tree(&self) -> TestNode { self.clone() }

    fn equals(&self, other: &TestNode) -> bool { self.eq(other) }
  }

  // Returns small generic tree for testing.
  fn get_small_test_tree_1() -> TestNode {
    TestNode::new(String::from("a1"), vec![
      TestNode::new(String::from("b1"), vec![
        TestNode::new(String::from("c1"), vec![]),
        TestNode::new(String::from("c2"), vec![])
      ]),
      TestNode::new(String::from("b2"), vec![
        TestNode::new(String::from("c3"), vec![])
      ]),
      TestNode::new(String::from("b3"), vec![])
    ])
  }

  // Returns small generic tree with each node having only one child.
  fn get_small_test_tree_2() -> TestNode {
    TestNode::new(String::from("a"), vec![
      TestNode::new(String::from("b"), vec![
        TestNode::new(String::from("c"), vec![
          TestNode::new(String::from("d"), vec![])
        ])
      ])
    ])
  }

  #[test]
  fn test_properties() {
    let tree = get_small_test_tree_1();
    assert_eq!(tree.node_name(), "a1");
    assert_eq!(tree.verbose_string(), "(a1)");
    assert_eq!(tree.num_children(), 3);
    assert_eq!(tree.is_leaf(), false);

    assert_eq!(tree.get_child(0).unwrap().node_name(), "b1");
    assert_eq!(tree.get_child(0).unwrap().verbose_string(), "(b1)");
    assert_eq!(tree.get_child(0).unwrap().num_children(), 2);
    assert_eq!(tree.get_child(0).unwrap().is_leaf(), false);

    assert_eq!(tree.get_child(1).unwrap().node_name(), "b2");
    assert_eq!(tree.get_child(1).unwrap().verbose_string(), "(b2)");
    assert_eq!(tree.get_child(1).unwrap().num_children(), 1);
    assert_eq!(tree.get_child(1).unwrap().is_leaf(), false);

    assert_eq!(tree.get_child(2).unwrap().node_name(), "b3");
    assert_eq!(tree.get_child(2).unwrap().verbose_string(), "(b3)");
    assert_eq!(tree.get_child(2).unwrap().num_children(), 0);
    assert_eq!(tree.get_child(2).unwrap().is_leaf(), true);
  }

  #[test]
  fn test_clone_equals() {
    let tree = get_small_test_tree_1();
    assert_eq!(tree.clone_tree(), tree);
    assert!(tree.equals(&tree.clone()));

    // should not match
    let tree1 = get_small_test_tree_1();
    let tree2 = get_small_test_tree_2();
    assert!(!tree1.equals(&tree2));
    assert!(!tree1.clone_tree().equals(&tree2.clone_tree()));
  }

  #[test]
  fn test_foreach() {
    let tree = get_small_test_tree_1();
    let mut labels = Vec::new();
    tree.foreach(&mut |node| {
      labels.push(node.node_name())
    });
    assert_eq!(labels, vec!["a1", "b1", "c1", "c2", "b2", "c3", "b3"]);
  }

  #[test]
  fn test_foreach_up() {
    let tree = get_small_test_tree_1();
    let mut labels = Vec::new();
    tree.foreach_up(&mut |node| {
      labels.push(node.node_name())
    });
    assert_eq!(labels, vec!["c1", "c2", "b1", "c3", "b2", "b3", "a1"]);
  }

  #[test]
  fn test_find() {
    let tree = get_small_test_tree_1();
    // child node in the tree
    let res = tree.find(&mut |node| node.node_name() == "c2");
    assert!(res.is_some());
    assert_eq!(res.unwrap().node_name(), "c2");

    // root of the tree
    let res = tree.find(&mut |node| node.num_children() == 3);
    assert!(res.is_some());
    assert_eq!(res.unwrap().node_name(), "a1");

    // no result
    let res = tree.find(&mut |node| node.node_name() == "<unknown>");
    assert!(res.is_none());
  }

  #[test]
  fn test_map() {
    let tree = get_small_test_tree_1();
    let res = tree.map(&mut |node| { node.node_name() });
    assert_eq!(res, vec!["a1", "b1", "c1", "c2", "b2", "c3", "b3"]);

    // map to a list of boolean leaf/non-leaf
    let res = tree.map(&mut |node| { node.is_leaf() });
    assert_eq!(res, vec![false, false, true, true, false, true, true]);
  }

  #[test]
  fn test_flat_map() {
    let tree = get_small_test_tree_1();
    let res = tree.flat_map(&mut |node| {
      let mut vec = Vec::new();
      for i in 0..node.num_children() {
        vec.push(node.get_child(i).unwrap().node_name());
      }
      vec
    });
    assert_eq!(res, vec!["b1", "b2", "b3", "c1", "c2", "c3"]);
  }

  #[test]
  fn test_collect() {
    let tree = get_small_test_tree_1();
    let res = tree.collect(&mut |node| {
      if !node.is_leaf() { Some (node.node_name()) } else { None }
    });
    assert_eq!(res, vec!["a1", "b1", "b2"]);
  }

  #[test]
  fn test_collect_leaves() {
    let tree = get_small_test_tree_1();
    let res = tree.collect_leaves();
    let expected = vec![
      TestNode::new(String::from("c1"), vec![]),
      TestNode::new(String::from("c2"), vec![]),
      TestNode::new(String::from("c3"), vec![]),
      TestNode::new(String::from("b3"), vec![])
    ];
    assert_eq!(res, expected);
  }

  #[test]
  fn test_map_children() {
    let tree = get_small_test_tree_1();
    let res = tree.map_children(&mut |node| {
      TestNode::new(format!("{}-#", node.node_name()), node.children.clone())
    });
    let mut labels = Vec::new();
    res.foreach(&mut |node| labels.push(node.node_name()));

    assert_eq!(labels, ["a1", "b1-#", "c1", "c2", "b2-#", "c3", "b3-#"]);
  }

  #[test]
  fn test_transform_down() {
    let tree = get_small_test_tree_1();
    let res = tree.transform_down(&mut |node| {
      if node.node_name() == "b1" || node.node_name() == "b2" {
        Some(TestNode::new(format!("{}-#", node.node_name()), vec![]))
      } else {
        None
      }
    });
    let expected = TestNode::new(String::from("a1"), vec![
      TestNode::new(String::from("b1-#"), vec![]),
      TestNode::new(String::from("b2-#"), vec![]),
      TestNode::new(String::from("b3"), vec![])
    ]);
    assert_eq!(res, expected);
    // should not modify original tree
    assert_eq!(tree, get_small_test_tree_1());
  }

  #[test]
  fn test_transform_up() {
    let tree = get_small_test_tree_1();
    let res = tree.transform_up(&mut |node| {
      let mut cloned = node.clone();
      while cloned.children.len() > 1 {
        cloned.children.pop();
      }
      Some(cloned)
    });
    let expected = TestNode::new(String::from("a1"), vec![
      TestNode::new(String::from("b1"), vec![
        TestNode::new(String::from("c1"), vec![])
      ])
    ]);
    assert_eq!(res, expected);
    // should not modify original tree
    assert_eq!(tree, get_small_test_tree_1());
  }

  #[test]
  fn test_tree_string() {
    let tree = get_small_test_tree_1();
    let res = tree.tree_string();
    assert_eq!(res, vec![
      "a1",
      ":- b1",
      ":  :- c1",
      ":  +- c2",
      ":- b2",
      ":  +- c3",
      "+- b3"
    ].join("\n"));

    let tree = get_small_test_tree_2();
    let res = tree.tree_string();
    assert_eq!(res, vec![
      "a",
      "+- b",
      "   +- c",
      "      +- d"
    ].join("\n"));
  }

  #[test]
  fn test_numbered_tree_string() {
    let tree = get_small_test_tree_1();
    let res = tree.numbered_tree_string();
    assert_eq!(res, vec![
      "01 a1",
      "02 :- b1",
      "03 :  :- c1",
      "04 :  +- c2",
      "05 :- b2",
      "06 :  +- c3",
      "07 +- b3"
    ].join("\n"));

    let tree = get_small_test_tree_2();
    let res = tree.numbered_tree_string();
    assert_eq!(res, vec![
      "01 a",
      "02 +- b",
      "03    +- c",
      "04       +- d"
    ].join("\n"));
  }
}
