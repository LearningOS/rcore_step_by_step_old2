mod processor;
mod scheduler;
mod structs;
mod thread_pool;

use self::scheduler::Scheduler;
use crate::fs::INodeExt;
use crate::fs::ROOT_INODE;
use alloc::boxed::Box;
use processor::Processor;
use structs::Thread;
use thread_pool::ThreadPool;

pub type Tid = usize;
pub type ExitCode = usize;

static CPU: Processor = Processor::new();

pub fn tick() {
    CPU.tick();
}

pub fn exit(code: usize) {
    CPU.exit(code);
}

pub fn kmain() {
    CPU.run();
}

pub fn init() {
    println!("+------ now to initialize process ------+");
    let scheduler = Scheduler::new(1);
    let thread_pool = ThreadPool::new(100, scheduler);
    println!("+------ now to initialize processor ------+");
    CPU.init(Thread::new_idle(), Box::new(thread_pool));
    let data = ROOT_INODE
        .lookup("rust/shell")
        .unwrap()
        .read_as_vec()
        .unwrap();
    println!("size of program {:#x}", data.len());
    let user = unsafe { Thread::new_user(data.as_slice()) };
    CPU.add_thread(user);
}

#[no_mangle]
pub extern "C" fn hello_thread(arg: usize) -> ! {
    println!("hello thread");
    println!("arg is {}", arg);
    for i in 0..100 {
        println!("{}{}{}{}{}{}{}{}", arg, arg, arg, arg, arg, arg, arg, arg);
        for j in 0..1000 {}
    }
    println!("end of thread {}", arg);
    CPU.exit(0)
}
