#[doc = "Register `WD_COUNT_PDATA` reader"]
pub type R = crate::R<WdCountPdataSpec>;
#[doc = "Field `WD_COUNTER_PD` reader - Watchdog Counter Process Data"]
pub type WdCounterPdR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Watchdog Counter Process Data"]
    #[inline(always)]
    pub fn wd_counter_pd(&self) -> WdCounterPdR {
        WdCounterPdR::new(self.bits)
    }
}
#[doc = "Watchdog Counter Process Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wd_count_pdata::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdCountPdataSpec;
impl crate::RegisterSpec for WdCountPdataSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wd_count_pdata::R`](R) reader structure"]
impl crate::Readable for WdCountPdataSpec {}
#[doc = "`reset()` method sets WD_COUNT_PDATA to value 0"]
impl crate::Resettable for WdCountPdataSpec {
    const RESET_VALUE: u8 = 0;
}
