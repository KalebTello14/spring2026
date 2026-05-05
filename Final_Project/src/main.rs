use std::collections::VecDeque;
use std::env;

const TOTAL_TASKS: usize = 1000;
const TASK_INSTERVAL_MS: u64 = 20;
const TASK_DURATION_MS: u64 = 200;
const TICK_MS: u64 = 10;
const MAX_WORKERS: usize = 8;
const MAX_CPU: u8 = 100;

#[derive(Clone, Copy, Debug, PartialEq)]
enum TaskType{
    Io,
    Cpu,
}

#[derive(Clone, Copy, Debug)]
struct Task{
    task_type: TaskType,
    cpu_cost: u8,
}

impl Task{
    fn new(task_type: TaskType) -> Self{
        match task_type {
            TaskType::Io => Self{
                task_type,
                cpu_cost: 10,
            },
            TaskType::Cpu => Self{
                task_type,
                cpu_cost: 35,
            },
        }
    }
}

#[derive(Clone, Debug)]
struct Worker {
    busy: bool,
    task: Option<Task>,
    finish_time: u64,
}

impl Worker{
    fn new() -> Self{
        Self{
            busy: false,
            task: None,
            finish_time: 0,
        }
    }
}

#[derive(Debug)]
struct LogEntry {
    _time: u64,
    cpu_usage: u8,
    active_workers: usize,
}

#[derive(Clone, Copy, Debug)]
enum Mode{
    Fifo,
    Optimized,
}

struct Lcg {
    state: u64,
}

impl Lcg{
    fn new(seed: u64) -> Self{
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

fn generate_task(rng: &mut Lcg) -> Task {
    // 70% IO, 30% CPU
    if rng.next_percent() < 70 {
        Task::new(TaskType::Io)
    } else {
        Task::new(TaskType::Cpu)
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
            Some(task) => *task,
            None => break,
        };

        let busy_count = workers.iter().filter(|w| w.busy).count();

        if can_schedule(&next_task, *cpu_usage, busy_count) {
            let task = queue.pop_front().unwrap();
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

        let task = queue.remove(chosen_index).unwrap();
        let busy_count = workers.iter().filter(|w| w.busy).count();

        if can_schedule(&task, *cpu_usage, busy_count) {
            *cpu_usage += task.cpu_cost;
            workers[i].busy = true;
            workers[i].task = Some(task);
            workers[i].finish_time = time + TASK_DURATION_MS;
        } else {
            queue.insert(chosen_index.min(queue.len()), task);
        }
    }
}

fn run_simulation(mode: Mode) {
    let mut rng = Lcg::new(42);
    let mut queue: VecDeque<Task> = VecDeque::new();
    let mut workers = vec![Worker::new(); MAX_WORKERS];
    let mut log: Vec<LogEntry> = Vec::new();

    let mut tasks_created = 0usize;
    let mut tasks_completed = 0usize;
    let mut cpu_usage: u8 = 0;
    let mut time: u64 = 0;

    while tasks_completed < TOTAL_TASKS {
        // this generates a task ever 20ms
        if time % TASK_INSTERVAL_MS == 0 && tasks_created < TOTAL_TASKS{
            queue.push_back(generate_task(&mut rng));
            tasks_created += 1;
        }

        //free's up completed workers 
        for worker in workers.iter_mut() {
            if worker.busy && time >= worker.finish_time {
                let finished = worker.task.take().unwrap();
                cpu_usage -= finished.cpu_cost;
                worker.busy = false;
                tasks_completed += 1;
            }
        }

        //schedules the tasks
        match mode{
            Mode::Fifo => schedule_fifo(&mut queue, &mut workers, &mut cpu_usage, time),
            Mode::Optimized => schedule_optimized(&mut queue, &mut workers, &mut cpu_usage, time),
        }

        //logs the metrics
        log.push(LogEntry {
            _time: time,
            cpu_usage,
            active_workers: active_workers(&workers),
        });

        time += TICK_MS;

    }
    let avg_cpu: f64 = 
        log.iter().map(|e| e.cpu_usage as u64).sum::<u64>() as f64 / log.len() as f64;

    let avg_workers: f64 = 
        log.iter().map(|e| e.active_workers as u64).sum::<u64>() as f64 / log.len() as f64;

    println!("=== {:?} Scheduler Results ===", mode);
    println!("Average CPU Usage: {:.2}%", avg_cpu);
    println!("Average Worker Usage: {:.2}", avg_workers);
    println!("Total Runtime: {}ms", time);
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
