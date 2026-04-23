# Concurrent Task Dispatcher (Rust)

## Build & Run

```bash
cargo build
cargo run --release
```

## Example Runs

Run FIFO scheduler:

```bash
cargo run -- fifo
```

Run optimized scheduler:

```bash
cargo run -- optimized
```

## Design Summary

This project simulates a task dispatcher with:

* A main thread generating 1000 tasks every 20ms
* A worker pool of up to 8 workers
* CPU constraint capped at 100%

Task types:

* IO tasks: 10% CPU, 200ms duration
* CPU tasks: 35% CPU, 200ms duration

The scheduler enforces:

* Worker availability
* CPU usage limits

Two scheduling strategies were implemented:

1. FIFO — simple queue
2. Optimized — prioritizes IO tasks when CPU usage is high

## Metrics Collected

* Average CPU usage
* Average worker utilization
* Total runtime

## Tool Use Disclosure

Tools used:

* ChatGPT (for structuring simulation and debugging logic) [pending change]

Advice accepted:

* Using a time-step simulation instead of real threads improved correctness and simplicity.

Advice rejected:

* Initially considered using real OS threads for workers, but rejected due to unnecessary complexity and lack of control over timing.
