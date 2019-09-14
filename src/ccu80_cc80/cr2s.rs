#[doc = "Reader of register CR2S"]
pub type R = crate::R<u32, super::CR2S>;
#[doc = "Writer for register CR2S"]
pub type W = crate::W<u32, super::CR2S>;
#[doc = "Register CR2S `reset()`'s with value 0"]
impl crate::ResetValue for super::CR2S {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CR2S`"]
pub type CR2S_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CR2S`"]
pub struct CR2S_W<'a> {
    w: &'a mut W,
}
impl<'a> CR2S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Shadow Compare Register for Channel 2"]
    #[inline(always)]
    pub fn cr2s(&self) -> CR2S_R {
        CR2S_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Shadow Compare Register for Channel 2"]
    #[inline(always)]
    pub fn cr2s(&mut self) -> CR2S_W {
        CR2S_W { w: self }
    }
}
