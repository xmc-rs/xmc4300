#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC ID Register"]
    pub id: ID,
    #[doc = "0x04 - RTC Control Register"]
    pub ctr: CTR,
    #[doc = "0x08 - RTC Raw Service Request Register"]
    pub rawstat: RAWSTAT,
    #[doc = "0x0c - RTC Service Request Status Register"]
    pub stssr: STSSR,
    #[doc = "0x10 - RTC Service Request Mask Register"]
    pub msksr: MSKSR,
    #[doc = "0x14 - RTC Clear Service Request Register"]
    pub clrsr: CLRSR,
    #[doc = "0x18 - RTC Alarm Time Register 0"]
    pub atim0: ATIM0,
    #[doc = "0x1c - RTC Alarm Time Register 1"]
    pub atim1: ATIM1,
    #[doc = "0x20 - RTC Time Register 0"]
    pub tim0: TIM0,
    #[doc = "0x24 - RTC Time Register 1"]
    pub tim1: TIM1,
}
#[doc = "RTC ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [id](id) module"]
pub type ID = crate::Reg<u32, _ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID;
#[doc = "`read()` method returns [id::R](id::R) reader structure"]
impl crate::Readable for ID {}
#[doc = "RTC ID Register"]
pub mod id;
#[doc = "RTC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctr](ctr) module"]
pub type CTR = crate::Reg<u32, _CTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTR;
#[doc = "`read()` method returns [ctr::R](ctr::R) reader structure"]
impl crate::Readable for CTR {}
#[doc = "`write(|w| ..)` method takes [ctr::W](ctr::W) writer structure"]
impl crate::Writable for CTR {}
#[doc = "RTC Control Register"]
pub mod ctr;
#[doc = "RTC Raw Service Request Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rawstat](rawstat) module"]
pub type RAWSTAT = crate::Reg<u32, _RAWSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAWSTAT;
#[doc = "`read()` method returns [rawstat::R](rawstat::R) reader structure"]
impl crate::Readable for RAWSTAT {}
#[doc = "RTC Raw Service Request Register"]
pub mod rawstat;
#[doc = "RTC Service Request Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [stssr](stssr) module"]
pub type STSSR = crate::Reg<u32, _STSSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STSSR;
#[doc = "`read()` method returns [stssr::R](stssr::R) reader structure"]
impl crate::Readable for STSSR {}
#[doc = "RTC Service Request Status Register"]
pub mod stssr;
#[doc = "RTC Service Request Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [msksr](msksr) module"]
pub type MSKSR = crate::Reg<u32, _MSKSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSKSR;
#[doc = "`read()` method returns [msksr::R](msksr::R) reader structure"]
impl crate::Readable for MSKSR {}
#[doc = "`write(|w| ..)` method takes [msksr::W](msksr::W) writer structure"]
impl crate::Writable for MSKSR {}
#[doc = "RTC Service Request Mask Register"]
pub mod msksr;
#[doc = "RTC Clear Service Request Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clrsr](clrsr) module"]
pub type CLRSR = crate::Reg<u32, _CLRSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLRSR;
#[doc = "`write(|w| ..)` method takes [clrsr::W](clrsr::W) writer structure"]
impl crate::Writable for CLRSR {}
#[doc = "RTC Clear Service Request Register"]
pub mod clrsr;
#[doc = "RTC Alarm Time Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [atim0](atim0) module"]
pub type ATIM0 = crate::Reg<u32, _ATIM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ATIM0;
#[doc = "`read()` method returns [atim0::R](atim0::R) reader structure"]
impl crate::Readable for ATIM0 {}
#[doc = "`write(|w| ..)` method takes [atim0::W](atim0::W) writer structure"]
impl crate::Writable for ATIM0 {}
#[doc = "RTC Alarm Time Register 0"]
pub mod atim0;
#[doc = "RTC Alarm Time Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [atim1](atim1) module"]
pub type ATIM1 = crate::Reg<u32, _ATIM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ATIM1;
#[doc = "`read()` method returns [atim1::R](atim1::R) reader structure"]
impl crate::Readable for ATIM1 {}
#[doc = "`write(|w| ..)` method takes [atim1::W](atim1::W) writer structure"]
impl crate::Writable for ATIM1 {}
#[doc = "RTC Alarm Time Register 1"]
pub mod atim1;
#[doc = "RTC Time Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tim0](tim0) module"]
pub type TIM0 = crate::Reg<u32, _TIM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM0;
#[doc = "`read()` method returns [tim0::R](tim0::R) reader structure"]
impl crate::Readable for TIM0 {}
#[doc = "`write(|w| ..)` method takes [tim0::W](tim0::W) writer structure"]
impl crate::Writable for TIM0 {}
#[doc = "RTC Time Register 0"]
pub mod tim0;
#[doc = "RTC Time Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tim1](tim1) module"]
pub type TIM1 = crate::Reg<u32, _TIM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1;
#[doc = "`read()` method returns [tim1::R](tim1::R) reader structure"]
impl crate::Readable for TIM1 {}
#[doc = "`write(|w| ..)` method takes [tim1::W](tim1::W) writer structure"]
impl crate::Writable for TIM1 {}
#[doc = "RTC Time Register 1"]
pub mod tim1;
