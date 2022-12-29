#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Node Control Register"]
    pub ncr: NCR,
    #[doc = "0x04 - Node Status Register"]
    pub nsr: NSR,
    #[doc = "0x08 - Node Interrupt Pointer Register"]
    pub nipr: NIPR,
    #[doc = "0x0c - Node Port Control Register"]
    pub npcr: NPCR,
    #[doc = "0x10 - Node Bit Timing Register"]
    pub nbtr: NBTR,
    #[doc = "0x14 - Node Error Counter Register"]
    pub necnt: NECNT,
    #[doc = "0x18 - Node Frame Counter Register"]
    pub nfcr: NFCR,
}
#[doc = "NCR (rw) register accessor: an alias for `Reg<NCR_SPEC>`"]
pub type NCR = crate::Reg<ncr::NCR_SPEC>;
#[doc = "Node Control Register"]
pub mod ncr;
#[doc = "NSR (rw) register accessor: an alias for `Reg<NSR_SPEC>`"]
pub type NSR = crate::Reg<nsr::NSR_SPEC>;
#[doc = "Node Status Register"]
pub mod nsr;
#[doc = "NIPR (rw) register accessor: an alias for `Reg<NIPR_SPEC>`"]
pub type NIPR = crate::Reg<nipr::NIPR_SPEC>;
#[doc = "Node Interrupt Pointer Register"]
pub mod nipr;
#[doc = "NPCR (rw) register accessor: an alias for `Reg<NPCR_SPEC>`"]
pub type NPCR = crate::Reg<npcr::NPCR_SPEC>;
#[doc = "Node Port Control Register"]
pub mod npcr;
#[doc = "NBTR (rw) register accessor: an alias for `Reg<NBTR_SPEC>`"]
pub type NBTR = crate::Reg<nbtr::NBTR_SPEC>;
#[doc = "Node Bit Timing Register"]
pub mod nbtr;
#[doc = "NECNT (rw) register accessor: an alias for `Reg<NECNT_SPEC>`"]
pub type NECNT = crate::Reg<necnt::NECNT_SPEC>;
#[doc = "Node Error Counter Register"]
pub mod necnt;
#[doc = "NFCR (rw) register accessor: an alias for `Reg<NFCR_SPEC>`"]
pub type NFCR = crate::Reg<nfcr::NFCR_SPEC>;
#[doc = "Node Frame Counter Register"]
pub mod nfcr;
