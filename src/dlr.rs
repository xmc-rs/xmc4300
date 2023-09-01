#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Overrun Status"]
    pub ovrstat: OVRSTAT,
    #[doc = "0x04 - Overrun Clear"]
    pub ovrclr: OVRCLR,
    #[doc = "0x08 - Service Request Selection 0"]
    pub srsel0: SRSEL0,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Line Enable"]
    pub lnen: LNEN,
}
#[doc = "OVRSTAT (r) register accessor: Overrun Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ovrstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ovrstat`]
module"]
pub type OVRSTAT = crate::Reg<ovrstat::OVRSTAT_SPEC>;
#[doc = "Overrun Status"]
pub mod ovrstat;
#[doc = "OVRCLR (w) register accessor: Overrun Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ovrclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ovrclr`]
module"]
pub type OVRCLR = crate::Reg<ovrclr::OVRCLR_SPEC>;
#[doc = "Overrun Clear"]
pub mod ovrclr;
#[doc = "SRSEL0 (rw) register accessor: Service Request Selection 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsel0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srsel0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`srsel0`]
module"]
pub type SRSEL0 = crate::Reg<srsel0::SRSEL0_SPEC>;
#[doc = "Service Request Selection 0"]
pub mod srsel0;
#[doc = "LNEN (rw) register accessor: Line Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lnen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lnen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lnen`]
module"]
pub type LNEN = crate::Reg<lnen::LNEN_SPEC>;
#[doc = "Line Enable"]
pub mod lnen;
