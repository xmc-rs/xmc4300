#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Module Identification Register"]
    pub id: ID,
    #[doc = "0x04 - Global Control Register"]
    pub globctl: GLOBCTL,
    #[doc = "0x08 - Function Control Register"]
    pub fnctl: FNCTL,
    #[doc = "0x0c - Event Flag Register"]
    pub evfr: EVFR,
    #[doc = "0x10 - Touch-sense TS-Counter Value"]
    pub tsval: TSVAL,
    #[doc = "0x14 - Line Pattern Register 0"]
    pub line0: LINE0,
    #[doc = "0x18 - Line Pattern Register 1"]
    pub line1: LINE1,
    #[doc = "0x1c - LED Compare Register 0"]
    pub ldcmp0: LDCMP0,
    #[doc = "0x20 - LED Compare Register 1"]
    pub ldcmp1: LDCMP1,
    #[doc = "0x24 - Touch-sense Compare Register 0"]
    pub tscmp0: TSCMP0,
    #[doc = "0x28 - Touch-sense Compare Register 1"]
    pub tscmp1: TSCMP1,
}
#[doc = "Module Identification Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [id](id) module"]
pub type ID = crate::Reg<u32, _ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID;
#[doc = "`read()` method returns [id::R](id::R) reader structure"]
impl crate::Readable for ID {}
#[doc = "Module Identification Register"]
pub mod id;
#[doc = "Global Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [globctl](globctl) module"]
pub type GLOBCTL = crate::Reg<u32, _GLOBCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GLOBCTL;
#[doc = "`read()` method returns [globctl::R](globctl::R) reader structure"]
impl crate::Readable for GLOBCTL {}
#[doc = "`write(|w| ..)` method takes [globctl::W](globctl::W) writer structure"]
impl crate::Writable for GLOBCTL {}
#[doc = "Global Control Register"]
pub mod globctl;
#[doc = "Function Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fnctl](fnctl) module"]
pub type FNCTL = crate::Reg<u32, _FNCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FNCTL;
#[doc = "`read()` method returns [fnctl::R](fnctl::R) reader structure"]
impl crate::Readable for FNCTL {}
#[doc = "`write(|w| ..)` method takes [fnctl::W](fnctl::W) writer structure"]
impl crate::Writable for FNCTL {}
#[doc = "Function Control Register"]
pub mod fnctl;
#[doc = "Event Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [evfr](evfr) module"]
pub type EVFR = crate::Reg<u32, _EVFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVFR;
#[doc = "`read()` method returns [evfr::R](evfr::R) reader structure"]
impl crate::Readable for EVFR {}
#[doc = "`write(|w| ..)` method takes [evfr::W](evfr::W) writer structure"]
impl crate::Writable for EVFR {}
#[doc = "Event Flag Register"]
pub mod evfr;
#[doc = "Touch-sense TS-Counter Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tsval](tsval) module"]
pub type TSVAL = crate::Reg<u32, _TSVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSVAL;
#[doc = "`read()` method returns [tsval::R](tsval::R) reader structure"]
impl crate::Readable for TSVAL {}
#[doc = "`write(|w| ..)` method takes [tsval::W](tsval::W) writer structure"]
impl crate::Writable for TSVAL {}
#[doc = "Touch-sense TS-Counter Value"]
pub mod tsval;
#[doc = "Line Pattern Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [line0](line0) module"]
pub type LINE0 = crate::Reg<u32, _LINE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LINE0;
#[doc = "`read()` method returns [line0::R](line0::R) reader structure"]
impl crate::Readable for LINE0 {}
#[doc = "`write(|w| ..)` method takes [line0::W](line0::W) writer structure"]
impl crate::Writable for LINE0 {}
#[doc = "Line Pattern Register 0"]
pub mod line0;
#[doc = "Line Pattern Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [line1](line1) module"]
pub type LINE1 = crate::Reg<u32, _LINE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LINE1;
#[doc = "`read()` method returns [line1::R](line1::R) reader structure"]
impl crate::Readable for LINE1 {}
#[doc = "`write(|w| ..)` method takes [line1::W](line1::W) writer structure"]
impl crate::Writable for LINE1 {}
#[doc = "Line Pattern Register 1"]
pub mod line1;
#[doc = "LED Compare Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ldcmp0](ldcmp0) module"]
pub type LDCMP0 = crate::Reg<u32, _LDCMP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LDCMP0;
#[doc = "`read()` method returns [ldcmp0::R](ldcmp0::R) reader structure"]
impl crate::Readable for LDCMP0 {}
#[doc = "`write(|w| ..)` method takes [ldcmp0::W](ldcmp0::W) writer structure"]
impl crate::Writable for LDCMP0 {}
#[doc = "LED Compare Register 0"]
pub mod ldcmp0;
#[doc = "LED Compare Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ldcmp1](ldcmp1) module"]
pub type LDCMP1 = crate::Reg<u32, _LDCMP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LDCMP1;
#[doc = "`read()` method returns [ldcmp1::R](ldcmp1::R) reader structure"]
impl crate::Readable for LDCMP1 {}
#[doc = "`write(|w| ..)` method takes [ldcmp1::W](ldcmp1::W) writer structure"]
impl crate::Writable for LDCMP1 {}
#[doc = "LED Compare Register 1"]
pub mod ldcmp1;
#[doc = "Touch-sense Compare Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tscmp0](tscmp0) module"]
pub type TSCMP0 = crate::Reg<u32, _TSCMP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSCMP0;
#[doc = "`read()` method returns [tscmp0::R](tscmp0::R) reader structure"]
impl crate::Readable for TSCMP0 {}
#[doc = "`write(|w| ..)` method takes [tscmp0::W](tscmp0::W) writer structure"]
impl crate::Writable for TSCMP0 {}
#[doc = "Touch-sense Compare Register 0"]
pub mod tscmp0;
#[doc = "Touch-sense Compare Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tscmp1](tscmp1) module"]
pub type TSCMP1 = crate::Reg<u32, _TSCMP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSCMP1;
#[doc = "`read()` method returns [tscmp1::R](tscmp1::R) reader structure"]
impl crate::Readable for TSCMP1 {}
#[doc = "`write(|w| ..)` method takes [tscmp1::W](tscmp1::W) writer structure"]
impl crate::Writable for TSCMP1 {}
#[doc = "Touch-sense Compare Register 1"]
pub mod tscmp1;
