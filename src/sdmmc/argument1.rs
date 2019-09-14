#[doc = "Reader of register ARGUMENT1"]
pub type R = crate::R<u32, super::ARGUMENT1>;
#[doc = "Writer for register ARGUMENT1"]
pub type W = crate::W<u32, super::ARGUMENT1>;
#[doc = "Register ARGUMENT1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ARGUMENT1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ARGUMENT1`"]
pub type ARGUMENT1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ARGUMENT1`"]
pub struct ARGUMENT1_W<'a> {
    w: &'a mut W,
}
impl<'a> ARGUMENT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Command Argument"]
    #[inline(always)]
    pub fn argument1(&self) -> ARGUMENT1_R {
        ARGUMENT1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command Argument"]
    #[inline(always)]
    pub fn argument1(&mut self) -> ARGUMENT1_W {
        ARGUMENT1_W { w: self }
    }
}
