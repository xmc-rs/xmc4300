#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SCU Service Request Status"]
    pub srstat: SRSTAT,
    #[doc = "0x04 - SCU Raw Service Request Status"]
    pub srraw: SRRAW,
    #[doc = "0x08 - SCU Service Request Mask"]
    pub srmsk: SRMSK,
    #[doc = "0x0c - SCU Service Request Clear"]
    pub srclr: SRCLR,
    #[doc = "0x10 - SCU Service Request Set"]
    pub srset: SRSET,
    #[doc = "0x14 - SCU Service Request Mask"]
    pub nmireqen: NMIREQEN,
}
#[doc = "SRSTAT (r) register accessor: an alias for `Reg<SRSTAT_SPEC>`"]
pub type SRSTAT = crate::Reg<srstat::SRSTAT_SPEC>;
#[doc = "SCU Service Request Status"]
pub mod srstat;
#[doc = "SRRAW (r) register accessor: an alias for `Reg<SRRAW_SPEC>`"]
pub type SRRAW = crate::Reg<srraw::SRRAW_SPEC>;
#[doc = "SCU Raw Service Request Status"]
pub mod srraw;
#[doc = "SRMSK (rw) register accessor: an alias for `Reg<SRMSK_SPEC>`"]
pub type SRMSK = crate::Reg<srmsk::SRMSK_SPEC>;
#[doc = "SCU Service Request Mask"]
pub mod srmsk;
#[doc = "SRCLR (w) register accessor: an alias for `Reg<SRCLR_SPEC>`"]
pub type SRCLR = crate::Reg<srclr::SRCLR_SPEC>;
#[doc = "SCU Service Request Clear"]
pub mod srclr;
#[doc = "SRSET (w) register accessor: an alias for `Reg<SRSET_SPEC>`"]
pub type SRSET = crate::Reg<srset::SRSET_SPEC>;
#[doc = "SCU Service Request Set"]
pub mod srset;
#[doc = "NMIREQEN (rw) register accessor: an alias for `Reg<NMIREQEN_SPEC>`"]
pub type NMIREQEN = crate::Reg<nmireqen::NMIREQEN_SPEC>;
#[doc = "SCU Service Request Mask"]
pub mod nmireqen;
