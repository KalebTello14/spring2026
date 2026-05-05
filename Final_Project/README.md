# Concurrent Task Dispatcher (Rust)

## Overview

This project simulates a concurrent task dispatcher in Rust. The system models a main task producer, a dispatcher queue, a worker pool, and a monitor/logger. The dispatcher processes 1000 incoming tasks while enforcing both worker availability and CPU resource constraints.

The simulation compares two scheduling strategies:

1. **FIFO Scheduler** — dispatches tasks in arrival order
2. **Optimized Scheduler** — dynamically prioritizes tasks based on CPU usage

The goal of the project is to evaluate how scheduling policy affects CPU utilization, worker utilization, and total runtime.

---

## Build Instructions

```bash
cargo build
```

---

## Run Instructions

Run FIFO scheduler (This will run in the terminal):

```bash
cargo run -- fifo
```

Run optimized scheduler (This will run in the terminal):

```bash
cargo run -- optimized
```

To generate clean output files for submission (This will save in the txt files.):

```bash
cargo run --release --quiet -- fifo > fifo_output.txt
cargo run --release --quiet -- optimized > optimized_output.txt
```

---

## Simulation Design

The system simulates:

* A main thread generating 1000 tasks at 20ms intervals
* A dispatcher queue that receives and schedules tasks
* A worker pool with a maximum of 8 workers
* A CPU limit capped at 100%
* A monitor that records metrics every 10ms

### Task Types

Two task types are simulated:

* **IO Task**

  * 10% CPU usage
  * 200ms execution time

* **CPU Task**

  * 35% CPU usage
  * 200ms execution time

Task generation follows a 70/30 distribution:

* 70% IO tasks
* 30% CPU tasks

---

## Scheduling Policies

### FIFO Scheduler

The FIFO scheduler processes tasks in strict arrival order. This approach is simple and predictable, but it can leave CPU capacity underutilized when high-cost CPU tasks block smaller IO tasks in the queue.

### Optimized Scheduler

The optimized scheduler improves utilization by making CPU-aware scheduling decisions:

* If CPU usage is high, prioritize IO tasks
* If CPU usage is low, prioritize CPU-heavy tasks

This allows the dispatcher to better pack tasks within the available CPU budget and improve throughput.

---

## Metrics Collected

The monitor records system metrics every 10ms and reports:

* Average CPU usage
* Average worker utilization
* Total runtime

---

## Experiment Results

### FIFO Scheduler

* Average CPU Usage: **89.65%**
* Average Worker Usage: **5.07**
* Total Runtime: **39430ms**

### Optimized Scheduler

* Average CPU Usage: **96.45%**
* Average Worker Usage: **5.46**
* Total Runtime: **36650ms**

### Comparison

The optimized scheduler outperformed FIFO in all measured categories.

* CPU utilization improved from **89.65%** to **96.45%**
* Worker utilization improved from **5.07** to **5.46**
* Total runtime decreased from **39430ms** to **36650ms**

This reduced total runtime by **2780ms** (approximately **7.1%**) while also improving overall CPU efficiency.

---

## Repository Contents

* `src/main.rs` — simulation implementation
* `Cargo.toml` — Rust project configuration
* `Cargo.lock` — dependency lockfile
* `fifo_output.txt` — FIFO experiment results
* `optimized_output.txt` — optimized experiment results
* `design_report.pdf` — final design report

---

## Tool Use Disclosure

### Tools Used

* ChatGPT (used for simulation structuring, scheduler refinement, and debugging support)

### Advice Accepted

* Using a time-step simulation instead of real OS threads improved correctness, simplified synchronization, and made scheduling behavior easier to analyze.

### Advice Rejected

* Real worker threads were considered initially, but rejected because they introduced unnecessary complexity and reduced control over deterministic simulation timing.
