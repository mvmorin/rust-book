use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool(Option<Constellation>);

struct Constellation {
    sender: mpsc::Sender<Job>,
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut threads = Vec::with_capacity(size);
        for i in 0..size {
            let id = i;
            let rc = Arc::clone(&receiver);
            threads.push( thread::spawn(move || worker_task(id, rc)) );
        }

        ThreadPool(Some(Constellation{sender,threads}))
    }

    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {
        let job = Box::new(f);
        self.0.as_ref().unwrap().sender.send(job).unwrap();
    }
}

fn worker_task(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) {
    loop {
        let message = receiver.lock().unwrap().recv();

        match message {
            Ok(job) => {
                println!("Worker {id} got a job; executing.");
                job();
            }
            Err(_) => {
                println!("Worker {id} disconnected; shutting down.");
                break;
            }
        };
    };
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        let pool = self.0.take().unwrap();

        drop(pool.sender);
        for thread in pool.threads {
            thread.join().unwrap();
        }
    }
}
