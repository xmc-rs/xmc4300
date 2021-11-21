#[doc = "Register `PDI_ERR_COUNT` reader"]
pub struct R(crate::R<PDI_ERR_COUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDI_ERR_COUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDI_ERR_COUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDI_ERR_COUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PDI_ERROR_COUNTER` reader - PDI Error counter"]
pub struct PDI_ERROR_COUNTER_R(crate::FieldReader<u8, u8>);
impl PDI_ERROR_COUNTER_R {
    pub(crate) fn new(bits: u8) -> Self {
        PDI_ERROR_COUNTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDI_ERROR_COUNTER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - PDI Error counter"]
    #[inline(always)]
    pub fn pdi_error_counter(&self) -> PDI_ERROR_COUNTER_R {
        PDI_ERROR_COUNTER_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "PDI Error Counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdi_err_count](index.html) module"]
pub struct PDI_ERR_COUNT_SPEC;
impl crate::RegisterSpec for PDI_ERR_COUNT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pdi_err_count::R](R) reader structure"]
impl crate::Readable for PDI_ERR_COUNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PDI_ERR_COUNT to value 0"]
impl crate::Resettable for PDI_ERR_COUNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
