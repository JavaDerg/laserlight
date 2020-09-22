use instant::Instant;
use futures_task::{ArcWake, Waker};
use std::collections::VecDeque;
use std::future::Future;
use std::time::Duration;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::ops::DerefMut;

pub struct AsyncSteppingRuntime {
    tasks: VecDeque<Mutex<Box<dyn Future<Output = ()> + 'static + Unpin>>>,
    waker: Waker,
    indicator: Arc<AtomicBool>,
}

struct LocalWaker(Arc<AtomicBool>);

pub enum StepResult {
    NoActionTake,
    PolledSome,
    CompletedTask,
    Timeout,
}

impl AsyncSteppingRuntime {
    pub fn new() -> Self {
        let atomic = Arc::new(AtomicBool::new(false));
        Self {
            tasks: VecDeque::new(),
            waker: futures_task::waker(Arc::new(LocalWaker(atomic.clone()))),
            indicator: atomic,
        }
    }

    pub fn spawn<F>(&mut self, future: F)
        where F: Future<Output = ()> + 'static + Unpin
    {
        let mutex = Mutex::new(Box::new(future) as Box<dyn Future<Output = ()> + Unpin>);
        self.tasks.push_back(mutex);
        self.indicator.store(true, Ordering::Release);
    }

    pub fn step_min_time(&mut self, time: Duration) -> StepResult {
        let start_time = Instant::now();
        while Instant::now() - start_time < time {
            let res = self.step();
            match &res {
                StepResult::NoActionTake
                | StepResult::CompletedTask => {
                    return res;
                }
                StepResult::PolledSome => continue,
                _ => unreachable!("Timeout can not be returned from step"),
            }
        }
        if Instant::now() - start_time < time {
            StepResult::Timeout
        } else {
            StepResult::PolledSome
        }
    }

    pub fn step(&mut self) -> StepResult {
        if !self.indicator.load(Ordering::Acquire) {
            return StepResult::NoActionTake;
        }

        for i in 0..self.tasks.len() {
            if match self.tasks.get(i) {
                Some(future) =>
                    if let Ok(mut pin) = future.lock() {
                        let pinned = unsafe { Pin::new_unchecked(pin.deref_mut()) } as Pin<&mut dyn Future<Output = ()>>;
                        let mut ctx = Context::from_waker(&self.waker);
                        match pinned.poll(&mut ctx) {
                            Poll::Ready(_) => true,
                            Poll::Pending => false,
                        }
                    } else {
                        false
                    },
                None => false,
            } {
                let _ = self.tasks.remove(i);
                return StepResult::CompletedTask;
            }
        }

        self.indicator.store(false, Ordering::Release);
        StepResult::PolledSome
    }

    #[inline]
    pub fn poll(&self, _future: Pin<&mut dyn Future<Output = ()>>) -> bool {
        let mut ctx = Context::from_waker(&self.waker);
        match _future.poll(&mut ctx) {
            Poll::Ready(_) => true,
            Poll::Pending => false,
        }
    }
}

impl ArcWake for LocalWaker {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.0.store(true, Ordering::Release);
    }
}
