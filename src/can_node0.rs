#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Node Control Register"]
    pub ncr: crate::Reg<ncr::NCR_SPEC>,
    #[doc = "0x04 - Node Status Register"]
    pub nsr: crate::Reg<nsr::NSR_SPEC>,
    #[doc = "0x08 - Node Interrupt Pointer Register"]
    pub nipr: crate::Reg<nipr::NIPR_SPEC>,
    #[doc = "0x0c - Node Port Control Register"]
    pub npcr: crate::Reg<npcr::NPCR_SPEC>,
    #[doc = "0x10 - Node Bit Timing Register"]
    pub nbtr: crate::Reg<nbtr::NBTR_SPEC>,
    #[doc = "0x14 - Node Error Counter Register"]
    pub necnt: crate::Reg<necnt::NECNT_SPEC>,
    #[doc = "0x18 - Node Frame Counter Register"]
    pub nfcr: crate::Reg<nfcr::NFCR_SPEC>,
}
#[doc = "NCR register accessor: an alias for `Reg<NCR_SPEC>`"]
pub type NCR = crate::Reg<ncr::NCR_SPEC>;
#[doc = "Node Control Register"]
pub mod ncr;
#[doc = "NSR register accessor: an alias for `Reg<NSR_SPEC>`"]
pub type NSR = crate::Reg<nsr::NSR_SPEC>;
#[doc = "Node Status Register"]
pub mod nsr;
#[doc = "NIPR register accessor: an alias for `Reg<NIPR_SPEC>`"]
pub type NIPR = crate::Reg<nipr::NIPR_SPEC>;
#[doc = "Node Interrupt Pointer Register"]
pub mod nipr;
#[doc = "NPCR register accessor: an alias for `Reg<NPCR_SPEC>`"]
pub type NPCR = crate::Reg<npcr::NPCR_SPEC>;
#[doc = "Node Port Control Register"]
pub mod npcr;
#[doc = "NBTR register accessor: an alias for `Reg<NBTR_SPEC>`"]
pub type NBTR = crate::Reg<nbtr::NBTR_SPEC>;
#[doc = "Node Bit Timing Register"]
pub mod nbtr;
#[doc = "NECNT register accessor: an alias for `Reg<NECNT_SPEC>`"]
pub type NECNT = crate::Reg<necnt::NECNT_SPEC>;
#[doc = "Node Error Counter Register"]
pub mod necnt;
#[doc = "NFCR register accessor: an alias for `Reg<NFCR_SPEC>`"]
pub type NFCR = crate::Reg<nfcr::NFCR_SPEC>;
#[doc = "Node Frame Counter Register"]
pub mod nfcr;
