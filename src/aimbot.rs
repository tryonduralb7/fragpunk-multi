pub struct Aimbot {
    enabled: bool,
}

impl Aimbot {
    pub fn new() -> Self {
        Aimbot { enabled: false }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}