/// Binary Tree node
#[derive(Debug)]
struct BTNode {
    data: i32,
    left: Option<Box<BTNode>>,
    right: Option<Box<BTNode>>,
}

macro_rules! node {
    () => {
        None
    };
    ($value: expr) => {
        Some(Box::new(BTNode {
            data: $value,
            left: None,
            right: None,
        }))
    };
    ($value: expr, $left: expr, $right: expr) => {
        Some(Box::new(BTNode {
            data: $value,
            left: $left,
            right: $right,
        }))
    };
}

impl BTNode {
    fn inorder_traverse(&self) {
        if self.left.is_some() {
            self.left.as_ref().unwrap().preorder_traverse();
        }
        println!("Data: {}", self.data);
        if self.right.is_some() {
            self.right.as_ref().unwrap().preorder_traverse();
        }
    }

    fn preorder_traverse(&self) {
        println!("Data: {}", self.data);
        if self.left.is_some() {
            self.left.as_ref().unwrap().preorder_traverse();
        }
        if self.right.is_some() {
            self.right.as_ref().unwrap().preorder_traverse();
        }
    }

    fn postorder_traverse(&self) {
        if self.left.is_some() {
            self.left.as_ref().unwrap().preorder_traverse();
        }
        if self.right.is_some() {
            self.right.as_ref().unwrap().preorder_traverse();
        }
        println!("Data: {}", self.data);
    }

    // 1. Full Binary Tree
    fn is_full_binary_tree(&self) -> bool {
        let is_left_full = if self.left.is_some() {
            self.left.as_ref().unwrap().is_full_binary_tree()
        } else {
            true
        };
        let is_right_full = if self.right.is_some() {
            self.right.as_ref().unwrap().is_full_binary_tree()
        } else {
            true
        };
        let is_this_full = if (self.left.is_some() && self.right.is_none())
            || (self.left.is_none() && self.right.is_some())
        {
            false
        } else {
            true
        };

        is_left_full && is_right_full && is_this_full
    }

    // 2. Perfect Binary Tree
    fn is_perfect_binary_tree(&self) -> bool {
        // 1. Check if all the leaf nodes are in same level
        let is_all_leaves_same_level = {
            let mut leaf_heights = vec![];
            self.get_leaf_heights(0, &mut leaf_heights);
            leaf_heights.dedup();

            leaf_heights.len() == 1
        };

        // 2. Check if all internal nodes have 2 children
        let is_all_internal_2_children = {
            fn check_all_internal_node(node: &BTNode) -> bool {
                if node.left.is_none() && node.right.is_none() {
                    return true;
                }

                if (node.left.is_none() && node.right.is_some())
                    || (node.left.is_some() && node.right.is_none())
                {
                    return false;
                }

                let left_check = if node.left.is_some() {
                    check_all_internal_node(node.left.as_ref().unwrap())
                } else {
                    true
                };
                let right_check = if node.right.is_some() {
                    check_all_internal_node(node.right.as_ref().unwrap())
                } else {
                    true
                };

                left_check && right_check
            }

            check_all_internal_node(&self)
        };

        is_all_leaves_same_level && is_all_internal_2_children
    }

    fn get_leaf_heights(&self, prev_height: usize, heights: &mut Vec<usize>) {
        let curr_height = prev_height + 1;

        if self.left.is_none() && self.right.is_none() {
            heights.push(curr_height);
        } else {
            if self.left.is_some() {
                self.left
                    .as_ref()
                    .unwrap()
                    .get_leaf_heights(curr_height, heights);
            }
            if self.right.is_some() {
                self.right
                    .as_ref()
                    .unwrap()
                    .get_leaf_heights(curr_height, heights);
            }
        }
    }

    // 3. Complete binary tree
    fn is_complete_binary_tree(&self) -> bool {
        let nodes = self.get_nodes_count();

        fn check_complete_tree(node: &BTNode, index: usize, nodes: usize) -> bool {
            if index >= nodes {
                return false;
            }

            if node.left.is_none() && node.right.is_none() {
                return true;
            }

            let is_left_complete = if node.left.is_some() {
                check_complete_tree(node.left.as_ref().unwrap(), 2 * index + 1, nodes)
            } else {
                true
            };
            let is_right_complete = if node.right.is_some() {
                check_complete_tree(node.right.as_ref().unwrap(), 2 * index + 2, nodes)
            } else {
                true
            };
            is_left_complete && is_right_complete
        }

        check_complete_tree(&self, 0, nodes)
    }

    fn get_nodes_count(&self) -> usize {
        if self.left.is_none() && self.right.is_none() {
            return 1;
        }

        let left_tree_count = if self.left.is_some() {
            self.left.as_ref().unwrap().get_nodes_count()
        } else {
            0
        };
        let right_tree_count = if self.right.is_some() {
            self.right.as_ref().unwrap().get_nodes_count()
        } else {
            0
        };

        1 + left_tree_count + right_tree_count
    }

    // 4. Degenerate or pathological tree
    fn is_degenerate(&self) -> bool {
        let is_left_degen = if self.left.is_some() {
            self.left.as_ref().unwrap().is_degenerate()
        } else {
            true
        };
        let is_right_degen = if self.right.is_some() {
            self.right.as_ref().unwrap().is_degenerate()
        } else {
            true
        };
        let is_this_degen = if (self.left.is_some() && self.right.is_none())
            || (self.left.is_none() && self.right.is_some())
            || (self.left.is_none() && self.right.is_none())
        {
            true
        } else {
            false
        };

        is_left_degen && is_right_degen && is_this_degen
    }

