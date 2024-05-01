#[doc = "Register `SYSTEM_TIME_NANOSECONDS` reader"]
pub type R = crate::R<SystemTimeNanosecondsSpec>;
#[doc = "Field `TSSS` reader - Timestamp Sub Seconds"]
pub type TsssR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:30 - Timestamp Sub Seconds"]
    #[inline(always)]
    pub fn tsss(&self) -> TsssR {
        TsssR::new(self.bits & 0x7fff_ffff)
    }
}
#[doc = "System Time Nanoseconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`system_time_nanoseconds::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SystemTimeNanosecondsSpec;
impl crate::RegisterSpec for SystemTimeNanosecondsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`system_time_nanoseconds::R`](R) reader structure"]
impl crate::Readable for SystemTimeNanosecondsSpec {}
#[doc = "`reset()` method sets SYSTEM_TIME_NANOSECONDS to value 0"]
impl crate::Resettable for SystemTimeNanosecondsSpec {
    const RESET_VALUE: u32 = 0;
}
