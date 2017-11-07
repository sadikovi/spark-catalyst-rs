pub trait TreeNode<A: TreeNode<A>> {
    fn label(&self) -> String;

    fn get(&self) -> &A;

    fn num_children(&self) -> usize;

    fn get_child(&self, idx: usize) -> Option<&A>;

    fn is_leaf(&self) -> bool { self.num_children() == 0 }

    // Run the given function recursively on this node and then on children
    fn foreach<F>(&self, func: &mut F) where F: FnMut(&A) {
        func(self.get());
        let mut idx = 0;
        while let Some(child) = self.get_child(idx) {
            child.foreach(func);
            idx += 1;
        }
    }

    // Run the given function recursively on children and then on this node
    fn foreach_up<F>(&self, func: &mut F) where F: FnMut(&A) {
        let mut idx = 0;
        while let Some(child) = self.get_child(idx) {
            child.foreach_up(func);
            idx += 1;
        }
        func(self.get());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // == Test node ==
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
        fn label(&self) -> String { self.label.clone() }

        fn get(&self) -> &TestNode { &self }

        fn num_children(&self) -> usize { self.children.len() }

        fn get_child(&self, idx: usize) -> Option<&TestNode> { self.children.get(idx) }
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
    fn test_testnode_properties() {
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
    fn test_testnode_foreach() {
        let tree = get_small_test_tree();
        let mut labels = Vec::new();
        tree.foreach(&mut |node| {
            labels.push(node.label())
        });
        assert_eq!(labels, vec!["a1", "b1", "c1", "c2", "b2", "c3", "b3"]);
    }

    #[test]
    fn test_testnode_foreach_up() {
        let tree = get_small_test_tree();
        let mut labels = Vec::new();
        tree.foreach_up(&mut |node| {
            labels.push(node.label())
        });
        assert_eq!(labels, vec!["c1", "c2", "b1", "c3", "b2", "b3", "a1"]);
    }
}
