#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ovrstat: OVRSTAT,
    ovrclr: OVRCLR,
    srsel0: SRSEL0,
    _reserved3: [u8; 0x04],
    lnen: LNEN,
}
impl RegisterBlock {
    #[doc = "0x00 - Overrun Status"]
    #[inline(always)]
    pub const fn ovrstat(&self) -> &OVRSTAT {
        &self.ovrstat
    }
    #[doc = "0x04 - Overrun Clear"]
    #[inline(always)]
    pub const fn ovrclr(&self) -> &OVRCLR {
        &self.ovrclr
    }
    #[doc = "0x08 - Service Request Selection 0"]
    #[inline(always)]
    pub const fn srsel0(&self) -> &SRSEL0 {
        &self.srsel0
    }
    #[doc = "0x10 - Line Enable"]
    #[inline(always)]
    pub const fn lnen(&self) -> &LNEN {
        &self.lnen
    }
}
#[doc = "OVRSTAT (r) register accessor: Overrun Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ovrstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ovrstat`]
module"]
pub type OVRSTAT = crate::Reg<ovrstat::OVRSTAT_SPEC>;
#[doc = "Overrun Status"]
pub mod ovrstat;
#[doc = "OVRCLR (w) register accessor: Overrun Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ovrclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ovrclr`]
module"]
pub type OVRCLR = crate::Reg<ovrclr::OVRCLR_SPEC>;
#[doc = "Overrun Clear"]
pub mod ovrclr;
#[doc = "SRSEL0 (rw) register accessor: Service Request Selection 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsel0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srsel0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srsel0`]
module"]
pub type SRSEL0 = crate::Reg<srsel0::SRSEL0_SPEC>;
#[doc = "Service Request Selection 0"]
pub mod srsel0;
#[doc = "LNEN (rw) register accessor: Line Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lnen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lnen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lnen`]
module"]
pub type LNEN = crate::Reg<lnen::LNEN_SPEC>;
#[doc = "Line Enable"]
pub mod lnen;
