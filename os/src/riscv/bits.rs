pub const TVEC_MODE: u32 = 0x3;
pub const TVEC_BASE: u32 = !TVEC_MODE;

pub const STATUS_UIE: u32 = 1 << 0;
pub const STATUS_SIE: u32 = 1 << 1;
pub const STATUS_UPIE: u32 = 1 << 4;
pub const STATUS_SPIE: u32 = 1 << 5;
pub const STATUS_SPP: u32 = 1 << 8;
pub const STATUS_FS: u32 = 3 << 13;
pub const STATUS_XS: u32 = 3 << 15;
pub const STATUS_SUM: u32 = 1 << 18;
pub const STATUS_MXR: u32 = 1 << 19;
pub const STATUS_SD: u32 = 1 << 31;

pub const STATUS_MPP_M: u32 = 3 << 11;
pub const STATUS_MPP_S: u32 = 1 << 11;
pub const STATUS_MPP_U: u32 = 0 << 11;

// Mask of writable bits in sstatus.
pub const SSTATUS_WRITABLE_MASK: u32 =
    STATUS_MXR |
STATUS_SUM |
STATUS_FS |
STATUS_SPP |
STATUS_SPIE |
STATUS_SIE;
pub const SSTATUS_DYNAMIC_MASK: u32 = STATUS_SD | STATUS_FS;

pub const IP_SSIP: u32 = 1 << 1;
pub const IP_STIP: u32 = 1 << 5;
pub const IP_SEIP: u32 = 1 << 9;

pub const IE_SSIE: u32 = 1 << 1;
pub const IE_STIE: u32 = 1 << 5;
pub const IE_SEIE: u32 = 1 << 9;

pub const SATP_MODE: u32 = 0x1 << 31;
pub const SATP_ASID: u32 = 0x1ff << 22;
pub const SATP_PPN: u32 = 0x3fffff;

pub const SSTACK_BASE: u32 = 0xc0a00000 - 32*4;

pub const SCAUSE_INSN_MISALIGNED: u32 = 0;
pub const SCAUSE_INSN_ACCESS_FAULT: u32 = 1;
pub const SCAUSE_ILLEGAL_INSN: u32 = 2;
pub const SCAUSE_BREAKPOINT: u32 = 3;
pub const SCAUSE_LOAD_ACCESS_FAULT: u32 = 5;
pub const SCAUSE_ATOMIC_MISALIGNED: u32 = 6;
pub const SCAUSE_STORE_ACCESS_FAULT: u32 = 7;
pub const SCAUSE_ENV_CALL: u32 = 8;
pub const SCAUSE_INSN_PAGE_FAULT: u32 = 12;
pub const SCAUSE_LOAD_PAGE_FAULT: u32 = 13;
pub const SCAUSE_STORE_PAGE_FAULT: u32 = 15;
