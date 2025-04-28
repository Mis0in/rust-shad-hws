#![forbid(unsafe_code)]

use std::cell::Cell;
use std::{cell::RefCell, collections::VecDeque, fmt::Debug, rc::Rc};
use thiserror::Error;

////////////////////////////////////////////////////////////////////////////////

#[derive(Error, Debug)]
#[error("channel is closed")]
pub struct SendError<T: Debug> {
    pub value: T,
}
pub struct Sender<T> {
    buffer: Rc<RefCell<VecDeque<T>>>,
    channel_closed: Rc<Cell<bool>>,
}

impl<T: Debug> Sender<T> {
    pub fn send(&self, value: T) -> Result<(), SendError<T>> {
        if self.is_closed() {
            return Err(SendError { value });
        }
        self.buffer.borrow_mut().push_back(value);
        Ok(())
    }

    pub fn is_closed(&self) -> bool {
        self.channel_closed.get()
    }

    pub fn same_channel(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.buffer, &other.buffer)
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Sender {
            buffer: self.buffer.clone(),
            channel_closed: self.channel_closed.clone(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Error, Debug)]
pub enum ReceiveError {
    #[error("channel is empty")]
    Empty,
    #[error("channel is closed")]
    Closed,
}

pub struct Receiver<T> {
    buffer: Rc<RefCell<VecDeque<T>>>,
    channel_closed: Rc<Cell<bool>>,
}

impl<T> Receiver<T> {
    fn check_closed(&self) -> bool {
        self.channel_closed.get() && self.buffer.borrow().is_empty()
    }

    fn check_no_senders(&self) -> bool {
        Rc::strong_count(&self.buffer) == 1
    }
    pub fn recv(&mut self) -> Result<T, ReceiveError> {
        if self.check_closed() || self.check_no_senders() {
            return Err(ReceiveError::Closed);
        }

        match self.buffer.borrow_mut().pop_front() {
            Some(value) => Ok(value),
            None => Err(ReceiveError::Empty),
        }
    }

    pub fn close(&mut self) {
        self.channel_closed.set(true);
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        self.close();
    }
}

////////////////////////////////////////////////////////////////////////////////

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let buffer: VecDeque<T> = VecDeque::new();
    let ref_buf = Rc::new(RefCell::new(buffer));
    let channel_state = Rc::new(Cell::new(false));
    (
        Sender {
            buffer: ref_buf.clone(),
            channel_closed: channel_state.clone(),
        },
        Receiver {
            buffer: ref_buf.clone(),
            channel_closed: channel_state.clone(),
        },
    )
}
