#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OSC_HP Status Register"]
    pub oschpstat: OSCHPSTAT,
    #[doc = "0x04 - OSC_HP Control Register"]
    pub oschpctrl: OSCHPCTRL,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - Clock Calibration Constant Register"]
    pub clkcalconst: CLKCALCONST,
}
#[doc = "OSCHPSTAT (r) register accessor: an alias for `Reg<OSCHPSTAT_SPEC>`"]
pub type OSCHPSTAT = crate::Reg<oschpstat::OSCHPSTAT_SPEC>;
#[doc = "OSC_HP Status Register"]
pub mod oschpstat;
#[doc = "OSCHPCTRL (rw) register accessor: an alias for `Reg<OSCHPCTRL_SPEC>`"]
pub type OSCHPCTRL = crate::Reg<oschpctrl::OSCHPCTRL_SPEC>;
#[doc = "OSC_HP Control Register"]
pub mod oschpctrl;
#[doc = "CLKCALCONST (r) register accessor: an alias for `Reg<CLKCALCONST_SPEC>`"]
pub type CLKCALCONST = crate::Reg<clkcalconst::CLKCALCONST_SPEC>;
#[doc = "Clock Calibration Constant Register"]
pub mod clkcalconst;
