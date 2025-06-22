use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Ordering::{Less, Greater};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: None,
        }))
    }
}

pub struct Solution;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut current = root;

        let p_val = p.as_ref().unwrap().borrow().val;
        let q_val = q.as_ref().unwrap().borrow().val;

        while let Some(node) = current {
            match (
                p_val.cmp(&node.borrow().val),
                q_val.cmp(&node.borrow().val),
            ) {
                (Greater, Greater) => current = node.borrow().right.clone(),
                (Less, Less) => current = node.borrow().left.clone(),
                _ => return Some(node.clone()),
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lowest_common_ancestor() {
        let root = TreeNode::new(6);
        let n2 = TreeNode::new(2);
        let n8 = TreeNode::new(8);
        let n0 = TreeNode::new(0);
        let n4 = TreeNode::new(4);
        let n7 = TreeNode::new(7);
        let n9 = TreeNode::new(9);
        let n3 = TreeNode::new(3);
        let n5 = TreeNode::new(5);

        root.borrow_mut().left = Some(n2.clone());
        root.borrow_mut().right = Some(n8.clone());

        n2.borrow_mut().left = Some(n0.clone());
        n2.borrow_mut().right = Some(n4.clone());

        n4.borrow_mut().left = Some(n3.clone());
        n4.borrow_mut().right = Some(n5.clone());

        n8.borrow_mut().left = Some(n7.clone());
        n8.borrow_mut().right = Some(n9.clone());

        let lca = Solution::lowest_common_ancestor(Some(root.clone()), Some(n2.clone()), Some(n8.clone()));
        assert_eq!(lca.unwrap().borrow().val, 6);

        let lca = Solution::lowest_common_ancestor(Some(root.clone()), Some(n2.clone()), Some(n4.clone()));
        assert_eq!(lca.unwrap().borrow().val, 2);

        let lca = Solution::lowest_common_ancestor(Some(root.clone()), Some(n3.clone()), Some(n5.clone()));
        assert_eq!(lca.unwrap().borrow().val, 4);
    }
}

fn main() {
    println!("Hello, world!");
}