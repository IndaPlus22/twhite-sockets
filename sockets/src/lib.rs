use std::{sync::mpsc, thread};
use std::sync::{Arc, Mutex};

pub struct Threadpool {
    workers: Vec<thread::JoinHandle<()>>,
    sender: mpsc::Sender<Job>,
}


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
        
        ThreadPool { workers }
    }
    
    /// Execute a function in the thread pool.
    pub fn execute<F>(&self, f: F)
    where
    F: FnOnce() + Send + 'static,
    {
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(|| {});
    }
}

struct Job;

impl Job {

}