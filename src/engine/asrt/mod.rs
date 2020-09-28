mod task;

use crate::engine::asrt::task::TaskState;
use crate::err::EngineError;
use instant::Instant;
use std::any::Any;
use std::collections::VecDeque;
use std::future::Future;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

pub struct Runtime {
    tasks: VecDeque<task::Task>,
    dirty: Arc<AtomicBool>,
    task_receiver: flume::Receiver<task::Task>,
    task_sender: flume::Sender<task::Task>,
}

#[derive(Clone)]
pub struct Handle {
    dirty: Arc<AtomicBool>,
    task_dispatcher: flume::Sender<task::Task>,
}

#[derive(Debug)]
pub enum StepResult {
    NoActionTake,
    PolledSome,
    CompletedTask,
    Timeout,
}

impl Runtime {
    pub fn new() -> Self {
        let atomic = Arc::new(AtomicBool::new(false));
        let (tx, rx) = flume::channel();
        Self {
            tasks: VecDeque::new(),
            dirty: atomic,
            task_receiver: rx,
            task_sender: tx,
        }
    }

    pub fn spawn<F, T>(&mut self, future: F) -> task::JoinHandle<T>
    where
        F: Future<Output = T> + 'static + Send,
        T: Any + 'static + Send,
    {
        let (task, handle) = task::Task::new(future);
        self.tasks.push_back(task);
        self.dirty.store(true, Ordering::Release);
        handle
    }

    pub fn step_min_time(&mut self, time: Duration) -> StepResult {
        self.check_channel();
        let start_time = Instant::now();
        while Instant::now() - start_time < time {
            let res = self.internal_step();
            match &res {
                StepResult::NoActionTake | StepResult::CompletedTask => {
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

    #[inline]
    #[allow(dead_code)]
    pub fn step(&mut self) -> StepResult {
        self.check_channel();
        self.internal_step()
    }

    #[inline]
    fn check_channel(&mut self) {
        while let Ok(task) = self.task_receiver.recv() {
            self.tasks.push_back(task);
        }
    }

    pub fn internal_step(&mut self) -> StepResult {
        if !self.dirty.load(Ordering::Acquire) {
            return StepResult::NoActionTake;
        }

        let handle = self.get_handle();
        for i in 0..self.tasks.len() {
            if match self.tasks.get(i) {
                Some(future) => future.poll(&handle) == TaskState::Complete,
                None => false,
            } {
                let _ = self.tasks.remove(i);
                return StepResult::CompletedTask;
            }
        }

        self.dirty.store(false, Ordering::Release);
        StepResult::PolledSome
    }

    pub fn get_handle(&self) -> Handle {
        Handle {
            dirty: self.dirty.clone(),
            task_dispatcher: self.task_sender.clone(),
        }
    }
}

impl Handle {
    pub fn spawn<F, T>(&mut self, future: F) -> Result<task::JoinHandle<T>, EngineError>
    where
        F: Future<Output = T> + 'static + Send,
        T: Any + 'static + Send,
    {
        let (task, handle) = task::Task::new(future);
        self.task_dispatcher.send(task)?;
        Ok(handle)
    }

    pub fn mark_dirty(&self) {
        self.dirty.store(true, Ordering::Release);
    }
}
