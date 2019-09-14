#[doc = "Reader of register WD_COUNT_PDI"]
pub type R = crate::R<u8, super::WD_COUNT_PDI>;
#[doc = "Reader of field `WD_COUNTER_PDI`"]
pub type WD_COUNTER_PDI_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Watchdog PDI counter"]
    #[inline(always)]
    pub fn wd_counter_pdi(&self) -> WD_COUNTER_PDI_R {
        WD_COUNTER_PDI_R::new((self.bits & 0xff) as u8)
    }
}
