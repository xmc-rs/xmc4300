#[doc = "Register `WDTSTS` reader"]
pub struct R(crate::R<WDTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ALMS` reader - Pre-warning Alarm"]
pub type ALMS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Pre-warning Alarm"]
    #[inline(always)]
    pub fn alms(&self) -> ALMS_R {
        ALMS_R::new((self.bits & 1) != 0)
    }
}
#[doc = "WDT Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtsts](index.html) module"]
pub struct WDTSTS_SPEC;
impl crate::RegisterSpec for WDTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdtsts::R](R) reader structure"]
impl crate::Readable for WDTSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WDTSTS to value 0"]
impl crate::Resettable for WDTSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
