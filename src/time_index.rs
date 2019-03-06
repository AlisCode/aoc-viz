pub struct TimeIndex {
    /// Minimum time index available
    pub min: usize,
    /// Maximum time index available
    pub max: usize,
    /// Current index
    pub current: usize,
}

impl TimeIndex {
    /// Creates a new instance of the TimeIndex
    pub fn new(min: usize, max: usize, current: usize) -> Self {
        TimeIndex { min, max, current }
    }

    /// Adds a maximum index
    pub fn add_max(&mut self) {
        self.max += 1;
    }

    /// Sets the current time frame, checking the min and max bounds.
    /// Returns:
    /// * true if the operation was successful,
    /// * false otherwise
    pub fn set_current(&mut self, new: usize) -> bool {
        if self.max < new || self.min > new {
            return false;
        }
        self.current = new;
        true
    }

    /// Moves the current time frame forward
    pub fn forward(&mut self) {
        self.current = match self.current.checked_add(1) {
            Some(new) if new <= self.max => new,
            _ => self.current,
        }
    }

    /// Moves the current time frame backward
    pub fn backward(&mut self) {
        self.current = match self.current.checked_sub(1) {
            Some(new) if new >= self.min => new,
            _ => self.current,
        }
    }
}
