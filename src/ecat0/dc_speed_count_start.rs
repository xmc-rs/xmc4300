#[doc = "Reader of register DC_SPEED_COUNT_START"]
pub type R = crate::R<u16, super::DC_SPEED_COUNT_START>;
#[doc = "Writer for register DC_SPEED_COUNT_START"]
pub type W = crate::W<u16, super::DC_SPEED_COUNT_START>;
#[doc = "Register DC_SPEED_COUNT_START `reset()`'s with value 0x1000"]
impl crate::ResetValue for super::DC_SPEED_COUNT_START {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1000
    }
}
#[doc = "Reader of field `COUNT_START`"]
pub type COUNT_START_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COUNT_START`"]
pub struct COUNT_START_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNT_START_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | ((value as u16) & 0x7fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:14 - Bandwidth for adjustment of local copy of System Time"]
    #[inline(always)]
    pub fn count_start(&self) -> COUNT_START_R {
        COUNT_START_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Bandwidth for adjustment of local copy of System Time"]
    #[inline(always)]
    pub fn count_start(&mut self) -> COUNT_START_W {
        COUNT_START_W { w: self }
    }
}
