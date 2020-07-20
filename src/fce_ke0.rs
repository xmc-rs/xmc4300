#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Input Register"]
    pub ir: IR,
    #[doc = "0x04 - CRC Result Register"]
    pub res: RES,
    #[doc = "0x08 - CRC Configuration Register"]
    pub cfg: CFG,
    #[doc = "0x0c - CRC Status Register"]
    pub sts: STS,
    #[doc = "0x10 - CRC Length Register"]
    pub length: LENGTH,
    #[doc = "0x14 - CRC Check Register"]
    pub check: CHECK,
    #[doc = "0x18 - CRC Register"]
    pub crc: CRC,
    #[doc = "0x1c - CRC Test Register"]
    pub ctr: CTR,
}
#[doc = "Input Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ir](ir) module"]
pub type IR = crate::Reg<u32, _IR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IR;
#[doc = "`read()` method returns [ir::R](ir::R) reader structure"]
impl crate::Readable for IR {}
#[doc = "`write(|w| ..)` method takes [ir::W](ir::W) writer structure"]
impl crate::Writable for IR {}
#[doc = "Input Register"]
pub mod ir;
#[doc = "CRC Result Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [res](res) module"]
pub type RES = crate::Reg<u32, _RES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RES;
#[doc = "`read()` method returns [res::R](res::R) reader structure"]
impl crate::Readable for RES {}
#[doc = "CRC Result Register"]
pub mod res;
#[doc = "CRC Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u32, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`read()` method returns [cfg::R](cfg::R) reader structure"]
impl crate::Readable for CFG {}
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "CRC Configuration Register"]
pub mod cfg;
#[doc = "CRC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts](sts) module"]
pub type STS = crate::Reg<u32, _STS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STS;
#[doc = "`read()` method returns [sts::R](sts::R) reader structure"]
impl crate::Readable for STS {}
#[doc = "`write(|w| ..)` method takes [sts::W](sts::W) writer structure"]
impl crate::Writable for STS {}
#[doc = "CRC Status Register"]
pub mod sts;
#[doc = "CRC Length Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [length](length) module"]
pub type LENGTH = crate::Reg<u32, _LENGTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LENGTH;
#[doc = "`read()` method returns [length::R](length::R) reader structure"]
impl crate::Readable for LENGTH {}
#[doc = "`write(|w| ..)` method takes [length::W](length::W) writer structure"]
impl crate::Writable for LENGTH {}
#[doc = "CRC Length Register"]
pub mod length;
#[doc = "CRC Check Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [check](check) module"]
pub type CHECK = crate::Reg<u32, _CHECK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHECK;
#[doc = "`read()` method returns [check::R](check::R) reader structure"]
impl crate::Readable for CHECK {}
#[doc = "`write(|w| ..)` method takes [check::W](check::W) writer structure"]
impl crate::Writable for CHECK {}
#[doc = "CRC Check Register"]
pub mod check;
#[doc = "CRC Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc](crc) module"]
pub type CRC = crate::Reg<u32, _CRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC;
#[doc = "`read()` method returns [crc::R](crc::R) reader structure"]
impl crate::Readable for CRC {}
#[doc = "`write(|w| ..)` method takes [crc::W](crc::W) writer structure"]
impl crate::Writable for CRC {}
#[doc = "CRC Register"]
pub mod crc;
#[doc = "CRC Test Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctr](ctr) module"]
pub type CTR = crate::Reg<u32, _CTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTR;
#[doc = "`read()` method returns [ctr::R](ctr::R) reader structure"]
impl crate::Readable for CTR {}
#[doc = "`write(|w| ..)` method takes [ctr::W](ctr::W) writer structure"]
impl crate::Writable for CTR {}
#[doc = "CRC Test Register"]
pub mod ctr;
