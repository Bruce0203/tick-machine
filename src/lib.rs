use std::{
    fmt::Debug,
    time::{Duration, Instant},
};

pub trait Tick {
    fn try_tick(&mut self);
}

pub struct TickState {
    start: Instant,
    last_tick: Duration,
    tick: Duration,
}

impl Debug for TickState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TickMachine")
            .field("tick", &self.tick)
            .finish()
    }
}

impl TickState {
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

pub struct TickMachine<F> {
    f: F,
    tick: TickState,
}

impl<F: Fn()> TickMachine<F> {
    pub fn new(tick: TickState, f: F) -> TickMachine<F> {
        TickMachine { f, tick }
    }
}

impl<F: Fn()> Tick for TickMachine<F> {
    fn try_tick(&mut self) {
        self.tick.tick(|| (self.f)());
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use crate::{Tick, TickMachine, TickState};

    #[test]
    fn test() {
        let tick = Duration::from_millis(50);
        let tick = TickState::new(tick);
        let mut machine = TickMachine::new(tick, || {});
        machine.try_tick();
    }
}
