#[doc = "Register `DC_ECAT_CNG_EV_TIME` reader"]
pub type R = crate::R<DC_ECAT_CNG_EV_TIME_SPEC>;
#[doc = "Field `ECAT_CNG_EV_TIME` reader - Register captures local time of the beginning of the frame which causes at least one SyncManager to assert an ECAT event"]
pub type ECAT_CNG_EV_TIME_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Register captures local time of the beginning of the frame which causes at least one SyncManager to assert an ECAT event"]
    #[inline(always)]
    pub fn ecat_cng_ev_time(&self) -> ECAT_CNG_EV_TIME_R {
        ECAT_CNG_EV_TIME_R::new(self.bits)
    }
}
#[doc = "EtherCAT Buffer Change Event Time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_ecat_cng_ev_time::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DC_ECAT_CNG_EV_TIME_SPEC;
impl crate::RegisterSpec for DC_ECAT_CNG_EV_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_ecat_cng_ev_time::R`](R) reader structure"]
impl crate::Readable for DC_ECAT_CNG_EV_TIME_SPEC {}
#[doc = "`reset()` method sets DC_ECAT_CNG_EV_TIME to value 0"]
impl crate::Resettable for DC_ECAT_CNG_EV_TIME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
