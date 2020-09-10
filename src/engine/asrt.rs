use instant::Instant;
use std::collections::VecDeque;
use std::future::Future;

use std::time::Duration;
use std::pin::Pin;
use futures_task::{ArcWake, Waker};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::task::Context;
use wasm_bindgen::__rt::core::task::Poll;
use wasm_bindgen::__rt::std::sync::Mutex;

pub struct AsyncSteppingRuntime {
    tasks: VecDeque<Mutex<impl Future<Output = ()> + 'static>>,
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

    pub fn spawn<F>(&mut self, future: impl Future<Output = ()> + 'static) {
        let mutex = Mutex::new(future);
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
                    if let Ok(pin) = future.lock() {
                        self.poll(*pin)
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
