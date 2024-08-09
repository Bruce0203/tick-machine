use std::time::{Duration, Instant};

pub struct TickMachine {
    start: Instant,
    last_tick: Duration,
    tick: Duration,
}

impl TickMachine {
    pub fn new(tick: Duration) -> Self {
        Self {
            start: Instant::now(),
            last_tick: Duration::ZERO,
            tick,
        }
    }

    pub fn tick<F>(&mut self, f: F)
    where
        F: FnOnce(),
    {
        if self.start.elapsed() - self.last_tick >= self.tick {
            f();
            self.last_tick += self.tick;
        }
    }
}
