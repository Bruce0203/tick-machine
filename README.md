# example
```rust 
let tick = Duration::from_millis(50);
let mut tick = TickState::new(tick);
loop {
    tick.try_tick(|| {
        println!("tick per 50ms");
    });
}

```
