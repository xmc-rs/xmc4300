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
#[doc = "PLLSTAT (r) register accessor: PLL Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pllstat`]
module"]
pub type PLLSTAT = crate::Reg<pllstat::PLLSTAT_SPEC>;
#[doc = "PLL Status Register"]
pub mod pllstat;
#[doc = "PLLCON0 (rw) register accessor: PLL Configuration 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllcon0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllcon0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pllcon0`]
module"]
pub type PLLCON0 = crate::Reg<pllcon0::PLLCON0_SPEC>;
#[doc = "PLL Configuration 0 Register"]
pub mod pllcon0;
#[doc = "PLLCON1 (rw) register accessor: PLL Configuration 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllcon1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllcon1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pllcon1`]
module"]
pub type PLLCON1 = crate::Reg<pllcon1::PLLCON1_SPEC>;
#[doc = "PLL Configuration 1 Register"]
pub mod pllcon1;
#[doc = "PLLCON2 (rw) register accessor: PLL Configuration 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllcon2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllcon2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pllcon2`]
module"]
pub type PLLCON2 = crate::Reg<pllcon2::PLLCON2_SPEC>;
#[doc = "PLL Configuration 2 Register"]
pub mod pllcon2;
#[doc = "USBPLLSTAT (r) register accessor: USB PLL Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbpllstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`usbpllstat`]
module"]
pub type USBPLLSTAT = crate::Reg<usbpllstat::USBPLLSTAT_SPEC>;
#[doc = "USB PLL Status Register"]
pub mod usbpllstat;
#[doc = "USBPLLCON (rw) register accessor: USB PLL Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbpllcon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbpllcon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`usbpllcon`]
module"]
pub type USBPLLCON = crate::Reg<usbpllcon::USBPLLCON_SPEC>;
#[doc = "USB PLL Configuration Register"]
pub mod usbpllcon;
#[doc = "CLKMXSTAT (r) register accessor: Clock Multiplexing Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkmxstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clkmxstat`]
module"]
pub type CLKMXSTAT = crate::Reg<clkmxstat::CLKMXSTAT_SPEC>;
#[doc = "Clock Multiplexing Status Register"]
pub mod clkmxstat;
