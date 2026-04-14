//Assignment 3: Thread Pool Implementation

use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// Message to be sent to the workers
enum Message {
    NewJob(Job),
    Terminate,
}

// Job type
type Job = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers...");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers...");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job!", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

fn main() {
    let pool = ThreadPool::new(4);

    for i in 1..=10 {
        pool.execute(move || {
            println!("Processing task {}", i);
            thread::sleep(std::time::Duration::from_millis(500));
            println!("Completed task {}", i);
        });
    }

    println!("Main thread waiting for tasks to complete...");
}

//Assigntment 4: Producer-Consumer Pattern with Termination Signal
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    const ITEM_COUNT: usize = 20;
    let num_producers = 2;
    let num_consumers = 3;

    // Create channel
    let (tx, rx) = mpsc::channel();

    // Share receiver across consumers
    let rx = Arc::new(Mutex::new(rx));

    let mut producer_handles = vec![];
    let mut consumer_handles = vec![];

    // Create 2 producer threads
    for i in 0..num_producers {
        let tx_clone = tx.clone();
        producer_handles.push(thread::spawn(move || {
            producer(i, tx_clone, ITEM_COUNT / num_producers);
        }));
    }

    // Create 3 consumer threads
    for i in 0..num_consumers {
        let rx_clone = Arc::clone(&rx);
        consumer_handles.push(thread::spawn(move || {
            consumer(i, rx_clone);
        }));
    }

    // Wait for producers to finish
    for handle in producer_handles {
        handle.join().unwrap();
    }

    // Send termination signal (one per consumer)
    for _ in 0..num_consumers {
        tx.send(TERMINATION_SIGNAL).unwrap();
    }

    // Wait for consumers to finish
    for handle in consumer_handles {
        handle.join().unwrap();
    }

    println!("All items have been produced and consumed!");
}

// Producer function
fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    let mut rng = rand::thread_rng();

    for _ in 0..item_count {
        let num = rng.gen_range(1..100);

        println!("Producer {} produced {}", id, num);

        tx.send(num).unwrap();

        thread::sleep(Duration::from_millis(200));
    }

    println!("Producer {} finished", id);
}

// Consumer function
fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    loop {
        let value = rx.lock().unwrap().recv().unwrap();

        if value == TERMINATION_SIGNAL {
            println!("Consumer {} received termination signal", id);
            break;
        }

        println!("Consumer {} processing {}", id, value);

        thread::sleep(Duration::from_millis(300));
    }

    println!("Consumer {} exiting", id);
}