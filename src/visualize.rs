use itertools::{EitherOrBoth, Itertools};
use std::hash::Hash;

/// Trait allowing cargo-aoc to Visualize an implementor
/// using the display function of the V type
/// on a plane defined by the given C coords type
pub trait Visualize<C: Hash + Eq, V> {
    /// Provides the value at given coordinates.
    /// If this method fails to provide a value, the default value
    /// will be used.
    fn get(&self, coords: C) -> Option<V>;

    /// Provides a list of all the coordinates that changed their value
    /// between the previous state and this one.
    fn delta(&self, previous: &Self) -> Vec<C>;

    /// Provides the default value to be shown if the implementor fails
    /// to provide a value for a specific coord
    fn default_viz(&self) -> V;
}

impl Visualize<(i32, i32), char> for String {
    fn default_viz(&self) -> char {
        ' '
    }

    fn get(&self, coords: (i32, i32)) -> Option<char> {
        match coords {
            (x, y) if x < 0 || y < 0 => None,
            (x, y) => self
                .lines()
                .skip(y as usize)
                .next()
                .and_then(|line| line.chars().skip(x as usize).next()),
        }
    }

    fn delta(&self, previous: &Self) -> Vec<(i32, i32)> {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        self.lines()
            .zip_longest(previous.lines())
            .flat_map(|(either_or_both)| match either_or_both {
                EitherOrBoth::Both(curr, prev) => vec![],
                EitherOrBoth::Left(val) | EitherOrBoth::Right(val) => vec![],
            })
            .collect()
    }
}
