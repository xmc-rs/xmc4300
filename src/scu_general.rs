#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SCU Module ID Register"]
    pub id: ID,
    #[doc = "0x04 - Chip ID Register"]
    pub idchip: IDCHIP,
    #[doc = "0x08 - Manufactory ID Register"]
    pub idmanuf: IDMANUF,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - Startup Configuration Register"]
    pub stcon: STCON,
    _reserved4: [u8; 24usize],
    #[doc = "0x2c - General Purpose Register 0"]
    pub gpr0: GPR0,
    #[doc = "0x30 - General Purpose Register 1"]
    pub gpr1: GPR1,
    _reserved6: [u8; 24usize],
    #[doc = "0x4c - CCU Control Register"]
    pub ccucon: CCUCON,
    _reserved7: [u8; 60usize],
    #[doc = "0x8c - Die Temperature Sensor Control Register"]
    pub dtscon: DTSCON,
    #[doc = "0x90 - Die Temperature Sensor Status Register"]
    pub dtsstat: DTSSTAT,
    _reserved9: [u8; 8usize],
    #[doc = "0x9c - SD-MMC Delay Control Register"]
    pub sdmmcdel: SDMMCDEL,
    #[doc = "0xa0 - Out of Range Comparator Enable Register 0"]
    pub g0orcen: G0ORCEN,
    #[doc = "0xa4 - Out of Range Comparator Enable Register 1"]
    pub g1orcen: G1ORCEN,
    _reserved12: [u8; 28usize],
    #[doc = "0xc4 - Mirror Write Status Register"]
    pub mirrsts: MIRRSTS,
    #[doc = "0xc8 - Retention Memory Access Control Register"]
    pub rmacr: RMACR,
    #[doc = "0xcc - Retention Memory Access Data Register"]
    pub rmdata: RMDATA,
}
#[doc = "SCU Module ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [id](id) module"]
pub type ID = crate::Reg<u32, _ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID;
#[doc = "`read()` method returns [id::R](id::R) reader structure"]
impl crate::Readable for ID {}
#[doc = "SCU Module ID Register"]
pub mod id;
#[doc = "Chip ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [idchip](idchip) module"]
pub type IDCHIP = crate::Reg<u32, _IDCHIP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDCHIP;
#[doc = "`read()` method returns [idchip::R](idchip::R) reader structure"]
impl crate::Readable for IDCHIP {}
#[doc = "Chip ID Register"]
pub mod idchip;
#[doc = "Manufactory ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [idmanuf](idmanuf) module"]
pub type IDMANUF = crate::Reg<u32, _IDMANUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDMANUF;
#[doc = "`read()` method returns [idmanuf::R](idmanuf::R) reader structure"]
impl crate::Readable for IDMANUF {}
#[doc = "Manufactory ID Register"]
pub mod idmanuf;
#[doc = "Startup Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [stcon](stcon) module"]
pub type STCON = crate::Reg<u32, _STCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STCON;
#[doc = "`read()` method returns [stcon::R](stcon::R) reader structure"]
impl crate::Readable for STCON {}
#[doc = "`write(|w| ..)` method takes [stcon::W](stcon::W) writer structure"]
impl crate::Writable for STCON {}
#[doc = "Startup Configuration Register"]
pub mod stcon;
#[doc = "General Purpose Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpr0](gpr0) module"]
pub type GPR0 = crate::Reg<u32, _GPR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR0;
#[doc = "`read()` method returns [gpr0::R](gpr0::R) reader structure"]
impl crate::Readable for GPR0 {}
#[doc = "`write(|w| ..)` method takes [gpr0::W](gpr0::W) writer structure"]
impl crate::Writable for GPR0 {}
#[doc = "General Purpose Register 0"]
pub mod gpr0;
#[doc = "General Purpose Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpr1](gpr1) module"]
pub type GPR1 = crate::Reg<u32, _GPR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR1;
#[doc = "`read()` method returns [gpr1::R](gpr1::R) reader structure"]
impl crate::Readable for GPR1 {}
#[doc = "`write(|w| ..)` method takes [gpr1::W](gpr1::W) writer structure"]
impl crate::Writable for GPR1 {}
#[doc = "General Purpose Register 1"]
pub mod gpr1;
#[doc = "CCU Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ccucon](ccucon) module"]
pub type CCUCON = crate::Reg<u32, _CCUCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCUCON;
#[doc = "`read()` method returns [ccucon::R](ccucon::R) reader structure"]
impl crate::Readable for CCUCON {}
#[doc = "`write(|w| ..)` method takes [ccucon::W](ccucon::W) writer structure"]
impl crate::Writable for CCUCON {}
#[doc = "CCU Control Register"]
pub mod ccucon;
#[doc = "Die Temperature Sensor Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dtscon](dtscon) module"]
pub type DTSCON = crate::Reg<u32, _DTSCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTSCON;
#[doc = "`read()` method returns [dtscon::R](dtscon::R) reader structure"]
impl crate::Readable for DTSCON {}
#[doc = "`write(|w| ..)` method takes [dtscon::W](dtscon::W) writer structure"]
impl crate::Writable for DTSCON {}
#[doc = "Die Temperature Sensor Control Register"]
pub mod dtscon;
#[doc = "Die Temperature Sensor Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dtsstat](dtsstat) module"]
pub type DTSSTAT = crate::Reg<u32, _DTSSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTSSTAT;
#[doc = "`read()` method returns [dtsstat::R](dtsstat::R) reader structure"]
impl crate::Readable for DTSSTAT {}
#[doc = "Die Temperature Sensor Status Register"]
pub mod dtsstat;
#[doc = "SD-MMC Delay Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sdmmcdel](sdmmcdel) module"]
pub type SDMMCDEL = crate::Reg<u32, _SDMMCDEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMCDEL;
#[doc = "`read()` method returns [sdmmcdel::R](sdmmcdel::R) reader structure"]
impl crate::Readable for SDMMCDEL {}
#[doc = "`write(|w| ..)` method takes [sdmmcdel::W](sdmmcdel::W) writer structure"]
impl crate::Writable for SDMMCDEL {}
#[doc = "SD-MMC Delay Control Register"]
pub mod sdmmcdel;
#[doc = "Out of Range Comparator Enable Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [g0orcen](g0orcen) module"]
pub type G0ORCEN = crate::Reg<u32, _G0ORCEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _G0ORCEN;
#[doc = "`read()` method returns [g0orcen::R](g0orcen::R) reader structure"]
impl crate::Readable for G0ORCEN {}
#[doc = "`write(|w| ..)` method takes [g0orcen::W](g0orcen::W) writer structure"]
impl crate::Writable for G0ORCEN {}
#[doc = "Out of Range Comparator Enable Register 0"]
pub mod g0orcen;
#[doc = "Out of Range Comparator Enable Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [g1orcen](g1orcen) module"]
pub type G1ORCEN = crate::Reg<u32, _G1ORCEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _G1ORCEN;
#[doc = "`read()` method returns [g1orcen::R](g1orcen::R) reader structure"]
impl crate::Readable for G1ORCEN {}
#[doc = "`write(|w| ..)` method takes [g1orcen::W](g1orcen::W) writer structure"]
impl crate::Writable for G1ORCEN {}
#[doc = "Out of Range Comparator Enable Register 1"]
pub mod g1orcen;
#[doc = "Mirror Write Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mirrsts](mirrsts) module"]
pub type MIRRSTS = crate::Reg<u32, _MIRRSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIRRSTS;
#[doc = "`read()` method returns [mirrsts::R](mirrsts::R) reader structure"]
impl crate::Readable for MIRRSTS {}
#[doc = "Mirror Write Status Register"]
pub mod mirrsts;
#[doc = "Retention Memory Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmacr](rmacr) module"]
pub type RMACR = crate::Reg<u32, _RMACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMACR;
#[doc = "`read()` method returns [rmacr::R](rmacr::R) reader structure"]
impl crate::Readable for RMACR {}
#[doc = "`write(|w| ..)` method takes [rmacr::W](rmacr::W) writer structure"]
impl crate::Writable for RMACR {}
#[doc = "Retention Memory Access Control Register"]
pub mod rmacr;
#[doc = "Retention Memory Access Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmdata](rmdata) module"]
pub type RMDATA = crate::Reg<u32, _RMDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMDATA;
#[doc = "`read()` method returns [rmdata::R](rmdata::R) reader structure"]
impl crate::Readable for RMDATA {}
#[doc = "`write(|w| ..)` method takes [rmdata::W](rmdata::W) writer structure"]
impl crate::Writable for RMDATA {}
#[doc = "Retention Memory Access Data Register"]
pub mod rmdata;
