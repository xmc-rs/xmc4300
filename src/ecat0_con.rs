#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EtherCAT 0 Control"]
    pub con: CON,
    #[doc = "0x04 - EtherCAT 0 Port 1 Control Register"]
    pub conp0: CONP0,
    #[doc = "0x08 - EtherCAT 0 Port 1 Control Register"]
    pub conp1: CONP1,
}
#[doc = "EtherCAT 0 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [con](con) module"]
pub type CON = crate::Reg<u32, _CON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CON;
#[doc = "`read()` method returns [con::R](con::R) reader structure"]
impl crate::Readable for CON {}
#[doc = "`write(|w| ..)` method takes [con::W](con::W) writer structure"]
impl crate::Writable for CON {}
#[doc = "EtherCAT 0 Control"]
pub mod con;
#[doc = "EtherCAT 0 Port 1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conp0](conp0) module"]
pub type CONP0 = crate::Reg<u32, _CONP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONP0;
#[doc = "`read()` method returns [conp0::R](conp0::R) reader structure"]
impl crate::Readable for CONP0 {}
#[doc = "`write(|w| ..)` method takes [conp0::W](conp0::W) writer structure"]
impl crate::Writable for CONP0 {}
#[doc = "EtherCAT 0 Port 1 Control Register"]
pub mod conp0;
#[doc = "EtherCAT 0 Port 1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conp1](conp1) module"]
pub type CONP1 = crate::Reg<u32, _CONP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONP1;
#[doc = "`read()` method returns [conp1::R](conp1::R) reader structure"]
impl crate::Readable for CONP1 {}
#[doc = "`write(|w| ..)` method takes [conp1::W](conp1::W) writer structure"]
impl crate::Writable for CONP1 {}
#[doc = "EtherCAT 0 Port 1 Control Register"]
pub mod conp1;
