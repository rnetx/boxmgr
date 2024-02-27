use std::{
    collections::VecDeque,
    sync::{Arc, RwLock},
};

use tokio::sync::{broadcast, mpsc};

#[derive(Clone)]
pub(crate) struct LogQueue<T: Clone> {
    cache: Arc<RwLock<VecDeque<T>>>,
    sender: broadcast::Sender<T>,
}

impl<T: Clone> LogQueue<T> {
    pub(crate) fn new(max_size: usize) -> Self {
        let (sender, _) = broadcast::channel(max_size);
        Self {
            cache: Arc::new(RwLock::new(VecDeque::with_capacity(max_size))),
            sender,
        }
    }

    pub(crate) fn push_data(&self, v: T) {
        let mut cache = self.cache.write().unwrap();
        if cache.len() == cache.capacity() {
            cache.pop_front();
        }
        cache.push_back(v.clone());
        drop(cache);
        let _ = self.sender.send(v);
    }

    pub(crate) fn subscribe(&self) -> LogQueueListener<T> {
        let receiver = self.sender.subscribe();
        LogQueueListener {
            cache: self.cache.clone(),
            receiver,
        }
    }
}

pub(crate) struct LogQueueListener<T: Clone> {
    cache: Arc<RwLock<VecDeque<T>>>,
    receiver: broadcast::Receiver<T>,
}

impl<T: Clone> LogQueueListener<T> {
    pub async fn listen(mut self, sender: mpsc::Sender<T>) {
        let cache = self.cache.read().unwrap().clone();
        for v in cache {
            if let Err(_) = sender.send(v).await {
                return;
            }
        }
        while let Ok(v) = self.receiver.recv().await {
            if let Err(_) = sender.send(v).await {
                return;
            }
        }
    }
}
