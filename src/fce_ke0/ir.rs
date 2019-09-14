#[doc = "Reader of register IR"]
pub type R = crate::R<u32, super::IR>;
#[doc = "Writer for register IR"]
pub type W = crate::W<u32, super::IR>;
#[doc = "Register IR `reset()`'s with value 0"]
impl crate::ResetValue for super::IR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IR`"]
pub type IR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IR`"]
pub struct IR_W<'a> {
    w: &'a mut W,
}
impl<'a> IR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Input Register"]
    #[inline(always)]
    pub fn ir(&self) -> IR_R {
        IR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Input Register"]
    #[inline(always)]
    pub fn ir(&mut self) -> IR_W {
        IR_W { w: self }
    }
}
