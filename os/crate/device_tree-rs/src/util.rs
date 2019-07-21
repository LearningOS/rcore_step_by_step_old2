use alloc::vec::Vec;
pub use core::{convert, fmt, option, result, str};

#[inline]
pub fn align(val: usize, to: usize) -> usize {
    val + (to - (val % to)) % to
}

#[derive(Debug)]
pub enum SliceReadError {
    UnexpectedEndOfInput,
}

pub type SliceReadResult<T> = Result<T, SliceReadError>;

pub trait SliceRead {
    fn read_be_u32(&self, pos: usize) -> SliceReadResult<u32>;
    fn read_be_u64(&self, pos: usize) -> SliceReadResult<u64>;
    fn read_bstring0(&self, pos: usize) -> SliceReadResult<&[u8]>;
    fn subslice(&self, start: usize, len: usize) -> SliceReadResult<&[u8]>;
}

impl<'a> SliceRead for &'a [u8] {
    fn read_be_u32(&self, pos: usize) -> SliceReadResult<u32> {
        // check size is valid
        if !(pos + 4 <= self.len()) {
            return Err(SliceReadError::UnexpectedEndOfInput);
        }

        Ok((self[pos] as u32) << 24
            | (self[pos + 1] as u32) << 16
            | (self[pos + 2] as u32) << 8
            | (self[pos + 3] as u32))
    }

    fn read_be_u64(&self, pos: usize) -> SliceReadResult<u64> {
        // check size is valid
        if !(pos + 8 <= self.len()) {
            return Err(SliceReadError::UnexpectedEndOfInput);
        }

        Ok((self[pos] as u64) << 56
            | (self[pos + 1] as u64) << 48
            | (self[pos + 2] as u64) << 40
            | (self[pos + 3] as u64) << 32
            | (self[pos + 4] as u64) << 24
            | (self[pos + 5] as u64) << 16
            | (self[pos + 6] as u64) << 8
            | (self[pos + 7] as u64))
    }

    fn read_bstring0(&self, pos: usize) -> SliceReadResult<&[u8]> {
        let mut cur = pos;
        while cur < self.len() {
            if self[cur] == 0 {
                return Ok(&self[pos..cur]);
            }

            cur += 1;
        }

        Err(SliceReadError::UnexpectedEndOfInput)
    }

    fn subslice(&self, start: usize, end: usize) -> SliceReadResult<&[u8]> {
        if !(end < self.len()) {
            return Err(SliceReadError::UnexpectedEndOfInput);
        }

        Ok(&self[start..end])
    }
}

#[derive(Debug)]
pub enum VecWriteError {
    NonContiguousWrite,
    UnalignedWrite,
}

pub type VecWriteResult = Result<(), VecWriteError>;

pub trait VecWrite {
    fn write_be_u32(&mut self, pos: usize, val: u32) -> VecWriteResult;
    fn write_be_u64(&mut self, pos: usize, val: u64) -> VecWriteResult;
    fn write_bstring0(&mut self, val: &str) -> VecWriteResult;
    fn pad(&mut self, alignment: usize) -> VecWriteResult;
}

impl VecWrite for Vec<u8> {
    fn write_be_u32(&mut self, pos: usize, val: u32) -> VecWriteResult {
        if pos % 4 != 0 {
            return Err(VecWriteError::UnalignedWrite);
        }
        if pos > self.len() {
            return Err(VecWriteError::NonContiguousWrite);
        }
        if pos + 4 > self.len() {
            for _ in 0..(pos + 4 - self.len()) {
                self.push(0);
            }
        }
        assert!(pos + 3 < self.len());
        self[pos] = ((val >> 24) & 0xff) as u8;
        self[pos + 1] = ((val >> 16) & 0xff) as u8;
        self[pos + 2] = ((val >> 8) & 0xff) as u8;
        self[pos + 3] = (val & 0xff) as u8;
        Ok(())
    }

    fn write_be_u64(&mut self, pos: usize, val: u64) -> VecWriteResult {
        if pos % 8 != 0 {
            return Err(VecWriteError::UnalignedWrite);
        }
        if pos > self.len() {
            return Err(VecWriteError::NonContiguousWrite);
        }
        if pos > self.len() - 8 {
            for _ in 0..(pos + 8 - self.len()) {
                self.push(0);
            }
        }
        assert!(pos + 7 < self.len());
        self[pos] = ((val >> 56) & 0xff) as u8;
        self[pos + 1] = ((val >> 48) & 0xff) as u8;
        self[pos + 2] = ((val >> 40) & 0xff) as u8;
        self[pos + 3] = ((val >> 32) & 0xff) as u8;
        self[pos + 4] = ((val >> 24) & 0xff) as u8;
        self[pos + 5] = ((val >> 16) & 0xff) as u8;
        self[pos + 6] = ((val >> 8) & 0xff) as u8;
        self[pos + 7] = (val & 0xff) as u8;
        Ok(())
    }

    fn write_bstring0(&mut self, val: &str) -> VecWriteResult {
        for b in val.bytes() {
            self.push(b);
        }
        self.push(0);
        Ok(())
    }

    fn pad(&mut self, alignment: usize) -> VecWriteResult {
        let misalignment = self.len() % alignment;
        if misalignment > 0 {
            for _ in 0..(alignment - misalignment) {
                self.push(0);
            }
        }
        Ok(())
    }
}
