#[doc = "Register `DC_PDI_CNG_EV_TIME` reader"]
pub type R = crate::R<DcPdiCngEvTimeSpec>;
#[doc = "Field `PDI_CNG_EV_TIME` reader - Register captures local time when at least one SyncManager asserts an PDI buffer change event"]
pub type PdiCngEvTimeR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Register captures local time when at least one SyncManager asserts an PDI buffer change event"]
    #[inline(always)]
    pub fn pdi_cng_ev_time(&self) -> PdiCngEvTimeR {
        PdiCngEvTimeR::new(self.bits)
    }
}
#[doc = "PDI Buffer Change Event Time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_pdi_cng_ev_time::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcPdiCngEvTimeSpec;
impl crate::RegisterSpec for DcPdiCngEvTimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_pdi_cng_ev_time::R`](R) reader structure"]
impl crate::Readable for DcPdiCngEvTimeSpec {}
#[doc = "`reset()` method sets DC_PDI_CNG_EV_TIME to value 0"]
impl crate::Resettable for DcPdiCngEvTimeSpec {
    const RESET_VALUE: u32 = 0;
}
