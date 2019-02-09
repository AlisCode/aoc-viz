use crate::diff_cache::DiffCache;
use crate::view::frame::FrameView;
use crate::visualize::{populate_cache, Visualize};
use cursive::Cursive;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

pub struct AocVizApp<F, T, V> {
    cursive: Cursive,
    cache: Arc<Mutex<DiffCache<(i32, i32), usize, char>>>,
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
            cache: Arc::new(Mutex::new(DiffCache::new('.'))),
            fn_user,
            _phantom_t: PhantomData,
            _phantom_v: PhantomData,
        }
    }

    /// Launches the viz application
    pub fn launch(&mut self) {
        // Populates the view
        self.cursive.add_layer(FrameView::new(self.cache.clone()));

        // Sets the various option callbacks
        self.cursive.add_global_callback('q', |c| c.quit());

        // Populates the cache by running the user's fn with a correct input
        CachePopulator::new(self.cache.clone(), Box::new(self.fn_user.clone())).launch();

        // Runs the cursive app
        self.cursive.run();
    }
}

struct CachePopulator<F> {
    cache: Arc<Mutex<DiffCache<(i32, i32), usize, char>>>,
    fn_user: Box<F>,
}

impl<F, T, V> CachePopulator<F>
where
    T: Iterator<Item = V>,
    V: Visualize<(i32, i32), char> + std::fmt::Debug,
    F: Fn(String) -> T + Send + Sync + 'static,
{
    pub fn new(cache: Arc<Mutex<DiffCache<(i32, i32), usize, char>>>, fn_user: Box<F>) -> Self {
        CachePopulator { cache, fn_user }
    }

    pub fn launch(self) {
        std::thread::spawn(|| {
            let input: String = "abc".into();
            populate_cache(self.cache, (self.fn_user)(input));
        });
    }
}
