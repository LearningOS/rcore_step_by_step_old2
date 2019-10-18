use rcore_fs::dev::*;
use spin::RwLock;

pub struct MemBuf(RwLock<&'static [u8]>);

impl MemBuf {
    pub unsafe fn new(data: &'static [u8]) -> Self {
        use core::slice;
        MemBuf(RwLock::new(data))
    }
}

impl Device for MemBuf {
    fn read_at(&self, offset: usize, buf: &mut [u8]) -> Result<usize> {
        let slice = self.0.read();
        let len = buf.len().min(slice.len() - offset); // 取磁盘剩余长度和 buf 大小的较小值
        buf[..len].copy_from_slice(&slice[offset..offset + len]);
        Ok(len)
    }

    fn write_at(&self, offset: usize, buf: &[u8]) -> Result<usize> {
        unimplemented!()
    }

    fn sync(&self) -> Result<()> {
        Ok(())
    }
}
