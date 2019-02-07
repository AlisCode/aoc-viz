use crate::diff_cache::DiffCache;
use crate::view::frame::FrameView;
use cursive::Cursive;
use std::sync::{Arc, Mutex};

pub struct AocVizApp {
    cursive: Cursive,
    cache: Arc<Mutex<DiffCache<(i32, i32), usize, char>>>,
}

impl AocVizApp {
    /// Creates an instance of an AocVizApp
    pub fn new() -> Self {
        AocVizApp {
            cursive: Cursive::default(),
            cache: Arc::new(Mutex::new(DiffCache::new('.'))),
        }
    }

    /// Launches the viz application
    pub fn launch(&mut self) {
        // TODO: Sets the theme
        
        // TODO TEST: Populate cache
        //let mut c = self.cache.lock().unwrap();

        // Populates the view
        self.cursive.add_layer(FrameView::new(self.cache.clone()));

        // Sets the various option callbacks
        self.cursive.add_global_callback('q', |c| c.quit());

        // Runs the cursive app
        self.cursive.run();
    }
}
