use std::fmt::Debug;
use std::ptr::NonNull;

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

impl UsizeTree
{
    // create a new empty tree
    pub fn new() -> Self
    {
        todo!();
    }

    // returns the number of elements in the tree
    pub fn len(&self) -> usize
    {
        todo!();
    }

    // returns whether tree is empty or not
    pub fn is_empty(&self) -> bool
    {
        todo!();
    }

    // insert an element into the tree
    // return if insertion was successful
    // insertion fails if the value is already in the tree
    pub fn insert(&mut self, elem: usize) -> bool
    {
        todo!();
    }

    // search the tree for a value equal to passed elem
    // returns whether equal value was found or not
    pub fn contains(&self, elem: &usize) -> bool
    {
        todo!();
    }

    // remove an element from the tree
    // return if removal was successful
    // removal fails if the value isn't in the tree
    pub fn remove(&mut self, elem: usize) -> bool
    {
        todo!();
    }

    // clear the tree
    // postcondition: tree is empty
    pub fn clear(&mut self) { todo!() }
}

impl Drop for UsizeTree
{
    fn drop(&mut self) { todo!() }
}

impl Clone for UsizeTree
{
    fn clone(&self) -> Self
    {
        todo!();
    }
}

impl Debug for UsizeTree
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        todo!();
    }
}

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

        assert_eq!(tree.contains(&2), false);
        assert_eq!(tree.contains(&17), false);

        assert_eq!(tree.insert(4), true);
        assert_eq!(tree.contains(&1), false);
        assert_eq!(tree.contains(&4), true);
        assert_eq!(tree.contains(&17), false);

        assert_eq!(tree.insert(2), true);
        assert_eq!(tree.contains(&2), true);
        assert_eq!(tree.contains(&4), true);
        assert_eq!(tree.contains(&17), false);

        assert_eq!(tree.insert(6), true);
        assert_eq!(tree.contains(&2), true);
        assert_eq!(tree.contains(&4), true);
        assert_eq!(tree.contains(&6), true);
        assert_eq!(tree.contains(&3), false);
        assert_eq!(tree.contains(&17), false);

        assert_eq!(tree.insert(2), false);
        assert_eq!(tree.contains(&2), true);
        assert_eq!(tree.contains(&4), true);
        assert_eq!(tree.contains(&6), true);
        assert_eq!(tree.contains(&3), false);
        assert_eq!(tree.contains(&17), false);

        assert_eq!(tree.insert(1), true);
        assert_eq!(tree.contains(&1), true);
        assert_eq!(tree.contains(&2), true);
        assert_eq!(tree.contains(&4), true);
        assert_eq!(tree.contains(&6), true);
        assert_eq!(tree.contains(&17), false);

        assert_eq!(tree.insert(3), true);
        assert_eq!(tree.contains(&1), true);
        assert_eq!(tree.contains(&2), true);
        assert_eq!(tree.contains(&3), true);
        assert_eq!(tree.contains(&4), true);
        assert_eq!(tree.contains(&6), true);
        assert_eq!(tree.contains(&5), false);
        assert_eq!(tree.contains(&17), false);

        assert_eq!(tree.insert(5), true);
        assert_eq!(tree.contains(&1), true);
        assert_eq!(tree.contains(&2), true);
        assert_eq!(tree.contains(&3), true);
        assert_eq!(tree.contains(&4), true);
        assert_eq!(tree.contains(&5), true);
        assert_eq!(tree.contains(&6), true);
        assert_eq!(tree.contains(&17), false);

        assert_eq!(tree.insert(7), true);
        assert_eq!(tree.contains(&1), true);
        assert_eq!(tree.contains(&2), true);
        assert_eq!(tree.contains(&3), true);
        assert_eq!(tree.contains(&4), true);
        assert_eq!(tree.contains(&5), true);
        assert_eq!(tree.contains(&6), true);
        assert_eq!(tree.contains(&7), true);
        assert_eq!(tree.contains(&17), false);

        assert_eq!(tree.insert(3), false);
        assert_eq!(tree.contains(&1), true);
        assert_eq!(tree.contains(&2), true);
        assert_eq!(tree.contains(&3), true);
        assert_eq!(tree.contains(&4), true);
        assert_eq!(tree.contains(&5), true);
        assert_eq!(tree.contains(&6), true);
        assert_eq!(tree.contains(&7), true);
        assert_eq!(tree.contains(&17), false);
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
        assert_eq!(tree.contains(&1), true);
        assert_eq!(tree.contains(&2), true);
        assert_eq!(tree.contains(&3), true);
        assert_eq!(tree.contains(&4), true);
        assert_eq!(tree.contains(&5), true);
        assert_eq!(tree.contains(&6), true);
        assert_eq!(tree.contains(&7), true);
        assert_eq!(tree.len(), 6);

        assert_eq!(tree.remove(5), true);
        assert_eq!(tree.contains(&5), false);
        assert_eq!(tree.contains(&1), true);
        assert_eq!(tree.contains(&2), true);
        assert_eq!(tree.contains(&3), true);
        assert_eq!(tree.contains(&4), true);
        assert_eq!(tree.contains(&6), true);
        assert_eq!(tree.contains(&7), true);
        assert_eq!(tree.len(), 6);

        assert_eq!(tree.remove(5), false);
        assert_eq!(tree.contains(&5), false);
        assert_eq!(tree.contains(&1), true);
        assert_eq!(tree.contains(&2), true);
        assert_eq!(tree.contains(&3), true);
        assert_eq!(tree.contains(&4), true);
        assert_eq!(tree.contains(&6), true);
        assert_eq!(tree.contains(&7), true);
        assert_eq!(tree.len(), 6);

        assert_eq!(tree.insert(5), true);
        assert_eq!(tree.contains(&5), true);
        assert_eq!(tree.len(), 7);

        assert_eq!(tree.remove(5), true);
        assert_eq!(tree.contains(&5), false);
        assert_eq!(tree.contains(&1), true);
        assert_eq!(tree.contains(&2), true);
        assert_eq!(tree.contains(&3), true);
        assert_eq!(tree.contains(&4), true);
        assert_eq!(tree.contains(&6), true);
        assert_eq!(tree.contains(&7), true);
        assert_eq!(tree.len(), 6);

        assert_eq!(tree.remove(2), true);
        assert_eq!(tree.contains(&2), false);
        assert_eq!(tree.contains(&5), false);
        assert_eq!(tree.contains(&1), true);
        assert_eq!(tree.contains(&3), true);
        assert_eq!(tree.contains(&4), true);
        assert_eq!(tree.contains(&6), true);
        assert_eq!(tree.contains(&7), true);
        assert_eq!(tree.len(), 5);

        assert_eq!(tree.remove(1), true);
        assert_eq!(tree.remove(3), true);
        assert_eq!(tree.remove(4), true);
        assert_eq!(tree.remove(6), true);
        assert_eq!(tree.remove(7), true);
        assert_eq!(tree.contains(&1), false);
        assert_eq!(tree.contains(&2), false);
        assert_eq!(tree.contains(&3), false);
        assert_eq!(tree.contains(&4), false);
        assert_eq!(tree.contains(&5), false);
        assert_eq!(tree.contains(&6), false);
        assert_eq!(tree.contains(&7), false);
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
