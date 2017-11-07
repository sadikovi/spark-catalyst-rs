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

use std::fmt::Display;

pub trait TreeNode<A: TreeNode<A> + Clone + Display + PartialEq> {
    // Get String label for the this node
    fn label(&self) -> String;

    // Return underlying instance A.
    fn get(&self) -> &A;

    // Number of children for this node.
    fn num_children(&self) -> usize;

    // Get child for a specified index.
    // If index is out of bound, return None, this should be in sync with `num_children()`.
    fn get_child(&self, idx: usize) -> Option<&A>;

    // Set new child at a specified index.
    // If index is out of bound, this should be no-op.
    fn set_child(&mut self, idx: usize, child: A);

    // Whether or not this node is a leaf node.
    fn is_leaf(&self) -> bool { self.num_children() == 0 }

    // Find first node that matches predicate function.
    // If no such node is found return None.
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

    // Run the given function recursively on this node and then on children.
    fn foreach<F>(&self, func: &mut F) where F: FnMut(&A) {
        func(self.get());
        let mut idx = 0;
        while let Some(child) = self.get_child(idx) {
            child.foreach(func);
            idx += 1;
        }
    }

    // Run the given function recursively on children and then on this node.
    fn foreach_up<F>(&self, func: &mut F) where F: FnMut(&A) {
        let mut idx = 0;
        while let Some(child) = self.get_child(idx) {
            child.foreach_up(func);
            idx += 1;
        }
        func(self.get());
    }

    // Internal method to recursively apply map for all nodes.
    fn internal_map<F, R>(&self, func: &mut F, res: &mut Vec<R>) where F: FnMut(&A) -> R {
        self.foreach(&mut |node| { res.push(func(node)) });
    }

    // Return vector of R instances by applying function to all nodes in pre-order traversal.
    fn map<F, R>(&self, func: &mut F) -> Vec<R> where F: FnMut(&A) -> R {
        let mut res = Vec::new();
        self.internal_map(func, &mut res);
        res
    }

    // Internal method to recursively apply flat_map for all nodes
    fn internal_flat_map<F, R>(&self, func: &mut F, res: &mut Vec<R>) where F: FnMut(&A) -> Vec<R> {
        self.foreach(&mut |node| { res.append(&mut func(node)) });
    }

    // Return vector of R instances by applying function to all nodes in pre-order traversal and
    // collect all returned sequences into resulting vector.
    fn flat_map<F, R>(&self, func: &mut F) -> Vec<R> where F: FnMut(&A) -> Vec<R> {
        let mut res = Vec::new();
        self.internal_flat_map(func, &mut res);
        res
    }

    // Returns vector containing the result of applying a partial function to all elements in this
    // tree on which the function is defined (returns Some(R)).
    fn collect<F, R>(&self, partial_func: &mut F) -> Vec<R> where F: FnMut(&A) -> Option<R> {
        let mut res = Vec::new();
        self.foreach(&mut |node| {
            if let Some(result) = partial_func(node) {
                res.push(result);
            }
        });
        res
    }

    // Return vector containing copies of all leaves in this tree.
    fn collect_leaves(&self) -> Vec<A> {
        self.collect(&mut |node| if node.is_leaf() { Some(node.clone()) } else { None } )
    }

    // Return copy of this node with modified children by applying `func` to all immediate children
    // of this node.
    fn map_children<F>(&self, func: &mut F) -> A where F: FnMut(&A) -> A {
        let mut cloned_node = self.get().clone();
        let mut idx = 0;
        while let Some(child) = self.get_child(idx) {
            cloned_node.set_child(idx, func(child));
            idx += 1;
        }
        cloned_node
    }

    // Returns a copy of this node where `rule` has been recursively applied to it and all of its
    // children (pre-order). When `rule` does not apply to a given node it is left unchanged.
    fn transform_down<F>(&self, rule: &mut F) -> A where F: FnMut(&A) -> Option<A> {
        match rule(&self.get()) {
            Some(after_rule) => after_rule.map_children(&mut |node| node.transform_down(rule)),
            None => self.map_children(&mut |node| node.transform_down(rule)),
        }
    }

