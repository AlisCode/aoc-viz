use aoc_viz::aoc_viz_app::AocVizApp;

fn test_aoc_viz(_input: String) -> impl Iterator<Item = &'static str> {
    vec![
        "Hello",
        "world!",
        "This",
        "is",
        "my",
        "cargo-aoc",
        "app"
    ].into_iter()
}

fn main() {
    let mut app = AocVizApp::new(test_aoc_viz);
    app.launch();
}
