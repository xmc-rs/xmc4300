#[doc = "Register `TIMER` reader"]
pub type R = crate::R<TIMER_SPEC>;
#[doc = "Register `TIMER` writer"]
pub type W = crate::W<TIMER_SPEC>;
#[doc = "Field `TVAL` reader - Timer Value"]
pub type TVAL_R = crate::FieldReader<u16>;
#[doc = "Field `TVAL` writer - Timer Value"]
pub type TVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timer Value"]
    #[inline(always)]
    pub fn tval(&self) -> TVAL_R {
        TVAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer Value"]
    #[inline(always)]
    #[must_use]
    pub fn tval(&mut self) -> TVAL_W<TIMER_SPEC> {
        TVAL_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timer Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER_SPEC;
impl crate::RegisterSpec for TIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer::R`](R) reader structure"]
impl crate::Readable for TIMER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer::W`](W) writer structure"]
impl crate::Writable for TIMER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER to value 0"]
impl crate::Resettable for TIMER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
