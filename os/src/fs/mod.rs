use alloc::{sync::Arc, vec::Vec};
use lazy_static::*;
use rcore_fs::vfs::*;
use rcore_fs_sfs::SimpleFileSystem;

mod device;

lazy_static! {
    /// The root of file system
    pub static ref ROOT_INODE: Arc<INode> = {

        let img = include_bytes!(env!("SFSIMG"));
        // 将存储磁盘文件的内存范围初始化为虚拟磁盘 Membuf
        let device = Arc::new(unsafe { device::MemBuf::new(img) });

        let sfs = SimpleFileSystem::open(device).expect("failed to open SFS");
        sfs.root_inode()
    };
}

pub trait INodeExt {
    fn read_as_vec(&self) -> Result<Vec<u8>>;
}

impl INodeExt for INode {
    fn read_as_vec(&self) -> Result<Vec<u8>> {
        let size = self.metadata()?.size;
        let mut buf = Vec::with_capacity(size);
        unsafe {
            buf.set_len(size);
        }
        self.read_at(0, buf.as_mut_slice())?;
        Ok(buf)
    }
}

pub fn init() {
    // 打印当前目录下的所有项的名字
    let mut id = 0;
    while let Ok(name) = ROOT_INODE.get_entry(id) {
        id += 1;
        println!("{}", name);
    }
}
