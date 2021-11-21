#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Overrun Status"]
    pub ovrstat: crate::Reg<ovrstat::OVRSTAT_SPEC>,
    #[doc = "0x04 - Overrun Clear"]
    pub ovrclr: crate::Reg<ovrclr::OVRCLR_SPEC>,
    #[doc = "0x08 - Service Request Selection 0"]
    pub srsel0: crate::Reg<srsel0::SRSEL0_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Line Enable"]
    pub lnen: crate::Reg<lnen::LNEN_SPEC>,
}
#[doc = "OVRSTAT register accessor: an alias for `Reg<OVRSTAT_SPEC>`"]
pub type OVRSTAT = crate::Reg<ovrstat::OVRSTAT_SPEC>;
#[doc = "Overrun Status"]
pub mod ovrstat;
#[doc = "OVRCLR register accessor: an alias for `Reg<OVRCLR_SPEC>`"]
pub type OVRCLR = crate::Reg<ovrclr::OVRCLR_SPEC>;
#[doc = "Overrun Clear"]
pub mod ovrclr;
#[doc = "SRSEL0 register accessor: an alias for `Reg<SRSEL0_SPEC>`"]
pub type SRSEL0 = crate::Reg<srsel0::SRSEL0_SPEC>;
#[doc = "Service Request Selection 0"]
pub mod srsel0;
#[doc = "LNEN register accessor: an alias for `Reg<LNEN_SPEC>`"]
pub type LNEN = crate::Reg<lnen::LNEN_SPEC>;
#[doc = "Line Enable"]
pub mod lnen;
