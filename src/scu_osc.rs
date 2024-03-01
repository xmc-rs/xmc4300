#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    oschpstat: Oschpstat,
    oschpctrl: Oschpctrl,
    _reserved2: [u8; 0x04],
    clkcalconst: Clkcalconst,
}
impl RegisterBlock {
    #[doc = "0x00 - OSC_HP Status Register"]
    #[inline(always)]
    pub const fn oschpstat(&self) -> &Oschpstat {
        &self.oschpstat
    }
    #[doc = "0x04 - OSC_HP Control Register"]
    #[inline(always)]
    pub const fn oschpctrl(&self) -> &Oschpctrl {
        &self.oschpctrl
    }
    #[doc = "0x0c - Clock Calibration Constant Register"]
    #[inline(always)]
    pub const fn clkcalconst(&self) -> &Clkcalconst {
        &self.clkcalconst
    }
}
#[doc = "OSCHPSTAT (r) register accessor: OSC_HP Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oschpstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oschpstat`]
module"]
#[doc(alias = "OSCHPSTAT")]
pub type Oschpstat = crate::Reg<oschpstat::OschpstatSpec>;
#[doc = "OSC_HP Status Register"]
pub mod oschpstat;
#[doc = "OSCHPCTRL (rw) register accessor: OSC_HP Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oschpctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oschpctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oschpctrl`]
module"]
#[doc(alias = "OSCHPCTRL")]
pub type Oschpctrl = crate::Reg<oschpctrl::OschpctrlSpec>;
#[doc = "OSC_HP Control Register"]
pub mod oschpctrl;
#[doc = "CLKCALCONST (r) register accessor: Clock Calibration Constant Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkcalconst::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkcalconst`]
module"]
#[doc(alias = "CLKCALCONST")]
pub type Clkcalconst = crate::Reg<clkcalconst::ClkcalconstSpec>;
#[doc = "Clock Calibration Constant Register"]
pub mod clkcalconst;
