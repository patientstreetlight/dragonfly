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

    fn start_up(&self) {}

    fn shut_down(&self) {}

    fn started(&self) -> bool {
        self.started
    }
}

struct GameManager {
    manager: Manager,
    game_over: bool,
    frame_time: usize,
}

impl GameManager {
    fn start_up(&mut self) {
        self.manager.start_up();
    }

    fn shut_down(&mut self) {
        self.manager.shut_down();
    }

    fn run(&mut self) {}

    fn set_game_over(&mut self, game_over: bool) {
        self.game_over = game_over;
    }

    fn is_game_over(&self) -> bool {
        self.game_over
    }

    fn frame_time(&self) -> usize {
        self.frame_time
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
