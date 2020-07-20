#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock Status Register"]
    pub clkstat: CLKSTAT,
    #[doc = "0x04 - CLK Set Register"]
    pub clkset: CLKSET,
    #[doc = "0x08 - CLK Clear Register"]
    pub clkclr: CLKCLR,
    #[doc = "0x0c - System Clock Control Register"]
    pub sysclkcr: SYSCLKCR,
    #[doc = "0x10 - CPU Clock Control Register"]
    pub cpuclkcr: CPUCLKCR,
    #[doc = "0x14 - Peripheral Bus Clock Control Register"]
    pub pbclkcr: PBCLKCR,
    #[doc = "0x18 - USB Clock Control Register"]
    pub usbclkcr: USBCLKCR,
    _reserved7: [u8; 4usize],
    #[doc = "0x20 - CCU Clock Control Register"]
    pub ccuclkcr: CCUCLKCR,
    #[doc = "0x24 - WDT Clock Control Register"]
    pub wdtclkcr: WDTCLKCR,
    #[doc = "0x28 - External Clock Control"]
    pub extclkcr: EXTCLKCR,
    #[doc = "0x2c - Multi-Link Clock Control"]
    pub mlinkclkcr: MLINKCLKCR,
    #[doc = "0x30 - Sleep Control Register"]
    pub sleepcr: SLEEPCR,
    #[doc = "0x34 - Deep Sleep Control Register"]
    pub dsleepcr: DSLEEPCR,
    #[doc = "0x38 - EtherCAT Clock Control Register"]
    pub ecatclkcr: ECATCLKCR,
    _reserved14: [u8; 4usize],
    #[doc = "0x40 - Peripheral 0 Clock Gating Status"]
    pub cgatstat0: CGATSTAT0,
    #[doc = "0x44 - Peripheral 0 Clock Gating Set"]
    pub cgatset0: CGATSET0,
    #[doc = "0x48 - Peripheral 0 Clock Gating Clear"]
    pub cgatclr0: CGATCLR0,
    #[doc = "0x4c - Peripheral 1 Clock Gating Status"]
    pub cgatstat1: CGATSTAT1,
    #[doc = "0x50 - Peripheral 1 Clock Gating Set"]
    pub cgatset1: CGATSET1,
    #[doc = "0x54 - Peripheral 1 Clock Gating Clear"]
    pub cgatclr1: CGATCLR1,
    #[doc = "0x58 - Peripheral 2 Clock Gating Status"]
    pub cgatstat2: CGATSTAT2,
    #[doc = "0x5c - Peripheral 2 Clock Gating Set"]
    pub cgatset2: CGATSET2,
    #[doc = "0x60 - Peripheral 2 Clock Gating Clear"]
    pub cgatclr2: CGATCLR2,
}
#[doc = "Clock Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkstat](clkstat) module"]
pub type CLKSTAT = crate::Reg<u32, _CLKSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKSTAT;
#[doc = "`read()` method returns [clkstat::R](clkstat::R) reader structure"]
impl crate::Readable for CLKSTAT {}
#[doc = "Clock Status Register"]
pub mod clkstat;
#[doc = "CLK Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkset](clkset) module"]
pub type CLKSET = crate::Reg<u32, _CLKSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKSET;
#[doc = "`write(|w| ..)` method takes [clkset::W](clkset::W) writer structure"]
impl crate::Writable for CLKSET {}
#[doc = "CLK Set Register"]
pub mod clkset;
#[doc = "CLK Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkclr](clkclr) module"]
pub type CLKCLR = crate::Reg<u32, _CLKCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKCLR;
#[doc = "`write(|w| ..)` method takes [clkclr::W](clkclr::W) writer structure"]
impl crate::Writable for CLKCLR {}
#[doc = "CLK Clear Register"]
pub mod clkclr;
#[doc = "System Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysclkcr](sysclkcr) module"]
pub type SYSCLKCR = crate::Reg<u32, _SYSCLKCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCLKCR;
#[doc = "`read()` method returns [sysclkcr::R](sysclkcr::R) reader structure"]
impl crate::Readable for SYSCLKCR {}
#[doc = "`write(|w| ..)` method takes [sysclkcr::W](sysclkcr::W) writer structure"]
impl crate::Writable for SYSCLKCR {}
#[doc = "System Clock Control Register"]
pub mod sysclkcr;
#[doc = "CPU Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuclkcr](cpuclkcr) module"]
pub type CPUCLKCR = crate::Reg<u32, _CPUCLKCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUCLKCR;
#[doc = "`read()` method returns [cpuclkcr::R](cpuclkcr::R) reader structure"]
impl crate::Readable for CPUCLKCR {}
#[doc = "`write(|w| ..)` method takes [cpuclkcr::W](cpuclkcr::W) writer structure"]
impl crate::Writable for CPUCLKCR {}
#[doc = "CPU Clock Control Register"]
pub mod cpuclkcr;
#[doc = "Peripheral Bus Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbclkcr](pbclkcr) module"]
pub type PBCLKCR = crate::Reg<u32, _PBCLKCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PBCLKCR;
#[doc = "`read()` method returns [pbclkcr::R](pbclkcr::R) reader structure"]
impl crate::Readable for PBCLKCR {}
#[doc = "`write(|w| ..)` method takes [pbclkcr::W](pbclkcr::W) writer structure"]
impl crate::Writable for PBCLKCR {}
#[doc = "Peripheral Bus Clock Control Register"]
pub mod pbclkcr;
#[doc = "USB Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbclkcr](usbclkcr) module"]
pub type USBCLKCR = crate::Reg<u32, _USBCLKCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBCLKCR;
#[doc = "`read()` method returns [usbclkcr::R](usbclkcr::R) reader structure"]
impl crate::Readable for USBCLKCR {}
#[doc = "`write(|w| ..)` method takes [usbclkcr::W](usbclkcr::W) writer structure"]
impl crate::Writable for USBCLKCR {}
#[doc = "USB Clock Control Register"]
pub mod usbclkcr;
#[doc = "CCU Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccuclkcr](ccuclkcr) module"]
pub type CCUCLKCR = crate::Reg<u32, _CCUCLKCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCUCLKCR;
#[doc = "`read()` method returns [ccuclkcr::R](ccuclkcr::R) reader structure"]
impl crate::Readable for CCUCLKCR {}
#[doc = "`write(|w| ..)` method takes [ccuclkcr::W](ccuclkcr::W) writer structure"]
impl crate::Writable for CCUCLKCR {}
#[doc = "CCU Clock Control Register"]
pub mod ccuclkcr;
#[doc = "WDT Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtclkcr](wdtclkcr) module"]
pub type WDTCLKCR = crate::Reg<u32, _WDTCLKCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTCLKCR;
#[doc = "`read()` method returns [wdtclkcr::R](wdtclkcr::R) reader structure"]
impl crate::Readable for WDTCLKCR {}
#[doc = "`write(|w| ..)` method takes [wdtclkcr::W](wdtclkcr::W) writer structure"]
impl crate::Writable for WDTCLKCR {}
#[doc = "WDT Clock Control Register"]
pub mod wdtclkcr;
#[doc = "External Clock Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extclkcr](extclkcr) module"]
pub type EXTCLKCR = crate::Reg<u32, _EXTCLKCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTCLKCR;
#[doc = "`read()` method returns [extclkcr::R](extclkcr::R) reader structure"]
impl crate::Readable for EXTCLKCR {}
#[doc = "`write(|w| ..)` method takes [extclkcr::W](extclkcr::W) writer structure"]
impl crate::Writable for EXTCLKCR {}
#[doc = "External Clock Control"]
pub mod extclkcr;
#[doc = "Multi-Link Clock Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mlinkclkcr](mlinkclkcr) module"]
pub type MLINKCLKCR = crate::Reg<u32, _MLINKCLKCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MLINKCLKCR;
#[doc = "`read()` method returns [mlinkclkcr::R](mlinkclkcr::R) reader structure"]
impl crate::Readable for MLINKCLKCR {}
#[doc = "`write(|w| ..)` method takes [mlinkclkcr::W](mlinkclkcr::W) writer structure"]
impl crate::Writable for MLINKCLKCR {}
#[doc = "Multi-Link Clock Control"]
pub mod mlinkclkcr;
#[doc = "Sleep Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sleepcr](sleepcr) module"]
pub type SLEEPCR = crate::Reg<u32, _SLEEPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLEEPCR;
#[doc = "`read()` method returns [sleepcr::R](sleepcr::R) reader structure"]
impl crate::Readable for SLEEPCR {}
#[doc = "`write(|w| ..)` method takes [sleepcr::W](sleepcr::W) writer structure"]
impl crate::Writable for SLEEPCR {}
#[doc = "Sleep Control Register"]
pub mod sleepcr;
#[doc = "Deep Sleep Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsleepcr](dsleepcr) module"]
pub type DSLEEPCR = crate::Reg<u32, _DSLEEPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSLEEPCR;
#[doc = "`read()` method returns [dsleepcr::R](dsleepcr::R) reader structure"]
impl crate::Readable for DSLEEPCR {}
#[doc = "`write(|w| ..)` method takes [dsleepcr::W](dsleepcr::W) writer structure"]
impl crate::Writable for DSLEEPCR {}
#[doc = "Deep Sleep Control Register"]
pub mod dsleepcr;
#[doc = "EtherCAT Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecatclkcr](ecatclkcr) module"]
pub type ECATCLKCR = crate::Reg<u32, _ECATCLKCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECATCLKCR;
#[doc = "`read()` method returns [ecatclkcr::R](ecatclkcr::R) reader structure"]
impl crate::Readable for ECATCLKCR {}
#[doc = "`write(|w| ..)` method takes [ecatclkcr::W](ecatclkcr::W) writer structure"]
impl crate::Writable for ECATCLKCR {}
#[doc = "EtherCAT Clock Control Register"]
pub mod ecatclkcr;
#[doc = "Peripheral 0 Clock Gating Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgatstat0](cgatstat0) module"]
pub type CGATSTAT0 = crate::Reg<u32, _CGATSTAT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CGATSTAT0;
#[doc = "`read()` method returns [cgatstat0::R](cgatstat0::R) reader structure"]
impl crate::Readable for CGATSTAT0 {}
#[doc = "Peripheral 0 Clock Gating Status"]
pub mod cgatstat0;
#[doc = "Peripheral 0 Clock Gating Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgatset0](cgatset0) module"]
pub type CGATSET0 = crate::Reg<u32, _CGATSET0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CGATSET0;
#[doc = "`write(|w| ..)` method takes [cgatset0::W](cgatset0::W) writer structure"]
impl crate::Writable for CGATSET0 {}
#[doc = "Peripheral 0 Clock Gating Set"]
pub mod cgatset0;
#[doc = "Peripheral 0 Clock Gating Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgatclr0](cgatclr0) module"]
pub type CGATCLR0 = crate::Reg<u32, _CGATCLR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CGATCLR0;
#[doc = "`write(|w| ..)` method takes [cgatclr0::W](cgatclr0::W) writer structure"]
impl crate::Writable for CGATCLR0 {}
#[doc = "Peripheral 0 Clock Gating Clear"]
pub mod cgatclr0;
#[doc = "Peripheral 1 Clock Gating Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgatstat1](cgatstat1) module"]
pub type CGATSTAT1 = crate::Reg<u32, _CGATSTAT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CGATSTAT1;
#[doc = "`read()` method returns [cgatstat1::R](cgatstat1::R) reader structure"]
impl crate::Readable for CGATSTAT1 {}
#[doc = "Peripheral 1 Clock Gating Status"]
pub mod cgatstat1;
#[doc = "Peripheral 1 Clock Gating Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgatset1](cgatset1) module"]
pub type CGATSET1 = crate::Reg<u32, _CGATSET1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CGATSET1;
#[doc = "`write(|w| ..)` method takes [cgatset1::W](cgatset1::W) writer structure"]
impl crate::Writable for CGATSET1 {}
#[doc = "Peripheral 1 Clock Gating Set"]
pub mod cgatset1;
#[doc = "Peripheral 1 Clock Gating Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgatclr1](cgatclr1) module"]
pub type CGATCLR1 = crate::Reg<u32, _CGATCLR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CGATCLR1;
#[doc = "`write(|w| ..)` method takes [cgatclr1::W](cgatclr1::W) writer structure"]
impl crate::Writable for CGATCLR1 {}
#[doc = "Peripheral 1 Clock Gating Clear"]
pub mod cgatclr1;
#[doc = "Peripheral 2 Clock Gating Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgatstat2](cgatstat2) module"]
pub type CGATSTAT2 = crate::Reg<u32, _CGATSTAT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CGATSTAT2;
#[doc = "`read()` method returns [cgatstat2::R](cgatstat2::R) reader structure"]
impl crate::Readable for CGATSTAT2 {}
#[doc = "Peripheral 2 Clock Gating Status"]
pub mod cgatstat2;
#[doc = "Peripheral 2 Clock Gating Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgatset2](cgatset2) module"]
pub type CGATSET2 = crate::Reg<u32, _CGATSET2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CGATSET2;
#[doc = "`write(|w| ..)` method takes [cgatset2::W](cgatset2::W) writer structure"]
impl crate::Writable for CGATSET2 {}
#[doc = "Peripheral 2 Clock Gating Set"]
pub mod cgatset2;
#[doc = "Peripheral 2 Clock Gating Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgatclr2](cgatclr2) module"]
pub type CGATCLR2 = crate::Reg<u32, _CGATCLR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CGATCLR2;
#[doc = "`write(|w| ..)` method takes [cgatclr2::W](cgatclr2::W) writer structure"]
impl crate::Writable for CGATCLR2 {}
#[doc = "Peripheral 2 Clock Gating Clear"]
pub mod cgatclr2;
