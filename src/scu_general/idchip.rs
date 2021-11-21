#[doc = "Register `IDCHIP` reader"]
pub struct R(crate::R<IDCHIP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDCHIP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDCHIP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDCHIP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IDCHIP` reader - Chip ID"]
pub struct IDCHIP_R(crate::FieldReader<u32, u32>);
impl IDCHIP_R {
    pub(crate) fn new(bits: u32) -> Self {
        IDCHIP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDCHIP_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Chip ID"]
    #[inline(always)]
    pub fn idchip(&self) -> IDCHIP_R {
        IDCHIP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Chip ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idchip](index.html) module"]
pub struct IDCHIP_SPEC;
impl crate::RegisterSpec for IDCHIP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idchip::R](R) reader structure"]
impl crate::Readable for IDCHIP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IDCHIP to value 0"]
impl crate::Resettable for IDCHIP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
