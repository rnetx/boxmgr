use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use tokio::sync::Notify;

pub(crate) struct State<T> {
    inner: T,
    notify: Arc<Notify>,
}

impl<T> State<T> {
    pub(crate) fn new(inner: T, notify: Arc<Notify>) -> Self {
        Self { inner, notify }
    }

    pub(crate) fn notify(&self) {
        self.notify.notify_waiters();
    }

    pub(crate) fn clone_notify(&self) -> Arc<Notify> {
        self.notify.clone()
    }
}

impl<T> Deref for State<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for State<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> AsRef<T> for State<T> {
    fn as_ref(&self) -> &T {
        &self.inner
    }
}
