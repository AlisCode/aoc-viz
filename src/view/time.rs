use cursive::direction::Direction;
use cursive::theme::{Color, ColorStyle, ColorType};
use cursive::vec::Vec2;
use cursive::view::View;
use cursive::Printer;

/// Struct that shows the handling of the time
/// in the current application
/// TODO: FIXDOC
pub struct TimeView {
    /// Minimum time index available
    min: usize,
    /// Maximum time index available
    max: usize,
    /// Current index
    current: usize,
    /// Size of the Time widget
    size: Vec2,
}

impl TimeView {
    pub fn new() -> Self {
        TimeView {
            min: 0,
            max: 10,
            current: 9,
            size: (0, 0).into(),
        }
    }
}

fn map(x: usize, in_min: usize, in_max: usize, out_min: usize, out_max: usize) -> usize {
    (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

impl View for TimeView {
    /// Draws the TimeView using a Cursive `Printer`
    fn draw(&self, printer: &Printer) {
        printer.print_box((0, 0), self.size, true);

        let str_min = format!("{}", self.min);

        let str_max = format!("{}", self.max);
        let pos_x_max = self.size.x - str_max.len() - 1;

        // Shows the minimum value
        printer.print((1, 1), &format!("{}", self.min));

        // Shows the maximum value
        printer.print((pos_x_max, 1), &str_max);

        // Shows the current value if it is different from the max
        if self.current != self.max && self.current != self.min {
            let str_current = format!("{}", self.current);
            let target_x_current = map(
                self.current,
                self.min,
                self.max,
                str_min.len() + 2,
                pos_x_max - str_current.len() - 2,
            );
            printer.print((target_x_current + 2, 1), &str_current);
        }

        // Computes the loading bar index
        let string_up_to_current = map(self.current, self.min, self.max, 1, self.size.x - 2);

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
        true
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
