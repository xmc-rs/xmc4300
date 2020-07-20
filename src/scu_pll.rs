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
    _reserved6: [u8; 16usize],
    #[doc = "0x28 - Clock Multiplexing Status Register"]
    pub clkmxstat: CLKMXSTAT,
}
#[doc = "PLL Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllstat](pllstat) module"]
pub type PLLSTAT = crate::Reg<u32, _PLLSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLLSTAT;
#[doc = "`read()` method returns [pllstat::R](pllstat::R) reader structure"]
impl crate::Readable for PLLSTAT {}
#[doc = "PLL Status Register"]
pub mod pllstat;
#[doc = "PLL Configuration 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllcon0](pllcon0) module"]
pub type PLLCON0 = crate::Reg<u32, _PLLCON0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLLCON0;
#[doc = "`read()` method returns [pllcon0::R](pllcon0::R) reader structure"]
impl crate::Readable for PLLCON0 {}
#[doc = "`write(|w| ..)` method takes [pllcon0::W](pllcon0::W) writer structure"]
impl crate::Writable for PLLCON0 {}
#[doc = "PLL Configuration 0 Register"]
pub mod pllcon0;
#[doc = "PLL Configuration 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllcon1](pllcon1) module"]
pub type PLLCON1 = crate::Reg<u32, _PLLCON1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLLCON1;
#[doc = "`read()` method returns [pllcon1::R](pllcon1::R) reader structure"]
impl crate::Readable for PLLCON1 {}
#[doc = "`write(|w| ..)` method takes [pllcon1::W](pllcon1::W) writer structure"]
impl crate::Writable for PLLCON1 {}
#[doc = "PLL Configuration 1 Register"]
pub mod pllcon1;
#[doc = "PLL Configuration 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllcon2](pllcon2) module"]
pub type PLLCON2 = crate::Reg<u32, _PLLCON2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLLCON2;
#[doc = "`read()` method returns [pllcon2::R](pllcon2::R) reader structure"]
impl crate::Readable for PLLCON2 {}
#[doc = "`write(|w| ..)` method takes [pllcon2::W](pllcon2::W) writer structure"]
impl crate::Writable for PLLCON2 {}
#[doc = "PLL Configuration 2 Register"]
pub mod pllcon2;
#[doc = "USB PLL Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbpllstat](usbpllstat) module"]
pub type USBPLLSTAT = crate::Reg<u32, _USBPLLSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBPLLSTAT;
#[doc = "`read()` method returns [usbpllstat::R](usbpllstat::R) reader structure"]
impl crate::Readable for USBPLLSTAT {}
#[doc = "USB PLL Status Register"]
pub mod usbpllstat;
#[doc = "USB PLL Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbpllcon](usbpllcon) module"]
pub type USBPLLCON = crate::Reg<u32, _USBPLLCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBPLLCON;
#[doc = "`read()` method returns [usbpllcon::R](usbpllcon::R) reader structure"]
impl crate::Readable for USBPLLCON {}
#[doc = "`write(|w| ..)` method takes [usbpllcon::W](usbpllcon::W) writer structure"]
impl crate::Writable for USBPLLCON {}
#[doc = "USB PLL Configuration Register"]
pub mod usbpllcon;
#[doc = "Clock Multiplexing Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkmxstat](clkmxstat) module"]
pub type CLKMXSTAT = crate::Reg<u32, _CLKMXSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKMXSTAT;
#[doc = "`read()` method returns [clkmxstat::R](clkmxstat::R) reader structure"]
impl crate::Readable for CLKMXSTAT {}
#[doc = "Clock Multiplexing Status Register"]
pub mod clkmxstat;
