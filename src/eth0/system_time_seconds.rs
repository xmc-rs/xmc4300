#[doc = "Register `SYSTEM_TIME_SECONDS` reader"]
pub type R = crate::R<SYSTEM_TIME_SECONDS_SPEC>;
#[doc = "Field `TSS` reader - Timestamp Second"]
pub type TSS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp Second"]
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new(self.bits)
    }
}
#[doc = "System Time - Seconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`system_time_seconds::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSTEM_TIME_SECONDS_SPEC;
impl crate::RegisterSpec for SYSTEM_TIME_SECONDS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`system_time_seconds::R`](R) reader structure"]
impl crate::Readable for SYSTEM_TIME_SECONDS_SPEC {}
#[doc = "`reset()` method sets SYSTEM_TIME_SECONDS to value 0"]
impl crate::Resettable for SYSTEM_TIME_SECONDS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
