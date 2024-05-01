#[doc = "Register `WD_COUNT_PDI` reader"]
pub type R = crate::R<WdCountPdiSpec>;
#[doc = "Field `WD_COUNTER_PDI` reader - Watchdog PDI counter"]
pub type WdCounterPdiR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Watchdog PDI counter"]
    #[inline(always)]
    pub fn wd_counter_pdi(&self) -> WdCounterPdiR {
        WdCounterPdiR::new(self.bits)
    }
}
#[doc = "Watchdog Counter PDI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wd_count_pdi::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdCountPdiSpec;
impl crate::RegisterSpec for WdCountPdiSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wd_count_pdi::R`](R) reader structure"]
impl crate::Readable for WdCountPdiSpec {}
#[doc = "`reset()` method sets WD_COUNT_PDI to value 0"]
impl crate::Resettable for WdCountPdiSpec {
    const RESET_VALUE: u8 = 0;
}
