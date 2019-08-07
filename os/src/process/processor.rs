use core::cell::UnsafeCell;
use alloc::boxed::Box;
use crate::process::Tid;
use crate::process::structs::*;
use crate::process::thread_pool::ThreadPool;

pub struct ProcessorInner {
    pool: Box<ThreadPool>,
    idle: Box<Thread>,
    current: Option<(Tid, Box<Thread>)>,
}

pub struct Processor {
    inner: UnsafeCell<Option<ProcessorInner>>,
}

unsafe impl Sync for Processor {}

use crate::interrupt::restore;

impl Processor {
    pub const fn new() -> Processor {
        Processor {
            inner: UnsafeCell::new(None),
        }
    }

    pub fn init(&self, idle: Box<Thread>, pool: Box<ThreadPool> ) {
        unsafe {
            *self.inner.get() = Some(ProcessorInner{
                pool,
                idle,
                current: None,
            });
        }
    }

    fn inner(&self) -> &mut ProcessorInner {
        unsafe { &mut *self.inner.get() }
            .as_mut()
            .expect("Processor is not initialized")
    }

    pub fn add_thread(&self, thread: Box<Thread>) {
        self.inner().pool.add(thread);
    }

    pub fn run(&self) -> !{
        let inner = self.inner();
        // 关闭中断，防止此时产生中断异常导致线程切换出错。
        disable_and_store();
        // 循环从线程池中寻找可调度线程
        loop {
            // 如果存在需要被调度的线程
            if let Some(thread) = inner.pool.acquire() {
                inner.current = Some(thread);
                // 切换至需要被调度的线程
                inner.idle.switch_to(&mut *inner.current.as_mut().unwrap().1);
                // 上一个线程已经结束或时间片用完，切换回 idle 线程
                let (tid, thread) = inner.current.take().unwrap();
                // println!("thread {} ran just now", tid);
                // 将上一个线程放回线程池中
                inner.pool.retrieve(tid, thread);
            } else {
                // 开启中断并等待中断产生
                enable_and_wfi();
                // 关闭中断，从线程池中寻找可调度线程
                disable_and_store();
            }
        }
    }

    pub fn tick(&self) {
        let inner = self.inner();
        if !inner.current.is_none() {
            if inner.pool.tick() {
                let flags = disable_and_store();
                inner
                    .current
                    .as_mut()
                    .unwrap()
                    .1
                    .switch_to(&mut inner.idle);
                // 恢复原先的中断状态
                restore(flags);
            }
        }
    }

    pub fn exit(&self, code: usize) -> ! {
        let inner = self.inner();
        let tid = inner.current.as_ref().unwrap().0;
        // 通知线程池该线程即将退出
        inner.pool.exit(tid, code);
        // 切换至 idle 线程，进入调度
        inner
            .current
            .as_mut()
            .unwrap()
            .1
            .switch_to(&mut inner.idle);
        loop {}
    }

    pub fn yield_now(&self) {
        let inner = self.inner();
        if !inner.current.is_none() {
            unsafe {
                let flags = disable_and_store(); // 禁止中断，获取当前 sstatus 的状态并保存。
                let tid = inner.current.as_mut().unwrap().0;
                let thread_info = inner.pool.threads[tid].as_mut().expect("thread not exits");
                if thread_info.present {
                    thread_info.status = Status::Sleeping;
                } else {
                    panic!("try to sleep an null thread !");
                }
                inner
                    .current
                    .as_mut()
                    .unwrap()
                    .1
                    .switch_to(&mut *inner.idle);   // 转到 idle 线程重新调度
                restore(flags);  // 使能中断，恢复 sstatus 的状态
            }
        }
    }

    pub fn wake_up(&self, tid: Tid) {
        let inner = self.inner();
        inner.pool.wakeup(tid);
    }

    pub fn current_tid(&self) -> usize {
        self.inner().current.as_mut().unwrap().0 as usize
    }
}

use crate::interrupt::{ disable_and_store, enable_and_wfi };