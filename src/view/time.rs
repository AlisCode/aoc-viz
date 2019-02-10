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
            max: 1,
            current: 0,
            size: (0, 0).into(),
        }
    }
}

fn map(x: usize, in_min: usize, in_max: usize, out_min: usize, out_max: usize) -> usize {
    (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

impl View for TimeView {
    fn draw(&self, printer: &Printer) {
        printer.print_box((0, 0), self.size, true);

        let str_min = format!("{}", self.min);

        let str_max = format!("{}", self.max);
        let pos_x_max = self.size.x - str_max.len() - 1;

        let str_current = format!("{}", self.current);
        let target_x_current = map(
            self.current,
            self.min,
            self.max,
            str_min.len() + 2,
            pos_x_max - str_current.len() - 2,
        );

        printer.print((1, 1), &format!("{}", self.min));
        printer.print((pos_x_max, 1), &str_max);
        printer.print((target_x_current, 1), &str_current);
        printer.with_color(
            ColorStyle::new(
                ColorType::Color(Color::Rgb(255, 255, 255)),
                ColorType::Color(Color::Rgb(255, 255, 255)),
            ),
            |p| {
                p.print((1, 2), "||||||||");
            },
        );
        printer.with_color(
            ColorStyle::new(
                ColorType::Color(Color::Rgb(0, 0, 0)),
                ColorType::Color(Color::Rgb(0, 0, 0)),
            ),
            |p| {
                p.print((9, 2), "|");
            },
        );
        printer.print((10, 2), "||||||||||");
    }

    /// Called when the size of the widget has been decided
    fn layout(&mut self, size: Vec2) {
        self.size = size;
    }

    fn required_size(&mut self, max: Vec2) -> Vec2 {
        (max.x, 4usize).into()
    }
}
