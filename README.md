# example
```rust 
let mut tick_machine = TickMachine::new(Duration::from_millis(50));
loop {
    tick_machine.tick(|| {
        //tick
    });
}

```
