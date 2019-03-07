use crate::diff_cache::DiffCache;
use crate::time_index::TimeIndex;
use itertools::{EitherOrBoth, Itertools};
use std::hash::Hash;
use std::string::ToString;
use std::sync::{Arc, Mutex};

/// Trait allowing cargo-aoc to Visualize an implementor
/// using the display function of the V type
/// on a plane defined by the given C coords type
pub trait Visualize<C: Hash + Eq, V> {
    /// Provides the value at given coordinates.
    /// If this method fails to provide a value, the default value
    /// will be used.
    fn get(&self, coords: &C) -> Option<V>;

    /// Provides a list of all the coordinates that changed their value
    /// between the previous state and this one.
    fn delta(&self, previous: &Self) -> Vec<C>;

    /// Provides the default value to be shown if the implementor fails
    /// to provide a value for a specific coord
    fn default_val(&self) -> V;
}

/// Reference implementation for String
impl<T> Visualize<(i32, i32), char> for T
where
    T: ToString,
{
    fn default_val(&self) -> char {
        ' '
    }

    fn get(&self, coords: &(i32, i32)) -> Option<char> {
        match coords {
            (x, y) if *x < 0 || *y < 0 => None,
            (x, y) => self
                .to_string()
                .lines()
                .skip(*y as usize)
                .next()
                .and_then(|line| line.chars().skip(*x as usize).next()),
        }
    }

    fn delta(&self, previous: &Self) -> Vec<(i32, i32)> {
        let mut x: i32 = 0;
        let mut y: i32 = -1;
        self.to_string()
            .lines()
            .zip_longest(previous.to_string().lines())
            .map(|either_or_both: EitherOrBoth<&str, &str>| {
                y += 1;
                x = -1;
                match either_or_both {
                    EitherOrBoth::Both(curr, prev) => curr
                        .chars()
                        .zip_longest(prev.chars())
                        .filter_map(|eob| {
                            x += 1;
                            match eob {
                                EitherOrBoth::Both(a, b) if a != b => Some((x, y)),
                                EitherOrBoth::Both(_, _) => None,
                                _ => Some((x, y)),
                            }
                        })
                        .collect(),
                    EitherOrBoth::Left(val) | EitherOrBoth::Right(val) => val
                        .chars()
                        .map(|_| {
                            x += 1;
                            (x, y)
                        })
                        .collect(),
                }
            })
            .flat_map(|i: Vec<(i32, i32)>| i.into_iter())
            .collect()
    }
}

pub fn populate_cache<T, C, V>(
    cache: Arc<Mutex<DiffCache<C, usize, V>>>,
    time_index: Arc<Mutex<TimeIndex>>,
    mut iter: impl Iterator<Item = T>,
) where
    T: Visualize<C, V> + std::fmt::Debug,
    V: std::fmt::Debug,
    C: Hash + Eq + std::fmt::Debug,
{
    let mut index: usize = 0;
    if let Some(first) = iter.next() {
        iter.fold(first, |acc, a| {
            let delta = a.delta(&acc).into_iter().filter_map(|c| {
                if let Some(v) = a.get(&c) {
                    return Some((c, index, v));
                }
                None
            });
            // Locks the cache and populate it
            cache.lock().unwrap().append(delta);

            // Locks the TimeIndex and add one to the max index
            time_index.lock().unwrap().add_max();

            index += 1;
            a
        });
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn visualize_str_default_val() {
        let string: String = String::new();
        assert_eq!(string.default_val(), ' ');
    }

    #[test]
    fn visualize_str_delta() {
        // Two identical strings should not have a delta
        let string: String = "abcdefg".into();
        let other_string: String = "abcdefg".into();
        assert_eq!(string.delta(&other_string), vec![]);

        // Same test, with multiple lines
        let string: String = "abc\ndef\ng".into();
        let other_string: String = "abc\ndef\ng".into();
        assert_eq!(string.delta(&other_string), vec![]);

        // Two different strings should have a delta
        let string: String = "abcdefgh".into();
        let other_string: String = "abcdefg".into();
        let delta: Vec<(i32, i32)> = string.delta(&other_string);
        assert_eq!(delta.len(), 1);
        assert_eq!(delta[0], (7, 0));

        let string: String = "a".into();
        let other_string: String = "b".into();
        let delta: Vec<(i32, i32)> = string.delta(&other_string);
        assert_eq!(delta.len(), 1);
        assert_eq!(delta[0], (0, 0));

        // Tests multiple edit in the string
        let string: String = "this is  a test".into();
        let other_string: String = "this was a tast".into();
        let delta: Vec<(i32, i32)> = string.delta(&other_string);
        assert_eq!(delta.len(), 4);
    }
}
