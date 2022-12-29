#[doc = "Register `WD_COUNT_PDI` reader"]
pub struct R(crate::R<WD_COUNT_PDI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WD_COUNT_PDI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WD_COUNT_PDI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WD_COUNT_PDI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WD_COUNTER_PDI` reader - Watchdog PDI counter"]
pub type WD_COUNTER_PDI_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Watchdog PDI counter"]
    #[inline(always)]
    pub fn wd_counter_pdi(&self) -> WD_COUNTER_PDI_R {
        WD_COUNTER_PDI_R::new(self.bits)
    }
}
#[doc = "Watchdog Counter PDI\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wd_count_pdi](index.html) module"]
pub struct WD_COUNT_PDI_SPEC;
impl crate::RegisterSpec for WD_COUNT_PDI_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [wd_count_pdi::R](R) reader structure"]
impl crate::Readable for WD_COUNT_PDI_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WD_COUNT_PDI to value 0"]
impl crate::Resettable for WD_COUNT_PDI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
