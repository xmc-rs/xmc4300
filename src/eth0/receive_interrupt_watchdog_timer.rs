#[doc = "Register `RECEIVE_INTERRUPT_WATCHDOG_TIMER` reader"]
pub type R = crate::R<ReceiveInterruptWatchdogTimerSpec>;
#[doc = "Register `RECEIVE_INTERRUPT_WATCHDOG_TIMER` writer"]
pub type W = crate::W<ReceiveInterruptWatchdogTimerSpec>;
#[doc = "Field `RIWT` reader - RI Watchdog Timer Count"]
pub type RiwtR = crate::FieldReader;
#[doc = "Field `RIWT` writer - RI Watchdog Timer Count"]
pub type RiwtW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - RI Watchdog Timer Count"]
    #[inline(always)]
    pub fn riwt(&self) -> RiwtR {
        RiwtR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RI Watchdog Timer Count"]
    #[inline(always)]
    #[must_use]
    pub fn riwt(&mut self) -> RiwtW<ReceiveInterruptWatchdogTimerSpec> {
        RiwtW::new(self, 0)
    }
}
#[doc = "Receive Interrupt Watchdog Timer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`receive_interrupt_watchdog_timer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`receive_interrupt_watchdog_timer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReceiveInterruptWatchdogTimerSpec;
impl crate::RegisterSpec for ReceiveInterruptWatchdogTimerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`receive_interrupt_watchdog_timer::R`](R) reader structure"]
impl crate::Readable for ReceiveInterruptWatchdogTimerSpec {}
#[doc = "`write(|w| ..)` method takes [`receive_interrupt_watchdog_timer::W`](W) writer structure"]
impl crate::Writable for ReceiveInterruptWatchdogTimerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RECEIVE_INTERRUPT_WATCHDOG_TIMER to value 0"]
impl crate::Resettable for ReceiveInterruptWatchdogTimerSpec {
    const RESET_VALUE: u32 = 0;
}
