#[doc = "Register `PDI_ERR_COUNT` reader"]
pub type R = crate::R<PDI_ERR_COUNT_SPEC>;
#[doc = "Field `PDI_ERROR_COUNTER` reader - PDI Error counter"]
pub type PDI_ERROR_COUNTER_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - PDI Error counter"]
    #[inline(always)]
    pub fn pdi_error_counter(&self) -> PDI_ERROR_COUNTER_R {
        PDI_ERROR_COUNTER_R::new(self.bits)
    }
}
#[doc = "PDI Error Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdi_err_count::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDI_ERR_COUNT_SPEC;
impl crate::RegisterSpec for PDI_ERR_COUNT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pdi_err_count::R`](R) reader structure"]
impl crate::Readable for PDI_ERR_COUNT_SPEC {}
#[doc = "`reset()` method sets PDI_ERR_COUNT to value 0"]
impl crate::Resettable for PDI_ERR_COUNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
