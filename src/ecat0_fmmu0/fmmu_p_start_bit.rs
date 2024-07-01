#[doc = "Register `FMMU_P_START_BIT` reader"]
pub type R = crate::R<FMMU_P_START_BIT_SPEC>;
#[doc = "Field `P_START_BIT` reader - Physical starting bit as target of logical start bit mapping"]
pub type P_START_BIT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Physical starting bit as target of logical start bit mapping"]
    #[inline(always)]
    pub fn p_start_bit(&self) -> P_START_BIT_R {
        P_START_BIT_R::new(self.bits & 7)
    }
}
#[doc = "Ph0sical Start bit FMMU y\n\nYou can [`read`](crate::Reg::read) this register and get [`fmmu_p_start_bit::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMMU_P_START_BIT_SPEC;
impl crate::RegisterSpec for FMMU_P_START_BIT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fmmu_p_start_bit::R`](R) reader structure"]
impl crate::Readable for FMMU_P_START_BIT_SPEC {}
#[doc = "`reset()` method sets FMMU_P_START_BIT to value 0"]
impl crate::Resettable for FMMU_P_START_BIT_SPEC {
    const RESET_VALUE: u8 = 0;
}
