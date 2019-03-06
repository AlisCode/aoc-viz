use crate::diff_cache::DiffCache;
use crate::time_index::TimeIndex;
use cursive::direction::Direction;
use cursive::event::{Event, EventResult, Key};
use cursive::view::View;
use cursive::{Printer, Vec2};
use std::sync::{Arc, Mutex};

/// Represents a Viewport that cargo-aoc-viz will use
/// to visualize a DiffTree
pub struct FrameView {
    /// Center (X/Y coordinates) of the viewport
    origin: (i32, i32),
    /// Size of the viewport
    size: Vec2,
    /// The TimeIndex to use
    time_index: Arc<Mutex<TimeIndex>>,
    /// Data source (an atomic ref to the DiffCache that this view is displaying)
    target: Arc<Mutex<DiffCache<(i32, i32), usize, char>>>,
}

impl FrameView {
    /// Creates a new instance of the FrameView
    pub fn new(
        target: Arc<Mutex<DiffCache<(i32, i32), usize, char>>>,
        time_index: Arc<Mutex<TimeIndex>>,
    ) -> Self {
        FrameView {
            origin: (0, 0),
            size: Vec2::new(0, 0),
            time_index,
            target,
        }
    }

    /// Moves the viewport in the given direction
    pub fn move_center(&mut self, x: i32, y: i32) {
        self.origin.0 += x;
        self.origin.1 += y;
    }

    /// Specifies the new time index to move to
    pub fn move_to_time_index(&mut self, new_index: usize) {
        self.time_index.lock().unwrap().set_current(new_index);
    }

    /// Forwards one time-unit
    /// TODO: remove. Should use move_to_time_index.
    pub fn time_forward(&mut self) {
        self.time_index.lock().unwrap().forward();
    }

    /// Backwards one time-unit
    /// TODO: remove. Should use move_to_time_index.
    pub fn time_backward(&mut self) {
        self.time_index.lock().unwrap().backward();
    }

    /// Generates the logical coordinates of the viewport
    pub fn get_screen_coords(&self) -> impl Iterator<Item = (i32, i32)> {
        let min_x = self.origin.0;
        let max_x = self.origin.0 + self.size.x as i32;
        let min_y = self.origin.1;
        let max_y = self.origin.1 + self.size.y as i32;
        (min_x..max_x).flat_map(move |x| (min_y..max_y).map(move |y| (x, y)))
    }
}

impl View for FrameView {
    /// Draws the FrameView using the given Printer
    /// FIXME: There should be a way to throw the lock when we're done by copying instead of taking
    /// a reference. This would reduce the lock time, but does the cloning overhead make it worth ?
    fn draw(&self, printer: &Printer) {
        // Creates local coordinates
        let local_coords =
            (0..self.size.x).flat_map(move |x| (0..self.size.y).map(move |y| (x, y)));

        let index = { self.time_index.lock().unwrap().current };

        // Maps each coord to the view of the DiffCache
        // Displays everything using the given printer
        self.target
            .lock()
            .unwrap()
            .view(self.get_screen_coords(), index)
            .zip(local_coords)
            .for_each(|(v, coord)| printer.print(coord, &v.to_string()))
    }

    /// Handles different input events arriving on the Frame
    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            // Basic keys (frame movement)
            // Includes HJKL movement
            Event::Key(k) if k == Key::Left => {
                self.move_center(-1, 0);
                return EventResult::Consumed(None);
            }
            Event::Char(c) if c == 'h' => {
                self.move_center(-1, 0);
                return EventResult::Consumed(None);
            }
            Event::Key(k) if k == Key::Down => {
                self.move_center(0, 1);
                return EventResult::Consumed(None);
            }
            Event::Char(c) if c == 'j' => {
                self.move_center(0, 1);
                return EventResult::Consumed(None);
            }
            Event::Key(k) if k == Key::Up => {
                self.move_center(0, -1);
                return EventResult::Consumed(None);
            }
            Event::Char(c) if c == 'k' => {
                self.move_center(0, -1);
                return EventResult::Consumed(None);
            }
            Event::Key(k) if k == Key::Right => {
                self.move_center(1, 0);
                return EventResult::Consumed(None);
            }
            Event::Char(c) if c == 'l' => {
                self.move_center(1, 0);
                return EventResult::Consumed(None);
            }
            // Time handling:
            // * moves forward (n = next)
            Event::Char(c) if c == 'n' => {
                self.time_forward();
                return EventResult::Consumed(None);
            }
            // * backwards (b = back)
            Event::Char(c) if c == 'b' => {
                self.time_backward();
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
        (constraint.x, constraint.y - 4).into()
    }

    /// When we're given focus, just say yes.
    fn take_focus(&mut self, _: Direction) -> bool {
        true
    }
}
