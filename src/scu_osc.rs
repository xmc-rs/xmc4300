#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OSC_HP Status Register"]
    pub oschpstat: crate::Reg<oschpstat::OSCHPSTAT_SPEC>,
    #[doc = "0x04 - OSC_HP Control Register"]
    pub oschpctrl: crate::Reg<oschpctrl::OSCHPCTRL_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - Clock Calibration Constant Register"]
    pub clkcalconst: crate::Reg<clkcalconst::CLKCALCONST_SPEC>,
}
#[doc = "OSCHPSTAT register accessor: an alias for `Reg<OSCHPSTAT_SPEC>`"]
pub type OSCHPSTAT = crate::Reg<oschpstat::OSCHPSTAT_SPEC>;
#[doc = "OSC_HP Status Register"]
pub mod oschpstat;
#[doc = "OSCHPCTRL register accessor: an alias for `Reg<OSCHPCTRL_SPEC>`"]
pub type OSCHPCTRL = crate::Reg<oschpctrl::OSCHPCTRL_SPEC>;
#[doc = "OSC_HP Control Register"]
pub mod oschpctrl;
#[doc = "CLKCALCONST register accessor: an alias for `Reg<CLKCALCONST_SPEC>`"]
pub type CLKCALCONST = crate::Reg<clkcalconst::CLKCALCONST_SPEC>;
#[doc = "Clock Calibration Constant Register"]
pub mod clkcalconst;
