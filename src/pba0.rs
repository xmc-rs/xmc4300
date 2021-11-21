#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Peripheral Bridge Status Register"]
    pub sts: crate::Reg<sts::STS_SPEC>,
    #[doc = "0x04 - PBA Write Error Address Register"]
    pub waddr: crate::Reg<waddr::WADDR_SPEC>,
}
#[doc = "STS register accessor: an alias for `Reg<STS_SPEC>`"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "Peripheral Bridge Status Register"]
pub mod sts;
#[doc = "WADDR register accessor: an alias for `Reg<WADDR_SPEC>`"]
pub type WADDR = crate::Reg<waddr::WADDR_SPEC>;
#[doc = "PBA Write Error Address Register"]
pub mod waddr;
