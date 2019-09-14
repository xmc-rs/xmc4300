#[doc = "Reader of register TARGET_TIME_NANOSECONDS"]
pub type R = crate::R<u32, super::TARGET_TIME_NANOSECONDS>;
#[doc = "Writer for register TARGET_TIME_NANOSECONDS"]
pub type W = crate::W<u32, super::TARGET_TIME_NANOSECONDS>;
#[doc = "Register TARGET_TIME_NANOSECONDS `reset()`'s with value 0"]
impl crate::ResetValue for super::TARGET_TIME_NANOSECONDS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TTSLO`"]
pub type TTSLO_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TTSLO`"]
pub struct TTSLO_W<'a> {
    w: &'a mut W,
}
impl<'a> TTSLO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff_ffff) | ((value as u32) & 0x7fff_ffff);
        self.w
    }
}
#[doc = "Reader of field `TRGTBUSY`"]
pub type TRGTBUSY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:30 - Target Timestamp Low Register"]
    #[inline(always)]
    pub fn ttslo(&self) -> TTSLO_R {
        TTSLO_R::new((self.bits & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 31 - Target Time Register Busy"]
    #[inline(always)]
    pub fn trgtbusy(&self) -> TRGTBUSY_R {
        TRGTBUSY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Target Timestamp Low Register"]
    #[inline(always)]
    pub fn ttslo(&mut self) -> TTSLO_W {
        TTSLO_W { w: self }
    }
}
