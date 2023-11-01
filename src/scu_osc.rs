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
#[doc = "OSCHPSTAT (r) register accessor: OSC_HP Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oschpstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oschpstat`]
module"]
pub type OSCHPSTAT = crate::Reg<oschpstat::OSCHPSTAT_SPEC>;
#[doc = "OSC_HP Status Register"]
pub mod oschpstat;
#[doc = "OSCHPCTRL (rw) register accessor: OSC_HP Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oschpctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oschpctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oschpctrl`]
module"]
pub type OSCHPCTRL = crate::Reg<oschpctrl::OSCHPCTRL_SPEC>;
#[doc = "OSC_HP Control Register"]
pub mod oschpctrl;
#[doc = "CLKCALCONST (r) register accessor: Clock Calibration Constant Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkcalconst::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkcalconst`]
module"]
pub type CLKCALCONST = crate::Reg<clkcalconst::CLKCALCONST_SPEC>;
#[doc = "Clock Calibration Constant Register"]
pub mod clkcalconst;
