#[doc = "Reader of register WD_TIME_PDATA"]
pub type R = crate::R<u16, super::WD_TIME_PDATA>;
#[doc = "Writer for register WD_TIME_PDATA"]
pub type W = crate::W<u16, super::WD_TIME_PDATA>;
#[doc = "Register WD_TIME_PDATA `reset()`'s with value 0x03e8"]
impl crate::ResetValue for super::WD_TIME_PDATA {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03e8
    }
}
#[doc = "Reader of field `WD_TIME_PD`"]
pub type WD_TIME_PD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WD_TIME_PD`"]
pub struct WD_TIME_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> WD_TIME_PD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Watchdog Time Process Data"]
    #[inline(always)]
    pub fn wd_time_pd(&self) -> WD_TIME_PD_R {
        WD_TIME_PD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Watchdog Time Process Data"]
    #[inline(always)]
    pub fn wd_time_pd(&mut self) -> WD_TIME_PD_W {
        WD_TIME_PD_W { w: self }
    }
}
