#[doc = "Register `RECEIVE_INTERRUPT_WATCHDOG_TIMER` reader"]
pub type R = crate::R<RECEIVE_INTERRUPT_WATCHDOG_TIMER_SPEC>;
#[doc = "Register `RECEIVE_INTERRUPT_WATCHDOG_TIMER` writer"]
pub type W = crate::W<RECEIVE_INTERRUPT_WATCHDOG_TIMER_SPEC>;
#[doc = "Field `RIWT` reader - RI Watchdog Timer Count"]
pub type RIWT_R = crate::FieldReader;
#[doc = "Field `RIWT` writer - RI Watchdog Timer Count"]
pub type RIWT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - RI Watchdog Timer Count"]
    #[inline(always)]
    pub fn riwt(&self) -> RIWT_R {
        RIWT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RI Watchdog Timer Count"]
    #[inline(always)]
    #[must_use]
    pub fn riwt(&mut self) -> RIWT_W<RECEIVE_INTERRUPT_WATCHDOG_TIMER_SPEC> {
        RIWT_W::new(self, 0)
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
#[doc = "Receive Interrupt Watchdog Timer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`receive_interrupt_watchdog_timer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`receive_interrupt_watchdog_timer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RECEIVE_INTERRUPT_WATCHDOG_TIMER_SPEC;
impl crate::RegisterSpec for RECEIVE_INTERRUPT_WATCHDOG_TIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`receive_interrupt_watchdog_timer::R`](R) reader structure"]
impl crate::Readable for RECEIVE_INTERRUPT_WATCHDOG_TIMER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`receive_interrupt_watchdog_timer::W`](W) writer structure"]
impl crate::Writable for RECEIVE_INTERRUPT_WATCHDOG_TIMER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RECEIVE_INTERRUPT_WATCHDOG_TIMER to value 0"]
impl crate::Resettable for RECEIVE_INTERRUPT_WATCHDOG_TIMER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
