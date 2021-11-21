#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Source Address Register"]
    pub sar: crate::Reg<sar::SAR_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Destination Address Register"]
    pub dar: crate::Reg<dar::DAR_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x10 - Linked List Pointer Register"]
    pub llp: crate::Reg<llp::LLP_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x18 - Control Register Low"]
    pub ctll: crate::Reg<ctll::CTLL_SPEC>,
    #[doc = "0x1c - Control Register High"]
    pub ctlh: crate::Reg<ctlh::CTLH_SPEC>,
    #[doc = "0x20 - Source Status Register"]
    pub sstat: crate::Reg<sstat::SSTAT_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x28 - Destination Status Register"]
    pub dstat: crate::Reg<dstat::DSTAT_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x30 - Source Status Address Register"]
    pub sstatar: crate::Reg<sstatar::SSTATAR_SPEC>,
    _reserved8: [u8; 0x04],
    #[doc = "0x38 - Destination Status Address Register"]
    pub dstatar: crate::Reg<dstatar::DSTATAR_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0x40 - Configuration Register Low"]
    pub cfgl: crate::Reg<cfgl::CFGL_SPEC>,
    #[doc = "0x44 - Configuration Register High"]
    pub cfgh: crate::Reg<cfgh::CFGH_SPEC>,
    #[doc = "0x48 - Source Gather Register"]
    pub sgr: crate::Reg<sgr::SGR_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x50 - Destination Scatter Register"]
    pub dsr: crate::Reg<dsr::DSR_SPEC>,
}
#[doc = "SAR register accessor: an alias for `Reg<SAR_SPEC>`"]
pub type SAR = crate::Reg<sar::SAR_SPEC>;
#[doc = "Source Address Register"]
pub mod sar;
#[doc = "DAR register accessor: an alias for `Reg<DAR_SPEC>`"]
pub type DAR = crate::Reg<dar::DAR_SPEC>;
#[doc = "Destination Address Register"]
pub mod dar;
#[doc = "LLP register accessor: an alias for `Reg<LLP_SPEC>`"]
pub type LLP = crate::Reg<llp::LLP_SPEC>;
#[doc = "Linked List Pointer Register"]
pub mod llp;
#[doc = "CTLL register accessor: an alias for `Reg<CTLL_SPEC>`"]
pub type CTLL = crate::Reg<ctll::CTLL_SPEC>;
#[doc = "Control Register Low"]
pub mod ctll;
#[doc = "CTLH register accessor: an alias for `Reg<CTLH_SPEC>`"]
pub type CTLH = crate::Reg<ctlh::CTLH_SPEC>;
#[doc = "Control Register High"]
pub mod ctlh;
#[doc = "SSTAT register accessor: an alias for `Reg<SSTAT_SPEC>`"]
pub type SSTAT = crate::Reg<sstat::SSTAT_SPEC>;
#[doc = "Source Status Register"]
pub mod sstat;
#[doc = "DSTAT register accessor: an alias for `Reg<DSTAT_SPEC>`"]
pub type DSTAT = crate::Reg<dstat::DSTAT_SPEC>;
#[doc = "Destination Status Register"]
pub mod dstat;
#[doc = "SSTATAR register accessor: an alias for `Reg<SSTATAR_SPEC>`"]
pub type SSTATAR = crate::Reg<sstatar::SSTATAR_SPEC>;
#[doc = "Source Status Address Register"]
pub mod sstatar;
#[doc = "DSTATAR register accessor: an alias for `Reg<DSTATAR_SPEC>`"]
pub type DSTATAR = crate::Reg<dstatar::DSTATAR_SPEC>;
#[doc = "Destination Status Address Register"]
pub mod dstatar;
#[doc = "CFGL register accessor: an alias for `Reg<CFGL_SPEC>`"]
pub type CFGL = crate::Reg<cfgl::CFGL_SPEC>;
#[doc = "Configuration Register Low"]
pub mod cfgl;
#[doc = "CFGH register accessor: an alias for `Reg<CFGH_SPEC>`"]
pub type CFGH = crate::Reg<cfgh::CFGH_SPEC>;
#[doc = "Configuration Register High"]
pub mod cfgh;
#[doc = "SGR register accessor: an alias for `Reg<SGR_SPEC>`"]
pub type SGR = crate::Reg<sgr::SGR_SPEC>;
#[doc = "Source Gather Register"]
pub mod sgr;
#[doc = "DSR register accessor: an alias for `Reg<DSR_SPEC>`"]
pub type DSR = crate::Reg<dsr::DSR_SPEC>;
#[doc = "Destination Scatter Register"]
pub mod dsr;
