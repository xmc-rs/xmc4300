#[doc = "Register `TIM` reader"]
pub type R = crate::R<TIM_SPEC>;
#[doc = "Field `TIM` reader - Timer Value"]
pub type TIM_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Timer Value"]
    #[inline(always)]
    pub fn tim(&self) -> TIM_R {
        TIM_R::new(self.bits)
    }
}
#[doc = "WDT Timer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM_SPEC;
impl crate::RegisterSpec for TIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim::R`](R) reader structure"]
impl crate::Readable for TIM_SPEC {}
#[doc = "`reset()` method sets TIM to value 0"]
impl crate::Resettable for TIM_SPEC {
    const RESET_VALUE: u32 = 0;
}
