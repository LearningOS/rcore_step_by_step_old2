use alloc::{ collections::VecDeque, sync::Arc };
use spin::Mutex;
use crate::process;
use crate::sync::condvar::*;

pub struct Stdin {
    buf: Mutex<VecDeque<char>>,
    pushed: Condvar,
}

impl Stdin {
    pub fn new() -> Self {
        Stdin {
            buf: Mutex::new(VecDeque::new()),
            pushed: Condvar::new(),
        }
    }

    pub fn push(&self, ch: char) {
        let mut lock = self.buf.lock();
        lock.push_back(ch);
        drop(lock);
        self.pushed.notify();
    }

    pub fn pop(&self) -> char {
        loop{
            let ret = self.buf.lock().pop_front();
            match ret {
                Some(ch) => {
                    return ch;
                },
                None => {
                    self.pushed.wait();
                },
            }
        }
    }
}

use lazy_static::*;
lazy_static!{
    pub static ref STDIN: Arc<Stdin> = Arc::new(Stdin::new());
}