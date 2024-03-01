#[doc = "Register `MITR` writer"]
pub type W = crate::W<MitrSpec>;
#[doc = "Field `IT` writer - Interrupt Trigger"]
pub type ItW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Interrupt Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn it(&mut self) -> ItW<MitrSpec> {
        ItW::new(self, 0)
    }
}
#[doc = "Module Interrupt Trigger Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mitr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MitrSpec;
impl crate::RegisterSpec for MitrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mitr::W`](W) writer structure"]
impl crate::Writable for MitrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MITR to value 0"]
impl crate::Resettable for MitrSpec {
    const RESET_VALUE: u32 = 0;
}
