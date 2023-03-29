use std::{thread, sync::{mpsc, Arc, Mutex}};

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: Option<mpsc::Sender<Job>>,
}

struct Worker {
  id: usize,
  // 1? thread type 
  thread: Option<thread::JoinHandle<()>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
      assert!(size>0);
      let (sender, receiver) = mpsc::channel();
      let mut workers = Vec::with_capacity(size);
      let receiver = Arc::new(Mutex::new(receiver));
      for id in 0..size {
        workers.push(Worker::new(id,  Arc::clone(&receiver)));
      }
      ThreadPool { workers, sender: Some(sender) }
    }
    pub fn excute<F>(&self, f: F)
    where
      F: FnOnce() + Send + 'static
    {
      let job = Box::new(f);
      self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Worker {
  // 2? reverver type
    fn new(id: usize, recerver: Arc<Mutex<mpsc::Receiver<Job>>>) -> 
    Worker {
      let thread = thread::spawn(move || loop {
          let message = recerver.lock().unwrap().recv();
          match message {
              Ok(job) => {
                println!("worker {id} work");
                job();
              },
              Err(_) =>{
                break;
              }
          }
      });
      Worker { id, thread: Some(thread) }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
          println!("shut down worker{}", worker.id);
          if let Some(thread) = worker.thread.take() {
            thread.join().unwrap();
          }
        }
    }
}