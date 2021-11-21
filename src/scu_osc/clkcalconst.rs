#[doc = "Register `CLKCALCONST` reader"]
pub struct R(crate::R<CLKCALCONST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKCALCONST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKCALCONST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKCALCONST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CALIBCONST` reader - Clock Calibration Constant Value"]
pub struct CALIBCONST_R(crate::FieldReader<u8, u8>);
impl CALIBCONST_R {
    pub(crate) fn new(bits: u8) -> Self {
        CALIBCONST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALIBCONST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Clock Calibration Constant Value"]
    #[inline(always)]
    pub fn calibconst(&self) -> CALIBCONST_R {
        CALIBCONST_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Clock Calibration Constant Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkcalconst](index.html) module"]
pub struct CLKCALCONST_SPEC;
impl crate::RegisterSpec for CLKCALCONST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkcalconst::R](R) reader structure"]
impl crate::Readable for CLKCALCONST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CLKCALCONST to value 0"]
impl crate::Resettable for CLKCALCONST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
