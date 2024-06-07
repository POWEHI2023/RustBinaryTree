use std::rc::{ Rc, Weak };
use std::cell::{ RefCell };

#[derive(Debug)]
pub struct TreeNode<T: std::fmt::Display> {
    _val: T,
    _left: Option<Rc<RefCell<TreeNode<T>>>>,
    _right: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T: std::fmt::Display> Drop for TreeNode<T> {
    fn drop(&mut self) {
        // println!("drop tree node, value is {}", self._val);
    }
}

impl<T: std::fmt::Display> TreeNode<T> {
    #[allow(dead_code)]
    fn new(_val: T) -> TreeNode<T> {
        TreeNode { _val, _left: None, _right: None }
    }

    #[allow(dead_code)]
    fn new_with_children(
        _val: T,
        _left: Weak<RefCell<TreeNode<T>>>,
        _right: Weak<RefCell<TreeNode<T>>>
    ) -> TreeNode<T> {
        let _left = _left.upgrade();
        let _right = _right.upgrade();

        TreeNode { _val, _left, _right }
    }

    // param _left should be Weak or Rc type ?
    // reset left child of current node, and return the old child
    #[allow(dead_code)]
    fn set_left_child(&mut self, _left: Weak<RefCell<TreeNode<T>>>) -> Option<Rc<RefCell<TreeNode<T>>>> {
        let mut _ret = None;
        match &self._left {
            Some(_old_left) => _ret = Some(Rc::clone(_old_left)),
            None => {},
        }
        self._left = _left.upgrade();

        _ret
    }

    #[allow(dead_code)]
    fn set_right_child(&mut self, _right: Weak<RefCell<TreeNode<T>>>) -> Option<Rc<RefCell<TreeNode<T>>>> {
        let mut _ret = None;
        match &self._right {
            Some(_old_right) => _ret = Some(Rc::clone(_old_right)),
            None => {},
        }
        self._right = _right.upgrade();

        _ret
    }
}

pub trait TreeType<ElemType: std::cmp::PartialOrd + std::fmt::Display> {

    fn insert_value(&mut self, _val: ElemType) -> bool;
    fn insert_node(&mut self, _node: &TreeNode<ElemType>) -> bool;

    // fn remove_value(&mut self, _val: ElemType) -> Rc<RefCell<TreeNode<ElemType>>>;
    // fn remove_node(&mut self, _node: &TreeNode<ElemType>) -> Rc<RefCell<TreeNode<ElemType>>>;

    // check if _val is in the tree
    // if we find a matched TreeNode, then return the WeakPtr of this node
    // or we return an empty WeakPtr
    fn find_value(&self, _val: ElemType) -> Weak<RefCell<TreeNode<ElemType>>>;
}

#[derive(Debug)]
struct Tree<T: std::cmp::PartialOrd + std::fmt::Display> {
    _root: Rc<RefCell<TreeNode<T>>>,
    _height: usize
}

impl<T: std::cmp::PartialOrd + std::fmt::Display> Drop for Tree<T> {
    fn drop(&mut self) {
        // println!("drop tree.");
    }
}

impl<T: std::cmp::PartialOrd + std::fmt::Display> Tree<T> {
    #[allow(dead_code)]
    pub fn new(_root_val: T) -> Tree<T> {
        Tree {
            _root: Rc::new(RefCell::new(TreeNode::new(_root_val))),
            _height: 1
        }
    }
}

// 为了避免对引用计数频繁加减，尽量使用downgrade Weak
// 如果使用Rc替代Weak会更方便实现
// Weak使用了unsafe是因为我不知道怎样实现内部引用......
impl<T: std::cmp::PartialOrd + std::fmt::Display> TreeType<T> for Tree<T> {
    #[allow(dead_code)]
    fn insert_value(&mut self, _val: T) -> bool {
        let mut _weak = Rc::downgrade(&self._root);

        loop {
            // muttable TreeNode<T>
            let mut _inner_weak = unsafe { (*(_weak.as_ptr())).borrow_mut() };

            // already involve _val node in the tree
            if _inner_weak._val == _val { return false; }
            
            if _inner_weak._val > _val {
                // check left child
                if let Some(_left_node) = &_inner_weak._left {
                    // _left_node is &Rc<RefCell<TreeNode<T>>> type
                    _weak = Rc::downgrade(_left_node);  // _weak move into the left node, and continue loop
                    continue;
                }
                // _left_node is None, then we can construct a new TreeNode<T> and insert in the left
                let _new_node = Rc::new(RefCell::new( TreeNode::<T>::new(_val) ));
                _inner_weak._left = Some(_new_node);  // now the left node is not None type
                return true;
            } else {
                // check right child
                if let Some(_right_node) = &_inner_weak._right {
                    _weak = Rc::downgrade(_right_node);
                    continue;
                }

                let _new_node = Rc::new(RefCell::new( TreeNode::<T>::new(_val) ));
                _inner_weak._right = Some(_new_node);
                return true;
            }
        }
    }

    // non-implementation yet
    #[allow(dead_code)]
    fn insert_node(&mut self, _node: &TreeNode<T>) -> bool {
        false
    }

    /*pub fn remove_value(&mut self, _val: T) -> Rc<RefCell<TreeNode<T>>> {
        // Debug
        // assume ElemType is i32
        Rc::new(RefCell::new( TreeNode::<T>::new(0) ))
    }

    pub fn remove_node(&mut self, _node: &TreeNode<T>) -> Rc<RefCell<TreeNode<T>>> {
        // Debug
        // assume ElemType is i32
        Rc::new(RefCell::new( TreeNode::<T>::new(0) ))
    }*/

    #[allow(dead_code)]
    fn find_value(&self, _val: T) -> Weak<RefCell<TreeNode<T>>> {
        // _root is Rc<RefCell<TreeNode<T>>> type
        let mut _node = Rc::downgrade(&self._root);
        loop {
            let _inner_node = unsafe { (*(_node.as_ptr())).borrow() };

            if _inner_node._val == _val {
                return _node;
            }

            let _inner_node = unsafe { (*(_node.as_ptr())).borrow() };

            if _inner_node._val > _val {
                if let Some(_left_node) = &_inner_node._left {
                    _node = Rc::downgrade(_left_node);
                    continue;
                }
                return Weak::new();
            } else {
                if let Some(_right_node) = &_inner_node._right {
                    _node = Rc::downgrade(_right_node);
                    continue;
                }
                return Weak::new();
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use std::cell::{ RefCell };
    use std::rc::Rc;

    #[test]
    fn prelude_test() {
        println!("Start test......");
    }

    use crate::TreeNode;
    #[test]
    fn tree_node_function_test() {
        let creator = |x: i32| -> Rc<RefCell<TreeNode<i32>>>{
            Rc::new(RefCell::new(TreeNode::<i32>::new(x)))
        };
        
        let _root = creator(1);

        _root.borrow_mut()._left = Some(creator(2));
        _root.borrow_mut()._right = Some(creator(3));

        let _new_node = creator(4);
        let _ret = _root.borrow_mut().set_left_child(Rc::downgrade(&_new_node));
    }

    use crate::{ Tree, TreeType };
    #[test]
    fn tree_basic_function_test() {
        let mut _tree = Tree::new(0);

        _tree.insert_value(1);
        _tree.insert_value(2);
        _tree.insert_value(-2);
        _tree.insert_value(-1);
    }
}