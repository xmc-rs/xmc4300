#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    hdstat: Hdstat,
    hdclr: Hdclr,
    hdset: Hdset,
    hdcr: Hdcr,
    _reserved4: [u8; 0x04],
    oscsictrl: Oscsictrl,
    osculstat: Osculstat,
    osculctrl: Osculctrl,
}
impl RegisterBlock {
    #[doc = "0x00 - Hibernate Domain Status Register"]
    #[inline(always)]
    pub const fn hdstat(&self) -> &Hdstat {
        &self.hdstat
    }
    #[doc = "0x04 - Hibernate Domain Status Clear Register"]
    #[inline(always)]
    pub const fn hdclr(&self) -> &Hdclr {
        &self.hdclr
    }
    #[doc = "0x08 - Hibernate Domain Status Set Register"]
    #[inline(always)]
    pub const fn hdset(&self) -> &Hdset {
        &self.hdset
    }
    #[doc = "0x0c - Hibernate Domain Control Register"]
    #[inline(always)]
    pub const fn hdcr(&self) -> &Hdcr {
        &self.hdcr
    }
    #[doc = "0x14 - fOSI Control Register"]
    #[inline(always)]
    pub const fn oscsictrl(&self) -> &Oscsictrl {
        &self.oscsictrl
    }
    #[doc = "0x18 - OSC_ULP Status Register"]
    #[inline(always)]
    pub const fn osculstat(&self) -> &Osculstat {
        &self.osculstat
    }
    #[doc = "0x1c - OSC_ULP Control Register"]
    #[inline(always)]
    pub const fn osculctrl(&self) -> &Osculctrl {
        &self.osculctrl
    }
}
#[doc = "HDSTAT (r) register accessor: Hibernate Domain Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdstat`]
module"]
#[doc(alias = "HDSTAT")]
pub type Hdstat = crate::Reg<hdstat::HdstatSpec>;
#[doc = "Hibernate Domain Status Register"]
pub mod hdstat;
#[doc = "HDCLR (w) register accessor: Hibernate Domain Status Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdclr`]
module"]
#[doc(alias = "HDCLR")]
pub type Hdclr = crate::Reg<hdclr::HdclrSpec>;
#[doc = "Hibernate Domain Status Clear Register"]
pub mod hdclr;
#[doc = "HDSET (w) register accessor: Hibernate Domain Status Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdset`]
module"]
#[doc(alias = "HDSET")]
pub type Hdset = crate::Reg<hdset::HdsetSpec>;
#[doc = "Hibernate Domain Status Set Register"]
pub mod hdset;
#[doc = "HDCR (rw) register accessor: Hibernate Domain Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcr`]
module"]
#[doc(alias = "HDCR")]
pub type Hdcr = crate::Reg<hdcr::HdcrSpec>;
#[doc = "Hibernate Domain Control Register"]
pub mod hdcr;
#[doc = "OSCSICTRL (rw) register accessor: fOSI Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oscsictrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oscsictrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oscsictrl`]
module"]
#[doc(alias = "OSCSICTRL")]
pub type Oscsictrl = crate::Reg<oscsictrl::OscsictrlSpec>;
#[doc = "fOSI Control Register"]
pub mod oscsictrl;
#[doc = "OSCULSTAT (r) register accessor: OSC_ULP Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osculstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osculstat`]
module"]
#[doc(alias = "OSCULSTAT")]
pub type Osculstat = crate::Reg<osculstat::OsculstatSpec>;
#[doc = "OSC_ULP Status Register"]
pub mod osculstat;
#[doc = "OSCULCTRL (rw) register accessor: OSC_ULP Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osculctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osculctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osculctrl`]
module"]
#[doc(alias = "OSCULCTRL")]
pub type Osculctrl = crate::Reg<osculctrl::OsculctrlSpec>;
#[doc = "OSC_ULP Control Register"]
pub mod osculctrl;
