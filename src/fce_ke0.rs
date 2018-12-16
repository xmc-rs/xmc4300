#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Input Register"]
    pub ir: IR,
    #[doc = "0x04 - CRC Result Register"]
    pub res: RES,
    #[doc = "0x08 - CRC Configuration Register"]
    pub cfg: CFG,
    #[doc = "0x0c - CRC Status Register"]
    pub sts: STS,
    #[doc = "0x10 - CRC Length Register"]
    pub length: LENGTH,
    #[doc = "0x14 - CRC Check Register"]
    pub check: CHECK,
    #[doc = "0x18 - CRC Register"]
    pub crc: CRC,
    #[doc = "0x1c - CRC Test Register"]
    pub ctr: CTR,
}
#[doc = "Input Register"]
pub struct IR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input Register"]
pub mod ir;
#[doc = "CRC Result Register"]
pub struct RES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Result Register"]
pub mod res;
#[doc = "CRC Configuration Register"]
pub struct CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Configuration Register"]
pub mod cfg;
#[doc = "CRC Status Register"]
pub struct STS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Status Register"]
pub mod sts;
#[doc = "CRC Length Register"]
pub struct LENGTH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Length Register"]
pub mod length;
#[doc = "CRC Check Register"]
pub struct CHECK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Check Register"]
pub mod check;
#[doc = "CRC Register"]
pub struct CRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Register"]
pub mod crc;
#[doc = "CRC Test Register"]
pub struct CTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Test Register"]
pub mod ctr;
