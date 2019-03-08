use aoc_viz::aoc_viz_app::AocVizApp;

#[derive(Debug)]
pub struct Day1Viz {
    curr_val: i32, 
    sum: i32,
}

impl ToString for Day1Viz {
    fn to_string(&self) -> String {
        if self.curr_val == 0 && self.sum == 0 { return format!(""); }
        format!("current: {}\nsum: {}", self.curr_val, self.sum)
    }
}

impl Default for Day1Viz {
    fn default() -> Self {
        Day1Viz {
            curr_val: 0,
            sum: 0,
        }
    }
}

fn compute(_input: String) -> impl Iterator<Item= Day1Viz> {
    let input = include_str!("input/2018_day1.txt");

    let mut sum = 0;
    input
    .lines()
    .filter_map(|a| a.parse::<i32>().ok())
    .map(move |x| {
        sum += x;
        Day1Viz {
            curr_val: x,
            sum: sum,
        }
    })

}

fn main() {

    let mut app = AocVizApp::new(compute);
    app.launch();

}