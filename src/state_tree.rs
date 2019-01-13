use std::cmp::Ordering;
use std::fmt::Debug;

/// A Binary Search Tree custom implementation allowing to store the
/// different states of a set of value over time.
///
/// Accessing an element is O(n) worst case, and O(log n) on average
/// Storing an element is O(n) worst case, and O(log n) on average
///
/// Time is generic, any K type implementing Ord + Eq + Debug is to
/// be considered as potentially OK to represent time.
///
/// Value is generic, any V type can be stored in the tree.       
pub struct StateTree<K: Ord + Eq + Debug, V> {
    /// The `StateTreeNode`, value contained in this tree's root.
    node: StateTreeNode<K, V>,
    /// The lower children (usually represented as the left one)
    /// cf. Binary Search Tree doc
    child_low: Option<Box<StateTree<K, V>>>,
    /// The higher children (usually represented as the right one)
    /// cf. Binary Search Tree doc
    child_high: Option<Box<StateTree<K, V>>>,
}

impl<K: Ord + Eq + Debug, V> StateTree<K, V> {
    /// Creates a new instance of a StateTree, the index and value.
    /// Children of a newly created StateTree will be None by default
    pub fn new(first_index: K, first_value: V) -> Self {
        StateTree {
            node: StateTreeNode {
                indexer: first_index,
                value: first_value,
            },
            child_low: None,
            child_high: None,
        }
    }

    /// Adds a new node to the tree
    pub fn push(&mut self, index: K, value: V) {
        let node = StateTreeNode {
            indexer: index,
            value,
        };
        self.push_inner(node);
    }

    /// Internal API to the push function, avoids leaking StateTreeNode
    fn push_inner(&mut self, node: StateTreeNode<K, V>) {
        // Compared to this node, the new node is either greater or lower.
        // This data structure does not support storing two different values
        // at the same index, because it does not make sense
        let next_node = match node.indexer.cmp(&self.node.indexer) {
            Ordering::Greater => &mut self.child_high,
            Ordering::Less => &mut self.child_low,
            _ => panic!("Already got a state at index {:?}", node.indexer),
        };

        // next_node is either None, (no node - so we store node as the next_node)
        // or contains a reference to a StateTreeNode (so we use push_inner on that one)
        // eventually storing node at some point
        match next_node {
            None => *next_node = Some(Box::new(node.as_root())),
            Some(ref mut n) => n.push_inner(node),
        }
    }

    /// Searches for an element in the tree, returning an Option because searching might fail
    ///
    /// Searching returns a reference to the value V indexed by the biggest K such as K <= index
    pub fn search(&self, index: K) -> Option<&V> {
        match index.cmp(&self.node.indexer) {
            Ordering::Equal => Some(&self.node.value),
            Ordering::Less => match self.child_low {
                Some(ref child) => child.search(index),
                _ => None,
            },
            Ordering::Greater => match self.child_high {
                Some(ref child) => {
                    let searched = child.search(index);
                    if searched.is_none() {
                        Some(&self.node.value)
                    } else {
                        searched
                    }
                }
                None => Some(&self.node.value),
            },
        }
    }
}

/// Inner type, should never be constructed manually
///
/// Represents a node on a `StateTree`
struct StateTreeNode<K, V> {
    indexer: K,
    value: V,
}

impl<K: Eq + Ord + Debug, V> StateTreeNode<K, V> {
    /// Consumes the Node, and transforms it into a new `StateTree`,
    /// the root of the newly created tree will have this node as its value
    pub fn as_root(self) -> StateTree<K, V> {
        StateTree::new(self.indexer, self.value)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    /// Creating a StateTree should not panic
    pub fn state_tree_create() {
        let _: StateTree<usize, char> = StateTree::new(0, 'a');
    }

    #[test]
    /// Pushing on a StateTree with different keys should never panic
    pub fn state_tree_push() {
        let mut tree: StateTree<usize, char> = StateTree::new(3, 'b');
        tree.push(5, 'd');
        tree.push(4, 'c');
        tree.push(2, 'a');

        assert!(tree.child_low.is_some());
        assert!(tree.child_high.is_some());

        let tree_2 = tree.child_high.unwrap();
        assert!(tree_2.child_low.is_some());
        assert!(tree_2.child_high.is_none());
    }

    #[test]
    #[should_panic]
    /// Pushing on a StateTree with a key that's already been placed should
    /// panic.
    /// TODO: API? Is it better to return a Result or to panic ?  
    pub fn state_tree_push_panic() {
        let mut tree: StateTree<usize, char> = StateTree::new(2, 'c');
        tree.push(0, 'a');
        tree.push(1, 'b');
        tree.push(2, 'c');
    }

    #[test]
    /// Searching on a StateTree should return a correct
    /// result and never panic
    pub fn state_tree_search() {
        let mut tree: StateTree<usize, char> = StateTree::new(20, 'c');
        tree.push(5, 'a');
        tree.push(10, 'b');
        tree.push(25, 'd');

        // Searching for undefined K (there is no K such as K < 0)
        (0..=4).for_each(|i| assert_eq!(tree.search(i), None));

        // Searching for defined K
        assert_eq!(tree.search(5), Some(&'a'));
        assert_eq!(tree.search(10), Some(&'b'));
        assert_eq!(tree.search(20), Some(&'c'));
        assert_eq!(tree.search(25), Some(&'d'));

        // Searching for not directly defined K
        assert_eq!(tree.search(6), Some(&'a'));
        assert_eq!(tree.search(11), Some(&'b'));
        assert_eq!(tree.search(21), Some(&'c'));
        assert_eq!(tree.search(26), Some(&'d'));

        // Overall checking
        (0..30).for_each(|i| {
            let result = match i {
                x if x < 5 => None,
                x if x < 10 => Some(&'a'),
                x if x < 20 => Some(&'b'),
                x if x < 25 => Some(&'c'),
                _ => Some(&'d'),
            };
            assert_eq!(tree.search(i), result);
        })
    }
}
