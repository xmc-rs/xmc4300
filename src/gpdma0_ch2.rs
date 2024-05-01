#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sar: Sar,
    _reserved1: [u8; 0x04],
    dar: Dar,
    _reserved2: [u8; 0x0c],
    ctll: Ctll,
    ctlh: Ctlh,
    _reserved4: [u8; 0x20],
    cfgl: Cfgl,
    cfgh: Cfgh,
}
impl RegisterBlock {
    #[doc = "0x00 - Source Address Register"]
    #[inline(always)]
    pub const fn sar(&self) -> &Sar {
        &self.sar
    }
    #[doc = "0x08 - Destination Address Register"]
    #[inline(always)]
    pub const fn dar(&self) -> &Dar {
        &self.dar
    }
    #[doc = "0x18 - Control Register Low"]
    #[inline(always)]
    pub const fn ctll(&self) -> &Ctll {
        &self.ctll
    }
    #[doc = "0x1c - Control Register High"]
    #[inline(always)]
    pub const fn ctlh(&self) -> &Ctlh {
        &self.ctlh
    }
    #[doc = "0x40 - Configuration Register Low"]
    #[inline(always)]
    pub const fn cfgl(&self) -> &Cfgl {
        &self.cfgl
    }
    #[doc = "0x44 - Configuration Register High"]
    #[inline(always)]
    pub const fn cfgh(&self) -> &Cfgh {
        &self.cfgh
    }
}
#[doc = "SAR (rw) register accessor: Source Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar`]
module"]
#[doc(alias = "SAR")]
pub type Sar = crate::Reg<sar::SarSpec>;
#[doc = "Source Address Register"]
pub mod sar;
#[doc = "DAR (rw) register accessor: Destination Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dar`]
module"]
#[doc(alias = "DAR")]
pub type Dar = crate::Reg<dar::DarSpec>;
#[doc = "Destination Address Register"]
pub mod dar;
#[doc = "CTLL (rw) register accessor: Control Register Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctll::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctll::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctll`]
module"]
#[doc(alias = "CTLL")]
pub type Ctll = crate::Reg<ctll::CtllSpec>;
#[doc = "Control Register Low"]
pub mod ctll;
#[doc = "CTLH (rw) register accessor: Control Register High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctlh`]
module"]
#[doc(alias = "CTLH")]
pub type Ctlh = crate::Reg<ctlh::CtlhSpec>;
#[doc = "Control Register High"]
pub mod ctlh;
#[doc = "CFGL (rw) register accessor: Configuration Register Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgl`]
module"]
#[doc(alias = "CFGL")]
pub type Cfgl = crate::Reg<cfgl::CfglSpec>;
#[doc = "Configuration Register Low"]
pub mod cfgl;
#[doc = "CFGH (rw) register accessor: Configuration Register High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgh`]
module"]
#[doc(alias = "CFGH")]
pub type Cfgh = crate::Reg<cfgh::CfghSpec>;
#[doc = "Configuration Register High"]
pub mod cfgh;
