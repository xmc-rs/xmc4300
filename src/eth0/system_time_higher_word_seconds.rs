#[doc = "Reader of register SYSTEM_TIME_HIGHER_WORD_SECONDS"]
pub type R = crate::R<u32, super::SYSTEM_TIME_HIGHER_WORD_SECONDS>;
#[doc = "Writer for register SYSTEM_TIME_HIGHER_WORD_SECONDS"]
pub type W = crate::W<u32, super::SYSTEM_TIME_HIGHER_WORD_SECONDS>;
#[doc = "Register SYSTEM_TIME_HIGHER_WORD_SECONDS `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSTEM_TIME_HIGHER_WORD_SECONDS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TSHWR`"]
pub type TSHWR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TSHWR`"]
pub struct TSHWR_W<'a> {
    w: &'a mut W,
}
impl<'a> TSHWR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Timestamp Higher Word Register"]
    #[inline(always)]
    pub fn tshwr(&self) -> TSHWR_R {
        TSHWR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timestamp Higher Word Register"]
    #[inline(always)]
    pub fn tshwr(&mut self) -> TSHWR_W {
        TSHWR_W { w: self }
    }
}
