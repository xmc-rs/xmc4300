#[doc = "Register `FMMU_P_START_ADR` reader"]
pub struct R(crate::R<FMMU_P_START_ADR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMMU_P_START_ADR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMMU_P_START_ADR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMMU_P_START_ADR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `P_START_ADDR` reader - Physical Start Address"]
pub type P_START_ADDR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Physical Start Address"]
    #[inline(always)]
    pub fn p_start_addr(&self) -> P_START_ADDR_R {
        P_START_ADDR_R::new(self.bits)
    }
}
#[doc = "Ph0sical Start address FMMU y\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmmu_p_start_adr](index.html) module"]
pub struct FMMU_P_START_ADR_SPEC;
impl crate::RegisterSpec for FMMU_P_START_ADR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [fmmu_p_start_adr::R](R) reader structure"]
impl crate::Readable for FMMU_P_START_ADR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FMMU_P_START_ADR to value 0"]
impl crate::Resettable for FMMU_P_START_ADR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
