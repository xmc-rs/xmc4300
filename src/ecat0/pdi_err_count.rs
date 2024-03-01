#[doc = "Register `PDI_ERR_COUNT` reader"]
pub type R = crate::R<PdiErrCountSpec>;
#[doc = "Field `PDI_ERROR_COUNTER` reader - PDI Error counter"]
pub type PdiErrorCounterR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - PDI Error counter"]
    #[inline(always)]
    pub fn pdi_error_counter(&self) -> PdiErrorCounterR {
        PdiErrorCounterR::new(self.bits)
    }
}
#[doc = "PDI Error Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdi_err_count::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdiErrCountSpec;
impl crate::RegisterSpec for PdiErrCountSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pdi_err_count::R`](R) reader structure"]
impl crate::Readable for PdiErrCountSpec {}
#[doc = "`reset()` method sets PDI_ERR_COUNT to value 0"]
impl crate::Resettable for PdiErrCountSpec {
    const RESET_VALUE: u8 = 0;
}
