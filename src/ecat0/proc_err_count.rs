#[doc = "Register `PROC_ERR_COUNT` reader"]
pub struct R(crate::R<PROC_ERR_COUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PROC_ERR_COUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PROC_ERR_COUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PROC_ERR_COUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UNIT_ERROR` reader - ECAT Processing Unit error counter"]
pub struct UNIT_ERROR_R(crate::FieldReader<u8, u8>);
impl UNIT_ERROR_R {
    pub(crate) fn new(bits: u8) -> Self {
        UNIT_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UNIT_ERROR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - ECAT Processing Unit error counter"]
    #[inline(always)]
    pub fn unit_error(&self) -> UNIT_ERROR_R {
        UNIT_ERROR_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "ECAT Processing Unit Error Counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [proc_err_count](index.html) module"]
pub struct PROC_ERR_COUNT_SPEC;
impl crate::RegisterSpec for PROC_ERR_COUNT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [proc_err_count::R](R) reader structure"]
impl crate::Readable for PROC_ERR_COUNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PROC_ERR_COUNT to value 0"]
impl crate::Resettable for PROC_ERR_COUNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
