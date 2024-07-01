#[doc = "Register `CR1S` reader"]
pub type R = crate::R<CR1S_SPEC>;
#[doc = "Register `CR1S` writer"]
pub type W = crate::W<CR1S_SPEC>;
#[doc = "Field `CR1S` reader - Shadow Compare Register for Channel 1"]
pub type CR1S_R = crate::FieldReader<u16>;
#[doc = "Field `CR1S` writer - Shadow Compare Register for Channel 1"]
pub type CR1S_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Shadow Compare Register for Channel 1"]
    #[inline(always)]
    pub fn cr1s(&self) -> CR1S_R {
        CR1S_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Shadow Compare Register for Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn cr1s(&mut self) -> CR1S_W<CR1S_SPEC> {
        CR1S_W::new(self, 0)
    }
}
#[doc = "Channel 1 Compare Shadow Value\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1s::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1s::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1S_SPEC;
impl crate::RegisterSpec for CR1S_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1s::R`](R) reader structure"]
impl crate::Readable for CR1S_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr1s::W`](W) writer structure"]
impl crate::Writable for CR1S_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR1S to value 0"]
impl crate::Resettable for CR1S_SPEC {
    const RESET_VALUE: u32 = 0;
}
