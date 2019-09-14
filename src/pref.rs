#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Prefetch Configuration Register"]
    pub pcon: PCON,
}
#[doc = "Prefetch Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcon](pcon) module"]
pub type PCON = crate::Reg<u32, _PCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCON;
#[doc = "`read()` method returns [pcon::R](pcon::R) reader structure"]
impl crate::Readable for PCON {}
#[doc = "`write(|w| ..)` method takes [pcon::W](pcon::W) writer structure"]
impl crate::Writable for PCON {}
#[doc = "Prefetch Configuration Register"]
pub mod pcon;
