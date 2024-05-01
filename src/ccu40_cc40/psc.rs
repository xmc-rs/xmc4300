#[doc = "Register `PSC` reader"]
pub type R = crate::R<PSC_SPEC>;
#[doc = "Register `PSC` writer"]
pub type W = crate::W<PSC_SPEC>;
#[doc = "Field `PSIV` reader - Prescaler Initial Value"]
pub type PSIV_R = crate::FieldReader;
#[doc = "Field `PSIV` writer - Prescaler Initial Value"]
pub type PSIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Prescaler Initial Value"]
    #[inline(always)]
    pub fn psiv(&self) -> PSIV_R {
        PSIV_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Prescaler Initial Value"]
    #[inline(always)]
    #[must_use]
    pub fn psiv(&mut self) -> PSIV_W<PSC_SPEC> {
        PSIV_W::new(self, 0)
    }
}
#[doc = "Prescaler Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSC_SPEC;
impl crate::RegisterSpec for PSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psc::R`](R) reader structure"]
impl crate::Readable for PSC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`psc::W`](W) writer structure"]
impl crate::Writable for PSC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSC to value 0"]
impl crate::Resettable for PSC_SPEC {
    const RESET_VALUE: u32 = 0;
}
