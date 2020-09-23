use crate::err::{EngineError, InnerError};
use flume::SendError;
use futures_task::ArcWake;
use std::future::Future;
use std::mem::swap;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, TryLockError};
use std::task::{Context, Poll};

#[derive(Clone)]
pub struct Task(Arc<Inner>);

struct Inner {
    inner: Mutex<Option<Pin<Box<dyn Future<Output = ()> + 'static + Send>>>>,
    completed: AtomicBool,
    ready: AtomicBool,
}

// Todo: implement `join`
pub struct JoinHandle(Arc<Inner>);

#[derive(Clone)]
struct TaskWaker(Arc<Inner>, super::Handle);

#[derive(PartialEq)]
pub(super) enum TaskState {
    NotReady,
    Locked,
    Complete,
}

impl Task {
    pub fn new<F>(future: F) -> (Self, JoinHandle)
    where
        F: Future<Output = ()> + 'static + Send,
    {
        let task = Arc::new(Inner {
            inner: Mutex::new(Some(
                Box::pin(future) as Pin<Box<dyn Future<Output = ()> + Send>>
            )),
            completed: AtomicBool::new(false),
            ready: AtomicBool::new(true),
        });
        (Self(task.clone()), JoinHandle(task))
    }

    pub(super) fn poll(&self, handle: &super::Handle) -> TaskState {
        if !self.0.ready.load(Ordering::Acquire) {
            return TaskState::NotReady;
        }

        let waker = futures_task::waker(Arc::new(TaskWaker(self.0.clone(), handle.clone())));
        let mut ctx = Context::from_waker(&waker);
        self.0.ready.store(false, Ordering::Release);
        let mut lock = match self.0.inner.try_lock() {
            Ok(lock) => lock,
            Err(err) => match err {
                TryLockError::Poisoned(poison) => {
                    log::error!("Found poisoned lock");
                    poison.into_inner()
                }
                TryLockError::WouldBlock => {
                    self.0.ready.store(true, Ordering::Release);
                    return TaskState::Locked;
                }
            },
        };
        let mut opt = None;
        swap(&mut *lock, &mut opt);
        let mut pin = match opt {
            Some(pin) => pin,
            None => {
                log::error!("Tried to poll complete task, recovered!");
                self.0.completed.store(true, Ordering::Release);
                return TaskState::Complete;
            }
        };
        match pin.as_mut().poll(&mut ctx) {
            Poll::Ready(()) => {
                *lock = None;
                self.0.completed.store(true, Ordering::Release);
                TaskState::Complete
            }
            Poll::Pending => TaskState::NotReady,
        }
        // If anything gets added after this, drop `lock` first to prevent spending any overtime with the MutexGuard
    }
}

impl ArcWake for TaskWaker {
    fn wake_by_ref(this: &Arc<Self>) {
        this.0.ready.store(true, Ordering::Release);
        this.1.mark_dirty();
    }
}

impl From<flume::SendError<Task>> for EngineError {
    fn from(_: SendError<Task>) -> Self {
        EngineError::new(InnerError::NoRuntime, String::from(""))
    }
}
