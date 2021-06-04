use async_trait::async_trait;
use anyhow::{Result, Error};
use codec::{Encode, Decode, Error as CodecError};

pub trait HasQueue<T: Clone> {
    fn push(&mut self, element: T) -> Result<()>;
    fn pop(&mut self) -> Result<T>;
    fn clear(&mut self) -> Result<()>;
    fn len(&mut self) -> usize;
}

#[async_trait]
pub trait HasAsyncQueue<T: Clone> {
    async fn push(&mut self, element: &Box<T>) -> Result<()>;
    async fn pop(&mut self) -> Result<Box<T>>;
    async fn clear(&mut self) -> Result<()>;
    async fn len(&mut self) -> Result<usize>;
}

/// input out serialize
pub trait CodecSerialization<T: Clone + Encode + Decode + ?Sized> {
    fn name(&self) -> &'static str;

    /// encode input value
    fn encode(&self, element: &Box<T>) -> Vec<u8> {
        element.encode()
    }

    /// decode element
    fn decode(&self, encoded_element: &Vec<u8>) -> Result<Box<T>, CodecError> {
        let mut ele: &[u8] = encoded_element;
        Box::<T>::decode(&mut ele)
    }
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


use redis::{Client as RedisClient, aio::Connection, AsyncCommands, RedisResult};
use crate::config::REDIS_TIMEOUT;

/// RedisLifoQueue is FIFO Queue data structure by memory
pub struct RedisLifoQueue<'a> {
    pub redis_connection: Connection,
    pub key: &'a str,
}

impl<'a> RedisLifoQueue<'a> {
    pub fn new(redis_connection: Connection, key: &str) -> RedisLifoQueue {
        RedisLifoQueue {
            redis_connection,
            key,
        }
    }
}

impl<'a, T: Clone + Encode + Decode> CodecSerialization<T> for RedisLifoQueue<'a> {
    fn name(&self) -> &'static str {
        "redis-lifo-queue"
    }
}

#[async_trait]
impl<'a, T: Clone + Encode + Decode + Sync> HasAsyncQueue<T> for RedisLifoQueue<'a> {
    async fn push(&mut self, element: &Box<T>) -> Result<()> {
        let mut encode_res: Vec<u8> = self.encode(&element);
        let res = self.redis_connection.lpush::<&str, &[u8], ()>(self.key, encode_res.as_ref()).await?;
        Ok(res)
    }

    async fn pop(&mut self) -> Result<Box<T>> {
        let pop_res:Vec<u8> = self.redis_connection.lpop::<&str, Vec<u8>>(self.key).await?;
        let mut encode_res: &[u8] = &pop_res;
        Box::<T>::decode(&mut encode_res).map_err(|_| Error::msg("RedisLifoQueue pop decode error"))
    }

    async fn clear(&mut self) -> Result<()> {
        self.redis_connection.del(self.key).await?;
        Ok(())
    }

    async fn len(&mut self) -> Result<usize> {
        let res = self.redis_connection.llen(self.key).await?;
        Ok(res)
    }
}


