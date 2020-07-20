#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4104usize],
    #[doc = "0x1008 - Flash Module Identification Register"]
    pub id: ID,
    _reserved1: [u8; 4usize],
    #[doc = "0x1010 - Flash Status Register"]
    pub fsr: FSR,
    #[doc = "0x1014 - Flash Configuration Register"]
    pub fcon: FCON,
    #[doc = "0x1018 - Margin Control Register PFLASH"]
    pub marp: MARP,
    _reserved4: [u8; 4usize],
    #[doc = "0x1020 - Flash Protection Configuration Register User 0"]
    pub procon0: PROCON0,
    #[doc = "0x1024 - Flash Protection Configuration Register User 1"]
    pub procon1: PROCON1,
    #[doc = "0x1028 - Flash Protection Configuration Register User 2"]
    pub procon2: PROCON2,
}
#[doc = "Flash Module Identification Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id](id) module"]
pub type ID = crate::Reg<u32, _ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID;
#[doc = "`read()` method returns [id::R](id::R) reader structure"]
impl crate::Readable for ID {}
#[doc = "Flash Module Identification Register"]
pub mod id;
#[doc = "Flash Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsr](fsr) module"]
pub type FSR = crate::Reg<u32, _FSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSR;
#[doc = "`read()` method returns [fsr::R](fsr::R) reader structure"]
impl crate::Readable for FSR {}
#[doc = "Flash Status Register"]
pub mod fsr;
#[doc = "Flash Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcon](fcon) module"]
pub type FCON = crate::Reg<u32, _FCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCON;
#[doc = "`read()` method returns [fcon::R](fcon::R) reader structure"]
impl crate::Readable for FCON {}
#[doc = "`write(|w| ..)` method takes [fcon::W](fcon::W) writer structure"]
impl crate::Writable for FCON {}
#[doc = "Flash Configuration Register"]
pub mod fcon;
#[doc = "Margin Control Register PFLASH\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [marp](marp) module"]
pub type MARP = crate::Reg<u32, _MARP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MARP;
#[doc = "`read()` method returns [marp::R](marp::R) reader structure"]
impl crate::Readable for MARP {}
#[doc = "`write(|w| ..)` method takes [marp::W](marp::W) writer structure"]
impl crate::Writable for MARP {}
#[doc = "Margin Control Register PFLASH"]
pub mod marp;
#[doc = "Flash Protection Configuration Register User 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [procon0](procon0) module"]
pub type PROCON0 = crate::Reg<u32, _PROCON0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROCON0;
#[doc = "`read()` method returns [procon0::R](procon0::R) reader structure"]
impl crate::Readable for PROCON0 {}
#[doc = "Flash Protection Configuration Register User 0"]
pub mod procon0;
#[doc = "Flash Protection Configuration Register User 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [procon1](procon1) module"]
pub type PROCON1 = crate::Reg<u32, _PROCON1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROCON1;
#[doc = "`read()` method returns [procon1::R](procon1::R) reader structure"]
impl crate::Readable for PROCON1 {}
#[doc = "Flash Protection Configuration Register User 1"]
pub mod procon1;
#[doc = "Flash Protection Configuration Register User 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [procon2](procon2) module"]
pub type PROCON2 = crate::Reg<u32, _PROCON2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROCON2;
#[doc = "`read()` method returns [procon2::R](procon2::R) reader structure"]
impl crate::Readable for PROCON2 {}
#[doc = "Flash Protection Configuration Register User 2"]
pub mod procon2;
