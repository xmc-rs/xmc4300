#[doc = "Register `PROC_ERR_COUNT` reader"]
pub type R = crate::R<ProcErrCountSpec>;
#[doc = "Field `UNIT_ERROR` reader - ECAT Processing Unit error counter"]
pub type UnitErrorR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - ECAT Processing Unit error counter"]
    #[inline(always)]
    pub fn unit_error(&self) -> UnitErrorR {
        UnitErrorR::new(self.bits)
    }
}
#[doc = "ECAT Processing Unit Error Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`proc_err_count::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ProcErrCountSpec;
impl crate::RegisterSpec for ProcErrCountSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`proc_err_count::R`](R) reader structure"]
impl crate::Readable for ProcErrCountSpec {}
#[doc = "`reset()` method sets PROC_ERR_COUNT to value 0"]
impl crate::Resettable for ProcErrCountSpec {
    const RESET_VALUE: u8 = 0;
}
