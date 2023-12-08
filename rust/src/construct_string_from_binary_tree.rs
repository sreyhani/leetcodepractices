use std::cell::RefCell;
use std::rc::Rc;

use crate::Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
pub fn check_childs(node: Rc<RefCell<TreeNode>>) -> String {
    let mut node = node.borrow_mut();
    match (node.left.take(), node.right.take()) {
        (Some(left), Some(right)) => format!(
            "({})({})",
            Solution::tree2str(Some(left)),
            Solution::tree2str(Some(right))
        ),
        (None, Some(right)) => format!("()({})", Solution::tree2str(Some(right))),
        (Some(left), None) => format!("({})", Solution::tree2str(Some(left))),
        (None, None) => "".to_string(),
    }
}

impl Solution {
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        match root {
            Some(node) => {
                let root_string = node.borrow().val.to_string();
                root_string + &check_childs(node)
            }
            None => "".to_string(),
        }
    }
}

#[test]
fn test_empty_node() {
    assert_eq!(Solution::tree2str(None), "");
}

#[test]
fn test_only_root() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: None,
    })));
    assert_eq!(Solution::tree2str(root), "1");
}

#[test]
fn test_root_with_left_child() {
    let child = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: None,
        right: None,
    })));
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: child,
        right: None,
    })));
    assert_eq!(Solution::tree2str(root), "1(2)");
}

#[test]
fn test_root_with_right_child() {
    let child = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: None,
        right: None,
    })));
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: child,
    })));
    assert_eq!(Solution::tree2str(root), "1()(2)");
}

#[test]
fn test_root_with_both_child() {
    let lchild = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: None,
        right: None,
    })));

    let rchild = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: None,
        right: None,
    })));

    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: lchild,
        right: rchild,
    })));
    assert_eq!(Solution::tree2str(root), "1(2)(3)");
}

#[test]
fn test1() {
    //root = [1,2,3,4]
    let n4 = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: None,
        right: None,
    })));

    let n3 = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: None,
        right: None,
    })));
    let n2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: n4,
        right: None,
    })));

    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: n2,
        right: n3,
    })));
    assert_eq!(Solution::tree2str(root), "1(2(4))(3)");
}

#[test]
fn test2() {
    //root = [1,2,3,null,4]
    let n4 = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: None,
        right: None,
    })));

    let n3 = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: None,
        right: None,
    })));
    let n2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: None,
        right: n4,
    })));

    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: n2,
        right: n3,
    })));
    assert_eq!(Solution::tree2str(root), "1(2()(4))(3)");
}
