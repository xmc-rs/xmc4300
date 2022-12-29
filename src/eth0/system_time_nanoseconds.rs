#[doc = "Register `SYSTEM_TIME_NANOSECONDS` reader"]
pub struct R(crate::R<SYSTEM_TIME_NANOSECONDS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSTEM_TIME_NANOSECONDS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSTEM_TIME_NANOSECONDS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSTEM_TIME_NANOSECONDS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TSSS` reader - Timestamp Sub Seconds"]
pub type TSSS_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:30 - Timestamp Sub Seconds"]
    #[inline(always)]
    pub fn tsss(&self) -> TSSS_R {
        TSSS_R::new(self.bits & 0x7fff_ffff)
    }
}
#[doc = "System Time Nanoseconds Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_time_nanoseconds](index.html) module"]
pub struct SYSTEM_TIME_NANOSECONDS_SPEC;
impl crate::RegisterSpec for SYSTEM_TIME_NANOSECONDS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [system_time_nanoseconds::R](R) reader structure"]
impl crate::Readable for SYSTEM_TIME_NANOSECONDS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYSTEM_TIME_NANOSECONDS to value 0"]
impl crate::Resettable for SYSTEM_TIME_NANOSECONDS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
