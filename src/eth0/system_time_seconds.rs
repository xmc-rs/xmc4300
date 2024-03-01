#[doc = "Register `SYSTEM_TIME_SECONDS` reader"]
pub type R = crate::R<SystemTimeSecondsSpec>;
#[doc = "Field `TSS` reader - Timestamp Second"]
pub type TssR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp Second"]
    #[inline(always)]
    pub fn tss(&self) -> TssR {
        TssR::new(self.bits)
    }
}
#[doc = "System Time - Seconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`system_time_seconds::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SystemTimeSecondsSpec;
impl crate::RegisterSpec for SystemTimeSecondsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`system_time_seconds::R`](R) reader structure"]
impl crate::Readable for SystemTimeSecondsSpec {}
#[doc = "`reset()` method sets SYSTEM_TIME_SECONDS to value 0"]
impl crate::Resettable for SystemTimeSecondsSpec {
    const RESET_VALUE: u32 = 0;
}
