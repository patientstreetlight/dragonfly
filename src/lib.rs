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

pub fn test_logging() {
    error!("ERROR");
    warn!("WARN");
    info!("INFO");
    debug!("DEBUG");
    trace!("TRACE");
}
