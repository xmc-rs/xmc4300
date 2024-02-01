#[doc = "Register `FMMU_L_START_BIT` reader"]
pub type R = crate::R<FMMU_L_START_BIT_SPEC>;
#[doc = "Field `L_START_BIT` reader - Logical starting bit that shall be mapped"]
pub type L_START_BIT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Logical starting bit that shall be mapped"]
    #[inline(always)]
    pub fn l_start_bit(&self) -> L_START_BIT_R {
        L_START_BIT_R::new(self.bits & 7)
    }
}
#[doc = "Start bit FMMU 0 in logical address space\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmmu_l_start_bit::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMMU_L_START_BIT_SPEC;
impl crate::RegisterSpec for FMMU_L_START_BIT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fmmu_l_start_bit::R`](R) reader structure"]
impl crate::Readable for FMMU_L_START_BIT_SPEC {}
#[doc = "`reset()` method sets FMMU_L_START_BIT to value 0"]
impl crate::Resettable for FMMU_L_START_BIT_SPEC {
    const RESET_VALUE: u8 = 0;
}
