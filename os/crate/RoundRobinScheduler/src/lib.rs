#![no_std]
#![feature(alloc)]

extern crate alloc;

use alloc::vec::Vec;

#[derive(Default)]
struct RRInfo {
    valid: bool,
    time: usize,
    prev: usize,
    next: usize,
}

pub struct RRScheduler {
    threads: Vec<RRInfo>,
    max_time: usize,
    current: usize,
}

impl RRScheduler {
    pub fn new(max_time_slice : usize) -> Self {
        let mut rr = RRScheduler{
            threads: Vec::default(),
            max_time: max_time_slice,
            current: 0,
        };
        rr.threads.push(RRInfo {
            valid: false,
            time: 0,
            prev: 0,
            next: 0,
        });
        rr
    }
    
    pub fn push(&mut self, tid : usize) {
        let tid = tid + 1;
        if tid + 1 > self.threads.len() {
            self.threads.resize_with(tid + 1, Default::default);
        }

        if self.threads[tid].time == 0 {
            self.threads[tid].time = self.max_time;
        }

        let prev = self.threads[0].prev;
        self.threads[tid].valid = true;
        self.threads[prev].next = tid;
        self.threads[tid].prev = prev;
        self.threads[0].prev = tid;
        self.threads[tid].next = 0;
    }

    pub fn pop(&mut self) -> Option<usize> {
        let ret = self.threads[0].next;
        if ret != 0 {
            let next = self.threads[ret].next;
            let prev = self.threads[ret].prev;
            self.threads[next].prev = prev;
            self.threads[prev].next = next;
            self.threads[ret].prev = 0;
            self.threads[ret].next = 0;
            self.threads[ret].valid = false;
            self.current = ret;
            Some(ret-1)
        }else{
            None
        }
    }

    pub fn tick(&mut self) -> bool{
        let tid = self.current;
        //println!("tick in scheduler, tid : {}", tid -1);
        if tid != 0 {
            self.threads[tid].time -= 1;
            if self.threads[tid].time == 0 {
                //println!("tick a 0, the tid is {}", tid - 1);
                return true;
            }else{
                return false;
            }
        }
        return true;
    }

    pub fn exit(&mut self, tid : usize) {
        let tid = tid + 1;
        if self.current == tid {
            self.current = 0;
        }
    }
}
