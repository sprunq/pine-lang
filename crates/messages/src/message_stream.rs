use base::source_id::SourceId;

use crate::message::Message;
use std::{
    collections::{vec_deque::Iter, VecDeque},
    iter::Filter,
    sync::{mpsc::Sender, Arc, Mutex},
};

pub struct MessageStream {
    messages: Arc<Mutex<VecDeque<Message>>>,
}

impl MessageStream {
    pub fn new() -> Self {
        Self {
            messages: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn init(&mut self) -> Sender<Message> {
        let (sender, receiver) = std::sync::mpsc::channel();
        let messages = self.messages.clone();
        std::thread::spawn(move || {
            for msg in receiver {
                messages.lock().unwrap().push_back(msg);
            }
        });
        sender
    }
}

impl Iterator for MessageStream {
    type Item = Message;

    fn next(&mut self) -> Option<Self::Item> {
        self.messages.lock().unwrap().pop_front()
    }
}
