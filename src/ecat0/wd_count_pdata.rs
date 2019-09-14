#[doc = "Reader of register WD_COUNT_PDATA"]
pub type R = crate::R<u8, super::WD_COUNT_PDATA>;
#[doc = "Reader of field `WD_COUNTER_PD`"]
pub type WD_COUNTER_PD_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Watchdog Counter Process Data"]
    #[inline(always)]
    pub fn wd_counter_pd(&self) -> WD_COUNTER_PD_R {
        WD_COUNTER_PD_R::new((self.bits & 0xff) as u8)
    }
}
