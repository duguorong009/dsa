use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
struct TreeNode {
    data: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(data: i32) -> Self {
        Self {
            data,
            left: None,
            right: None,
        }
    }
}

macro_rules! node {
    ($value: expr) => {
        Some(Rc::new(RefCell::new(TreeNode::new($value))))
    };
    ($value: expr, $left: expr, $right: expr) => {
        Some(Rc::new(RefCell::new(TreeNode {
            data: $value,
            left: $left,
            right: $right,
        })))
    };
    () => {
        None
    };
}

fn search_bst(root: &Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<i32> {
    if let Some(node) = root {
        let node = node.borrow();
        if target < node.data {
            search_bst(&node.left, target)
        } else if target > node.data {
            search_bst(&node.right, target)
        } else {
            Some(target)
        }
    } else {
        None
    }
}

fn insert_bst(root: Option<Rc<RefCell<TreeNode>>>, value: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = root.clone() {
        let mut node = node.borrow_mut();

        if value < node.data {
            let left = node.left.take();
            node.left = insert_bst(left, value);
        } else {
            let right = node.right.take();
            node.right = insert_bst(right, value);
        };

        root
    } else {
        node!(value)
    }
}

fn delete_bst(root: Option<Rc<RefCell<TreeNode>>>, value: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = root.clone() {
        let mut node = node.borrow_mut();

        if value < node.data {
            let left = node.left.take();
            node.left = delete_bst(left, value);
        } else if value > node.data {
            let right = node.right.take();
            node.right = delete_bst(right, value);
        } else {
            if node.left.is_none() {
                let right = node.right.take();
                return right;
            } else if node.right.is_none() {
                let left = node.left.take();
                return left;
            }

            let min_value = find_min_value(&node.right);
            node.data = min_value;

            let right = node.right.take();
            let temp = delete_bst(right, min_value);
            node.right = temp;
        }

        root
    } else {
        None
    }
}

fn find_min_value(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(node) = root {
        let node = node.as_ref().borrow();
        if node.left.is_none() {
            return node.data;
        } else {
            find_min_value(&node.left)
        }
    } else {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro() {
        let root = node!(3);
        println!("root: {:?}", root);

        let root = node!(3, node!(1), node!(4));
        println!("node: {:?}", root);
    }

    #[test]
    fn test_search() {
        let root = node!(
            8,
            node!(3, node!(1), node!(6, node!(4), node!(7))),
            node!(10, None, node!(14))
        );
        let res = search_bst(&root, 14);
        assert!(res == Some(14));

        let res = search_bst(&root, 24);
        assert!(res == None);
    }

    #[test]
    fn test_insert() {
        let root = node!(
            8,
            node!(3, node!(1), node!(6, None, node!(7))),
            node!(10, None, node!(14))
        );

        let res = search_bst(&root, 4);
        assert!(res == None);

        let updated_root = insert_bst(root, 4);

        let res = search_bst(&updated_root, 4);
        assert!(res == Some(4));
    }

    #[test]
    fn test_delete() {
        let root = node!(8);
        let root = insert_bst(root, 3);
        let res = search_bst(&root, 3);
        assert!(res == Some(3));

        let root = delete_bst(root, 3);
        let res = search_bst(&root, 3);
        assert!(res == None);

        let root = node!(
            8,
            node!(3, node!(1), node!(6, node!(4), node!(7))),
            node!(10, None, node!(14))
        );
        let res = search_bst(&root, 3);
        assert!(res == Some(3));

        let root = delete_bst(root, 3);
        let res = search_bst(&root, 3);
        assert!(res == None);
    }
}
