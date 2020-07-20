#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RCU Reset Status"]
    pub rststat: RSTSTAT,
    #[doc = "0x04 - RCU Reset Set Register"]
    pub rstset: RSTSET,
    #[doc = "0x08 - RCU Reset Clear Register"]
    pub rstclr: RSTCLR,
    #[doc = "0x0c - RCU Peripheral 0 Reset Status"]
    pub prstat0: PRSTAT0,
    #[doc = "0x10 - RCU Peripheral 0 Reset Set"]
    pub prset0: PRSET0,
    #[doc = "0x14 - RCU Peripheral 0 Reset Clear"]
    pub prclr0: PRCLR0,
    #[doc = "0x18 - RCU Peripheral 1 Reset Status"]
    pub prstat1: PRSTAT1,
    #[doc = "0x1c - RCU Peripheral 1 Reset Set"]
    pub prset1: PRSET1,
    #[doc = "0x20 - RCU Peripheral 1 Reset Clear"]
    pub prclr1: PRCLR1,
    #[doc = "0x24 - RCU Peripheral 2 Reset Status"]
    pub prstat2: PRSTAT2,
    #[doc = "0x28 - RCU Peripheral 2 Reset Set"]
    pub prset2: PRSET2,
    #[doc = "0x2c - RCU Peripheral 2 Reset Clear"]
    pub prclr2: PRCLR2,
}
#[doc = "RCU Reset Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rststat](rststat) module"]
pub type RSTSTAT = crate::Reg<u32, _RSTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTSTAT;
#[doc = "`read()` method returns [rststat::R](rststat::R) reader structure"]
impl crate::Readable for RSTSTAT {}
#[doc = "RCU Reset Status"]
pub mod rststat;
#[doc = "RCU Reset Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstset](rstset) module"]
pub type RSTSET = crate::Reg<u32, _RSTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTSET;
#[doc = "`write(|w| ..)` method takes [rstset::W](rstset::W) writer structure"]
impl crate::Writable for RSTSET {}
#[doc = "RCU Reset Set Register"]
pub mod rstset;
#[doc = "RCU Reset Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstclr](rstclr) module"]
pub type RSTCLR = crate::Reg<u32, _RSTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTCLR;
#[doc = "`write(|w| ..)` method takes [rstclr::W](rstclr::W) writer structure"]
impl crate::Writable for RSTCLR {}
#[doc = "RCU Reset Clear Register"]
pub mod rstclr;
#[doc = "RCU Peripheral 0 Reset Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prstat0](prstat0) module"]
pub type PRSTAT0 = crate::Reg<u32, _PRSTAT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRSTAT0;
#[doc = "`read()` method returns [prstat0::R](prstat0::R) reader structure"]
impl crate::Readable for PRSTAT0 {}
#[doc = "RCU Peripheral 0 Reset Status"]
pub mod prstat0;
#[doc = "RCU Peripheral 0 Reset Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prset0](prset0) module"]
pub type PRSET0 = crate::Reg<u32, _PRSET0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRSET0;
#[doc = "`write(|w| ..)` method takes [prset0::W](prset0::W) writer structure"]
impl crate::Writable for PRSET0 {}
#[doc = "RCU Peripheral 0 Reset Set"]
pub mod prset0;
#[doc = "RCU Peripheral 0 Reset Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prclr0](prclr0) module"]
pub type PRCLR0 = crate::Reg<u32, _PRCLR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRCLR0;
#[doc = "`write(|w| ..)` method takes [prclr0::W](prclr0::W) writer structure"]
impl crate::Writable for PRCLR0 {}
#[doc = "RCU Peripheral 0 Reset Clear"]
pub mod prclr0;
#[doc = "RCU Peripheral 1 Reset Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prstat1](prstat1) module"]
pub type PRSTAT1 = crate::Reg<u32, _PRSTAT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRSTAT1;
#[doc = "`read()` method returns [prstat1::R](prstat1::R) reader structure"]
impl crate::Readable for PRSTAT1 {}
#[doc = "RCU Peripheral 1 Reset Status"]
pub mod prstat1;
#[doc = "RCU Peripheral 1 Reset Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prset1](prset1) module"]
pub type PRSET1 = crate::Reg<u32, _PRSET1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRSET1;
#[doc = "`write(|w| ..)` method takes [prset1::W](prset1::W) writer structure"]
impl crate::Writable for PRSET1 {}
#[doc = "RCU Peripheral 1 Reset Set"]
pub mod prset1;
#[doc = "RCU Peripheral 1 Reset Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prclr1](prclr1) module"]
pub type PRCLR1 = crate::Reg<u32, _PRCLR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRCLR1;
#[doc = "`write(|w| ..)` method takes [prclr1::W](prclr1::W) writer structure"]
impl crate::Writable for PRCLR1 {}
#[doc = "RCU Peripheral 1 Reset Clear"]
pub mod prclr1;
#[doc = "RCU Peripheral 2 Reset Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prstat2](prstat2) module"]
pub type PRSTAT2 = crate::Reg<u32, _PRSTAT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRSTAT2;
#[doc = "`read()` method returns [prstat2::R](prstat2::R) reader structure"]
impl crate::Readable for PRSTAT2 {}
#[doc = "RCU Peripheral 2 Reset Status"]
pub mod prstat2;
#[doc = "RCU Peripheral 2 Reset Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prset2](prset2) module"]
pub type PRSET2 = crate::Reg<u32, _PRSET2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRSET2;
#[doc = "`write(|w| ..)` method takes [prset2::W](prset2::W) writer structure"]
impl crate::Writable for PRSET2 {}
#[doc = "RCU Peripheral 2 Reset Set"]
pub mod prset2;
#[doc = "RCU Peripheral 2 Reset Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prclr2](prclr2) module"]
pub type PRCLR2 = crate::Reg<u32, _PRCLR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRCLR2;
#[doc = "`write(|w| ..)` method takes [prclr2::W](prclr2::W) writer structure"]
impl crate::Writable for PRCLR2 {}
#[doc = "RCU Peripheral 2 Reset Clear"]
pub mod prclr2;
