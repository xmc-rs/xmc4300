#[doc = "Register `WD_COUNT_PDATA` reader"]
pub struct R(crate::R<WD_COUNT_PDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WD_COUNT_PDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WD_COUNT_PDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WD_COUNT_PDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WD_COUNTER_PD` reader - Watchdog Counter Process Data"]
pub type WD_COUNTER_PD_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Watchdog Counter Process Data"]
    #[inline(always)]
    pub fn wd_counter_pd(&self) -> WD_COUNTER_PD_R {
        WD_COUNTER_PD_R::new(self.bits)
    }
}
#[doc = "Watchdog Counter Process Data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wd_count_pdata](index.html) module"]
pub struct WD_COUNT_PDATA_SPEC;
impl crate::RegisterSpec for WD_COUNT_PDATA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [wd_count_pdata::R](R) reader structure"]
impl crate::Readable for WD_COUNT_PDATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WD_COUNT_PDATA to value 0"]
impl crate::Resettable for WD_COUNT_PDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
