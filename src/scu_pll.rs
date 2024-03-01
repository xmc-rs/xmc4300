#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pllstat: Pllstat,
    pllcon0: Pllcon0,
    pllcon1: Pllcon1,
    pllcon2: Pllcon2,
    usbpllstat: Usbpllstat,
    usbpllcon: Usbpllcon,
    _reserved6: [u8; 0x10],
    clkmxstat: Clkmxstat,
}
impl RegisterBlock {
    #[doc = "0x00 - PLL Status Register"]
    #[inline(always)]
    pub const fn pllstat(&self) -> &Pllstat {
        &self.pllstat
    }
    #[doc = "0x04 - PLL Configuration 0 Register"]
    #[inline(always)]
    pub const fn pllcon0(&self) -> &Pllcon0 {
        &self.pllcon0
    }
    #[doc = "0x08 - PLL Configuration 1 Register"]
    #[inline(always)]
    pub const fn pllcon1(&self) -> &Pllcon1 {
        &self.pllcon1
    }
    #[doc = "0x0c - PLL Configuration 2 Register"]
    #[inline(always)]
    pub const fn pllcon2(&self) -> &Pllcon2 {
        &self.pllcon2
    }
    #[doc = "0x10 - USB PLL Status Register"]
    #[inline(always)]
    pub const fn usbpllstat(&self) -> &Usbpllstat {
        &self.usbpllstat
    }
    #[doc = "0x14 - USB PLL Configuration Register"]
    #[inline(always)]
    pub const fn usbpllcon(&self) -> &Usbpllcon {
        &self.usbpllcon
    }
    #[doc = "0x28 - Clock Multiplexing Status Register"]
    #[inline(always)]
    pub const fn clkmxstat(&self) -> &Clkmxstat {
        &self.clkmxstat
    }
}
#[doc = "PLLSTAT (r) register accessor: PLL Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllstat`]
module"]
#[doc(alias = "PLLSTAT")]
pub type Pllstat = crate::Reg<pllstat::PllstatSpec>;
#[doc = "PLL Status Register"]
pub mod pllstat;
#[doc = "PLLCON0 (rw) register accessor: PLL Configuration 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllcon0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllcon0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllcon0`]
module"]
#[doc(alias = "PLLCON0")]
pub type Pllcon0 = crate::Reg<pllcon0::Pllcon0Spec>;
#[doc = "PLL Configuration 0 Register"]
pub mod pllcon0;
#[doc = "PLLCON1 (rw) register accessor: PLL Configuration 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllcon1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllcon1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllcon1`]
module"]
#[doc(alias = "PLLCON1")]
pub type Pllcon1 = crate::Reg<pllcon1::Pllcon1Spec>;
#[doc = "PLL Configuration 1 Register"]
pub mod pllcon1;
#[doc = "PLLCON2 (rw) register accessor: PLL Configuration 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllcon2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllcon2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllcon2`]
module"]
#[doc(alias = "PLLCON2")]
pub type Pllcon2 = crate::Reg<pllcon2::Pllcon2Spec>;
#[doc = "PLL Configuration 2 Register"]
pub mod pllcon2;
#[doc = "USBPLLSTAT (r) register accessor: USB PLL Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbpllstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbpllstat`]
module"]
#[doc(alias = "USBPLLSTAT")]
pub type Usbpllstat = crate::Reg<usbpllstat::UsbpllstatSpec>;
#[doc = "USB PLL Status Register"]
pub mod usbpllstat;
#[doc = "USBPLLCON (rw) register accessor: USB PLL Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbpllcon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbpllcon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbpllcon`]
module"]
#[doc(alias = "USBPLLCON")]
pub type Usbpllcon = crate::Reg<usbpllcon::UsbpllconSpec>;
#[doc = "USB PLL Configuration Register"]
pub mod usbpllcon;
#[doc = "CLKMXSTAT (r) register accessor: Clock Multiplexing Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkmxstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkmxstat`]
module"]
#[doc(alias = "CLKMXSTAT")]
pub type Clkmxstat = crate::Reg<clkmxstat::ClkmxstatSpec>;
#[doc = "Clock Multiplexing Status Register"]
pub mod clkmxstat;
