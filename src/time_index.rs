pub struct TimeIndex {
    /// Minimum time index available
    pub min: usize,
    /// Maximum time index available
    pub max: usize,
    /// Current index
    pub current: usize,
}

impl TimeIndex {
    pub fn new(min: usize, max: usize, current: usize) -> Self {
        TimeIndex { min, max, current }
    }

    pub fn add_max(&mut self) {
        self.max += 1;
    }

    pub fn set_current(&mut self, new: usize) {
        self.current = new;
    }

    pub fn forward(&mut self) {
        self.current = match self.current.checked_add(1) {
            Some(new) => new,
            _ => self.current,
        }
    }

    pub fn backward(&mut self) {
        self.current = match self.current.checked_sub(1) {
            Some(new) => new,
            _ => self.current,
        }
    }
}
