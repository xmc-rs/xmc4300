#[doc = "Reader of register WD_TIME_PDI"]
pub type R = crate::R<u16, super::WD_TIME_PDI>;
#[doc = "Writer for register WD_TIME_PDI"]
pub type W = crate::W<u16, super::WD_TIME_PDI>;
#[doc = "Register WD_TIME_PDI `reset()`'s with value 0x03e8"]
impl crate::ResetValue for super::WD_TIME_PDI {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03e8
    }
}
#[doc = "Reader of field `WD_TIME_PDI`"]
pub type WD_TIME_PDI_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WD_TIME_PDI`"]
pub struct WD_TIME_PDI_W<'a> {
    w: &'a mut W,
}
impl<'a> WD_TIME_PDI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Watchdog Time PDI"]
    #[inline(always)]
    pub fn wd_time_pdi(&self) -> WD_TIME_PDI_R {
        WD_TIME_PDI_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Watchdog Time PDI"]
    #[inline(always)]
    pub fn wd_time_pdi(&mut self) -> WD_TIME_PDI_W {
        WD_TIME_PDI_W { w: self }
    }
}
