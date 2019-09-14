#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Node Control Register"]
    pub ncr: NCR,
    #[doc = "0x04 - Node Status Register"]
    pub nsr: NSR,
    #[doc = "0x08 - Node Interrupt Pointer Register"]
    pub nipr: NIPR,
    #[doc = "0x0c - Node Port Control Register"]
    pub npcr: NPCR,
    #[doc = "0x10 - Node Bit Timing Register"]
    pub nbtr: NBTR,
    #[doc = "0x14 - Node Error Counter Register"]
    pub necnt: NECNT,
    #[doc = "0x18 - Node Frame Counter Register"]
    pub nfcr: NFCR,
}
#[doc = "Node Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ncr](ncr) module"]
pub type NCR = crate::Reg<u32, _NCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NCR;
#[doc = "`read()` method returns [ncr::R](ncr::R) reader structure"]
impl crate::Readable for NCR {}
#[doc = "`write(|w| ..)` method takes [ncr::W](ncr::W) writer structure"]
impl crate::Writable for NCR {}
#[doc = "Node Control Register"]
pub mod ncr;
#[doc = "Node Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nsr](nsr) module"]
pub type NSR = crate::Reg<u32, _NSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NSR;
#[doc = "`read()` method returns [nsr::R](nsr::R) reader structure"]
impl crate::Readable for NSR {}
#[doc = "`write(|w| ..)` method takes [nsr::W](nsr::W) writer structure"]
impl crate::Writable for NSR {}
#[doc = "Node Status Register"]
pub mod nsr;
#[doc = "Node Interrupt Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nipr](nipr) module"]
pub type NIPR = crate::Reg<u32, _NIPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NIPR;
#[doc = "`read()` method returns [nipr::R](nipr::R) reader structure"]
impl crate::Readable for NIPR {}
#[doc = "`write(|w| ..)` method takes [nipr::W](nipr::W) writer structure"]
impl crate::Writable for NIPR {}
#[doc = "Node Interrupt Pointer Register"]
pub mod nipr;
#[doc = "Node Port Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [npcr](npcr) module"]
pub type NPCR = crate::Reg<u32, _NPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NPCR;
#[doc = "`read()` method returns [npcr::R](npcr::R) reader structure"]
impl crate::Readable for NPCR {}
#[doc = "`write(|w| ..)` method takes [npcr::W](npcr::W) writer structure"]
impl crate::Writable for NPCR {}
#[doc = "Node Port Control Register"]
pub mod npcr;
#[doc = "Node Bit Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nbtr](nbtr) module"]
pub type NBTR = crate::Reg<u32, _NBTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NBTR;
#[doc = "`read()` method returns [nbtr::R](nbtr::R) reader structure"]
impl crate::Readable for NBTR {}
#[doc = "`write(|w| ..)` method takes [nbtr::W](nbtr::W) writer structure"]
impl crate::Writable for NBTR {}
#[doc = "Node Bit Timing Register"]
pub mod nbtr;
#[doc = "Node Error Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [necnt](necnt) module"]
pub type NECNT = crate::Reg<u32, _NECNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NECNT;
#[doc = "`read()` method returns [necnt::R](necnt::R) reader structure"]
impl crate::Readable for NECNT {}
#[doc = "`write(|w| ..)` method takes [necnt::W](necnt::W) writer structure"]
impl crate::Writable for NECNT {}
#[doc = "Node Error Counter Register"]
pub mod necnt;
#[doc = "Node Frame Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nfcr](nfcr) module"]
pub type NFCR = crate::Reg<u32, _NFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NFCR;
#[doc = "`read()` method returns [nfcr::R](nfcr::R) reader structure"]
impl crate::Readable for NFCR {}
#[doc = "`write(|w| ..)` method takes [nfcr::W](nfcr::W) writer structure"]
impl crate::Writable for NFCR {}
#[doc = "Node Frame Counter Register"]
pub mod nfcr;
