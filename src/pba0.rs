#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Peripheral Bridge Status Register"]
    pub sts: STS,
    #[doc = "0x04 - PBA Write Error Address Register"]
    pub waddr: WADDR,
}
#[doc = "STS (rw) register accessor: an alias for `Reg<STS_SPEC>`"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "Peripheral Bridge Status Register"]
pub mod sts;
#[doc = "WADDR (r) register accessor: an alias for `Reg<WADDR_SPEC>`"]
pub type WADDR = crate::Reg<waddr::WADDR_SPEC>;
#[doc = "PBA Write Error Address Register"]
pub mod waddr;
