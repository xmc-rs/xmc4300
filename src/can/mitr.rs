#[doc = "Register `MITR` writer"]
pub type W = crate::W<MITR_SPEC>;
#[doc = "Field `IT` writer - Interrupt Trigger"]
pub type IT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Interrupt Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn it(&mut self) -> IT_W<MITR_SPEC> {
        IT_W::new(self, 0)
    }
}
#[doc = "Module Interrupt Trigger Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mitr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MITR_SPEC;
impl crate::RegisterSpec for MITR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mitr::W`](W) writer structure"]
impl crate::Writable for MITR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MITR to value 0"]
impl crate::Resettable for MITR_SPEC {
    const RESET_VALUE: u32 = 0;
}
