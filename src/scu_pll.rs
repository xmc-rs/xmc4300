#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PLL Status Register"]
    pub pllstat: PLLSTAT,
    #[doc = "0x04 - PLL Configuration 0 Register"]
    pub pllcon0: PLLCON0,
    #[doc = "0x08 - PLL Configuration 1 Register"]
    pub pllcon1: PLLCON1,
    #[doc = "0x0c - PLL Configuration 2 Register"]
    pub pllcon2: PLLCON2,
    #[doc = "0x10 - USB PLL Status Register"]
    pub usbpllstat: USBPLLSTAT,
    #[doc = "0x14 - USB PLL Configuration Register"]
    pub usbpllcon: USBPLLCON,
    _reserved6: [u8; 0x10],
    #[doc = "0x28 - Clock Multiplexing Status Register"]
    pub clkmxstat: CLKMXSTAT,
}
#[doc = "PLLSTAT (r) register accessor: an alias for `Reg<PLLSTAT_SPEC>`"]
pub type PLLSTAT = crate::Reg<pllstat::PLLSTAT_SPEC>;
#[doc = "PLL Status Register"]
pub mod pllstat;
#[doc = "PLLCON0 (rw) register accessor: an alias for `Reg<PLLCON0_SPEC>`"]
pub type PLLCON0 = crate::Reg<pllcon0::PLLCON0_SPEC>;
#[doc = "PLL Configuration 0 Register"]
pub mod pllcon0;
#[doc = "PLLCON1 (rw) register accessor: an alias for `Reg<PLLCON1_SPEC>`"]
pub type PLLCON1 = crate::Reg<pllcon1::PLLCON1_SPEC>;
#[doc = "PLL Configuration 1 Register"]
pub mod pllcon1;
#[doc = "PLLCON2 (rw) register accessor: an alias for `Reg<PLLCON2_SPEC>`"]
pub type PLLCON2 = crate::Reg<pllcon2::PLLCON2_SPEC>;
#[doc = "PLL Configuration 2 Register"]
pub mod pllcon2;
#[doc = "USBPLLSTAT (r) register accessor: an alias for `Reg<USBPLLSTAT_SPEC>`"]
pub type USBPLLSTAT = crate::Reg<usbpllstat::USBPLLSTAT_SPEC>;
#[doc = "USB PLL Status Register"]
pub mod usbpllstat;
#[doc = "USBPLLCON (rw) register accessor: an alias for `Reg<USBPLLCON_SPEC>`"]
pub type USBPLLCON = crate::Reg<usbpllcon::USBPLLCON_SPEC>;
#[doc = "USB PLL Configuration Register"]
pub mod usbpllcon;
#[doc = "CLKMXSTAT (r) register accessor: an alias for `Reg<CLKMXSTAT_SPEC>`"]
pub type CLKMXSTAT = crate::Reg<clkmxstat::CLKMXSTAT_SPEC>;
#[doc = "Clock Multiplexing Status Register"]
pub mod clkmxstat;
