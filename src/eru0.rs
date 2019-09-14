#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Event Input Select"]
    pub exisel: EXISEL,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - Event Input Control"]
    pub exicon: [EXICON; 4],
    #[doc = "0x20 - Event Output Trigger Control"]
    pub exocon: [EXOCON; 4],
}
#[doc = "Event Input Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [exisel](exisel) module"]
pub type EXISEL = crate::Reg<u32, _EXISEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXISEL;
#[doc = "`read()` method returns [exisel::R](exisel::R) reader structure"]
impl crate::Readable for EXISEL {}
#[doc = "`write(|w| ..)` method takes [exisel::W](exisel::W) writer structure"]
impl crate::Writable for EXISEL {}
#[doc = "Event Input Select"]
pub mod exisel;
#[doc = "Event Input Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [exicon](exicon) module"]
pub type EXICON = crate::Reg<u32, _EXICON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXICON;
#[doc = "`read()` method returns [exicon::R](exicon::R) reader structure"]
impl crate::Readable for EXICON {}
#[doc = "`write(|w| ..)` method takes [exicon::W](exicon::W) writer structure"]
impl crate::Writable for EXICON {}
#[doc = "Event Input Control"]
pub mod exicon;
#[doc = "Event Output Trigger Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [exocon](exocon) module"]
pub type EXOCON = crate::Reg<u32, _EXOCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXOCON;
#[doc = "`read()` method returns [exocon::R](exocon::R) reader structure"]
impl crate::Readable for EXOCON {}
#[doc = "`write(|w| ..)` method takes [exocon::W](exocon::W) writer structure"]
impl crate::Writable for EXOCON {}
#[doc = "Event Output Trigger Control"]
pub mod exocon;
