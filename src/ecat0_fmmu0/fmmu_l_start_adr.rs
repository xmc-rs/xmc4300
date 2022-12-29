#[doc = "Register `FMMU_L_START_ADR` reader"]
pub struct R(crate::R<FMMU_L_START_ADR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMMU_L_START_ADR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMMU_L_START_ADR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMMU_L_START_ADR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L_START_ADDR` reader - Logical start address within the EtherCAT Address Space"]
pub type L_START_ADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Logical start address within the EtherCAT Address Space"]
    #[inline(always)]
    pub fn l_start_addr(&self) -> L_START_ADDR_R {
        L_START_ADDR_R::new(self.bits)
    }
}
#[doc = "Logical Start address FMMU\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmmu_l_start_adr](index.html) module"]
pub struct FMMU_L_START_ADR_SPEC;
impl crate::RegisterSpec for FMMU_L_START_ADR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmmu_l_start_adr::R](R) reader structure"]
impl crate::Readable for FMMU_L_START_ADR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FMMU_L_START_ADR to value 0"]
impl crate::Resettable for FMMU_L_START_ADR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
