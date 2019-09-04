use core::fmt::{write, Arguments, Error, Write};
use device_tree::{util::SliceRead, Node};
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::{ReadOnly, WriteOnly};

lazy_static! {
    static ref PRINT_MUTEX: Mutex<Option<&'static mut Serial16550>> = Mutex::new(None);
}

bitflags! {
    struct LineStatusRegister: u8 {
        // Tranmit hold register empty
        // i.e. can send
        const THRE = 0x20;

        // Receiver data ready
        // i.e. can read
        const DR = 0x01;
    }
}

#[repr(C)]
struct Serial16550 {
    dll: WriteOnly<u8>,
    dlm: WriteOnly<u8>,
    fcr: WriteOnly<u8>,
    lcr: WriteOnly<u8>,
    mcr: WriteOnly<u8>,
    lsr: ReadOnly<u8>,
    msr: ReadOnly<u8>,
}

impl Serial16550 {
    fn putc(&mut self, ch: u8) {
        while !LineStatusRegister::from_bits_truncate(self.lsr.read())
            .contains(LineStatusRegister::THRE)
        {}
        self.dll.write(ch);
    }
}

impl Write for Serial16550 {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        for ch in s.bytes() {
            self.putc(ch);
        }
        Ok(())
    }
}

pub fn init(node: &Node) {
    if let Some(reg) = node.prop_raw("reg") {
        let base = reg.as_slice().read_be_u64(0).unwrap();
        let serial = unsafe { &mut *(base as *mut Serial16550) };
        *PRINT_MUTEX.lock() = Some(serial);
    }
}

pub fn print(args: Arguments) {
    let mut serial = PRINT_MUTEX.lock();
    write(serial.as_mut().unwrap(), args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::serial::print(format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}