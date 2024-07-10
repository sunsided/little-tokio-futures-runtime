use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::thread;
use std::time::Duration;

use futures::task::{waker_ref, ArcWake};

pub struct Runtime {
    tasks: Arc<Mutex<Vec<Arc<Task>>>>,
}

struct Task {
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
    completed: Mutex<bool>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn spawn<F>(&self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = Arc::new(Task {
            future: Mutex::new(Box::pin(future)),
            completed: Mutex::new(false),
        });

        self.tasks.lock().unwrap().push(task.clone());
        task.poll();
    }

    pub fn run(&self) {
        loop {
            let mut tasks = self.tasks.lock().unwrap();
            tasks.retain(|task| {
                let completed = task.completed.lock().unwrap();
                !*completed
            });

            if tasks.is_empty() {
                break;
            }

            thread::sleep(Duration::from_millis(10));
        }
    }
}

impl Task {
    fn poll(self: Arc<Self>) {
        let waker = waker_ref(&self);
        let mut context = Context::from_waker(&*waker);

        let mut future = self.future.lock().unwrap();
        if let Poll::Ready(_) = future.as_mut().poll(&mut context) {
            let mut completed = self.completed.lock().unwrap();
            *completed = true;
        }
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        thread::spawn(move || {
            cloned.poll();
        });
    }
}
