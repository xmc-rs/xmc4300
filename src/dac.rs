#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Module Identification Register"]
    pub id: ID,
    #[doc = "0x04 - DAC0 Configuration Register 0"]
    pub dac0cfg0: DAC0CFG0,
    #[doc = "0x08 - DAC0 Configuration Register 1"]
    pub dac0cfg1: DAC0CFG1,
    #[doc = "0x0c - DAC1 Configuration Register 0"]
    pub dac1cfg0: DAC1CFG0,
    #[doc = "0x10 - DAC1 Configuration Register 1"]
    pub dac1cfg1: DAC1CFG1,
    #[doc = "0x14 - DAC0 Data Register"]
    pub dac0data: DAC0DATA,
    #[doc = "0x18 - DAC1 Data Register"]
    pub dac1data: DAC1DATA,
    #[doc = "0x1c - DAC01 Data Register"]
    pub dac01data: DAC01DATA,
    #[doc = "0x20 - DAC0 Lower Pattern Register"]
    pub dac0patl: DAC0PATL,
    #[doc = "0x24 - DAC0 Higher Pattern Register"]
    pub dac0path: DAC0PATH,
    #[doc = "0x28 - DAC1 Lower Pattern Register"]
    pub dac1patl: DAC1PATL,
    #[doc = "0x2c - DAC1 Higher Pattern Register"]
    pub dac1path: DAC1PATH,
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
#[doc = "DAC0 Configuration Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dac0cfg0](dac0cfg0) module"]
pub type DAC0CFG0 = crate::Reg<u32, _DAC0CFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC0CFG0;
#[doc = "`read()` method returns [dac0cfg0::R](dac0cfg0::R) reader structure"]
impl crate::Readable for DAC0CFG0 {}
#[doc = "`write(|w| ..)` method takes [dac0cfg0::W](dac0cfg0::W) writer structure"]
impl crate::Writable for DAC0CFG0 {}
#[doc = "DAC0 Configuration Register 0"]
pub mod dac0cfg0;
#[doc = "DAC0 Configuration Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dac0cfg1](dac0cfg1) module"]
pub type DAC0CFG1 = crate::Reg<u32, _DAC0CFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC0CFG1;
#[doc = "`read()` method returns [dac0cfg1::R](dac0cfg1::R) reader structure"]
impl crate::Readable for DAC0CFG1 {}
#[doc = "`write(|w| ..)` method takes [dac0cfg1::W](dac0cfg1::W) writer structure"]
impl crate::Writable for DAC0CFG1 {}
#[doc = "DAC0 Configuration Register 1"]
pub mod dac0cfg1;
#[doc = "DAC1 Configuration Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dac1cfg0](dac1cfg0) module"]
pub type DAC1CFG0 = crate::Reg<u32, _DAC1CFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC1CFG0;
#[doc = "`read()` method returns [dac1cfg0::R](dac1cfg0::R) reader structure"]
impl crate::Readable for DAC1CFG0 {}
#[doc = "`write(|w| ..)` method takes [dac1cfg0::W](dac1cfg0::W) writer structure"]
impl crate::Writable for DAC1CFG0 {}
#[doc = "DAC1 Configuration Register 0"]
pub mod dac1cfg0;
#[doc = "DAC1 Configuration Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dac1cfg1](dac1cfg1) module"]
pub type DAC1CFG1 = crate::Reg<u32, _DAC1CFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC1CFG1;
#[doc = "`read()` method returns [dac1cfg1::R](dac1cfg1::R) reader structure"]
impl crate::Readable for DAC1CFG1 {}
#[doc = "`write(|w| ..)` method takes [dac1cfg1::W](dac1cfg1::W) writer structure"]
impl crate::Writable for DAC1CFG1 {}
#[doc = "DAC1 Configuration Register 1"]
pub mod dac1cfg1;
#[doc = "DAC0 Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dac0data](dac0data) module"]
pub type DAC0DATA = crate::Reg<u32, _DAC0DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC0DATA;
#[doc = "`read()` method returns [dac0data::R](dac0data::R) reader structure"]
impl crate::Readable for DAC0DATA {}
#[doc = "`write(|w| ..)` method takes [dac0data::W](dac0data::W) writer structure"]
impl crate::Writable for DAC0DATA {}
#[doc = "DAC0 Data Register"]
pub mod dac0data;
#[doc = "DAC1 Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dac1data](dac1data) module"]
pub type DAC1DATA = crate::Reg<u32, _DAC1DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC1DATA;
#[doc = "`read()` method returns [dac1data::R](dac1data::R) reader structure"]
impl crate::Readable for DAC1DATA {}
#[doc = "`write(|w| ..)` method takes [dac1data::W](dac1data::W) writer structure"]
impl crate::Writable for DAC1DATA {}
#[doc = "DAC1 Data Register"]
pub mod dac1data;
#[doc = "DAC01 Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dac01data](dac01data) module"]
pub type DAC01DATA = crate::Reg<u32, _DAC01DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC01DATA;
#[doc = "`read()` method returns [dac01data::R](dac01data::R) reader structure"]
impl crate::Readable for DAC01DATA {}
#[doc = "`write(|w| ..)` method takes [dac01data::W](dac01data::W) writer structure"]
impl crate::Writable for DAC01DATA {}
#[doc = "DAC01 Data Register"]
pub mod dac01data;
#[doc = "DAC0 Lower Pattern Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dac0patl](dac0patl) module"]
pub type DAC0PATL = crate::Reg<u32, _DAC0PATL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC0PATL;
#[doc = "`read()` method returns [dac0patl::R](dac0patl::R) reader structure"]
impl crate::Readable for DAC0PATL {}
#[doc = "`write(|w| ..)` method takes [dac0patl::W](dac0patl::W) writer structure"]
impl crate::Writable for DAC0PATL {}
#[doc = "DAC0 Lower Pattern Register"]
pub mod dac0patl;
#[doc = "DAC0 Higher Pattern Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dac0path](dac0path) module"]
pub type DAC0PATH = crate::Reg<u32, _DAC0PATH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC0PATH;
#[doc = "`read()` method returns [dac0path::R](dac0path::R) reader structure"]
impl crate::Readable for DAC0PATH {}
#[doc = "`write(|w| ..)` method takes [dac0path::W](dac0path::W) writer structure"]
impl crate::Writable for DAC0PATH {}
#[doc = "DAC0 Higher Pattern Register"]
pub mod dac0path;
#[doc = "DAC1 Lower Pattern Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dac1patl](dac1patl) module"]
pub type DAC1PATL = crate::Reg<u32, _DAC1PATL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC1PATL;
#[doc = "`read()` method returns [dac1patl::R](dac1patl::R) reader structure"]
impl crate::Readable for DAC1PATL {}
#[doc = "`write(|w| ..)` method takes [dac1patl::W](dac1patl::W) writer structure"]
impl crate::Writable for DAC1PATL {}
#[doc = "DAC1 Lower Pattern Register"]
pub mod dac1patl;
#[doc = "DAC1 Higher Pattern Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dac1path](dac1path) module"]
pub type DAC1PATH = crate::Reg<u32, _DAC1PATH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC1PATH;
#[doc = "`read()` method returns [dac1path::R](dac1path::R) reader structure"]
impl crate::Readable for DAC1PATH {}
#[doc = "`write(|w| ..)` method takes [dac1path::W](dac1path::W) writer structure"]
impl crate::Writable for DAC1PATH {}
#[doc = "DAC1 Higher Pattern Register"]
pub mod dac1path;
