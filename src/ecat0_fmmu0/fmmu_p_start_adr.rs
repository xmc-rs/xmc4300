#[doc = "Register `FMMU_P_START_ADR` reader"]
pub type R = crate::R<FMMU_P_START_ADR_SPEC>;
#[doc = "Field `P_START_ADDR` reader - Physical Start Address"]
pub type P_START_ADDR_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Physical Start Address"]
    #[inline(always)]
    pub fn p_start_addr(&self) -> P_START_ADDR_R {
        P_START_ADDR_R::new(self.bits)
    }
}
#[doc = "Ph0sical Start address FMMU y\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmmu_p_start_adr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMMU_P_START_ADR_SPEC;
impl crate::RegisterSpec for FMMU_P_START_ADR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fmmu_p_start_adr::R`](R) reader structure"]
impl crate::Readable for FMMU_P_START_ADR_SPEC {}
#[doc = "`reset()` method sets FMMU_P_START_ADR to value 0"]
impl crate::Resettable for FMMU_P_START_ADR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
