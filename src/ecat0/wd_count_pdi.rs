#[doc = "Register `WD_COUNT_PDI` reader"]
pub type R = crate::R<WD_COUNT_PDI_SPEC>;
#[doc = "Field `WD_COUNTER_PDI` reader - Watchdog PDI counter"]
pub type WD_COUNTER_PDI_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Watchdog PDI counter"]
    #[inline(always)]
    pub fn wd_counter_pdi(&self) -> WD_COUNTER_PDI_R {
        WD_COUNTER_PDI_R::new(self.bits)
    }
}
#[doc = "Watchdog Counter PDI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wd_count_pdi::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WD_COUNT_PDI_SPEC;
impl crate::RegisterSpec for WD_COUNT_PDI_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wd_count_pdi::R`](R) reader structure"]
impl crate::Readable for WD_COUNT_PDI_SPEC {}
#[doc = "`reset()` method sets WD_COUNT_PDI to value 0"]
impl crate::Resettable for WD_COUNT_PDI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
