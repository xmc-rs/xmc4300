#[doc = "Reader of register RECEIVE_INTERRUPT_WATCHDOG_TIMER"]
pub type R = crate::R<u32, super::RECEIVE_INTERRUPT_WATCHDOG_TIMER>;
#[doc = "Writer for register RECEIVE_INTERRUPT_WATCHDOG_TIMER"]
pub type W = crate::W<u32, super::RECEIVE_INTERRUPT_WATCHDOG_TIMER>;
#[doc = "Register RECEIVE_INTERRUPT_WATCHDOG_TIMER `reset()`'s with value 0"]
impl crate::ResetValue for super::RECEIVE_INTERRUPT_WATCHDOG_TIMER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RIWT`"]
pub type RIWT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RIWT`"]
pub struct RIWT_W<'a> {
    w: &'a mut W,
}
impl<'a> RIWT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
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
}
