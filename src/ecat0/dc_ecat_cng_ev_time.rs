#[doc = "Register `DC_ECAT_CNG_EV_TIME` reader"]
pub type R = crate::R<DcEcatCngEvTimeSpec>;
#[doc = "Field `ECAT_CNG_EV_TIME` reader - Register captures local time of the beginning of the frame which causes at least one SyncManager to assert an ECAT event"]
pub type EcatCngEvTimeR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Register captures local time of the beginning of the frame which causes at least one SyncManager to assert an ECAT event"]
    #[inline(always)]
    pub fn ecat_cng_ev_time(&self) -> EcatCngEvTimeR {
        EcatCngEvTimeR::new(self.bits)
    }
}
#[doc = "EtherCAT Buffer Change Event Time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_ecat_cng_ev_time::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcEcatCngEvTimeSpec;
impl crate::RegisterSpec for DcEcatCngEvTimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_ecat_cng_ev_time::R`](R) reader structure"]
impl crate::Readable for DcEcatCngEvTimeSpec {}
#[doc = "`reset()` method sets DC_ECAT_CNG_EV_TIME to value 0"]
impl crate::Resettable for DcEcatCngEvTimeSpec {
    const RESET_VALUE: u32 = 0;
}
