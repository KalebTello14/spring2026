use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::Write;

const TOTAL_TASKS: usize = 1000;
const TASK_INTERVAL_MS: u64 = 20;
const TASK_DURATION_MS: u64 = 200;
const TICK_MS: u64 = 10;
const MAX_WORKERS: usize = 8;
const MAX_CPU: u8 = 100;

#[derive(Clone, Copy, Debug, PartialEq)]
enum TaskType {
    Io,
    Cpu,
}

#[derive(Clone, Debug)]
struct Task {
    id: usize,
    task_type: TaskType,
    cpu_cost: u8,
    created_at: u64,
    started_at: Option<u64>,
    finished_at: Option<u64>,
}

impl Task {
    fn new(id: usize, task_type: TaskType, created_at: u64) -> Self {
        let cpu_cost = match task_type {
            TaskType::Io => 10,
            TaskType::Cpu => 35,
        };

        Self {
            id,
            task_type,
            cpu_cost,
            created_at,
            started_at: None,
            finished_at: None,
        }
    }

    fn wait_time(&self) -> u64 {
        self.started_at.unwrap() - self.created_at
    }

    fn turnaround_time(&self) -> u64 {
        self.finished_at.unwrap() - self.created_at
    }
}

#[derive(Clone, Debug)]
struct Worker {
    busy: bool,
    task: Option<Task>,
    finish_time: u64,
}

impl Worker {
    fn new() -> Self {
        Self {
            busy: false,
            task: None,
            finish_time: 0,
        }
    }
}

#[derive(Debug)]
struct LogEntry {
    time: u64,
    cpu_usage: u8,
    active_workers: usize,
    queue_len: usize,
}

#[derive(Clone, Copy, Debug)]
enum Mode {
    Fifo,
    Optimized,
}

struct Lcg {
    state: u64,
}

impl Lcg {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u32(&mut self) -> u32 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1);
        (self.state >> 32) as u32
    }

    fn next_percent(&mut self) -> u32 {
        self.next_u32() % 100
    }
}

fn generate_task(rng: &mut Lcg, id: usize, time: u64) -> Task {
    if rng.next_percent() < 70 {
        Task::new(id, TaskType::Io, time)
    } else {
        Task::new(id, TaskType::Cpu, time)
    }
}

fn active_workers(workers: &[Worker]) -> usize {
    workers.iter().filter(|w| w.busy).count()
}

fn can_schedule(task: &Task, current_cpu: u8, workers_busy: usize) -> bool {
    workers_busy < MAX_WORKERS && current_cpu + task.cpu_cost <= MAX_CPU
}

fn schedule_fifo(
    queue: &mut VecDeque<Task>,
    workers: &mut [Worker],
    cpu_usage: &mut u8,
    time: u64,
) {
    for i in 0..workers.len() {
        if workers[i].busy {
            continue;
        }

        let next_task = match queue.front() {
            Some(task) => task.clone(),
            None => break,
        };

        let busy_count = active_workers(workers);

        if can_schedule(&next_task, *cpu_usage, busy_count) {
            let mut task = queue.pop_front().unwrap();
            task.started_at = Some(time);

            *cpu_usage += task.cpu_cost;
            workers[i].busy = true;
            workers[i].task = Some(task);
            workers[i].finish_time = time + TASK_DURATION_MS;
        }
    }
}

fn schedule_optimized(
    queue: &mut VecDeque<Task>,
    workers: &mut [Worker],
    cpu_usage: &mut u8,
    time: u64,
) {
    for i in 0..workers.len() {
        if workers[i].busy {
            continue;
        }

        if queue.is_empty() {
            break;
        }

        let chosen_index = if *cpu_usage >= 70 {
            queue.iter().position(|t| t.task_type == TaskType::Io).unwrap_or(0)
        } else {
            queue.iter().position(|t| t.task_type == TaskType::Cpu).unwrap_or(0)
        };

        let mut task = queue.remove(chosen_index).unwrap();
        let busy_count = active_workers(workers);

        if can_schedule(&task, *cpu_usage, busy_count) {
            task.started_at = Some(time);

            *cpu_usage += task.cpu_cost;
            workers[i].busy = true;
            workers[i].task = Some(task);
            workers[i].finish_time = time + TASK_DURATION_MS;
        } else {
            queue.insert(chosen_index.min(queue.len()), task);
        }
    }
}

fn write_monitor_csv(filename: &str, log: &[LogEntry]) {
    let mut file = File::create(filename).unwrap();
    writeln!(file, "time,cpu_usage,active_workers,queue_len").unwrap();

    for entry in log {
        writeln!(
            file,
            "{},{},{},{}",
            entry.time, entry.cpu_usage, entry.active_workers, entry.queue_len
        )
        .unwrap();
    }
}

