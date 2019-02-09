use crate::state_tree::StateTree;
use core::fmt::Debug;
use std::collections::HashMap;
use std::hash::Hash;

/// A generic struct that provides a way to keep track of changes
/// applied on a value V, present at coordinates C, over a given index K.
pub struct DiffCache<C: Hash + Eq, K: Ord + Eq + Copy + Debug, V> {
    /// Underlying data of the DiffCache
    data: HashMap<C, StateTree<K, V>>,
    /// Default representation of a Value
    default: V,
}

impl<C: Hash + Eq, K: Ord + Eq + Copy + Debug, V> DiffCache<C, K, V> {
    /// Creates a new instance of a DiffCache with the given generics
    pub fn new(default: V) -> Self {
        DiffCache {
            data: HashMap::default(),
            default,
        }
    }

    /// Adds a V value information for a given C coordinate and a given K index
    /// Panics if we failed to insert a StateTree in the inner data
    /// TODO: Should not panic, maybe return a boolean ?
    pub fn push(&mut self, coords: C, index: K, value: V) {
        let mut entry = self.data.get_mut(&coords);
        match entry {
            Some(ref mut tree) => tree.push(index, value),
            None => {
                self.data.insert(coords, StateTree::new(index, value));
            }
        };
    }

    /// Adds all the information contained in the input `Iterator`
    pub fn append(&mut self, input: impl Iterator<Item = (C, K, V)>) {
        input.for_each(|i| self.push(i.0, i.1, i.2))
    }

    /// Searches for a value V at given coordinates for a given index
    /// Returns an Option because the search might fail
    pub fn search(&self, coords: C, index: K) -> Option<&V> {
        match self.data.get(&coords) {
            Some(state_tree) => state_tree.search(index),
            _ => None,
        }
    }

    /// Takes a view into the DiffCache, maps all the given coordinates
    /// to their representation at index K.
    /// If no data was found, then default is returned.
    pub fn view(&self, coords: impl Iterator<Item = C>, index: K) -> impl Iterator<Item = &V> {
        coords.map(move |c| self.search(c, index).unwrap_or(&self.default))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Creating a DiffCache should not panic
    pub fn diffcache_create() {
        let _: DiffCache<(i32, i32), usize, char> = DiffCache::new('.');
    }

    #[test]
    /// Inserting in a DiffCache should not panic
    pub fn diffcache_push() {
        let mut cache: DiffCache<(i32, i32), usize, char> = DiffCache::new('.');
        cache.push((0, 0), 0, 'a');
        cache.push((0, 0), 5, 'b');
        cache.push((1, 0), 0, 'a');
        cache.push((1, 0), 3, 'b');
        cache.push((0, 1), 0, 'a');
        cache.push((0, 1), 2, 'b');
        cache.push((1, 1), 0, 'a');
        cache.push((1, 1), 1, 'b');
    }

    #[test]
    /// Appending the DiffCache should not panic
    pub fn diffcache_append() {
        let mut cache: DiffCache<(i32, i32), usize, char> = DiffCache::new('.');
        let infos = vec![
            ((0, 0), 0, 'a'),
            ((0, 0), 5, 'b'),
            ((1, 0), 0, 'a'),
            ((1, 0), 3, 'b'),
            ((0, 1), 0, 'a'),
            ((0, 1), 2, 'b'),
            ((1, 1), 0, 'a'),
            ((1, 1), 1, 'b'),
        ];
        cache.append(infos.into_iter());
    }

    #[test]
    pub fn diffcache_search() {
        let mut cache: DiffCache<(i32, i32), usize, char> = DiffCache::new('.');
        let infos = vec![
            ((0, 0), 0, 'a'),
            ((0, 0), 5, 'b'),
            ((1, 0), 0, 'a'),
            ((1, 0), 3, 'b'),
            ((0, 1), 0, 'a'),
            ((0, 1), 2, 'b'),
            ((1, 1), 0, 'a'),
            ((1, 1), 1, 'b'),
        ];
        cache.append(infos.into_iter());

        assert_eq!(cache.search((0, 0), 1), Some(&'a'));
        assert_eq!(cache.search((0, 0), 5), Some(&'b'));
    }

    #[test]
    pub fn diffcache_view() {
        let mut cache: DiffCache<(i32, i32), usize, char> = DiffCache::new('.');
        let infos = vec![
            ((0, 0), 0, 'a'),
            ((0, 0), 5, 'b'),
            ((1, 0), 0, 'a'),
            ((1, 0), 3, 'b'),
            ((0, 1), 0, 'a'),
            ((0, 1), 2, 'b'),
            ((1, 1), 0, 'a'),
            ((1, 1), 1, 'b'),
        ];
        cache.append(infos.into_iter());
        let expected: Vec<&char> = vec![&'a', &'b', &'b', &'b'];
        let actual: Vec<&char> = cache
            .view((0..2).flat_map(|y| (0..2).map(move |x| (x, y))), 4)
            .collect();
        assert_eq!(expected, actual);
    }
}
