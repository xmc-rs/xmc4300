#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Parity Error Enable Register"]
    pub peen: PEEN,
    #[doc = "0x04 - Memory Checking Control Register"]
    pub mchkcon: MCHKCON,
    #[doc = "0x08 - Parity Error Trap Enable Register"]
    pub pete: PETE,
    #[doc = "0x0c - Parity Error Reset Enable Register"]
    pub persten: PERSTEN,
    _reserved4: [u8; 0x04],
    #[doc = "0x14 - Parity Error Flag Register"]
    pub peflag: PEFLAG,
    #[doc = "0x18 - Parity Memory Test Pattern Register"]
    pub pmtpr: PMTPR,
    #[doc = "0x1c - Parity Memory Test Select Register"]
    pub pmtsr: PMTSR,
}
#[doc = "PEEN (rw) register accessor: an alias for `Reg<PEEN_SPEC>`"]
pub type PEEN = crate::Reg<peen::PEEN_SPEC>;
#[doc = "Parity Error Enable Register"]
pub mod peen;
#[doc = "MCHKCON (rw) register accessor: an alias for `Reg<MCHKCON_SPEC>`"]
pub type MCHKCON = crate::Reg<mchkcon::MCHKCON_SPEC>;
#[doc = "Memory Checking Control Register"]
pub mod mchkcon;
#[doc = "PETE (rw) register accessor: an alias for `Reg<PETE_SPEC>`"]
pub type PETE = crate::Reg<pete::PETE_SPEC>;
#[doc = "Parity Error Trap Enable Register"]
pub mod pete;
#[doc = "PERSTEN (rw) register accessor: an alias for `Reg<PERSTEN_SPEC>`"]
pub type PERSTEN = crate::Reg<persten::PERSTEN_SPEC>;
#[doc = "Parity Error Reset Enable Register"]
pub mod persten;
#[doc = "PEFLAG (rw) register accessor: an alias for `Reg<PEFLAG_SPEC>`"]
pub type PEFLAG = crate::Reg<peflag::PEFLAG_SPEC>;
#[doc = "Parity Error Flag Register"]
pub mod peflag;
#[doc = "PMTPR (rw) register accessor: an alias for `Reg<PMTPR_SPEC>`"]
pub type PMTPR = crate::Reg<pmtpr::PMTPR_SPEC>;
#[doc = "Parity Memory Test Pattern Register"]
pub mod pmtpr;
#[doc = "PMTSR (rw) register accessor: an alias for `Reg<PMTSR_SPEC>`"]
pub type PMTSR = crate::Reg<pmtsr::PMTSR_SPEC>;
#[doc = "Parity Memory Test Select Register"]
pub mod pmtsr;
