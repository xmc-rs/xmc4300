#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Source Address Register"]
    pub sar: crate::Reg<sar::SAR_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Destination Address Register"]
    pub dar: crate::Reg<dar::DAR_SPEC>,
    _reserved2: [u8; 0x0c],
    #[doc = "0x18 - Control Register Low"]
    pub ctll: crate::Reg<ctll::CTLL_SPEC>,
    #[doc = "0x1c - Control Register High"]
    pub ctlh: crate::Reg<ctlh::CTLH_SPEC>,
    _reserved4: [u8; 0x20],
    #[doc = "0x40 - Configuration Register Low"]
    pub cfgl: crate::Reg<cfgl::CFGL_SPEC>,
    #[doc = "0x44 - Configuration Register High"]
    pub cfgh: crate::Reg<cfgh::CFGH_SPEC>,
}
#[doc = "SAR register accessor: an alias for `Reg<SAR_SPEC>`"]
pub type SAR = crate::Reg<sar::SAR_SPEC>;
#[doc = "Source Address Register"]
pub mod sar;
#[doc = "DAR register accessor: an alias for `Reg<DAR_SPEC>`"]
pub type DAR = crate::Reg<dar::DAR_SPEC>;
#[doc = "Destination Address Register"]
pub mod dar;
#[doc = "CTLL register accessor: an alias for `Reg<CTLL_SPEC>`"]
pub type CTLL = crate::Reg<ctll::CTLL_SPEC>;
#[doc = "Control Register Low"]
pub mod ctll;
#[doc = "CTLH register accessor: an alias for `Reg<CTLH_SPEC>`"]
pub type CTLH = crate::Reg<ctlh::CTLH_SPEC>;
#[doc = "Control Register High"]
pub mod ctlh;
#[doc = "CFGL register accessor: an alias for `Reg<CFGL_SPEC>`"]
pub type CFGL = crate::Reg<cfgl::CFGL_SPEC>;
#[doc = "Configuration Register Low"]
pub mod cfgl;
#[doc = "CFGH register accessor: an alias for `Reg<CFGH_SPEC>`"]
pub type CFGH = crate::Reg<cfgh::CFGH_SPEC>;
#[doc = "Configuration Register High"]
pub mod cfgh;
