use std::fmt::Debug;
use std::ptr::NonNull;
use either::Either;

type UsizeLink = Option<NonNull<UsizeNode>>;

struct UsizeNode
{
    elem: usize,
    left: UsizeLink,
    right: UsizeLink,
}

pub struct UsizeTree
{
    root: UsizeLink,
    len: usize,
}

fn box_to_raw(elem: Box<UsizeNode>) -> Option<NonNull<UsizeNode>> {
    Some(NonNull::new(Box::into_raw(elem)).unwrap())
}

impl UsizeTree
{
    // create a new empty tree
    pub fn new() -> Self
    {
        Self {
            root: None,
            len: 0
        }
    }

    // returns the number of elements in the tree
    pub fn len(&self) -> usize
    {
       self.len
    }

    // returns whether tree is empty or not
    pub fn is_empty(&self) -> bool
    {
        self.len == 0
    }

    // insert an element into the tree
    // return if insertion was successful
    // insertion fails if the value is already in the tree
    pub fn insert(&mut self, elem: usize) -> bool
    {

        if self.contains(elem) {
            return false;
        }
        let new_node = Box::new(UsizeNode {
            elem: elem,
            left: None,
            right: None,
        });

            unsafe {
                if let Some(mut tree_root) = self.root {

                    // Traverse and insert at the first available position
                    loop {
                        let current_ref = tree_root.as_mut();
    
                        if elem < current_ref.elem {
                            if let Some(left) = current_ref.left {
                                tree_root = left;
                            } else {
                                current_ref.left =
                                    box_to_raw(new_node);
                                    self.len+= 1;
                                break;
                            }
                        } else {
                            if let Some(right) = current_ref.right {
                                tree_root = right;
                            } else {
                                current_ref.right = box_to_raw(new_node);
                                self.len+= 1;
                                break;
                            }
                        }
                    }
                } else {

                    let root_node = Box::new(UsizeNode {
                        elem,
                        left: None,
                        right: None,
                    });
            
                    // Convert the Box<UsizeNode> to NonNull<UsizeNode>
                    self.root = box_to_raw(root_node);
                    self.len+= 1;
                }
                true
            }
    }
    pub fn contains(&self, value: usize) -> bool {
        Self::contains_node(self.root, &value)
    }

    pub fn contains_node(node: UsizeLink, elem: &usize) -> bool
    {
        unsafe {
            if let Some(non_null_node) = node {
                let current = non_null_node.as_ref();
                if current.elem == *elem {
                    return true;
                }

                // TODO
                
                //should be able to know which side the tree is on based
                // on a value comparison (e.g.  greater than, less than)
                // no need to burn down both sides of the tree all the time

                // could just have this function call find_node.
                // if find_node returns anything at all, obviously, it exists
                // having extra logic is wasteful and violating DRY and single point of failure
            
                // Check left and right children
                return Self::contains_node(current.left, elem)
                    || Self::contains_node(current.right, elem);
            }
    
            false
        }
    }

    pub fn find_node(link: UsizeLink, value: usize) -> Option<&'static UsizeNode> {
        if let Some(non_null_node) = link {
            unsafe {
                let node_ref = non_null_node.as_ref();

                if node_ref.elem == value {
                    return Some(node_ref); // Found the node
                }
    
                // TODO
                
                //should be able to know which side the tree is on based
                // on a value comparison (e.g.  greater than, less than)
                // no need to burn down both sides of the tree all the time

                // Recursively search left and right subtrees
                let left_result = Self::find_node(node_ref.left, value);
                if left_result.is_some() {
                    return left_result;
                }
    
                let right_result = Self::find_node(node_ref.right, value);
                if right_result.is_some() {
                    return right_result;
                }
            }
        }

