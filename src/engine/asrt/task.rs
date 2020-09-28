use crate::err::{EngineError, InnerError};
use flume::SendError;
use futures_task::ArcWake;
use std::any::Any;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, TryLockError};
use std::task::{Context, Poll};
use wasm_bindgen::__rt::core::marker::PhantomData;

#[derive(Clone)]
pub struct Task(Arc<Inner>);

type PinnedFuture = Pin<Box<dyn Future<Output = Box<dyn Any + 'static + Send>> + 'static + Send>>;

struct Inner {
    inner: Mutex<Option<PinnedFuture>>,
    complete: AtomicBool,
    working: AtomicBool,
    ready: AtomicBool,
    result: flume::Sender<Box<dyn Any + 'static + Send>>,
}

pub struct JoinHandle<T: Any + 'static + Send>(
    Arc<Inner>,
    flume::Receiver<Box<dyn Any + 'static + Send>>,
    PhantomData<T>,
);

#[derive(Clone)]
struct TaskWaker(Arc<Inner>, super::Handle);

#[derive(PartialEq)]
pub(super) enum TaskState {
    NotReady,
    Locked,
    Complete,
}

pub enum TaskError {
    TaskNonResponsiveOrPanic,
}

impl Task {
    pub fn new<F, T>(future: F) -> (Self, JoinHandle<T>)
    where
        F: Future<Output = T> + 'static + Send,
        T: Any + 'static + Send,
    {
        let (tx, rx) = flume::channel();
        let task = Arc::new(Inner {
            inner: Mutex::new(Some(Box::pin(async {
                Box::new(future.await) as Box<dyn Any + 'static + Send>
            }) as PinnedFuture)),
            complete: AtomicBool::new(false),
            working: AtomicBool::new(false),
            ready: AtomicBool::new(true),
            result: tx,
        });
        (
            Self(task.clone()),
            JoinHandle(task, rx, PhantomData::default()),
        )
    }

    pub(super) fn poll(&self, handle: &super::Handle) -> TaskState {
        if self.0.working.load(Ordering::Acquire) || !self.0.ready.load(Ordering::Acquire) {
            return TaskState::NotReady;
        }

        self.0.working.store(true, Ordering::Release);

        let waker = futures_task::waker(Arc::new(TaskWaker(self.0.clone(), handle.clone())));
        let mut ctx = Context::from_waker(&waker);
        let old = self.0.ready.swap(false, Ordering::AcqRel);
        let mut lock = match self.0.inner.try_lock() {
            Ok(lock) => lock,
            Err(err) => match err {
                TryLockError::Poisoned(poison) => {
                    log::error!("Found poisoned lock");
                    poison.into_inner()
                }
                TryLockError::WouldBlock => {
                    self.0.ready.store(old, Ordering::Relaxed);
                    self.0.working.store(false, Ordering::Release);
                    return TaskState::Locked;
                }
            },
        };

        let mut pin = match lock.take() {
            Some(pin) => pin,
            None => {
                log::error!("Tried to poll complete task, recovered!");
                self.0.complete.store(true, Ordering::Release);
                self.0.working.store(false, Ordering::Release);
                return TaskState::Complete;
            }
        };
        let result = match pin.as_mut().poll(&mut ctx) {
            Poll::Ready(result) => {
                let _ = self.0.result.send(result);
                self.0.complete.store(true, Ordering::Release);
                self.0.working.store(false, Ordering::Release);
                TaskState::Complete
            }
            Poll::Pending => TaskState::NotReady,
        };
        *lock = Some(pin);
        self.0.working.store(false, Ordering::Release);
        result
        // If anything gets added after this, drop `lock` first to prevent spending any overtime with the MutexGuard
    }
}

impl<T: Any + 'static + Send> JoinHandle<T> {
    pub fn join(self) -> Result<T, TaskError> {
        *self
            .1
            .recv()
            .map_err(|_| TaskError::TaskNonResponsiveOrPanic)?
            .downcast()
            .expect("task type not promised type")
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
