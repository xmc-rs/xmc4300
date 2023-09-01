#[doc = "Register `PROC_ERR_COUNT` reader"]
pub type R = crate::R<PROC_ERR_COUNT_SPEC>;
#[doc = "Field `UNIT_ERROR` reader - ECAT Processing Unit error counter"]
pub type UNIT_ERROR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - ECAT Processing Unit error counter"]
    #[inline(always)]
    pub fn unit_error(&self) -> UNIT_ERROR_R {
        UNIT_ERROR_R::new(self.bits)
    }
}
#[doc = "ECAT Processing Unit Error Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`proc_err_count::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PROC_ERR_COUNT_SPEC;
impl crate::RegisterSpec for PROC_ERR_COUNT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`proc_err_count::R`](R) reader structure"]
impl crate::Readable for PROC_ERR_COUNT_SPEC {}
#[doc = "`reset()` method sets PROC_ERR_COUNT to value 0"]
impl crate::Resettable for PROC_ERR_COUNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