        None // Node not found
    }
    
    fn find_link_mut(&mut self, elem: usize) -> &mut UsizeLink
    {
        unsafe {
            let mut cur_node = &mut self.root;
            while let Some(cur) = cur_node
            {
                let parent = &mut (*cur.as_ptr());
                if elem == parent.elem
                {
                    return cur_node;
                }

                cur_node = if elem < parent.elem { &mut parent.left } else { &mut parent.right };
            }
            cur_node
        }
    }

    fn find_link(&self, elem: usize) -> &UsizeLink
    {
        unsafe {
            let mut cur_node = &self.root;
            while let Some(cur) = cur_node
            {
                let parent = &mut (*cur.as_ptr());
                if elem == parent.elem
                {
                    return cur_node;
                }

                cur_node = if elem < parent.elem { &mut parent.left } else { &mut parent.right };
            }
            cur_node
        }
    }

    // remove an element from the tree
    // return if removal was successful
    // removal fails if the value isn't in the tree
    pub fn remove(&mut self, elem: usize) -> bool
    {
        unsafe {
            let remove_link = self.find_link_mut(elem);

            if remove_link.is_some()
            {
                let remove_node = &mut (*remove_link.unwrap().as_ptr());

                if remove_node.left.is_none()
                {
                    let old = Box::from_raw(remove_link.unwrap().as_ptr());
                    *remove_link = remove_node.right.take();
                    drop(old);
                }
                else if remove_node.right.is_none()
                {
                    let old = Box::from_raw(remove_link.unwrap().as_ptr());
                    *remove_link = remove_node.left.take();
                    drop(old);
                }
                else
                {
                    let mut successor_link = &mut remove_node.right;
                    while (*successor_link.unwrap().as_ptr()).left.is_some()
                    {
                        successor_link = &mut (*successor_link.unwrap().as_ptr()).left;
                    }

                    remove_node.elem = (*successor_link.unwrap().as_ptr()).elem;

                    let successor_node = &mut (*successor_link.unwrap().as_ptr());
                    let old = Box::from_raw(successor_link.unwrap().as_ptr());
                    *successor_link = successor_node.right.take();
                    drop(old);
                }
                self.len -= 1;
                return true;
            }

            false
        }
    }


    // clear the tree
    // postcondition: tree is empty
    //  pub fn clear(&mut self) { 
    //     unsafe {
    //         Self::drop_node(self.root);
    //     }
    //     self.len = 0;
    // }
    // clear the tree
    // postcondition: tree is empty
    pub fn clear(&mut self)
    {
        unsafe {
            while let Some(root_node) = self.root
            {
                let root_elem = (*root_node.as_ptr()).elem;
                self.remove(root_elem);
            }
        }
    }

    // Helper function to free nodes recursively
    // unsafe fn drop_node(node: UsizeLink) {
    //     if let Some(non_null_node) = node {
    //         let boxed_node = Box::from_raw(non_null_node.as_ptr());

    //         //I am being optimistic here and assuming the tree isn't length gorillion
    //         Self::drop_node(boxed_node.left);
    //         Self::drop_node(boxed_node.right);
    //     }
    // }
}

impl Drop for UsizeTree {
    // fn drop(&mut self) {
    //     unsafe {
    //         Self::drop_node(self.root);
    //     }
    // }
        fn drop(&mut self) { self.clear(); }    
}

// impl Clone for UsizeTree
// {
//     fn clone(&self) -> Self
//     {
//         todo!();
//     }
// }

