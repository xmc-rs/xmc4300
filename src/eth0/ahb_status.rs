#[doc = "Register `AHB_STATUS` reader"]
pub struct R(crate::R<AHB_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AHBMS` reader - AHB Master Status"]
pub struct AHBMS_R(crate::FieldReader<bool, bool>);
impl AHBMS_R {
    pub(crate) fn new(bits: bool) -> Self {
        AHBMS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHBMS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - AHB Master Status"]
    #[inline(always)]
    pub fn ahbms(&self) -> AHBMS_R {
        AHBMS_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "AHB Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb_status](index.html) module"]
pub struct AHB_STATUS_SPEC;
impl crate::RegisterSpec for AHB_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb_status::R](R) reader structure"]
impl crate::Readable for AHB_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AHB_STATUS to value 0"]
impl crate::Resettable for AHB_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
