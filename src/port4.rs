#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port 4 Output Register"]
    pub out: OUT,
    #[doc = "0x04 - Port 4 Output Modification Register"]
    pub omr: OMR,
    _reserved2: [u8; 8usize],
    #[doc = "0x10 - Port 4 Input/Output Control Register 0"]
    pub iocr0: IOCR0,
    _reserved3: [u8; 16usize],
    #[doc = "0x24 - Port 4 Input Register"]
    pub in_: IN,
    _reserved4: [u8; 24usize],
    #[doc = "0x40 - Port 4 Pad Driver Mode 0 Register"]
    pub pdr0: PDR0,
    _reserved5: [u8; 28usize],
    #[doc = "0x60 - Port 4 Pin Function Decision Control Register"]
    pub pdisc: PDISC,
    _reserved6: [u8; 12usize],
    #[doc = "0x70 - Port 4 Pin Power Save Register"]
    pub pps: PPS,
    #[doc = "0x74 - Port 4 Pin Hardware Select Register"]
    pub hwsel: HWSEL,
}
#[doc = "Port 4 Output Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [out](out) module"]
pub type OUT = crate::Reg<u32, _OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT;
#[doc = "`read()` method returns [out::R](out::R) reader structure"]
impl crate::Readable for OUT {}
#[doc = "`write(|w| ..)` method takes [out::W](out::W) writer structure"]
impl crate::Writable for OUT {}
#[doc = "Port 4 Output Register"]
pub mod out;
#[doc = "Port 4 Output Modification Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [omr](omr) module"]
pub type OMR = crate::Reg<u32, _OMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OMR;
#[doc = "`write(|w| ..)` method takes [omr::W](omr::W) writer structure"]
impl crate::Writable for OMR {}
#[doc = "Port 4 Output Modification Register"]
pub mod omr;
#[doc = "Port 4 Input/Output Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocr0](iocr0) module"]
pub type IOCR0 = crate::Reg<u32, _IOCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCR0;
#[doc = "`read()` method returns [iocr0::R](iocr0::R) reader structure"]
impl crate::Readable for IOCR0 {}
#[doc = "`write(|w| ..)` method takes [iocr0::W](iocr0::W) writer structure"]
impl crate::Writable for IOCR0 {}
#[doc = "Port 4 Input/Output Control Register 0"]
pub mod iocr0;
#[doc = "Port 4 Input Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [in_](in_) module"]
pub type IN = crate::Reg<u32, _IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IN;
#[doc = "`read()` method returns [in_::R](in_::R) reader structure"]
impl crate::Readable for IN {}
#[doc = "Port 4 Input Register"]
pub mod in_;
#[doc = "Port 4 Pad Driver Mode 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdr0](pdr0) module"]
pub type PDR0 = crate::Reg<u32, _PDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDR0;
#[doc = "`read()` method returns [pdr0::R](pdr0::R) reader structure"]
impl crate::Readable for PDR0 {}
#[doc = "`write(|w| ..)` method takes [pdr0::W](pdr0::W) writer structure"]
impl crate::Writable for PDR0 {}
#[doc = "Port 4 Pad Driver Mode 0 Register"]
pub mod pdr0;
#[doc = "Port 4 Pin Function Decision Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdisc](pdisc) module"]
pub type PDISC = crate::Reg<u32, _PDISC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDISC;
#[doc = "`read()` method returns [pdisc::R](pdisc::R) reader structure"]
impl crate::Readable for PDISC {}
#[doc = "Port 4 Pin Function Decision Control Register"]
pub mod pdisc;
#[doc = "Port 4 Pin Power Save Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pps](pps) module"]
pub type PPS = crate::Reg<u32, _PPS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPS;
#[doc = "`read()` method returns [pps::R](pps::R) reader structure"]
impl crate::Readable for PPS {}
#[doc = "`write(|w| ..)` method takes [pps::W](pps::W) writer structure"]
impl crate::Writable for PPS {}
#[doc = "Port 4 Pin Power Save Register"]
pub mod pps;
#[doc = "Port 4 Pin Hardware Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hwsel](hwsel) module"]
pub type HWSEL = crate::Reg<u32, _HWSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWSEL;
#[doc = "`read()` method returns [hwsel::R](hwsel::R) reader structure"]
impl crate::Readable for HWSEL {}
#[doc = "`write(|w| ..)` method takes [hwsel::W](hwsel::W) writer structure"]
impl crate::Writable for HWSEL {}
#[doc = "Port 4 Pin Hardware Select Register"]
pub mod hwsel;