// impl Debug for UsizeTree
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
//     {
//         todo!();
//     }
// }

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_new_len()
    {
        let tree = UsizeTree::new();
        assert_eq!(tree.len(), 0);
    }

    #[test]
    fn test_new_is_empty()
    {
        let tree = UsizeTree::new();
        assert_eq!(tree.is_empty(), true);
    }

    #[test]
    fn test_insert()
    {
        let mut tree = UsizeTree::new();
        tree.insert(1);
        assert_eq!(tree.insert(4), true);
    }

    #[test]
    fn test_insert_single_basic()
    {
        let mut tree = UsizeTree::new();

        assert_eq!(tree.len(), 0);
        assert_eq!(tree.is_empty(), true);

        assert_eq!(tree.insert(4), true);

        assert_eq!(tree.len(), 1);
        assert_eq!(tree.is_empty(), false);
    }

    #[test]
    fn test_insert_multiple_basic()
    {
        let mut tree = UsizeTree::new();

        assert_eq!(tree.len(), 0);
        assert_eq!(tree.is_empty(), true);

        assert_eq!(tree.insert(4), true);
        assert_eq!(tree.len(), 1);
        assert_eq!(tree.is_empty(), false);

        assert_eq!(tree.insert(2), true);
        assert_eq!(tree.len(), 2);
        assert_eq!(tree.is_empty(), false);

        assert_eq!(tree.insert(6), true);
        assert_eq!(tree.len(), 3);
        assert_eq!(tree.is_empty(), false);

        assert_eq!(tree.insert(1), true);
        assert_eq!(tree.len(), 4);
        assert_eq!(tree.is_empty(), false);

        assert_eq!(tree.insert(3), true);
        assert_eq!(tree.len(), 5);
        assert_eq!(tree.is_empty(), false);

        assert_eq!(tree.insert(5), true);
        assert_eq!(tree.len(), 6);
        assert_eq!(tree.is_empty(), false);

        assert_eq!(tree.insert(7), true);
        assert_eq!(tree.len(), 7);
        assert_eq!(tree.is_empty(), false);
    }

    #[test]
    fn test_insert_multiple_with_collisions()
    {
        let mut tree = UsizeTree::new();

        assert_eq!(tree.len(), 0);
        assert_eq!(tree.is_empty(), true);

        assert_eq!(tree.insert(4), true);
        assert_eq!(tree.len(), 1);
        assert_eq!(tree.is_empty(), false);

        assert_eq!(tree.insert(2), true);
        assert_eq!(tree.len(), 2);
        assert_eq!(tree.is_empty(), false);

        assert_eq!(tree.insert(6), true);
        assert_eq!(tree.len(), 3);
        assert_eq!(tree.is_empty(), false);

        assert_eq!(tree.insert(2), false);
        assert_eq!(tree.len(), 3);
        assert_eq!(tree.is_empty(), false);

        assert_eq!(tree.insert(1), true);
        assert_eq!(tree.len(), 4);
        assert_eq!(tree.is_empty(), false);

        assert_eq!(tree.insert(3), true);
        assert_eq!(tree.len(), 5);
        assert_eq!(tree.is_empty(), false);

        assert_eq!(tree.insert(5), true);
        assert_eq!(tree.len(), 6);
        assert_eq!(tree.is_empty(), false);

        assert_eq!(tree.insert(7), true);
        assert_eq!(tree.len(), 7);
        assert_eq!(tree.is_empty(), false);

        assert_eq!(tree.insert(3), false);
        assert_eq!(tree.len(), 7);
        assert_eq!(tree.is_empty(), false);
    }

    #[test]
    fn test_contains()
    {
        let mut tree = UsizeTree::new();

        assert_eq!(tree.contains(2), false);
        assert_eq!(tree.contains(17), false);

        assert_eq!(tree.insert(4), true);
        assert_eq!(tree.contains(1), false);
        assert_eq!(tree.contains(4), true);
        assert_eq!(tree.contains(17), false);

        assert_eq!(tree.insert(2), true);
        assert_eq!(tree.contains(2), true);
        assert_eq!(tree.contains(4), true);
        assert_eq!(tree.contains(17), false);

        assert_eq!(tree.insert(6), true);
        assert_eq!(tree.contains(2), true);
        assert_eq!(tree.contains(4), true);
        assert_eq!(tree.contains(6), true);
        assert_eq!(tree.contains(3), false);
        assert_eq!(tree.contains(17), false);

        assert_eq!(tree.insert(2), false);
        assert_eq!(tree.contains(2), true);
        assert_eq!(tree.contains(4), true);
        assert_eq!(tree.contains(6), true);
        assert_eq!(tree.contains(3), false);
        assert_eq!(tree.contains(17), false);

        assert_eq!(tree.insert(1), true);
        assert_eq!(tree.contains(1), true);
        assert_eq!(tree.contains(2), true);
        assert_eq!(tree.contains(4), true);
        assert_eq!(tree.contains(6), true);
        assert_eq!(tree.contains(17), false);

        assert_eq!(tree.insert(3), true);
        assert_eq!(tree.contains(1), true);
        assert_eq!(tree.contains(2), true);
        assert_eq!(tree.contains(3), true);
        assert_eq!(tree.contains(4), true);
        assert_eq!(tree.contains(6), true);
        assert_eq!(tree.contains(5), false);
        assert_eq!(tree.contains(17), false);

        assert_eq!(tree.insert(5), true);
        assert_eq!(tree.contains(1), true);
        assert_eq!(tree.contains(2), true);
        assert_eq!(tree.contains(3), true);
        assert_eq!(tree.contains(4), true);
        assert_eq!(tree.contains(5), true);
        assert_eq!(tree.contains(6), true);
        assert_eq!(tree.contains(17), false);

        assert_eq!(tree.insert(7), true);
        assert_eq!(tree.contains(1), true);
        assert_eq!(tree.contains(2), true);
        assert_eq!(tree.contains(3), true);
        assert_eq!(tree.contains(4), true);
        assert_eq!(tree.contains(5), true);
        assert_eq!(tree.contains(6), true);
        assert_eq!(tree.contains(7), true);
        assert_eq!(tree.contains(17), false);

        assert_eq!(tree.insert(3), false);
        assert_eq!(tree.contains(1), true);
        assert_eq!(tree.contains(2), true);
        assert_eq!(tree.contains(3), true);
        assert_eq!(tree.contains(4), true);
        assert_eq!(tree.contains(5), true);
        assert_eq!(tree.contains(6), true);
        assert_eq!(tree.contains(7), true);
        assert_eq!(tree.contains(17), false);
    }

    #[test]
    fn test_remove()
    {
        let mut tree = UsizeTree::new();

        assert_eq!(tree.insert(4), true);
        assert_eq!(tree.insert(2), true);
        assert_eq!(tree.insert(6), true);
        assert_eq!(tree.insert(2), false);
        assert_eq!(tree.insert(1), true);
        assert_eq!(tree.insert(3), true);
        assert_eq!(tree.insert(5), true);
        assert_eq!(tree.insert(7), true);
        assert_eq!(tree.insert(3), false);

        assert_eq!(tree.remove(17), false);
        assert_eq!(tree.contains(1), true);
        assert_eq!(tree.contains(2), true);
        assert_eq!(tree.contains(3), true);
        assert_eq!(tree.contains(4), true);
        assert_eq!(tree.contains(5), true);
        assert_eq!(tree.contains(6), true);
        assert_eq!(tree.contains(7), true);
        assert_eq!(tree.len(), 7);

        assert_eq!(tree.remove(5), true);
        assert_eq!(tree.contains(5), false);
        assert_eq!(tree.contains(1), true);
        assert_eq!(tree.contains(2), true);
        assert_eq!(tree.contains(3), true);
        assert_eq!(tree.contains(4), true);
        assert_eq!(tree.contains(6), true);
        assert_eq!(tree.contains(7), true);
        assert_eq!(tree.len(), 6);

        assert_eq!(tree.remove(5), false);
        assert_eq!(tree.contains(5), false);
        assert_eq!(tree.contains(1), true);
        assert_eq!(tree.contains(2), true);
        assert_eq!(tree.contains(3), true);
        assert_eq!(tree.contains(4), true);
        assert_eq!(tree.contains(6), true);
        assert_eq!(tree.contains(7), true);
        assert_eq!(tree.len(), 6);

        assert_eq!(tree.insert(5), true);
        assert_eq!(tree.contains(5), true);
        assert_eq!(tree.len(), 7);

        assert_eq!(tree.remove(5), true);
        assert_eq!(tree.contains(5), false);
        assert_eq!(tree.contains(1), true);
        assert_eq!(tree.contains(2), true);
        assert_eq!(tree.contains(3), true);
        assert_eq!(tree.contains(4), true);
        assert_eq!(tree.contains(6), true);
        assert_eq!(tree.contains(7), true);
        assert_eq!(tree.len(), 6);

        assert_eq!(tree.remove(2), true);
        assert_eq!(tree.contains(2), false);
        assert_eq!(tree.contains(5), false);
        assert_eq!(tree.contains(1), true);
        assert_eq!(tree.contains(3), true);
        assert_eq!(tree.contains(4), true);
        assert_eq!(tree.contains(6), true);
        assert_eq!(tree.contains(7), true);
        assert_eq!(tree.len(), 5);

        assert_eq!(tree.remove(1), true);
        assert_eq!(tree.remove(3), true);
        assert_eq!(tree.remove(4), true);
        assert_eq!(tree.remove(6), true);
        assert_eq!(tree.remove(7), true);
        assert_eq!(tree.contains(1), false);
        assert_eq!(tree.contains(2), false);
        assert_eq!(tree.contains(3), false);
        assert_eq!(tree.contains(4), false);
        assert_eq!(tree.contains(5), false);
        assert_eq!(tree.contains(6), false);
        assert_eq!(tree.contains(7), false);
        assert_eq!(tree.len(), 0);
        assert_eq!(tree.is_empty(), true);

        assert_eq!(tree.remove(17), false);
        assert_eq!(tree.len(), 0);
        assert_eq!(tree.is_empty(), true);
    }

    #[test]
    fn test_clear()
    {
        let mut tree = UsizeTree::new();
        assert_eq!(tree.insert(1), true);
        assert_eq!(tree.insert(2), true);
        assert_eq!(tree.insert(3), true);
        assert_eq!(tree.insert(4), true);
        assert_eq!(tree.insert(5), true);
        assert_eq!(tree.insert(6), true);
        assert_eq!(tree.insert(7), true);
        tree.clear();
        assert_eq!(tree.len(), 0);
        assert_eq!(tree.is_empty(), true);
    }
}
