#[doc = "Register `DC_PDI_START_EV_TIME` reader"]
pub type R = crate::R<DcPdiStartEvTimeSpec>;
#[doc = "Field `PDI_START_EV_TIME` reader - Register captures local time when at least one SyncManager asserts an PDI buffer start event"]
pub type PdiStartEvTimeR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Register captures local time when at least one SyncManager asserts an PDI buffer start event"]
    #[inline(always)]
    pub fn pdi_start_ev_time(&self) -> PdiStartEvTimeR {
        PdiStartEvTimeR::new(self.bits)
    }
}
#[doc = "PDI Buffer Start Event Time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_pdi_start_ev_time::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcPdiStartEvTimeSpec;
impl crate::RegisterSpec for DcPdiStartEvTimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_pdi_start_ev_time::R`](R) reader structure"]
impl crate::Readable for DcPdiStartEvTimeSpec {}
#[doc = "`reset()` method sets DC_PDI_START_EV_TIME to value 0"]
impl crate::Resettable for DcPdiStartEvTimeSpec {
    const RESET_VALUE: u32 = 0;
}
