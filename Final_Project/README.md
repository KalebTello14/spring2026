# Concurrent Task Dispatcher (Rust)

## Overview

This project implements a simulated concurrent task dispatcher in Rust. The system models a task generator, dispatcher queue, worker pool, and monitor to evaluate how scheduling policy affects throughput, CPU utilization, worker utilization, and task responsiveness under constrained resources.

The simulation processes 1000 tasks generated at fixed 20ms intervals. Each task is classified as either an IO task or a CPU task using a deterministic 70/30 distribution. IO tasks simulate lighter work and consume 10% CPU for 200ms, while CPU tasks simulate heavier work and consume 35% CPU for 200ms.

The dispatcher enforces two system-wide constraints:
- a maximum of 8 concurrent workers
- a strict CPU usage cap of 100%

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

Each run also generates a monitor CSV file:
- ```fifo_monitor.csv ```
- ```optimized_monitor.csv ```

These files contain time-series monitor data for post-trun  analysis.

---

## Simulation Design

The system simulates four major components:

* task generator
* dispatcher queue
* worker pool
* monitor

### Task Generator

The task generator produces 1000 tasks at fixed 20ms intervals. Task generation is deterministic and uses a fixed pseudo-random seed so that both schedulers process the same workload.

### Dispatcher Queue

Generated tasks enter a central queue implemented with ```VecDeque<Task>```. The queue acts as the intermediary between task creation and task execution.

### Worker Pool

The worker pool contains up to 8 workers. Each worker may process one task at a time. A task is dispatched only when:

* a worker is available
* CPU usage remains within the 100% cap

### Monitor

The monitor records system state every 10ms and stores:

* simulation time
* CPU usage
* active workers
* queue length

This data is stored during execution and exported as CSV after the run completes.

---

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

The FIFO scheduler processes tasks in strict arrival order. It always evaluates the front of the queue first and dispatches tasks when worker and CPU constraints allow.

This policy is simple and predictable, but it may underutilize CPU resources when expensive CPU tasks block smaller IO tasks behind them.

### Optimized Scheduler

The optimized scheduler uses CPU-aware task selection to improve resource utilization.

* If CPU usage is high, prioritize IO tasks
* If CPU usage is low, prioritize CPU-heavy tasks

This improves CPU packing efficiency and reduces idle CPU headroom, at the cost of relaxing strict arrival ordering.

---

## Metrics Collected

The simulation records both aggregate metrics and task-level metrics.

### Aggregate Metrics

* total runtime
* makespan
* average CPU usage
* average worker utilization
* monitor sample count

### Task-Level Metrics

* tasks completed (total, IO, CPU)
* average wait time
* average wait time for IO tasks
* average wait time for CPU tasks
* average turnaround time
* maximum wait time

These metrics allow the schedulers to be evaluated on both throughput and responsiveness.

---

## Experiment Results

Both schedulers were tested using the same deterministic workload of 1000 tasks generated at 20ms intervals with a fixed 70/30 IO-to-CPU distribution.

### FIFO Scheduler

* Total Runtime: 39430ms
* Makespan: 39430ms
* Tasks Completed: 1000
* Average CPU Usage: 89.65%
* Average Worker Usage: 5.07

### Optimized Scheduler

* Total Runtime: 36650ms
* Makespan: 36650ms
* Tasks Completed: 1000
* Average CPU Usage: 96.45%
* Average Worker Usage: 5.46
* Average Wait Time: 9471.28ms
* Average Wait (IO): 11178.79ms
* Average Wait (CPU): 5616.87ms
* Average Turnaround Time: 9671.28ms
* Max Wait Time: 17240ms (task #685)

### Comparison

The optimized scheduler outperformed FIFO in every major performance category.

* CPU utilization improved from 89.65% to 96.45%
* Worker utilization improved from 5.07 to 5.46
* Total runtime decreased from 39430ms to 36650ms

This reduced total runtime by 2780ms (approximately 7.1%) while also improving overall CPU efficiency and task throughput.

---

## Repository Contents

* `src/main.rs` — simulation implementation
* `Cargo.toml` — Rust project configuration
* `Cargo.lock` — dependency lockfile
* `fifo_output.txt` — FIFO experiment results
* `optimized_output.txt` — optimized experiment results
* `fifo_monitor.csv` — FIFO monitor log
* `optimized_monitor.csv` — optimized monitor log
* `design_report.pdf` — final design report
* `README.md` — project documentation

---

## Tool Use Disclosure

### Tools Used

* ChatGPT (used for simulation structuring, scheduler refinement, and debugging support)

### Advice Accepted

* Using a deterministic time-step simulation instead of real OS threads improved correctness, simplified synchronization, and made scheduling behavior easier to analyze and reproduce.

### Advice Rejected

* Real worker threads were considered initially, but rejected because they introduced unnecessary complexity, nondeterministic timing, and made fair scheduler comparison more difficult.
