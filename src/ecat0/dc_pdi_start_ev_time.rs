#[doc = "Reader of register DC_PDI_START_EV_TIME"]
pub type R = crate::R<u32, super::DC_PDI_START_EV_TIME>;
#[doc = "Reader of field `PDI_START_EV_TIME`"]
pub type PDI_START_EV_TIME_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Register captures local time when at least one SyncManager asserts an PDI buffer start event"]
    #[inline(always)]
    pub fn pdi_start_ev_time(&self) -> PDI_START_EV_TIME_R {
        PDI_START_EV_TIME_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
