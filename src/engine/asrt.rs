use instant::Instant;
use std::collections::VecDeque;
use std::future::Future;

use std::time::Duration;
use std::pin::Pin;
use futures_task::ArcWake;
use wasm_bindgen::__rt::std::sync::Arc;
use std::sync::atomic::AtomicBool;

#[derive(Default)]
pub struct AsyncSteppingRuntime {
    tasks: VecDeque<Pin<Box<dyn Future<Output = ()>>>>,
    waker: Arc<Waker>,
    indicator: Arc<AtomicBool>,
}

#[derive(Default)]
struct Waker(Arc<AtomicBool>);

impl AsyncSteppingRuntime {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn spawn<F>(&mut self, future: Box<F>)
    where
        F: 'static + Future<Output = ()> + Unpin + Sized,
    {
        let future = Pin::new(future as Box<dyn Future<Output = _> + Unpin>);
        self.tasks.push_back(future);
        todo!()
    }

    pub fn step_min_time(&mut self, time: Duration) -> bool {
        let start_time = Instant::now();
        while Instant::now() - start_time < time {
            match self.step() {
                true => return true,
                false => continue,
            }
        }
        false
    }

    pub fn step(&mut self) -> bool {
        for i in 0..self.tasks.len() {
            if match self.tasks.get(i) {
                Some(future) => self.poll(future),
                None => false,
            } {
                let _ = self.tasks.remove(i);
                return self.tasks.is_empty();
            }
        }
        false
    }

    #[inline]
    pub fn poll(&self, _future: &dyn Future<Output = ()>) -> bool {
        todo!()
    }
}

impl ArcWake for Waker {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        todo!()
    }
}
