#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Raw IntTfr Status"]
    pub rawtfr: RAWTFR,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - Raw IntBlock Status"]
    pub rawblock: RAWBLOCK,
    _reserved2: [u8; 4usize],
    #[doc = "0x10 - Raw IntSrcTran Status"]
    pub rawsrctran: RAWSRCTRAN,
    _reserved3: [u8; 4usize],
    #[doc = "0x18 - Raw IntBlock Status"]
    pub rawdsttran: RAWDSTTRAN,
    _reserved4: [u8; 4usize],
    #[doc = "0x20 - Raw IntErr Status"]
    pub rawerr: RAWERR,
    _reserved5: [u8; 4usize],
    #[doc = "0x28 - IntTfr Status"]
    pub statustfr: STATUSTFR,
    _reserved6: [u8; 4usize],
    #[doc = "0x30 - IntBlock Status"]
    pub statusblock: STATUSBLOCK,
    _reserved7: [u8; 4usize],
    #[doc = "0x38 - IntSrcTran Status"]
    pub statussrctran: STATUSSRCTRAN,
    _reserved8: [u8; 4usize],
    #[doc = "0x40 - IntBlock Status"]
    pub statusdsttran: STATUSDSTTRAN,
    _reserved9: [u8; 4usize],
    #[doc = "0x48 - IntErr Status"]
    pub statuserr: STATUSERR,
    _reserved10: [u8; 4usize],
    #[doc = "0x50 - Mask for Raw IntTfr Status"]
    pub masktfr: MASKTFR,
    _reserved11: [u8; 4usize],
    #[doc = "0x58 - Mask for Raw IntBlock Status"]
    pub maskblock: MASKBLOCK,
    _reserved12: [u8; 4usize],
    #[doc = "0x60 - Mask for Raw IntSrcTran Status"]
    pub masksrctran: MASKSRCTRAN,
    _reserved13: [u8; 4usize],
    #[doc = "0x68 - Mask for Raw IntBlock Status"]
    pub maskdsttran: MASKDSTTRAN,
    _reserved14: [u8; 4usize],
    #[doc = "0x70 - Mask for Raw IntErr Status"]
    pub maskerr: MASKERR,
    _reserved15: [u8; 4usize],
    #[doc = "0x78 - IntTfr Status"]
    pub cleartfr: CLEARTFR,
    _reserved16: [u8; 4usize],
    #[doc = "0x80 - IntBlock Status"]
    pub clearblock: CLEARBLOCK,
    _reserved17: [u8; 4usize],
    #[doc = "0x88 - IntSrcTran Status"]
    pub clearsrctran: CLEARSRCTRAN,
    _reserved18: [u8; 4usize],
    #[doc = "0x90 - IntBlock Status"]
    pub cleardsttran: CLEARDSTTRAN,
    _reserved19: [u8; 4usize],
    #[doc = "0x98 - IntErr Status"]
    pub clearerr: CLEARERR,
    _reserved20: [u8; 4usize],
    #[doc = "0xa0 - Combined Interrupt Status Register"]
    pub statusint: STATUSINT,
    _reserved21: [u8; 4usize],
    #[doc = "0xa8 - Source Software Transaction Request Register"]
    pub reqsrcreg: REQSRCREG,
    _reserved22: [u8; 4usize],
    #[doc = "0xb0 - Destination Software Transaction Request Register"]
    pub reqdstreg: REQDSTREG,
    _reserved23: [u8; 4usize],
    #[doc = "0xb8 - Single Source Transaction Request Register"]
    pub sglreqsrcreg: SGLREQSRCREG,
    _reserved24: [u8; 4usize],
    #[doc = "0xc0 - Single Destination Transaction Request Register"]
    pub sglreqdstreg: SGLREQDSTREG,
    _reserved25: [u8; 4usize],
    #[doc = "0xc8 - Last Source Transaction Request Register"]
    pub lstsrcreg: LSTSRCREG,
    _reserved26: [u8; 4usize],
    #[doc = "0xd0 - Last Destination Transaction Request Register"]
    pub lstdstreg: LSTDSTREG,
    _reserved27: [u8; 4usize],
    #[doc = "0xd8 - GPDMA Configuration Register"]
    pub dmacfgreg: DMACFGREG,
    _reserved28: [u8; 4usize],
    #[doc = "0xe0 - GPDMA Channel Enable Register"]
    pub chenreg: CHENREG,
    _reserved29: [u8; 4usize],
    #[doc = "0xe8 - GPDMA0 ID Register"]
    pub id: ID,
    _reserved30: [u8; 76usize],
    #[doc = "0x138 - GPDMA Component Type"]
    pub type_: TYPE,
    #[doc = "0x13c - DMA Component Version"]
    pub version: VERSION,
}
#[doc = "Raw IntTfr Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rawtfr](rawtfr) module"]
pub type RAWTFR = crate::Reg<u32, _RAWTFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAWTFR;
#[doc = "`read()` method returns [rawtfr::R](rawtfr::R) reader structure"]
impl crate::Readable for RAWTFR {}
#[doc = "`write(|w| ..)` method takes [rawtfr::W](rawtfr::W) writer structure"]
impl crate::Writable for RAWTFR {}
#[doc = "Raw IntTfr Status"]
pub mod rawtfr;
#[doc = "Raw IntBlock Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rawblock](rawblock) module"]
pub type RAWBLOCK = crate::Reg<u32, _RAWBLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAWBLOCK;
#[doc = "`read()` method returns [rawblock::R](rawblock::R) reader structure"]
impl crate::Readable for RAWBLOCK {}
#[doc = "`write(|w| ..)` method takes [rawblock::W](rawblock::W) writer structure"]
impl crate::Writable for RAWBLOCK {}
#[doc = "Raw IntBlock Status"]
pub mod rawblock;
#[doc = "Raw IntSrcTran Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rawsrctran](rawsrctran) module"]
pub type RAWSRCTRAN = crate::Reg<u32, _RAWSRCTRAN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAWSRCTRAN;
#[doc = "`read()` method returns [rawsrctran::R](rawsrctran::R) reader structure"]
impl crate::Readable for RAWSRCTRAN {}
#[doc = "`write(|w| ..)` method takes [rawsrctran::W](rawsrctran::W) writer structure"]
impl crate::Writable for RAWSRCTRAN {}
#[doc = "Raw IntSrcTran Status"]
pub mod rawsrctran;
#[doc = "Raw IntBlock Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rawdsttran](rawdsttran) module"]
pub type RAWDSTTRAN = crate::Reg<u32, _RAWDSTTRAN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAWDSTTRAN;
#[doc = "`read()` method returns [rawdsttran::R](rawdsttran::R) reader structure"]
impl crate::Readable for RAWDSTTRAN {}
#[doc = "`write(|w| ..)` method takes [rawdsttran::W](rawdsttran::W) writer structure"]
impl crate::Writable for RAWDSTTRAN {}
#[doc = "Raw IntBlock Status"]
pub mod rawdsttran;
#[doc = "Raw IntErr Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rawerr](rawerr) module"]
pub type RAWERR = crate::Reg<u32, _RAWERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAWERR;
#[doc = "`read()` method returns [rawerr::R](rawerr::R) reader structure"]
impl crate::Readable for RAWERR {}
#[doc = "`write(|w| ..)` method takes [rawerr::W](rawerr::W) writer structure"]
impl crate::Writable for RAWERR {}
#[doc = "Raw IntErr Status"]
pub mod rawerr;
#[doc = "IntTfr Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [statustfr](statustfr) module"]
pub type STATUSTFR = crate::Reg<u32, _STATUSTFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUSTFR;
#[doc = "`read()` method returns [statustfr::R](statustfr::R) reader structure"]
impl crate::Readable for STATUSTFR {}
#[doc = "IntTfr Status"]
pub mod statustfr;
#[doc = "IntBlock Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [statusblock](statusblock) module"]
pub type STATUSBLOCK = crate::Reg<u32, _STATUSBLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUSBLOCK;
#[doc = "`read()` method returns [statusblock::R](statusblock::R) reader structure"]
impl crate::Readable for STATUSBLOCK {}
#[doc = "IntBlock Status"]
pub mod statusblock;
#[doc = "IntSrcTran Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [statussrctran](statussrctran) module"]
pub type STATUSSRCTRAN = crate::Reg<u32, _STATUSSRCTRAN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUSSRCTRAN;
#[doc = "`read()` method returns [statussrctran::R](statussrctran::R) reader structure"]
impl crate::Readable for STATUSSRCTRAN {}
#[doc = "IntSrcTran Status"]
pub mod statussrctran;
#[doc = "IntBlock Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [statusdsttran](statusdsttran) module"]
pub type STATUSDSTTRAN = crate::Reg<u32, _STATUSDSTTRAN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUSDSTTRAN;
#[doc = "`read()` method returns [statusdsttran::R](statusdsttran::R) reader structure"]
impl crate::Readable for STATUSDSTTRAN {}
#[doc = "IntBlock Status"]
pub mod statusdsttran;
#[doc = "IntErr Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [statuserr](statuserr) module"]
pub type STATUSERR = crate::Reg<u32, _STATUSERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUSERR;
#[doc = "`read()` method returns [statuserr::R](statuserr::R) reader structure"]
impl crate::Readable for STATUSERR {}
#[doc = "IntErr Status"]
pub mod statuserr;
#[doc = "Mask for Raw IntTfr Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [masktfr](masktfr) module"]
pub type MASKTFR = crate::Reg<u32, _MASKTFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASKTFR;
#[doc = "`read()` method returns [masktfr::R](masktfr::R) reader structure"]
impl crate::Readable for MASKTFR {}
#[doc = "`write(|w| ..)` method takes [masktfr::W](masktfr::W) writer structure"]
impl crate::Writable for MASKTFR {}
#[doc = "Mask for Raw IntTfr Status"]
pub mod masktfr;
#[doc = "Mask for Raw IntBlock Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [maskblock](maskblock) module"]
pub type MASKBLOCK = crate::Reg<u32, _MASKBLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASKBLOCK;
#[doc = "`read()` method returns [maskblock::R](maskblock::R) reader structure"]
impl crate::Readable for MASKBLOCK {}
#[doc = "`write(|w| ..)` method takes [maskblock::W](maskblock::W) writer structure"]
impl crate::Writable for MASKBLOCK {}
#[doc = "Mask for Raw IntBlock Status"]
pub mod maskblock;
#[doc = "Mask for Raw IntSrcTran Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [masksrctran](masksrctran) module"]
pub type MASKSRCTRAN = crate::Reg<u32, _MASKSRCTRAN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASKSRCTRAN;
#[doc = "`read()` method returns [masksrctran::R](masksrctran::R) reader structure"]
impl crate::Readable for MASKSRCTRAN {}
#[doc = "`write(|w| ..)` method takes [masksrctran::W](masksrctran::W) writer structure"]
impl crate::Writable for MASKSRCTRAN {}
#[doc = "Mask for Raw IntSrcTran Status"]
pub mod masksrctran;
#[doc = "Mask for Raw IntBlock Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [maskdsttran](maskdsttran) module"]
pub type MASKDSTTRAN = crate::Reg<u32, _MASKDSTTRAN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASKDSTTRAN;
#[doc = "`read()` method returns [maskdsttran::R](maskdsttran::R) reader structure"]
impl crate::Readable for MASKDSTTRAN {}
#[doc = "`write(|w| ..)` method takes [maskdsttran::W](maskdsttran::W) writer structure"]
impl crate::Writable for MASKDSTTRAN {}
#[doc = "Mask for Raw IntBlock Status"]
pub mod maskdsttran;
#[doc = "Mask for Raw IntErr Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [maskerr](maskerr) module"]
pub type MASKERR = crate::Reg<u32, _MASKERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASKERR;
#[doc = "`read()` method returns [maskerr::R](maskerr::R) reader structure"]
impl crate::Readable for MASKERR {}
#[doc = "`write(|w| ..)` method takes [maskerr::W](maskerr::W) writer structure"]
impl crate::Writable for MASKERR {}
#[doc = "Mask for Raw IntErr Status"]
pub mod maskerr;
#[doc = "IntTfr Status\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cleartfr](cleartfr) module"]
pub type CLEARTFR = crate::Reg<u32, _CLEARTFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLEARTFR;
#[doc = "`write(|w| ..)` method takes [cleartfr::W](cleartfr::W) writer structure"]
impl crate::Writable for CLEARTFR {}
#[doc = "IntTfr Status"]
pub mod cleartfr;
#[doc = "IntBlock Status\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clearblock](clearblock) module"]
pub type CLEARBLOCK = crate::Reg<u32, _CLEARBLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLEARBLOCK;
#[doc = "`write(|w| ..)` method takes [clearblock::W](clearblock::W) writer structure"]
impl crate::Writable for CLEARBLOCK {}
#[doc = "IntBlock Status"]
pub mod clearblock;
#[doc = "IntSrcTran Status\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clearsrctran](clearsrctran) module"]
pub type CLEARSRCTRAN = crate::Reg<u32, _CLEARSRCTRAN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLEARSRCTRAN;
#[doc = "`write(|w| ..)` method takes [clearsrctran::W](clearsrctran::W) writer structure"]
impl crate::Writable for CLEARSRCTRAN {}
#[doc = "IntSrcTran Status"]
pub mod clearsrctran;
#[doc = "IntBlock Status\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cleardsttran](cleardsttran) module"]
pub type CLEARDSTTRAN = crate::Reg<u32, _CLEARDSTTRAN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLEARDSTTRAN;
#[doc = "`write(|w| ..)` method takes [cleardsttran::W](cleardsttran::W) writer structure"]
impl crate::Writable for CLEARDSTTRAN {}
#[doc = "IntBlock Status"]
pub mod cleardsttran;
#[doc = "IntErr Status\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clearerr](clearerr) module"]
pub type CLEARERR = crate::Reg<u32, _CLEARERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLEARERR;
#[doc = "`write(|w| ..)` method takes [clearerr::W](clearerr::W) writer structure"]
impl crate::Writable for CLEARERR {}
#[doc = "IntErr Status"]
pub mod clearerr;
#[doc = "Combined Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [statusint](statusint) module"]
pub type STATUSINT = crate::Reg<u32, _STATUSINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUSINT;
#[doc = "`read()` method returns [statusint::R](statusint::R) reader structure"]
impl crate::Readable for STATUSINT {}
#[doc = "Combined Interrupt Status Register"]
pub mod statusint;
#[doc = "Source Software Transaction Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [reqsrcreg](reqsrcreg) module"]
pub type REQSRCREG = crate::Reg<u32, _REQSRCREG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REQSRCREG;
#[doc = "`read()` method returns [reqsrcreg::R](reqsrcreg::R) reader structure"]
impl crate::Readable for REQSRCREG {}
#[doc = "`write(|w| ..)` method takes [reqsrcreg::W](reqsrcreg::W) writer structure"]
impl crate::Writable for REQSRCREG {}
#[doc = "Source Software Transaction Request Register"]
pub mod reqsrcreg;
#[doc = "Destination Software Transaction Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [reqdstreg](reqdstreg) module"]
pub type REQDSTREG = crate::Reg<u32, _REQDSTREG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REQDSTREG;
#[doc = "`read()` method returns [reqdstreg::R](reqdstreg::R) reader structure"]
impl crate::Readable for REQDSTREG {}
#[doc = "`write(|w| ..)` method takes [reqdstreg::W](reqdstreg::W) writer structure"]
impl crate::Writable for REQDSTREG {}
#[doc = "Destination Software Transaction Request Register"]
pub mod reqdstreg;
#[doc = "Single Source Transaction Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sglreqsrcreg](sglreqsrcreg) module"]
pub type SGLREQSRCREG = crate::Reg<u32, _SGLREQSRCREG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SGLREQSRCREG;
#[doc = "`read()` method returns [sglreqsrcreg::R](sglreqsrcreg::R) reader structure"]
impl crate::Readable for SGLREQSRCREG {}
#[doc = "`write(|w| ..)` method takes [sglreqsrcreg::W](sglreqsrcreg::W) writer structure"]
impl crate::Writable for SGLREQSRCREG {}
#[doc = "Single Source Transaction Request Register"]
pub mod sglreqsrcreg;
#[doc = "Single Destination Transaction Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sglreqdstreg](sglreqdstreg) module"]
pub type SGLREQDSTREG = crate::Reg<u32, _SGLREQDSTREG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SGLREQDSTREG;
#[doc = "`read()` method returns [sglreqdstreg::R](sglreqdstreg::R) reader structure"]
impl crate::Readable for SGLREQDSTREG {}
#[doc = "`write(|w| ..)` method takes [sglreqdstreg::W](sglreqdstreg::W) writer structure"]
impl crate::Writable for SGLREQDSTREG {}
#[doc = "Single Destination Transaction Request Register"]
pub mod sglreqdstreg;
#[doc = "Last Source Transaction Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lstsrcreg](lstsrcreg) module"]
pub type LSTSRCREG = crate::Reg<u32, _LSTSRCREG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSTSRCREG;
#[doc = "`read()` method returns [lstsrcreg::R](lstsrcreg::R) reader structure"]
impl crate::Readable for LSTSRCREG {}
#[doc = "`write(|w| ..)` method takes [lstsrcreg::W](lstsrcreg::W) writer structure"]
impl crate::Writable for LSTSRCREG {}
#[doc = "Last Source Transaction Request Register"]
pub mod lstsrcreg;
#[doc = "Last Destination Transaction Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lstdstreg](lstdstreg) module"]
pub type LSTDSTREG = crate::Reg<u32, _LSTDSTREG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSTDSTREG;
#[doc = "`read()` method returns [lstdstreg::R](lstdstreg::R) reader structure"]
impl crate::Readable for LSTDSTREG {}
#[doc = "`write(|w| ..)` method takes [lstdstreg::W](lstdstreg::W) writer structure"]
impl crate::Writable for LSTDSTREG {}
#[doc = "Last Destination Transaction Request Register"]
pub mod lstdstreg;
#[doc = "GPDMA Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmacfgreg](dmacfgreg) module"]
pub type DMACFGREG = crate::Reg<u32, _DMACFGREG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACFGREG;
#[doc = "`read()` method returns [dmacfgreg::R](dmacfgreg::R) reader structure"]
impl crate::Readable for DMACFGREG {}
#[doc = "`write(|w| ..)` method takes [dmacfgreg::W](dmacfgreg::W) writer structure"]
impl crate::Writable for DMACFGREG {}
#[doc = "GPDMA Configuration Register"]
pub mod dmacfgreg;
#[doc = "GPDMA Channel Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chenreg](chenreg) module"]
pub type CHENREG = crate::Reg<u32, _CHENREG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHENREG;
#[doc = "`read()` method returns [chenreg::R](chenreg::R) reader structure"]
impl crate::Readable for CHENREG {}
#[doc = "`write(|w| ..)` method takes [chenreg::W](chenreg::W) writer structure"]
impl crate::Writable for CHENREG {}
#[doc = "GPDMA Channel Enable Register"]
pub mod chenreg;
#[doc = "GPDMA0 ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [id](id) module"]
pub type ID = crate::Reg<u32, _ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID;
#[doc = "`read()` method returns [id::R](id::R) reader structure"]
impl crate::Readable for ID {}
#[doc = "GPDMA0 ID Register"]
pub mod id;
#[doc = "GPDMA Component Type\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [type_](type_) module"]
pub type TYPE = crate::Reg<u32, _TYPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TYPE;
#[doc = "`read()` method returns [type_::R](type_::R) reader structure"]
impl crate::Readable for TYPE {}
#[doc = "GPDMA Component Type"]
pub mod type_;
#[doc = "DMA Component Version\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [version](version) module"]
pub type VERSION = crate::Reg<u32, _VERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VERSION;
#[doc = "`read()` method returns [version::R](version::R) reader structure"]
impl crate::Readable for VERSION {}
#[doc = "DMA Component Version"]
pub mod version;
