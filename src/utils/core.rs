/// Represent single core (processor) on which task times are scheduled.
pub struct Core {
    working_time: u128,
}

impl Core {
    /// Creates new Core object with empty timeline (no task scheduled on it).
    pub fn new() -> Self {
        Self { working_time: 0 }
    }

    /// Adds a task to timeline
    pub fn add(&mut self, time: u64) -> &mut Self {
        self.working_time += time as u128;
        self
    }

    /// Returns working time of core (timeline length).
    pub fn working_time(&self) -> u128 {
        self.working_time
    }
}