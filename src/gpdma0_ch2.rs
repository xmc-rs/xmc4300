#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    sar: SAR,
    _reserved1: [u8; 0x04],
    dar: DAR,
    _reserved2: [u8; 0x0c],
    ctll: CTLL,
    ctlh: CTLH,
    _reserved4: [u8; 0x20],
    cfgl: CFGL,
    cfgh: CFGH,
}
impl RegisterBlock {
    #[doc = "0x00 - Source Address Register"]
    #[inline(always)]
    pub const fn sar(&self) -> &SAR {
        &self.sar
    }
    #[doc = "0x08 - Destination Address Register"]
    #[inline(always)]
    pub const fn dar(&self) -> &DAR {
        &self.dar
    }
    #[doc = "0x18 - Control Register Low"]
    #[inline(always)]
    pub const fn ctll(&self) -> &CTLL {
        &self.ctll
    }
    #[doc = "0x1c - Control Register High"]
    #[inline(always)]
    pub const fn ctlh(&self) -> &CTLH {
        &self.ctlh
    }
    #[doc = "0x40 - Configuration Register Low"]
    #[inline(always)]
    pub const fn cfgl(&self) -> &CFGL {
        &self.cfgl
    }
    #[doc = "0x44 - Configuration Register High"]
    #[inline(always)]
    pub const fn cfgh(&self) -> &CFGH {
        &self.cfgh
    }
}
#[doc = "SAR (rw) register accessor: Source Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar`]
module"]
pub type SAR = crate::Reg<sar::SAR_SPEC>;
#[doc = "Source Address Register"]
pub mod sar;
#[doc = "DAR (rw) register accessor: Destination Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dar`]
module"]
pub type DAR = crate::Reg<dar::DAR_SPEC>;
#[doc = "Destination Address Register"]
pub mod dar;
#[doc = "CTLL (rw) register accessor: Control Register Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctll::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctll::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctll`]
module"]
pub type CTLL = crate::Reg<ctll::CTLL_SPEC>;
#[doc = "Control Register Low"]
pub mod ctll;
#[doc = "CTLH (rw) register accessor: Control Register High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctlh`]
module"]
pub type CTLH = crate::Reg<ctlh::CTLH_SPEC>;
#[doc = "Control Register High"]
pub mod ctlh;
#[doc = "CFGL (rw) register accessor: Configuration Register Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgl`]
module"]
pub type CFGL = crate::Reg<cfgl::CFGL_SPEC>;
#[doc = "Configuration Register Low"]
pub mod cfgl;
#[doc = "CFGH (rw) register accessor: Configuration Register High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgh`]
module"]
pub type CFGH = crate::Reg<cfgh::CFGH_SPEC>;
#[doc = "Configuration Register High"]
pub mod cfgh;
