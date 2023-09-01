#[doc = "Register `DC_PDI_CNG_EV_TIME` reader"]
pub type R = crate::R<DC_PDI_CNG_EV_TIME_SPEC>;
#[doc = "Field `PDI_CNG_EV_TIME` reader - Register captures local time when at least one SyncManager asserts an PDI buffer change event"]
pub type PDI_CNG_EV_TIME_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Register captures local time when at least one SyncManager asserts an PDI buffer change event"]
    #[inline(always)]
    pub fn pdi_cng_ev_time(&self) -> PDI_CNG_EV_TIME_R {
        PDI_CNG_EV_TIME_R::new(self.bits)
    }
}
#[doc = "PDI Buffer Change Event Time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_pdi_cng_ev_time::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DC_PDI_CNG_EV_TIME_SPEC;
impl crate::RegisterSpec for DC_PDI_CNG_EV_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_pdi_cng_ev_time::R`](R) reader structure"]
impl crate::Readable for DC_PDI_CNG_EV_TIME_SPEC {}
#[doc = "`reset()` method sets DC_PDI_CNG_EV_TIME to value 0"]
impl crate::Resettable for DC_PDI_CNG_EV_TIME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
