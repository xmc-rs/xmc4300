#[doc = "Register `TIMER` reader"]
pub type R = crate::R<TimerSpec>;
#[doc = "Register `TIMER` writer"]
pub type W = crate::W<TimerSpec>;
#[doc = "Field `TVAL` reader - Timer Value"]
pub type TvalR = crate::FieldReader<u16>;
#[doc = "Field `TVAL` writer - Timer Value"]
pub type TvalW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timer Value"]
    #[inline(always)]
    pub fn tval(&self) -> TvalR {
        TvalR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer Value"]
    #[inline(always)]
    #[must_use]
    pub fn tval(&mut self) -> TvalW<TimerSpec> {
        TvalW::new(self, 0)
    }
}
#[doc = "Timer Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimerSpec;
impl crate::RegisterSpec for TimerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer::R`](R) reader structure"]
impl crate::Readable for TimerSpec {}
#[doc = "`write(|w| ..)` method takes [`timer::W`](W) writer structure"]
impl crate::Writable for TimerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER to value 0"]
impl crate::Resettable for TimerSpec {
    const RESET_VALUE: u32 = 0;
}
