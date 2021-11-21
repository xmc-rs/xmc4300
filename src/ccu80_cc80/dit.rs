#[doc = "Register `DIT` reader"]
pub struct R(crate::R<DIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DCV` reader - Dither compare Value"]
pub struct DCV_R(crate::FieldReader<u8, u8>);
impl DCV_R {
    pub(crate) fn new(bits: u8) -> Self {
        DCV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCNT` reader - Dither counter actual value"]
pub struct DCNT_R(crate::FieldReader<u8, u8>);
impl DCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        DCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Dither compare Value"]
    #[inline(always)]
    pub fn dcv(&self) -> DCV_R {
        DCV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Dither counter actual value"]
    #[inline(always)]
    pub fn dcnt(&self) -> DCNT_R {
        DCNT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[doc = "Dither Config\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dit](index.html) module"]
pub struct DIT_SPEC;
impl crate::RegisterSpec for DIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dit::R](R) reader structure"]
impl crate::Readable for DIT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DIT to value 0"]
impl crate::Resettable for DIT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
