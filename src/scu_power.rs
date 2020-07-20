#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PCU Status Register"]
    pub pwrstat: PWRSTAT,
    #[doc = "0x04 - PCU Set Control Register"]
    pub pwrset: PWRSET,
    #[doc = "0x08 - PCU Clear Control Register"]
    pub pwrclr: PWRCLR,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - EVR Status Register"]
    pub evrstat: EVRSTAT,
    #[doc = "0x14 - EVR VADC Status Register"]
    pub evrvadcstat: EVRVADCSTAT,
    _reserved5: [u8; 20usize],
    #[doc = "0x2c - Power Monitor Control"]
    pub pwrmon: PWRMON,
}
#[doc = "PCU Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrstat](pwrstat) module"]
pub type PWRSTAT = crate::Reg<u32, _PWRSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRSTAT;
#[doc = "`read()` method returns [pwrstat::R](pwrstat::R) reader structure"]
impl crate::Readable for PWRSTAT {}
#[doc = "PCU Status Register"]
pub mod pwrstat;
#[doc = "PCU Set Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrset](pwrset) module"]
pub type PWRSET = crate::Reg<u32, _PWRSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRSET;
#[doc = "`write(|w| ..)` method takes [pwrset::W](pwrset::W) writer structure"]
impl crate::Writable for PWRSET {}
#[doc = "PCU Set Control Register"]
pub mod pwrset;
#[doc = "PCU Clear Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrclr](pwrclr) module"]
pub type PWRCLR = crate::Reg<u32, _PWRCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRCLR;
#[doc = "`write(|w| ..)` method takes [pwrclr::W](pwrclr::W) writer structure"]
impl crate::Writable for PWRCLR {}
#[doc = "PCU Clear Control Register"]
pub mod pwrclr;
#[doc = "EVR Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evrstat](evrstat) module"]
pub type EVRSTAT = crate::Reg<u32, _EVRSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVRSTAT;
#[doc = "`read()` method returns [evrstat::R](evrstat::R) reader structure"]
impl crate::Readable for EVRSTAT {}
#[doc = "EVR Status Register"]
pub mod evrstat;
#[doc = "EVR VADC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evrvadcstat](evrvadcstat) module"]
pub type EVRVADCSTAT = crate::Reg<u32, _EVRVADCSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVRVADCSTAT;
#[doc = "`read()` method returns [evrvadcstat::R](evrvadcstat::R) reader structure"]
impl crate::Readable for EVRVADCSTAT {}
#[doc = "EVR VADC Status Register"]
pub mod evrvadcstat;
#[doc = "Power Monitor Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrmon](pwrmon) module"]
pub type PWRMON = crate::Reg<u32, _PWRMON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRMON;
#[doc = "`read()` method returns [pwrmon::R](pwrmon::R) reader structure"]
impl crate::Readable for PWRMON {}
#[doc = "`write(|w| ..)` method takes [pwrmon::W](pwrmon::W) writer structure"]
impl crate::Writable for PWRMON {}
#[doc = "Power Monitor Control"]
pub mod pwrmon;