    // 5. Skewed binary tree
    fn is_skewed_binary_tree(&self) -> Option<String> {
        if !self.is_degenerate() {
            return None;
        }

        fn skew_check(node: &BTNode, skewness: String) -> bool {
            if node.left.is_none() && node.right.is_none() {
                return true;
            }

            match skewness.as_str() {
                "l" => {
                    if node.left.is_none() {
                        return false;
                    } else {
                        skew_check(node.left.as_ref().unwrap(), skewness)
                    }
                }
                "r" => {
                    if node.right.is_none() {
                        return false;
                    } else {
                        skew_check(node.right.as_ref().unwrap(), skewness)
                    }
                }
                _ => unreachable!(),
            }
        }

        let res = if self.left.is_some() {
            if skew_check(self.left.as_ref().unwrap(), "l".to_string()) {
                Some("left".to_string())
            } else {
                None
            }
        } else if self.right.is_some() {
            if skew_check(self.right.as_ref().unwrap(), "r".to_string()) {
                Some("right".to_string())
            } else {
                None
            }
        } else {
            None
        };

        res
    }

    // 6. Balanced binary tree
    fn is_balanced_binary_tree(&self) -> bool {
        if self.left.is_none() && self.right.is_none() {
            return true;
        }

        let left_height = if self.left.is_some() {
            self.left.as_ref().unwrap().get_height()
        } else {
            0
        };
        let right_height = if self.right.is_some() {
            self.right.as_ref().unwrap().get_height()
        } else {
            0
        };
        if left_height.abs_diff(right_height) > 1 {
            return false;
        }

        let left_check = if self.left.is_some() {
            self.left.as_ref().unwrap().is_balanced_binary_tree()
        } else {
            true
        };
        let right_check = if self.right.is_some() {
            self.right.as_ref().unwrap().is_balanced_binary_tree()
        } else {
            true
        };
        left_check && right_check
    }

    fn get_height(&self) -> usize {
        let left_height = if self.left.is_some() {
            self.left.as_ref().unwrap().get_height()
        } else {
            0
        };

        let right_height = if self.right.is_some() {
            self.right.as_ref().unwrap().get_height()
        } else {
            0
        };

        left_height.max(right_height) + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preorder_traverse() {
        let root = node!(3, node!(1, node!(2), None), node!(5));
        root.unwrap().preorder_traverse();
    }

    #[test]
    fn test_inorder_traverse() {
        let root = node!(3, node!(1), node!(5));
        root.unwrap().inorder_traverse();
    }

    #[test]
    fn test_postorder_traverse() {
        let root = node!(3, node!(1), node!(5));
        root.unwrap().postorder_traverse();
    }

    #[test]
    fn test_is_full_binary_tree() {
        let root = node!(3, node!(1), node!(5));
        assert!(root.unwrap().is_full_binary_tree());

        let root = node!(3, node!(1), None);
        assert!(!root.unwrap().is_full_binary_tree());
    }

    #[test]
    fn test_is_perfect_binary_tree() {
        let root = node!(
            3,
            node!(1, node!(2), node!(3)),
            node!(3, node!(5), node!(7))
        );
        assert!(root.unwrap().is_perfect_binary_tree());

        let root = node!(1);
        assert!(root.unwrap().is_perfect_binary_tree());

        let root = node!(3, node!(1), None);
        assert!(!root.unwrap().is_perfect_binary_tree());
    }

    #[test]
    fn test_is_complete_binary_tree() {
        let root = node!(1, node!(2, node!(4), node!(5)), node!(3, node!(6), None));
        assert!(root.unwrap().is_complete_binary_tree());

        let root = node!(1, node!(2, node!(4), node!(5)), node!(3, None, node!(6)));
        assert!(!root.unwrap().is_complete_binary_tree());
    }

    #[test]
    fn test_is_degenerate() {
        let root = node!(1, node!(3, None, node!(4)), None);
        assert!(root.unwrap().is_degenerate());

        let root = node!(1, node!(3, node!(4), node!(5)), None);
        assert!(!root.unwrap().is_degenerate());
    }

    #[test]
    fn test_is_skewed_binary_tree() {
        let root = node!(1, node!(3, node!(5, node!(7), None), None), None);
        assert!(root.unwrap().is_skewed_binary_tree() == Some("left".to_string()));

        let root = node!(1, None, node!(3, None, node!(5, None, node!(7))));
        assert!(root.unwrap().is_skewed_binary_tree() == Some("right".to_string()));

        let root = node!(1, None, node!(3, node!(5), None));
        assert!(root.unwrap().is_skewed_binary_tree() == None);

        let root = node!(1);
        assert!(root.unwrap().is_skewed_binary_tree() == None);
    }

    #[test]
    fn test_is_balanced_binary_tree() {
        let root = node!(1, node!(2, node!(4), node!(5)), node!(3));
        assert!(root.unwrap().is_balanced_binary_tree());

        let root = node!(1, node!(2, node!(4, node!(8), None), node!(5)), node!(3));
        assert!(!root.unwrap().is_balanced_binary_tree());
    }

    #[test]
    fn test_get_height() {
        let root = node!(1);
        assert!(root.unwrap().get_height() == 1);

        let root = node!(1, node!(2), None);
        assert!(root.unwrap().get_height() == 2);

        let root = node!(
            1,
            node!(2, node!(3), None),
            node!(2, None, node!(3, None, node!(4)))
        );
        assert!(root.unwrap().get_height() == 4);
    }

    #[test]
    fn test_get_nodes_count() {
        let root = node!(1);
        assert!(root.unwrap().get_nodes_count() == 1);

        let root = node!(1, node!(2, None, node!(3)), node!(4));
        assert!(root.unwrap().get_nodes_count() == 4);
    }
}
