use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::time::{Duration, Instant};
use std::thread;

/// A time delay that can be `.await`ed.
pub struct Delay {
    when: Instant,
    waker: Arc<Mutex<Option<Waker>>>,
}

impl Delay {
    /// Creates a new time delay.
    pub fn new(duration: Duration) -> Self {
        let when = Instant::now() + duration;
        Delay {
            when,
            waker: Arc::new(Mutex::new(None)),
        }
    }

    fn check_waker(waker: Arc<Mutex<Option<Waker>>>, when: Instant) {
        // If the wait is over, immediately call the waker and ignore the sleep.
        let now = Instant::now();
        if now >= when {
            if let Some(w) = waker.lock().unwrap().take() {
                w.wake();
            }
            return;
        }

        // Actively wait in the background. Not optimal, but gets the job done.
        let wait_time = when - now;
        thread::spawn(move || {
            thread::sleep(wait_time);
            if let Some(w) = waker.lock().unwrap().take() {
                w.wake();
            }
        });
    }
}

impl Future for Delay {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Early exit when possible.
        let now = Instant::now();
        if now >= self.when {
            return Poll::Ready(());
        }

        let mut waker = self.waker.lock().unwrap();
        if waker.is_none() {
            *waker = Some(cx.waker().clone());

            // NOTE: Potential for a race condition here with returning Pending.
            Self::check_waker(self.waker.clone(), self.when);
        }
        Poll::Pending
    }
}
