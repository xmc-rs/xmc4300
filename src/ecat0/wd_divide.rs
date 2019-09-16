#[doc = "Reader of register WD_DIVIDE"]
pub type R = crate::R<u16, super::WD_DIVIDE>;
#[doc = "Writer for register WD_DIVIDE"]
pub type W = crate::W<u16, super::WD_DIVIDE>;
#[doc = "Register WD_DIVIDE `reset()`'s with value 0x09c2"]
impl crate::ResetValue for super::WD_DIVIDE {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x09c2
    }
}
#[doc = "Reader of field `WD_DIV`"]
pub type WD_DIV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WD_DIV`"]
pub struct WD_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> WD_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Watchdog divider"]
    #[inline(always)]
    pub fn wd_div(&self) -> WD_DIV_R {
        WD_DIV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Watchdog divider"]
    #[inline(always)]
    pub fn wd_div(&mut self) -> WD_DIV_W {
        WD_DIV_W { w: self }
    }
}