    // Return a copy of this node where `rule` has been recursively applied first to all of its
    // children and then itself (post-order). When `rule` does not apply to a given node, it is left
    // unchanged.
    fn transform_up<F>(&self, rule: &mut F) -> A where F: FnMut(&A) -> Option<A> {
        let updated_node = self.map_children(&mut |node| node.transform_up(rule));
        match rule(&updated_node) {
            Some(after_rule) => after_rule,
            None => updated_node,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::{Error, Formatter};

    // == Test node ==
    #[derive(Clone,Debug)]
    struct TestNode {
        label: String,
        children: Vec<TestNode>
    }

    impl TestNode {
        fn new(label: String, children: Vec<TestNode>) -> Self {
            Self { label: label, children: children }
        }
    }

    impl Display for TestNode {
        fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
            write!(f, "({})", self.label)
        }
    }

    impl PartialEq for TestNode {
        fn eq(&self, other: &TestNode) -> bool {
            self.label == other.label && self.children == other.children
        }
    }

    impl TreeNode<TestNode> for TestNode {
        fn label(&self) -> String { self.label.clone() }

        fn get(&self) -> &TestNode { &self }

        fn num_children(&self) -> usize { self.children.len() }

        fn get_child(&self, idx: usize) -> Option<&TestNode> { self.children.get(idx) }

        fn set_child(&mut self, idx: usize, child: TestNode) { self.children[idx] = child; }
    }

    // Get small generic tree for testing
    fn get_small_test_tree() -> TestNode {
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

    #[test]
    fn test_properties() {
        let tree = get_small_test_tree();
        assert_eq!(tree.label(), "a1");
        assert_eq!(tree.num_children(), 3);
        assert_eq!(tree.is_leaf(), false);

        assert_eq!(tree.get_child(0).unwrap().label(), "b1");
        assert_eq!(tree.get_child(0).unwrap().num_children(), 2);
        assert_eq!(tree.get_child(0).unwrap().is_leaf(), false);

        assert_eq!(tree.get_child(1).unwrap().label(), "b2");
        assert_eq!(tree.get_child(1).unwrap().num_children(), 1);
        assert_eq!(tree.get_child(1).unwrap().is_leaf(), false);

        assert_eq!(tree.get_child(2).unwrap().label(), "b3");
        assert_eq!(tree.get_child(2).unwrap().num_children(), 0);
        assert_eq!(tree.get_child(2).unwrap().is_leaf(), true);
    }

    #[test]
    fn test_foreach() {
        let tree = get_small_test_tree();
        let mut labels = Vec::new();
        tree.foreach(&mut |node| {
            labels.push(node.label())
        });
        assert_eq!(labels, vec!["a1", "b1", "c1", "c2", "b2", "c3", "b3"]);
    }

    #[test]
    fn test_foreach_up() {
        let tree = get_small_test_tree();
        let mut labels = Vec::new();
        tree.foreach_up(&mut |node| {
            labels.push(node.label())
        });
        assert_eq!(labels, vec!["c1", "c2", "b1", "c3", "b2", "b3", "a1"]);
    }

    #[test]
    fn test_find() {
        let tree = get_small_test_tree();
        // child node in the tree
        let res = tree.find(&mut |node| node.label() == "c2");
        assert!(res.is_some());
        assert_eq!(res.unwrap().label(), "c2");

        // root of the tree
        let res = tree.find(&mut |node| node.num_children() == 3);
        assert!(res.is_some());
        assert_eq!(res.unwrap().label(), "a1");

        // no result
        let res = tree.find(&mut |node| node.label() == "<unknown>");
        assert!(res.is_none());
    }

    #[test]
    fn test_map() {
        let tree = get_small_test_tree();
        let res = tree.map(&mut |node| { node.label() });
        assert_eq!(res, vec!["a1", "b1", "c1", "c2", "b2", "c3", "b3"]);

        // map to a list of boolean leaf/non-leaf
        let res = tree.map(&mut |node| { node.is_leaf() });
        assert_eq!(res, vec![false, false, true, true, false, true, true]);
    }

    #[test]
    fn test_flat_map() {
        let tree = get_small_test_tree();
        let res = tree.flat_map(&mut |node| {
            let mut vec = Vec::new();
            for i in 0..node.num_children() {
                vec.push(node.get_child(i).unwrap().label());
            }
            vec
        });
        assert_eq!(res, vec!["b1", "b2", "b3", "c1", "c2", "c3"]);
    }

    #[test]
    fn test_collect() {
        let tree = get_small_test_tree();
        let res = tree.collect(&mut |node| {
            if !node.is_leaf() { Some (node.label()) } else { None }
        });
        assert_eq!(res, vec!["a1", "b1", "b2"]);
    }

    #[test]
    fn test_collect_leaves() {
        let tree = get_small_test_tree();
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
        let tree = get_small_test_tree();
        let res = tree.map_children(&mut |node| {
            TestNode::new(format!("{}-#", node.label()), node.children.clone())
        });
        let mut labels = Vec::new();
        res.foreach(&mut |node| labels.push(node.label()));

        assert_eq!(labels, ["a1", "b1-#", "c1", "c2", "b2-#", "c3", "b3-#"]);
    }

    #[test]
    fn test_transform_down() {
        let tree = get_small_test_tree();
        let res = tree.transform_down(&mut |node| {
            if node.label() == "b1" || node.label() == "b2" {
                Some(TestNode::new(format!("{}-#", node.label()), vec![]))
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
        assert_eq!(tree, get_small_test_tree());
    }

    #[test]
    fn test_transform_up() {
        let tree = get_small_test_tree();
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
        assert_eq!(tree, get_small_test_tree());
    }
}
