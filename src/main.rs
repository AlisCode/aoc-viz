use cursive::views::TextView;
use cursive::Cursive;

fn main() {
    let mut crsv = Cursive::default();

    crsv.add_layer(TextView::new("Hello world!"));
    crsv.add_global_callback('q', |c| c.quit());

    crsv.run();
}
