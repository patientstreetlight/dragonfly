use std::time::Duration;

use tracing::{debug, error, info, trace, warn};

struct Manager {
    kind: String,
    started: bool,
}

impl Manager {
    fn set_kind(&mut self, kind: String) {
        self.kind = kind;
    }

    fn kind(&self) -> &str {
        &self.kind
    }

    fn start_up() {}

    fn shut_down() {}

    fn started(&self) -> bool {
        self.started
    }
}

pub struct Clock {
    previous_time: std::time::Instant,
}

impl Clock {
    pub fn new() -> Self {
        let previous_time = std::time::Instant::now();
        Self { previous_time }
    }

    /// Time since previous time. Resets previous time.
    pub fn delta(&mut self) -> Duration {
        let now = std::time::Instant::now();
        let elapsed = now - self.previous_time;
        self.previous_time = now;
        elapsed
    }

    /// Time since previous time
    pub fn split(&self) -> Duration {
        self.previous_time.elapsed()
    }

    pub fn reset(&mut self) {
        self.previous_time = std::time::Instant::now();
    }
}

pub fn test_logging() {
    error!("ERROR");
    warn!("WARN");
    info!("INFO");
    debug!("DEBUG");
    trace!("TRACE");
}
