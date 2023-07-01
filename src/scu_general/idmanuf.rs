#[doc = "Register `IDMANUF` reader"]
pub struct R(crate::R<IDMANUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDMANUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDMANUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDMANUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DEPT` reader - Department Identification Number"]
pub type DEPT_R = crate::FieldReader;
#[doc = "Field `MANUF` reader - Manufacturer Identification Number"]
pub type MANUF_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:4 - Department Identification Number"]
    #[inline(always)]
    pub fn dept(&self) -> DEPT_R {
        DEPT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:15 - Manufacturer Identification Number"]
    #[inline(always)]
    pub fn manuf(&self) -> MANUF_R {
        MANUF_R::new(((self.bits >> 5) & 0x07ff) as u16)
    }
}
#[doc = "Manufactory ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idmanuf](index.html) module"]
pub struct IDMANUF_SPEC;
impl crate::RegisterSpec for IDMANUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idmanuf::R](R) reader structure"]
impl crate::Readable for IDMANUF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IDMANUF to value 0x1820"]
impl crate::Resettable for IDMANUF_SPEC {
    const RESET_VALUE: Self::Ux = 0x1820;
}
