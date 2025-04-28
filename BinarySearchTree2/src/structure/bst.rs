use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type BstNodeLink = Rc<RefCell<BstNode>>;
pub type WeakBstNodeLink = Weak<RefCell<BstNode>>;

//this package implement BST wrapper
#[derive(Debug, Clone)]
pub struct BstNode {
    pub key: Option<i32>,
    pub parent: Option<WeakBstNodeLink>,
    pub left: Option<BstNodeLink>,
    pub right: Option<BstNodeLink>,
}

impl BstNode {
    //private interface
    fn new(key: i32) -> Self {
        BstNode {
            key: Some(key),
            left: None,
            right: None,
            parent: None,
        }
    }

    pub fn new_bst_nodelink(value: i32) -> BstNodeLink {
        let currentnode = BstNode::new(value);
        let currentlink = Rc::new(RefCell::new(currentnode));
        currentlink
    }

    /**
     * Get a copy of node link
     */
    pub fn get_bst_nodelink_copy(&self) -> BstNodeLink {
        Rc::new(RefCell::new(self.clone()))
    }

    fn downgrade(node: &BstNodeLink) -> WeakBstNodeLink {
        Rc::<RefCell<BstNode>>::downgrade(node)
    }

    //private interface
    fn new_with_parent(parent: &BstNodeLink, value: i32) -> BstNodeLink {
        let mut currentnode = BstNode::new(value);
        //currentnode.add_parent(Rc::<RefCell<BstNode>>::downgrade(parent));
        currentnode.parent = Some(BstNode::downgrade(parent));
        let currentlink = Rc::new(RefCell::new(currentnode));
        currentlink
    }

    //add new left child, set the parent to current_node_link
    pub fn add_left_child(&mut self, current_node_link: &BstNodeLink, value: i32) {
        let new_node = BstNode::new_with_parent(current_node_link, value);
        self.left = Some(new_node);
    }

    //add new left child, set the parent to current_node_link
    pub fn add_right_child(&mut self, current_node_link: &BstNodeLink, value: i32) {
        let new_node = BstNode::new_with_parent(current_node_link, value);
        self.right = Some(new_node);
    }

    //search the current tree which node fit the value
    pub fn tree_search(&self, value: &i32) -> Option<BstNodeLink> {
        //TODO
        //default if current node is NIL
        if self.key == Some(*value) {
            return Some(self.get_bst_nodelink_copy());
        }else if Some(*value) > self.key {
            let right_side = self.right.as_ref().unwrap().borrow().clone();
            return Self::tree_search(&right_side, value);
        }else if Some(*value) < self.key{
            let left_side = self.left.as_ref().unwrap().borrow().clone();
            return Self::tree_search(&left_side, value);
        }else {
            None
        }
    }

    /**seek minimum by recurs
     * in BST minimum always on the left
     */
    pub fn minimum(&self) -> BstNodeLink {
        //TODO
        if self.left.is_some(){
            let new_left= self.left.as_ref().unwrap().borrow().clone();
            return Self::minimum(&new_left);
        }else{
            return self.get_bst_nodelink_copy()
        }
    }

    pub fn maximum(&self) -> BstNodeLink {
        //TODO
        if self.right.is_some(){
            let new_right = self.right.as_ref().unwrap().borrow().clone();
            return Self::maximum(&new_right);
        }else{
            return self.get_bst_nodelink_copy()
        }
    }

    /**
     * Return the root of a node, return self if not exist
     */
    pub fn get_root(node: &BstNodeLink) -> BstNodeLink {
        let parent = BstNode::upgrade_weak_to_strong(node.borrow().parent.clone());
        if parent.is_none() {
            return node.clone();
        }
        return BstNode::get_root(&parent.unwrap());
    }

    /**
     * Find node successor according to the book
     * Possible to return self, if x_node is the highest key in the tree
     */
    pub fn tree_successor(x_node: &BstNodeLink) -> BstNodeLink {
        //TODO
        if x_node.borrow().right.is_some(){
            let right = x_node.as_ref().borrow().clone();
            return Self::minimum(&right);
        }

        let x_parent = Self::upgrade_weak_to_strong(x_node.borrow().parent.clone());
        let x_parent_right_key = x_parent.unwrap().borrow().right.as_ref().unwrap().borrow().key;
        if x_parent.is_some() && x_node.borrow().key == x_parent_right_key{
        }
        return x_node.clone();
    }

    //helper function to compare both nodelink
    fn is_node_match_option(node1: Option<BstNodeLink>, node2: Option<BstNodeLink>) -> bool {
        if node1.is_none() && node2.is_none() {
            return true;
        }
        if let Some(node1v) = node1 {
            return node2.is_some_and(|x: BstNodeLink| x.borrow().key == node1v.borrow().key);
        }
        return false;
    }

    fn is_node_match(anode: &BstNodeLink, bnode: &BstNodeLink) -> bool {
        if anode.borrow().key == bnode.borrow().key {
            return true;
        }
        return false;
    }

    /**
     * As the name implied, used to upgrade parent node to strong nodelink
     */
    fn upgrade_weak_to_strong(node: Option<WeakBstNodeLink>) -> Option<BstNodeLink> {
        match node {
            None => None,
            Some(x) => Some(x.upgrade().unwrap()),
        }
    }
}
