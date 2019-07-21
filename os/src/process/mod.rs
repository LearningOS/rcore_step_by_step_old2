mod processor;
mod scheduler;
mod structs;
mod thread_pool;

use self::scheduler::Scheduler;
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

pub fn init() {
    println!("+------ now to initialize process ------+");
    let scheduler = Scheduler::new(1);
    let thread_pool = ThreadPool::new(100, scheduler);
    println!("+------ now to initialize processor ------+");
    CPU.init(Thread::new_idle(), Box::new(thread_pool));
    println!("+------ now to initialize threads ------+");
    let thread0 = Thread::new_kernel(hello_thread, 0);
    CPU.add_thread(thread0);
    let thread1 = Thread::new_kernel(hello_thread, 1);
    CPU.add_thread(thread1);
    let thread2 = Thread::new_kernel(hello_thread, 2);
    CPU.add_thread(thread2);
    let thread3 = Thread::new_kernel(hello_thread, 3);
    CPU.add_thread(thread3);
    let thread4 = Thread::new_kernel(hello_thread, 4);
    CPU.add_thread(thread4);
    let data = include_bytes!(env!("SFSIMG"));
    let user = unsafe { Thread::new_user(data) };
    CPU.add_thread(user);
    CPU.run();
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
