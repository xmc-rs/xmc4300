#[doc = "Reader of register TIMER"]
pub type R = crate::R<u32, super::TIMER>;
#[doc = "Writer for register TIMER"]
pub type W = crate::W<u32, super::TIMER>;
#[doc = "Register TIMER `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TVAL`"]
pub type TVAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TVAL`"]
pub struct TVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> TVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Timer Value"]
    #[inline(always)]
    pub fn tval(&self) -> TVAL_R {
        TVAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer Value"]
    #[inline(always)]
    pub fn tval(&mut self) -> TVAL_W {
        TVAL_W { w: self }
    }
}
