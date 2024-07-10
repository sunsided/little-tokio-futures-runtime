pub mod delay;

use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::thread;
use std::time::Duration;

use futures::task::{waker_ref, ArcWake};

/// `Runtime` is a struct that represents a runtime for managing `Task`s concurrently.
pub struct Runtime {
    tasks: Arc<Mutex<Vec<Arc<Task>>>>,
}

/// A type representing a task.
///
/// The `Task` struct encapsulates a future that can be executed asynchronously.
struct Task {
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
    completed: Mutex<bool>,
}

impl Runtime {
    /// Creates a new runtime.
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Spawns a new task onto the executor.
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

    /// Runs the task manager in a loop until all tasks are completed.
    ///
    /// The `run` method continuously checks the tasks in the task manager and removes
    /// completed tasks until there are no more tasks remaining. During each iteration,
    /// it also sleeps for 10 milliseconds to avoid unnecessarily consuming CPU resources.
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
        // Get a waker to the task. Works since it implements ArcWake.
        let waker = waker_ref(&self);
        let mut context = Context::from_waker(&*waker);

        // Poll the task future and flag it as completed when it is ready, so that
        // it can be removed from the runtime's task vector.
        let mut future = self.future.lock().unwrap();
        if let Poll::Ready(_) = future.as_mut().poll(&mut context) {
            let mut completed = self.completed.lock().unwrap();
            *completed = true;
        }
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let task = arc_self.clone();
        thread::spawn(move || {
            task.poll();
        });
    }
}
