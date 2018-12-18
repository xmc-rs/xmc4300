#[doc = r" Register block"]
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
    _reserved0: [u8; 16usize],
    #[doc = "0x28 - Clock Multiplexing Status Register"]
    pub clkmxstat: CLKMXSTAT,
}
#[doc = "PLL Status Register"]
pub struct PLLSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL Status Register"]
pub mod pllstat;
#[doc = "PLL Configuration 0 Register"]
pub struct PLLCON0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL Configuration 0 Register"]
pub mod pllcon0;
#[doc = "PLL Configuration 1 Register"]
pub struct PLLCON1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL Configuration 1 Register"]
pub mod pllcon1;
#[doc = "PLL Configuration 2 Register"]
pub struct PLLCON2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL Configuration 2 Register"]
pub mod pllcon2;
#[doc = "USB PLL Status Register"]
pub struct USBPLLSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PLL Status Register"]
pub mod usbpllstat;
#[doc = "USB PLL Configuration Register"]
pub struct USBPLLCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PLL Configuration Register"]
pub mod usbpllcon;
#[doc = "Clock Multiplexing Status Register"]
pub struct CLKMXSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Multiplexing Status Register"]
pub mod clkmxstat;
