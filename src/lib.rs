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
