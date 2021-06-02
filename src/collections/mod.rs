use anyhow::{Result, Error};


pub trait HasQueue<T: Clone> {
    fn push(&mut self, element: T) -> Result<()>;
    fn pop(&mut self) -> Result<T>;
    fn clear(&mut self)-> Result<()>;
    fn len(&mut self) -> usize;
}

pub trait InputOutputSerialization<T: Clone> {
    fn encode(&self, element: T) -> &[u8];
    fn decode(&self, encoded_element: &[u8]) -> T;
}


/// FifoQueue is FIFO Queue data structure by memory
#[derive(Debug)]
pub struct FifoQueue<T: Clone> {
    queue: Vec<T>,
    capacity: usize,
}

impl<T: Clone> FifoQueue<T> {
    pub fn new(capacity: usize) -> FifoQueue<T> {
        FifoQueue {
            queue: vec![],
            capacity,
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

impl<T: Clone> HasQueue<T> for FifoQueue<T> {
    fn push(&mut self, element: T) -> Result<()> {
        if self.queue.len() < self.capacity {
            self.queue.push(element);
            Ok(())
        } else {
            Err(Error::msg("FifoQueue is full"))
        }
    }

    fn pop(&mut self) -> Result<T> {
        match self.queue.first() {
            Some(val) => Ok(val.clone()),
            None => Err(Error::msg("FifoQueue is empty"))
        }
    }

    fn clear(&mut self) -> Result<()> {
        if self.queue.len() > 0 {
            Ok(self.queue.clear())
        } else {
            Err(Error::msg("FifoQueue is empty"))
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

/// LifoQueue is FIFO Queue data structure by memory
#[derive(Debug)]
pub struct LifoQueue<T: Clone> {
    queue: Vec<T>,
    capacity: usize,
}

impl<T: Clone> LifoQueue<T> {
    pub fn new(capacity: usize) -> LifoQueue<T> {
        LifoQueue {
            queue: vec![],
            capacity,
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

impl<T: Clone> HasQueue<T> for LifoQueue<T> {
    fn push(&mut self, element: T) -> Result<()> {
        if self.queue.len() < self.capacity {
            self.queue.push(element);
            Ok(())
        } else {
            Err(Error::msg("LifoQueue is full"))
        }
    }

    fn pop(&mut self) -> Result<T> {
        match self.queue.pop() {
            Some(val) => Ok(val.clone()),
            None => Err(Error::msg("LifoQueue is empty"))
        }
    }

    fn clear(&mut self) -> Result<()> {
        if self.queue.len() > 0 {
            Ok(self.queue.clear())
        } else {
            Err(Error::msg("LifoQueue is empty"))
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



