use std::time::Duration;

use tracing::{debug, error, info, trace, warn};

trait Manager {
    fn kind(&self) -> &str;

    fn start_up(&mut self);

    fn shut_down(&mut self);

    fn started(&self) -> bool;
}

struct GameManager {
    started: bool,
    game_over: bool,
    frame_time: Duration,
}

impl GameManager {
    const KIND: &str = "GameManager";

    fn run(&mut self) {
        // game loop
    }

    fn set_game_over(&mut self, game_over: bool) {
        self.game_over = game_over;
    }

    fn is_game_over(&self) -> bool {
        self.game_over
    }

    fn frame_time(&self) -> Duration {
        self.frame_time
    }
}

impl Manager for GameManager {
    fn kind(&self) -> &str {
        Self::KIND
    }

    fn start_up(&mut self) {
        self.started = true;
    }

    fn shut_down(&mut self) {
        self.set_game_over(true);
        self.started = false;
    }

    fn started(&self) -> bool {
        todo!()
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
