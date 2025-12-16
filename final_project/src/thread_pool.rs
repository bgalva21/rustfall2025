use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    Run(Job),
    Shutdown,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Message>>,
    // shared receiver for all workers
    receiver: Arc<Mutex<mpsc::Receiver<Message>>>,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
    stop_flag: Arc<Mutex<bool>>, 
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel::<Message>();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut pool = ThreadPool {
            workers: Vec::new(),
            sender: Some(sender),
            receiver,
        };

        pool.resize(size);
        pool
    }

    pub fn size(&self) -> usize {
        self.workers.len()
    }

    //can resize dynamically
    pub fn resize(&mut self, new_size: usize) {
        assert!(new_size > 0);

        let current = self.workers.len();
        if new_size == current {
            return;
        }

        if new_size > current {
            for id in current..new_size {
                self.workers.push(Worker::new(id, Arc::clone(&self.receiver)));
            }
        } else {
            while self.workers.len() > new_size {
                if let Some(mut w) = self.workers.pop() {
                    if let Ok(mut flag) = w.stop_flag.lock() {
                        *flag = true;
                    }
                    if let Some(handle) = w.thread.take() {
                        let _ = handle.join();
                    }
                }
            }
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        if let Some(sender) = &self.sender {
            // if send fails then pool shuts down.
            let _ = sender.send(Message::Run(job));
        }
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let stop_flag = Arc::new(Mutex::new(false));
        let stop_flag_in_thread = Arc::clone(&stop_flag); 

        let thread = thread::spawn(move || loop {
            if let Ok(flag) = stop_flag_in_thread.lock() {
                if *flag {
                    break;
                }
            }

            let msg = {
                let rx_lock = receiver.lock();
                if rx_lock.is_err() {
                    break;
                }
                let rx = rx_lock.unwrap();
                rx.recv_timeout(Duration::from_millis(100))
            };

            match msg {
                Ok(Message::Run(job)) => job(),
                Ok(Message::Shutdown) => break,
                Err(mpsc::RecvTimeoutError::Timeout) => continue,
                Err(mpsc::RecvTimeoutError::Disconnected) => break,
            }
        });

        Worker {
            id,
            thread: Some(thread),
            stop_flag,
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        if let Some(sender) = self.sender.take() {
            for _ in &self.workers {
                let _ = sender.send(Message::Shutdown);
            }
        }

        for w in &mut self.workers {
            if let Some(handle) = w.thread.take() {
                let _ = handle.join();
            }
        }
    }
}
