use super::syscall;

pub fn putchar(ch: char) {
    syscall::sys_write(ch as u8);
}

pub fn puts(s: &str) {
    for ch in s.chars() {
        putchar(ch);
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::io::_print(format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

use core::fmt::{self, Write};

struct StdOut;

impl fmt::Write for StdOut {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        puts(s);
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    StdOut.write_fmt(args).unwrap();
}

pub const STDIN: usize = 0;

pub fn getc() -> u8 {
    let mut c = 0u8;
    loop {
        let len = syscall::sys_read(STDIN, &mut c, 1);
        match len {
            1 => return c,
            0 => continue,
            _ => panic!("read stdin len = {}", len),
        }
    }
}