#[doc = r"Register block"]
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
#[doc = "IR (rw) register accessor: an alias for `Reg<IR_SPEC>`"]
pub type IR = crate::Reg<ir::IR_SPEC>;
#[doc = "Input Register"]
pub mod ir;
#[doc = "RES (r) register accessor: an alias for `Reg<RES_SPEC>`"]
pub type RES = crate::Reg<res::RES_SPEC>;
#[doc = "CRC Result Register"]
pub mod res;
#[doc = "CFG (rw) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "CRC Configuration Register"]
pub mod cfg;
#[doc = "STS (rw) register accessor: an alias for `Reg<STS_SPEC>`"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "CRC Status Register"]
pub mod sts;
#[doc = "LENGTH (rw) register accessor: an alias for `Reg<LENGTH_SPEC>`"]
pub type LENGTH = crate::Reg<length::LENGTH_SPEC>;
#[doc = "CRC Length Register"]
pub mod length;
#[doc = "CHECK (rw) register accessor: an alias for `Reg<CHECK_SPEC>`"]
pub type CHECK = crate::Reg<check::CHECK_SPEC>;
#[doc = "CRC Check Register"]
pub mod check;
#[doc = "CRC (rw) register accessor: an alias for `Reg<CRC_SPEC>`"]
pub type CRC = crate::Reg<crc::CRC_SPEC>;
#[doc = "CRC Register"]
pub mod crc;
#[doc = "CTR (rw) register accessor: an alias for `Reg<CTR_SPEC>`"]
pub type CTR = crate::Reg<ctr::CTR_SPEC>;
#[doc = "CRC Test Register"]
pub mod ctr;
