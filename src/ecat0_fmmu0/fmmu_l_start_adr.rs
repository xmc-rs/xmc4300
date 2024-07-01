#[doc = "Register `FMMU_L_START_ADR` reader"]
pub type R = crate::R<FMMU_L_START_ADR_SPEC>;
#[doc = "Field `L_START_ADDR` reader - Logical start address within the EtherCAT Address Space"]
pub type L_START_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Logical start address within the EtherCAT Address Space"]
    #[inline(always)]
    pub fn l_start_addr(&self) -> L_START_ADDR_R {
        L_START_ADDR_R::new(self.bits)
    }
}
#[doc = "Logical Start address FMMU\n\nYou can [`read`](crate::Reg::read) this register and get [`fmmu_l_start_adr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMMU_L_START_ADR_SPEC;
impl crate::RegisterSpec for FMMU_L_START_ADR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmmu_l_start_adr::R`](R) reader structure"]
impl crate::Readable for FMMU_L_START_ADR_SPEC {}
#[doc = "`reset()` method sets FMMU_L_START_ADR to value 0"]
impl crate::Resettable for FMMU_L_START_ADR_SPEC {
    const RESET_VALUE: u32 = 0;
}
