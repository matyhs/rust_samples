use std::sync::Arc;
use std::sync::mpsc;
use std::sync::Mutex;
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>
}

struct Worker {
    id: usize,
    handle: Option<thread::JoinHandle<()>>
}

enum Message {
    NewJob(Job),
    Terminate
}

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

        let (tx, tr) = mpsc::channel();
        let mut workers = Vec::with_capacity(size);
        let tr = Arc::new(Mutex::new(tr));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&tr)));
        }

        ThreadPool {workers, sender:tx}
    }

    /// Executes job on the threads within the pool
    ///
    /// F represents the job to execute
    pub fn execute<F>(&self, f: F) 
    where
        F : FnOnce() + Send + 'static{
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Commence termination of thread workers");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all worker threads");

        for worker in &mut self.workers {
            println!("Shutting down worker {}...", worker.id);

            if let Some(thread) = worker.handle.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl Worker {
    /// Creates a new worker thread
    ///
    /// id is the identifer for the spawned worker thread.
    ///
    /// receiver is the receiving end of the channel containing the job to execute
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let handle = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} is executing a job...", id);
                    job();
                },
                Message::Terminate => {
                    println!("Terminating worker {}", id);
                    break;
                }
            }
        });
        Worker {id, handle: Some(handle)}
    }
}