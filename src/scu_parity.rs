#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Parity Error Enable Register"]
    pub peen: PEEN,
    #[doc = "0x04 - Memory Checking Control Register"]
    pub mchkcon: MCHKCON,
    #[doc = "0x08 - Parity Error Trap Enable Register"]
    pub pete: PETE,
    #[doc = "0x0c - Parity Error Reset Enable Register"]
    pub persten: PERSTEN,
    _reserved4: [u8; 4usize],
    #[doc = "0x14 - Parity Error Flag Register"]
    pub peflag: PEFLAG,
    #[doc = "0x18 - Parity Memory Test Pattern Register"]
    pub pmtpr: PMTPR,
    #[doc = "0x1c - Parity Memory Test Select Register"]
    pub pmtsr: PMTSR,
}
#[doc = "Parity Error Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peen](peen) module"]
pub type PEEN = crate::Reg<u32, _PEEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEEN;
#[doc = "`read()` method returns [peen::R](peen::R) reader structure"]
impl crate::Readable for PEEN {}
#[doc = "`write(|w| ..)` method takes [peen::W](peen::W) writer structure"]
impl crate::Writable for PEEN {}
#[doc = "Parity Error Enable Register"]
pub mod peen;
#[doc = "Memory Checking Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mchkcon](mchkcon) module"]
pub type MCHKCON = crate::Reg<u32, _MCHKCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCHKCON;
#[doc = "`read()` method returns [mchkcon::R](mchkcon::R) reader structure"]
impl crate::Readable for MCHKCON {}
#[doc = "`write(|w| ..)` method takes [mchkcon::W](mchkcon::W) writer structure"]
impl crate::Writable for MCHKCON {}
#[doc = "Memory Checking Control Register"]
pub mod mchkcon;
#[doc = "Parity Error Trap Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pete](pete) module"]
pub type PETE = crate::Reg<u32, _PETE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PETE;
#[doc = "`read()` method returns [pete::R](pete::R) reader structure"]
impl crate::Readable for PETE {}
#[doc = "`write(|w| ..)` method takes [pete::W](pete::W) writer structure"]
impl crate::Writable for PETE {}
#[doc = "Parity Error Trap Enable Register"]
pub mod pete;
#[doc = "Parity Error Reset Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [persten](persten) module"]
pub type PERSTEN = crate::Reg<u32, _PERSTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERSTEN;
#[doc = "`read()` method returns [persten::R](persten::R) reader structure"]
impl crate::Readable for PERSTEN {}
#[doc = "`write(|w| ..)` method takes [persten::W](persten::W) writer structure"]
impl crate::Writable for PERSTEN {}
#[doc = "Parity Error Reset Enable Register"]
pub mod persten;
#[doc = "Parity Error Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peflag](peflag) module"]
pub type PEFLAG = crate::Reg<u32, _PEFLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEFLAG;
#[doc = "`read()` method returns [peflag::R](peflag::R) reader structure"]
impl crate::Readable for PEFLAG {}
#[doc = "`write(|w| ..)` method takes [peflag::W](peflag::W) writer structure"]
impl crate::Writable for PEFLAG {}
#[doc = "Parity Error Flag Register"]
pub mod peflag;
#[doc = "Parity Memory Test Pattern Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmtpr](pmtpr) module"]
pub type PMTPR = crate::Reg<u32, _PMTPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMTPR;
#[doc = "`read()` method returns [pmtpr::R](pmtpr::R) reader structure"]
impl crate::Readable for PMTPR {}
#[doc = "`write(|w| ..)` method takes [pmtpr::W](pmtpr::W) writer structure"]
impl crate::Writable for PMTPR {}
#[doc = "Parity Memory Test Pattern Register"]
pub mod pmtpr;
#[doc = "Parity Memory Test Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmtsr](pmtsr) module"]
pub type PMTSR = crate::Reg<u32, _PMTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMTSR;
#[doc = "`read()` method returns [pmtsr::R](pmtsr::R) reader structure"]
impl crate::Readable for PMTSR {}
#[doc = "`write(|w| ..)` method takes [pmtsr::W](pmtsr::W) writer structure"]
impl crate::Writable for PMTSR {}
#[doc = "Parity Memory Test Select Register"]
pub mod pmtsr;
