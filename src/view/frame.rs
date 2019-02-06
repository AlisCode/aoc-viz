use crate::diff_cache::DiffCache;
use cursive::direction::Direction;
use cursive::event::{Event, EventResult, Key};
use cursive::view::View;
use cursive::{Printer, Vec2};
use std::sync::Arc;

/// Represents a Viewport that cargo-aoc-viz will use
/// to visualize a DiffTree
pub struct FrameView {
    /// Center (X/Y coordinates) of the viewport
    center: (i32, i32),
    /// Size of the viewport
    size: Vec2,
    /// Current time index to draw
    index: usize,
    /// Data source (an atomic ref to the DiffCache that this view is displaying)
    target: Arc<DiffCache<(i32, i32), usize, char>>,
}

impl FrameView {
    /// Creates a new instance of the FrameView
    pub fn new(target: Arc<DiffCache<(i32, i32), usize, char>>) -> Self {
        FrameView {
            center: (0, 0),
            size: Vec2::new(0, 0),
            index: 0,
            target,
        }
    }

    /// Moves the viewport in the given direction
    pub fn move_center(&mut self, x: i32, y: i32) {
        self.center.0 += x;
        self.center.1 += y;
    }
}

impl View for FrameView {
    /// Draws the FrameView using the given Printer
    fn draw(&self, printer: &Printer) {
        // Iterates over each coords on the frame
        let coords = (self.center.0 - (self.size.x as i32 / 2)
            ..self.center.0 + (self.size.x as i32 / 2))
            .flat_map(|x| {
                (self.center.1 - (self.size.y as i32 / 2)..self.center.1 + (self.size.y as i32 / 2))
                    .map(move |y| (x, y))
            });

        // Maps each coord to the view of the DiffCache
        // Displays everything using the given printer
        self.target
            .view(coords.clone(), self.index)
            .zip(coords.map(|c| {
                // Zips the coords normalized at (0,0) top corner
                (
                    c.0 - self.center.0 - self.size.x as i32 / 2,
                    c.1 - self.center.1 - self.size.y as i32 / 2,
                )
            }))
            .for_each(|(v, coord)| printer.print(coord, &v.to_string()))
    }

    /// Handles different input events arriving on the Frame
    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            // Basic keys (frame movement)
            Event::Key(k) if k == Key::Left => {
                self.move_center(-1, 0);
                return EventResult::Consumed(None);
            }
            Event::Key(k) if k == Key::Down => {
                self.move_center(0, -1);
                return EventResult::Consumed(None);
            }
            Event::Key(k) if k == Key::Up => {
                self.move_center(0, 1);
                return EventResult::Consumed(None);
            }
            Event::Key(k) if k == Key::Left => {
                self.move_center(1, 0);
                return EventResult::Consumed(None);
            }
            _ => EventResult::Ignored,
        }
    }

    /// Called once the size of this view has been decided ; Changes the
    /// size to fit accordingly
    fn layout(&mut self, size: Vec2) {
        self.size = size;
    }

    /// Minimum size that we require, given the constraints. Let's just set fullscreen.
    fn required_size(&mut self, constraint: Vec2) -> Vec2 {
        constraint
    }

    /// When we're given focus, just say yes.
    fn take_focus(&mut self, _: Direction) -> bool {
        true
    }
}
