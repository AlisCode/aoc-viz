use crate::diff_cache::DiffCache;
use crate::time_index::TimeIndex;
use crate::view::frame::FrameView;
use crate::view::time_view::TimeView;
use crate::visualize::{populate_cache, Visualize};
use cursive::direction::Orientation;
use cursive::view::{Boxable, Identifiable};
use cursive::views::{Dialog, EditView, LinearLayout};
use cursive::Cursive;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

pub struct AocVizApp<F, T, V> {
    cursive: Cursive,
    cache: Arc<Mutex<DiffCache<(i32, i32), usize, char>>>,
    time_index: Arc<Mutex<TimeIndex>>,
    fn_user: F,
    _phantom_t: PhantomData<T>,
    _phantom_v: PhantomData<V>,
}

impl<F, T, V> AocVizApp<F, T, V>
where
    F: Fn(String) -> T + Clone + Send + Sync + 'static,
    T: Iterator<Item = V>,
    V: Visualize<(i32, i32), char> + std::fmt::Debug,
{
    /// Creates an instance of an AocVizApp
    pub fn new(fn_user: F) -> Self {
        // Creates the cursive
        let mut cursive = Cursive::default();
        cursive
            .load_theme_file("assets/default_theme.toml")
            .expect("Failed to load theme");

        AocVizApp {
            cursive,
            cache: Arc::new(Mutex::new(DiffCache::new(' '))),
            time_index: Arc::new(Mutex::new(TimeIndex::new(0, 0, 0))),
            fn_user,
            _phantom_t: PhantomData,
            _phantom_v: PhantomData,
        }
    }

    /// Launches the viz application
    pub fn launch(&mut self) {
        // Populates the view
        let mut layout = LinearLayout::new(Orientation::Vertical);
        layout.add_child(FrameView::new(self.cache.clone(), self.time_index.clone()));
        layout.add_child(TimeView::new(self.time_index.clone()));

        self.cursive.add_layer(layout);

        // Sets the various option callbacks
        self.cursive.add_global_callback('q', |c| c.quit());
        self.cursive.add_global_callback('g', |c| {
            c.add_layer(
                Dialog::new()
                    .title("Goto")
                    .padding((1, 1, 1, 0))
                    .content(
                        EditView::new()
                            // Call `show_popup` when the user presses `Enter`
                            .on_submit(|c, response| {
                                eprintln!("ok");
                            })
                            // Give the `EditView` a name so we can refer to it later.
                            .with_id("name")
                            // Wrap this in a `BoxView` with a fixed width.
                            // Do this _after_ `with_id` or the name will point to the
                            // `BoxView` instead of `EditView`!
                            .fixed_width(20),
                    )
                    .button("Go", |c| c.quit()),
            )
        });

        // Populates the cache by running the user's fn with a correct input
        CachePopulator::new(
            self.cache.clone(),
            Box::new(self.fn_user.clone()),
            self.time_index.clone(),
        )
        .launch();

        // Runs the cursive app
        self.cursive.run();
    }
}

struct CachePopulator<F> {
    cache: Arc<Mutex<DiffCache<(i32, i32), usize, char>>>,
    fn_user: Box<F>,
    time_index: Arc<Mutex<TimeIndex>>,
}

impl<F, T, V> CachePopulator<F>
where
    T: Iterator<Item = V>,
    V: Visualize<(i32, i32), char> + std::fmt::Debug,
    F: Fn(String) -> T + Send + Sync + 'static,
{
    pub fn new(
        cache: Arc<Mutex<DiffCache<(i32, i32), usize, char>>>,
        fn_user: Box<F>,
        time_index: Arc<Mutex<TimeIndex>>,
    ) -> Self {
        CachePopulator {
            cache,
            fn_user,
            time_index,
        }
    }

    pub fn launch(self) {
        std::thread::spawn(|| {
            let input: String = "abc".into();
            populate_cache(self.cache, self.time_index, (self.fn_user)(input));
        });
    }
}
