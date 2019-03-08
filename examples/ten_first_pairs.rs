use aoc_viz::aoc_viz_app::AocVizApp;

fn test_aoc_viz(_input: String) -> impl Iterator<Item = u32> {
    (1..10).filter(|x| x % 2 == 0)
}

fn main() {
    let mut app = AocVizApp::new(test_aoc_viz);
    app.launch();
}
