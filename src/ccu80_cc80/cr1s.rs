#[doc = "Reader of register CR1S"]
pub type R = crate::R<u32, super::CR1S>;
#[doc = "Writer for register CR1S"]
pub type W = crate::W<u32, super::CR1S>;
#[doc = "Register CR1S `reset()`'s with value 0"]
impl crate::ResetValue for super::CR1S {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CR1S`"]
pub type CR1S_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CR1S`"]
pub struct CR1S_W<'a> {
    w: &'a mut W,
}
impl<'a> CR1S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Shadow Compare Register for Channel 1"]
    #[inline(always)]
    pub fn cr1s(&self) -> CR1S_R {
        CR1S_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Shadow Compare Register for Channel 1"]
    #[inline(always)]
    pub fn cr1s(&mut self) -> CR1S_W {
        CR1S_W { w: self }
    }
}
