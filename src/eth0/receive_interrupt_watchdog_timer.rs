#[doc = "Register `RECEIVE_INTERRUPT_WATCHDOG_TIMER` reader"]
pub struct R(crate::R<RECEIVE_INTERRUPT_WATCHDOG_TIMER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RECEIVE_INTERRUPT_WATCHDOG_TIMER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RECEIVE_INTERRUPT_WATCHDOG_TIMER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RECEIVE_INTERRUPT_WATCHDOG_TIMER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RECEIVE_INTERRUPT_WATCHDOG_TIMER` writer"]
pub struct W(crate::W<RECEIVE_INTERRUPT_WATCHDOG_TIMER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RECEIVE_INTERRUPT_WATCHDOG_TIMER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<RECEIVE_INTERRUPT_WATCHDOG_TIMER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RECEIVE_INTERRUPT_WATCHDOG_TIMER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RIWT` reader - RI Watchdog Timer Count"]
pub struct RIWT_R(crate::FieldReader<u8, u8>);
impl RIWT_R {
    pub(crate) fn new(bits: u8) -> Self {
        RIWT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RIWT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RIWT` writer - RI Watchdog Timer Count"]
pub struct RIWT_W<'a> {
    w: &'a mut W,
}
impl<'a> RIWT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
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
    pub fn riwt(&mut self) -> RIWT_W {
        RIWT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Interrupt Watchdog Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [receive_interrupt_watchdog_timer](index.html) module"]
pub struct RECEIVE_INTERRUPT_WATCHDOG_TIMER_SPEC;
impl crate::RegisterSpec for RECEIVE_INTERRUPT_WATCHDOG_TIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [receive_interrupt_watchdog_timer::R](R) reader structure"]
impl crate::Readable for RECEIVE_INTERRUPT_WATCHDOG_TIMER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [receive_interrupt_watchdog_timer::W](W) writer structure"]
impl crate::Writable for RECEIVE_INTERRUPT_WATCHDOG_TIMER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RECEIVE_INTERRUPT_WATCHDOG_TIMER to value 0"]
impl crate::Resettable for RECEIVE_INTERRUPT_WATCHDOG_TIMER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
