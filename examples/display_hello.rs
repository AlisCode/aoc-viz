use aoc_viz::aoc_viz_app::AocVizApp;

fn test_aoc_viz(_input: String) -> impl Iterator<Item = String> {
    (0..10).map(|x| format!("Hello world ! {}", x))
}

fn main() {
    let mut app = AocVizApp::new(test_aoc_viz);
    app.launch();
}
