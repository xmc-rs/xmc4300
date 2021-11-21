#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Hibernate Domain Status Register"]
    pub hdstat: crate::Reg<hdstat::HDSTAT_SPEC>,
    #[doc = "0x04 - Hibernate Domain Status Clear Register"]
    pub hdclr: crate::Reg<hdclr::HDCLR_SPEC>,
    #[doc = "0x08 - Hibernate Domain Status Set Register"]
    pub hdset: crate::Reg<hdset::HDSET_SPEC>,
    #[doc = "0x0c - Hibernate Domain Control Register"]
    pub hdcr: crate::Reg<hdcr::HDCR_SPEC>,
    _reserved4: [u8; 0x04],
    #[doc = "0x14 - fOSI Control Register"]
    pub oscsictrl: crate::Reg<oscsictrl::OSCSICTRL_SPEC>,
    #[doc = "0x18 - OSC_ULP Status Register"]
    pub osculstat: crate::Reg<osculstat::OSCULSTAT_SPEC>,
    #[doc = "0x1c - OSC_ULP Control Register"]
    pub osculctrl: crate::Reg<osculctrl::OSCULCTRL_SPEC>,
}
#[doc = "HDSTAT register accessor: an alias for `Reg<HDSTAT_SPEC>`"]
pub type HDSTAT = crate::Reg<hdstat::HDSTAT_SPEC>;
#[doc = "Hibernate Domain Status Register"]
pub mod hdstat;
#[doc = "HDCLR register accessor: an alias for `Reg<HDCLR_SPEC>`"]
pub type HDCLR = crate::Reg<hdclr::HDCLR_SPEC>;
#[doc = "Hibernate Domain Status Clear Register"]
pub mod hdclr;
#[doc = "HDSET register accessor: an alias for `Reg<HDSET_SPEC>`"]
pub type HDSET = crate::Reg<hdset::HDSET_SPEC>;
#[doc = "Hibernate Domain Status Set Register"]
pub mod hdset;
#[doc = "HDCR register accessor: an alias for `Reg<HDCR_SPEC>`"]
pub type HDCR = crate::Reg<hdcr::HDCR_SPEC>;
#[doc = "Hibernate Domain Control Register"]
pub mod hdcr;
#[doc = "OSCSICTRL register accessor: an alias for `Reg<OSCSICTRL_SPEC>`"]
pub type OSCSICTRL = crate::Reg<oscsictrl::OSCSICTRL_SPEC>;
#[doc = "fOSI Control Register"]
pub mod oscsictrl;
#[doc = "OSCULSTAT register accessor: an alias for `Reg<OSCULSTAT_SPEC>`"]
pub type OSCULSTAT = crate::Reg<osculstat::OSCULSTAT_SPEC>;
#[doc = "OSC_ULP Status Register"]
pub mod osculstat;
#[doc = "OSCULCTRL register accessor: an alias for `Reg<OSCULCTRL_SPEC>`"]
pub type OSCULCTRL = crate::Reg<osculctrl::OSCULCTRL_SPEC>;
#[doc = "OSC_ULP Control Register"]
pub mod osculctrl;
