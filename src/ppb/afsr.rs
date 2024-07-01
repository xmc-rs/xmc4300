#[doc = "Register `AFSR` reader"]
pub type R = crate::R<AFSR_SPEC>;
#[doc = "Register `AFSR` writer"]
pub type W = crate::W<AFSR_SPEC>;
#[doc = "Field `VALUE` reader - Reserved"]
pub type VALUE_R = crate::FieldReader<u32>;
#[doc = "Field `VALUE` writer - Reserved"]
pub type VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Reserved"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<AFSR_SPEC> {
        VALUE_W::new(self, 0)
    }
}
#[doc = "Auxiliary Fault Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`afsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AFSR_SPEC;
impl crate::RegisterSpec for AFSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afsr::R`](R) reader structure"]
impl crate::Readable for AFSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`afsr::W`](W) writer structure"]
impl crate::Writable for AFSR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AFSR to value 0"]
impl crate::Resettable for AFSR_SPEC {
    const RESET_VALUE: u32 = 0;
}
