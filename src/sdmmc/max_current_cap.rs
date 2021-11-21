#[doc = "Register `MAX_CURRENT_CAP` reader"]
pub struct R(crate::R<MAX_CURRENT_CAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAX_CURRENT_CAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAX_CURRENT_CAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAX_CURRENT_CAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MAX_CURRENT_FOR_3_3V` reader - Maximum Current for 3.3V"]
pub struct MAX_CURRENT_FOR_3_3V_R(crate::FieldReader<u8, u8>);
impl MAX_CURRENT_FOR_3_3V_R {
    pub(crate) fn new(bits: u8) -> Self {
        MAX_CURRENT_FOR_3_3V_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAX_CURRENT_FOR_3_3V_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Maximum Current for 3.3V"]
    #[inline(always)]
    pub fn max_current_for_3_3v(&self) -> MAX_CURRENT_FOR_3_3V_R {
        MAX_CURRENT_FOR_3_3V_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Maximum Current Capabilities Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [max_current_cap](index.html) module"]
pub struct MAX_CURRENT_CAP_SPEC;
impl crate::RegisterSpec for MAX_CURRENT_CAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [max_current_cap::R](R) reader structure"]
impl crate::Readable for MAX_CURRENT_CAP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MAX_CURRENT_CAP to value 0x01"]
impl crate::Resettable for MAX_CURRENT_CAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
