#[doc = "Register `SYSTEM_TIME_NANOSECONDS` reader"]
pub type R = crate::R<SYSTEM_TIME_NANOSECONDS_SPEC>;
#[doc = "Field `TSSS` reader - Timestamp Sub Seconds"]
pub type TSSS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:30 - Timestamp Sub Seconds"]
    #[inline(always)]
    pub fn tsss(&self) -> TSSS_R {
        TSSS_R::new(self.bits & 0x7fff_ffff)
    }
}
#[doc = "System Time Nanoseconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`system_time_nanoseconds::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSTEM_TIME_NANOSECONDS_SPEC;
impl crate::RegisterSpec for SYSTEM_TIME_NANOSECONDS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`system_time_nanoseconds::R`](R) reader structure"]
impl crate::Readable for SYSTEM_TIME_NANOSECONDS_SPEC {}
#[doc = "`reset()` method sets SYSTEM_TIME_NANOSECONDS to value 0"]
impl crate::Resettable for SYSTEM_TIME_NANOSECONDS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
