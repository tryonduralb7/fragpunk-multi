pub struct Speedhack {
    multiplier: f32,
}

impl Speedhack {
    pub fn new() -> Self {
        Speedhack { multiplier: 1.0 }
    }

    pub fn set_multiplier(&mut self, multiplier: f32) {
        self.multiplier = multiplier;
    }

    pub fn get_multiplier(&self) -> f32 {
        self.multiplier
    }
}