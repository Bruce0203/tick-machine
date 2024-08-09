use std::{hint::black_box, time::Duration};

use tick_machine::{TickState, TickMachine};

#[divan::bench]
fn benchmark_tick() {
    let tick = Duration::from_millis(50);
    let mut tick = TickState::new(tick);
    tick.tick(|| {});
}

#[divan::bench]
fn benchmark_machine() {
    let tick = Duration::from_millis(50);
    let tick = TickState::new(tick);
    let mut machine = TickMachine::new(tick, || {});
    machine.tick();
}

fn main() {
    divan::main()
}
