use std::sync::{Arc, Mutex};
use std::{sync::mpsc, thread};

/// A thread pool.
///  
/// The pool manages a fixed number of threads, dispatching work to them and collecting the results.
///  
/// The pool recreates the threads if a thread panics.
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

// Type alias for a trait object that holds the type of closure that the function execute receives
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///  
    /// The size is the number of threads in the pool.
    ///    
    /// # Panics
    ///  
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // create some workers and store them in the vector
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    /// Execute a function in the thread pool.
    ///  
    /// The function will be executed by one of the threads in the pool.
    ///     
    /// # Panics
    ///     
    /// The `execute` function will panic if the receiving end of the channel is disconnected.
    pub fn execute<F>(&self, f: F)
    where
        // `FnOnce` means the closure takes ownership of the captured variables and can be called only once
        // `Send` means the closure can be sent to another thread
        // `'static` means the closure lives as long as the program
        F: FnOnce() + Send + 'static,
    {
        // `Box` is a smart pointer that allows us to store data on the heap
        // while keeping the same size as a pointer
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

/// A worker in the thread pool.
///
/// Each worker has an id and a thread.
/// The id is the worker's id and the thread is the thread the worker is running on.
///  
/// The worker is created by the `ThreadPool` and runs in the background.
/// When a job is received, the worker executes the job.
/// When the `ThreadPool` is dropped, the worker stops running.
struct Worker {
    // The id of the worker.
    id: usize,
    // The thread the worker is running on.
    thread: thread::JoinHandle<()>,
}

impl Worker {
    /// Create a new Worker.
    ///  
    /// The id is the worker's id and the receiver is the channel on which the worker will receive jobs.
    ///  
    /// # Panics
    ///  
    /// The `new` function will panic if the receiving end of the channel is disconnected.
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} got a job; executing.", id);
            job();
        });

        Worker { id, thread }
    }
}
