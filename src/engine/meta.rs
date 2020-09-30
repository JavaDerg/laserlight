use instant::Instant;
use std::time::Duration;

pub struct EngineMeta {
    last_tick: Instant,
    total_frames: u128,
    delta: Duration,
}

impl EngineMeta {
    pub fn new() -> Self {
        Self {
            last_tick: Instant::now(),
            total_frames: 0,
            delta: Duration::new(0, 0),
        }
    }

    pub fn update(&mut self) {}

    pub fn update_delta(&mut self) {
        let nt = Instant::now();
        self.delta = nt - self.last_tick;
        self.last_tick = nt;
    }

    pub fn render(&mut self) {
        self.total_frames += 1;
    }

    pub fn delta_dur(&self) -> &Duration {
        &self.delta
    }

    pub fn delta_sec(&self) -> f32 {
        self.delta.as_secs_f32()
    }
}
