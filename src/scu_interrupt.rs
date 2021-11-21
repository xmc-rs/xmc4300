#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SCU Service Request Status"]
    pub srstat: crate::Reg<srstat::SRSTAT_SPEC>,
    #[doc = "0x04 - SCU Raw Service Request Status"]
    pub srraw: crate::Reg<srraw::SRRAW_SPEC>,
    #[doc = "0x08 - SCU Service Request Mask"]
    pub srmsk: crate::Reg<srmsk::SRMSK_SPEC>,
    #[doc = "0x0c - SCU Service Request Clear"]
    pub srclr: crate::Reg<srclr::SRCLR_SPEC>,
    #[doc = "0x10 - SCU Service Request Set"]
    pub srset: crate::Reg<srset::SRSET_SPEC>,
    #[doc = "0x14 - SCU Service Request Mask"]
    pub nmireqen: crate::Reg<nmireqen::NMIREQEN_SPEC>,
}
#[doc = "SRSTAT register accessor: an alias for `Reg<SRSTAT_SPEC>`"]
pub type SRSTAT = crate::Reg<srstat::SRSTAT_SPEC>;
#[doc = "SCU Service Request Status"]
pub mod srstat;
#[doc = "SRRAW register accessor: an alias for `Reg<SRRAW_SPEC>`"]
pub type SRRAW = crate::Reg<srraw::SRRAW_SPEC>;
#[doc = "SCU Raw Service Request Status"]
pub mod srraw;
#[doc = "SRMSK register accessor: an alias for `Reg<SRMSK_SPEC>`"]
pub type SRMSK = crate::Reg<srmsk::SRMSK_SPEC>;
#[doc = "SCU Service Request Mask"]
pub mod srmsk;
#[doc = "SRCLR register accessor: an alias for `Reg<SRCLR_SPEC>`"]
pub type SRCLR = crate::Reg<srclr::SRCLR_SPEC>;
#[doc = "SCU Service Request Clear"]
pub mod srclr;
#[doc = "SRSET register accessor: an alias for `Reg<SRSET_SPEC>`"]
pub type SRSET = crate::Reg<srset::SRSET_SPEC>;
#[doc = "SCU Service Request Set"]
pub mod srset;
#[doc = "NMIREQEN register accessor: an alias for `Reg<NMIREQEN_SPEC>`"]
pub type NMIREQEN = crate::Reg<nmireqen::NMIREQEN_SPEC>;
#[doc = "SCU Service Request Mask"]
pub mod nmireqen;
