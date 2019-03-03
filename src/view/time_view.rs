use crate::time_index::TimeIndex;
use cursive::direction::Direction;
use cursive::theme::{Color, ColorStyle, ColorType};
use cursive::vec::Vec2;
use cursive::view::View;
use cursive::Printer;
use std::sync::{Arc, Mutex};

/// View that shows the current time index.
/// Stores a shared-reference over a TimeIndex in multiple threads
pub struct TimeView {
    /// Keeps track of the time index
    time_index: Arc<Mutex<TimeIndex>>,
    /// Size of the Time widget
    size: Vec2,
}

impl TimeView {
    /// Creates a new instance of the TimeView
    pub fn new(time_index: Arc<Mutex<TimeIndex>>) -> Self {
        TimeView {
            time_index,
            size: (0, 0).into(),
        }
    }
}

/// Utility function, linearly maps the number x contained in the min range to
/// a y number contained in the max range
fn map(x: usize, in_min: usize, in_max: usize, out_min: usize, out_max: usize) -> usize {
    (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

impl View for TimeView {
    /// Draws the TimeView using a Cursive `Printer`
    fn draw(&self, printer: &Printer) {
        let time_index = self
            .time_index
            .lock()
            .expect("Failed to get lock on TimeIndex");

        printer.print_box((0, 0), self.size, true);

        let str_min = format!("{}", time_index.min);

        let str_max = format!("{}", time_index.max);
        let pos_x_max = self.size.x - str_max.len() - 1;

        // Shows the minimum value
        printer.print((1, 1), &format!("{}", time_index.min));

        // Shows the maximum value
        printer.print((pos_x_max, 1), &str_max);

        // Shows the current value if it is different from the max
        if time_index.current != time_index.max && time_index.current != time_index.min {
            let str_current = format!("{}", time_index.current);
            let target_x_current = map(
                time_index.current,
                time_index.min,
                time_index.max,
                str_min.len() + 2,
                pos_x_max - str_current.len() - 2,
            );
            printer.print((target_x_current + 2, 1), &str_current);
        }

        // Computes the loading bar index
        let string_up_to_current = map(
            time_index.current,
            time_index.min,
            time_index.max,
            1,
            self.size.x - 2,
        );

        // Shows the "loading bar" (how far we are on the time index)
        printer.with_color(
            ColorStyle::new(
                ColorType::Color(Color::Rgb(255, 255, 255)),
                ColorType::Color(Color::Rgb(255, 255, 255)),
            ),
            |p| {
                p.print(
                    (1, 2),
                    &(1..=string_up_to_current).map(|_| '|').collect::<String>(),
                );
            },
        );

        // If we have yet to load something, shows the rest of the loading bar
        if string_up_to_current < self.size.x {
            printer.print(
                (string_up_to_current, 2),
                &(string_up_to_current..=self.size.x - 2)
                    .map(|_| '|')
                    .collect::<String>(),
            );
        }

        // Prints the current time cursor
        printer.with_color(
            ColorStyle::new(
                ColorType::Color(Color::Rgb(0, 0, 0)),
                ColorType::Color(Color::Rgb(0, 0, 0)),
            ),
            |p| {
                p.print((string_up_to_current, 2), "|");
            },
        );
    }

    /// Takes the focus for the TimeView
    fn take_focus(&mut self, _: Direction) -> bool {
        false
    }

    /// Called when the size of the widget has been decided
    fn layout(&mut self, size: Vec2) {
        self.size = size;
    }

    /// Called when Cursive wants to add a constraint to the TimeView's layout
    fn required_size(&mut self, max: Vec2) -> Vec2 {
        (max.x, 4usize).into()
    }
}
