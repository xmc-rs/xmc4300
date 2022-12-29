#[doc = "Register `SYSTEM_TIME_SECONDS` reader"]
pub struct R(crate::R<SYSTEM_TIME_SECONDS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSTEM_TIME_SECONDS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSTEM_TIME_SECONDS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSTEM_TIME_SECONDS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TSS` reader - Timestamp Second"]
pub type TSS_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp Second"]
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new(self.bits)
    }
}
#[doc = "System Time - Seconds Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_time_seconds](index.html) module"]
pub struct SYSTEM_TIME_SECONDS_SPEC;
impl crate::RegisterSpec for SYSTEM_TIME_SECONDS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [system_time_seconds::R](R) reader structure"]
impl crate::Readable for SYSTEM_TIME_SECONDS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYSTEM_TIME_SECONDS to value 0"]
impl crate::Resettable for SYSTEM_TIME_SECONDS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
