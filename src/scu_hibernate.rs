#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    hdstat: HDSTAT,
    hdclr: HDCLR,
    hdset: HDSET,
    hdcr: HDCR,
    _reserved4: [u8; 0x04],
    oscsictrl: OSCSICTRL,
    osculstat: OSCULSTAT,
    osculctrl: OSCULCTRL,
}
impl RegisterBlock {
    #[doc = "0x00 - Hibernate Domain Status Register"]
    #[inline(always)]
    pub const fn hdstat(&self) -> &HDSTAT {
        &self.hdstat
    }
    #[doc = "0x04 - Hibernate Domain Status Clear Register"]
    #[inline(always)]
    pub const fn hdclr(&self) -> &HDCLR {
        &self.hdclr
    }
    #[doc = "0x08 - Hibernate Domain Status Set Register"]
    #[inline(always)]
    pub const fn hdset(&self) -> &HDSET {
        &self.hdset
    }
    #[doc = "0x0c - Hibernate Domain Control Register"]
    #[inline(always)]
    pub const fn hdcr(&self) -> &HDCR {
        &self.hdcr
    }
    #[doc = "0x14 - fOSI Control Register"]
    #[inline(always)]
    pub const fn oscsictrl(&self) -> &OSCSICTRL {
        &self.oscsictrl
    }
    #[doc = "0x18 - OSC_ULP Status Register"]
    #[inline(always)]
    pub const fn osculstat(&self) -> &OSCULSTAT {
        &self.osculstat
    }
    #[doc = "0x1c - OSC_ULP Control Register"]
    #[inline(always)]
    pub const fn osculctrl(&self) -> &OSCULCTRL {
        &self.osculctrl
    }
}
#[doc = "HDSTAT (r) register accessor: Hibernate Domain Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hdstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdstat`]
module"]
pub type HDSTAT = crate::Reg<hdstat::HDSTAT_SPEC>;
#[doc = "Hibernate Domain Status Register"]
pub mod hdstat;
#[doc = "HDCLR (w) register accessor: Hibernate Domain Status Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdclr`]
module"]
pub type HDCLR = crate::Reg<hdclr::HDCLR_SPEC>;
#[doc = "Hibernate Domain Status Clear Register"]
pub mod hdclr;
#[doc = "HDSET (w) register accessor: Hibernate Domain Status Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdset`]
module"]
pub type HDSET = crate::Reg<hdset::HDSET_SPEC>;
#[doc = "Hibernate Domain Status Set Register"]
pub mod hdset;
#[doc = "HDCR (rw) register accessor: Hibernate Domain Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcr`]
module"]
pub type HDCR = crate::Reg<hdcr::HDCR_SPEC>;
#[doc = "Hibernate Domain Control Register"]
pub mod hdcr;
#[doc = "OSCSICTRL (rw) register accessor: fOSI Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`oscsictrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oscsictrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oscsictrl`]
module"]
pub type OSCSICTRL = crate::Reg<oscsictrl::OSCSICTRL_SPEC>;
#[doc = "fOSI Control Register"]
pub mod oscsictrl;
#[doc = "OSCULSTAT (r) register accessor: OSC_ULP Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`osculstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osculstat`]
module"]
pub type OSCULSTAT = crate::Reg<osculstat::OSCULSTAT_SPEC>;
#[doc = "OSC_ULP Status Register"]
pub mod osculstat;
#[doc = "OSCULCTRL (rw) register accessor: OSC_ULP Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`osculctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osculctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osculctrl`]
module"]
pub type OSCULCTRL = crate::Reg<osculctrl::OSCULCTRL_SPEC>;
#[doc = "OSC_ULP Control Register"]
pub mod osculctrl;
