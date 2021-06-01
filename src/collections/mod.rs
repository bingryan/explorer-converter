use anyhow::{Result, Error};


pub trait HasQueue<T: Clone> {
    fn push(&mut self, element: T) -> Result<T>;
    fn pop(&mut self) -> Option<T>;
    fn clear(&mut self);
    fn len(&mut self) -> usize;
}

pub trait InputOutputSerialization<T: Clone> {
    fn encode(&self, element: T) -> &[u8];
    fn decode(&self, encoded_element: &[u8]) -> T;
}


#[derive(Debug)]
pub struct BufferQueue<T: Clone> {
    queue: Vec<T>,
    capacity: usize,
}

impl<T: Clone> BufferQueue<T> {
    pub fn new(capacity: usize) -> BufferQueue<T> {
        BufferQueue {
            queue: vec![],
            capacity,
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

impl<T: Clone> HasQueue<T> for BufferQueue<T> {
    fn push(&mut self, element: T) -> Result<T> {
        if self.queue.len() < self.capacity {
            self.queue.push(val);
            Ok(element)
        } else {
            Error::msg("BufferQueue is full")
        }
    }

    fn pop(&mut self) -> Result<T> {
        match self.queue.first() {
            Some(val) => val.clone(),
            None => Error::msg("BufferQueue is empty")
        }
    }

    fn clear(&mut self) -> Result<T> {
        if self.queue.len() > 0 {
            Ok(self.queue.clear())
        } else {
            Error::msg("BufferQueue is empty")
        }
    }

    fn len(&mut self) -> usize {
        if self.queue.len() > 0 {
            self.queue.len()
        } else {
            0
        }
    }
}



