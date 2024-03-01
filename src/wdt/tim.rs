#[doc = "Register `TIM` reader"]
pub type R = crate::R<TimSpec>;
#[doc = "Field `TIM` reader - Timer Value"]
pub type TimR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Timer Value"]
    #[inline(always)]
    pub fn tim(&self) -> TimR {
        TimR::new(self.bits)
    }
}
#[doc = "WDT Timer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimSpec;
impl crate::RegisterSpec for TimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim::R`](R) reader structure"]
impl crate::Readable for TimSpec {}
#[doc = "`reset()` method sets TIM to value 0"]
impl crate::Resettable for TimSpec {
    const RESET_VALUE: u32 = 0;
}