fn run_simulation(mode: Mode) {
    let mut rng = Lcg::new(42);
    let mut queue: VecDeque<Task> = VecDeque::new();
    let mut workers = vec![Worker::new(); MAX_WORKERS];
    let mut log: Vec<LogEntry> = Vec::new();
    let mut completed: Vec<Task> = Vec::new();

    let mut tasks_created = 0usize;
    let mut tasks_completed = 0usize;
    let mut cpu_usage: u8 = 0;
    let mut time: u64 = 0;

    while tasks_completed < TOTAL_TASKS {
        if time % TASK_INTERVAL_MS == 0 && tasks_created < TOTAL_TASKS {
            queue.push_back(generate_task(&mut rng, tasks_created, time));
            tasks_created += 1;
        }

        for worker in workers.iter_mut() {
            if worker.busy && time >= worker.finish_time {
                let mut finished = worker.task.take().unwrap();
                finished.finished_at = Some(time);

                cpu_usage -= finished.cpu_cost;
                worker.busy = false;
                tasks_completed += 1;
                completed.push(finished);
            }
        }

        match mode {
            Mode::Fifo => schedule_fifo(&mut queue, &mut workers, &mut cpu_usage, time),
            Mode::Optimized => schedule_optimized(&mut queue, &mut workers, &mut cpu_usage, time),
        }

        log.push(LogEntry {
            time,
            cpu_usage,
            active_workers: active_workers(&workers),
            queue_len: queue.len(),
        });

        time += TICK_MS;
    }

    let avg_cpu =
        log.iter().map(|e| e.cpu_usage as u64).sum::<u64>() as f64 / log.len() as f64;

    let avg_workers =
        log.iter().map(|e| e.active_workers as u64).sum::<u64>() as f64 / log.len() as f64;

    let io_tasks: Vec<&Task> = completed.iter().filter(|t| t.task_type == TaskType::Io).collect();
    let cpu_tasks: Vec<&Task> = completed.iter().filter(|t| t.task_type == TaskType::Cpu).collect();

    let avg_wait =
        completed.iter().map(|t| t.wait_time()).sum::<u64>() as f64 / completed.len() as f64;

    let avg_wait_io =
        io_tasks.iter().map(|t| t.wait_time()).sum::<u64>() as f64 / io_tasks.len() as f64;

    let avg_wait_cpu =
        cpu_tasks.iter().map(|t| t.wait_time()).sum::<u64>() as f64 / cpu_tasks.len() as f64;

    let avg_turnaround =
        completed.iter().map(|t| t.turnaround_time()).sum::<u64>() as f64 / completed.len() as f64;

    let max_wait_task = completed.iter().max_by_key(|t| t.wait_time()).unwrap();

    let csv_name = match mode {
        Mode::Fifo => "fifo_monitor.csv",
        Mode::Optimized => "optimized_monitor.csv",
    };

    write_monitor_csv(csv_name, &log);

    println!(
        "=== {} simulation ===",
        match mode {
            Mode::Fifo => "FIFO",
            Mode::Optimized => "Optimized",
        }
    );
    println!("1000 tasks, 70% IO / 30% CPU, 8 workers, cap 100%\n");
    println!("-- results --");
    println!("total runtime         : {}ms", time);
    println!("makespan              : {}ms", time);
    println!(
        "tasks completed       : {} (IO={}, CPU={})",
        completed.len(),
        io_tasks.len(),
        cpu_tasks.len()
    );
    println!("avg wait time         : {:.2}ms", avg_wait);

    if let Mode::Optimized = mode {
        println!("avg wait (IO Only)    : {:.2}ms", avg_wait_io);
        println!("avg wait (CPU Only)   : {:.2}ms", avg_wait_cpu);
    }

    println!("avg turnaround time   : {:.2}ms", avg_turnaround);
    println!(
        "max wait time         : {}ms (task #{})",
        max_wait_task.wait_time(),
        max_wait_task.id
    );
    println!("avg CPU usage         : {:.2}%", avg_cpu);
    println!("avg workers active    : {:.2}", avg_workers);
    println!("monitor samples       : {}", log.len());
    println!("monitor csv           : {}", csv_name);
}
fn main() {
    let args: Vec<String> = env::args().collect();

    let mode = match args.get(1).map(|s| s.as_str()) {
        Some("fifo") => Mode::Fifo,
        Some("optimized") => Mode::Optimized,
        _ => {
            eprintln!("Usage:");
            eprintln!("  cargo run -- fifo");
            eprintln!("  cargo run -- optimized");
            return;
        }
    };

    run_simulation(mode);
}
