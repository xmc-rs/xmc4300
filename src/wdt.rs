#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - WDT ID Register"]
    pub id: ID,
    #[doc = "0x04 - WDT Control Register"]
    pub ctr: CTR,
    #[doc = "0x08 - WDT Service Register"]
    pub srv: SRV,
    #[doc = "0x0c - WDT Timer Register"]
    pub tim: TIM,
    #[doc = "0x10 - WDT Window Lower Bound Register"]
    pub wlb: WLB,
    #[doc = "0x14 - WDT Window Upper Bound Register"]
    pub wub: WUB,
    #[doc = "0x18 - WDT Status Register"]
    pub wdtsts: WDTSTS,
    #[doc = "0x1c - WDT Clear Register"]
    pub wdtclr: WDTCLR,
}
#[doc = "WDT ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [id](id) module"]
pub type ID = crate::Reg<u32, _ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID;
#[doc = "`read()` method returns [id::R](id::R) reader structure"]
impl crate::Readable for ID {}
#[doc = "WDT ID Register"]
pub mod id;
#[doc = "WDT Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctr](ctr) module"]
pub type CTR = crate::Reg<u32, _CTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTR;
#[doc = "`read()` method returns [ctr::R](ctr::R) reader structure"]
impl crate::Readable for CTR {}
#[doc = "`write(|w| ..)` method takes [ctr::W](ctr::W) writer structure"]
impl crate::Writable for CTR {}
#[doc = "WDT Control Register"]
pub mod ctr;
#[doc = "WDT Service Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srv](srv) module"]
pub type SRV = crate::Reg<u32, _SRV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRV;
#[doc = "`write(|w| ..)` method takes [srv::W](srv::W) writer structure"]
impl crate::Writable for SRV {}
#[doc = "WDT Service Register"]
pub mod srv;
#[doc = "WDT Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tim](tim) module"]
pub type TIM = crate::Reg<u32, _TIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM;
#[doc = "`read()` method returns [tim::R](tim::R) reader structure"]
impl crate::Readable for TIM {}
#[doc = "WDT Timer Register"]
pub mod tim;
#[doc = "WDT Window Lower Bound Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wlb](wlb) module"]
pub type WLB = crate::Reg<u32, _WLB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WLB;
#[doc = "`read()` method returns [wlb::R](wlb::R) reader structure"]
impl crate::Readable for WLB {}
#[doc = "`write(|w| ..)` method takes [wlb::W](wlb::W) writer structure"]
impl crate::Writable for WLB {}
#[doc = "WDT Window Lower Bound Register"]
pub mod wlb;
#[doc = "WDT Window Upper Bound Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wub](wub) module"]
pub type WUB = crate::Reg<u32, _WUB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WUB;
#[doc = "`read()` method returns [wub::R](wub::R) reader structure"]
impl crate::Readable for WUB {}
#[doc = "`write(|w| ..)` method takes [wub::W](wub::W) writer structure"]
impl crate::Writable for WUB {}
#[doc = "WDT Window Upper Bound Register"]
pub mod wub;
#[doc = "WDT Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdtsts](wdtsts) module"]
pub type WDTSTS = crate::Reg<u32, _WDTSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTSTS;
#[doc = "`read()` method returns [wdtsts::R](wdtsts::R) reader structure"]
impl crate::Readable for WDTSTS {}
#[doc = "WDT Status Register"]
pub mod wdtsts;
#[doc = "WDT Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdtclr](wdtclr) module"]
pub type WDTCLR = crate::Reg<u32, _WDTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTCLR;
#[doc = "`write(|w| ..)` method takes [wdtclr::W](wdtclr::W) writer structure"]
impl crate::Writable for WDTCLR {}
#[doc = "WDT Clear Register"]
pub mod wdtclr;
