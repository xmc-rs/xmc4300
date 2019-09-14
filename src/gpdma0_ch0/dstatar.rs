#[doc = "Reader of register DSTATAR"]
pub type R = crate::R<u32, super::DSTATAR>;
#[doc = "Writer for register DSTATAR"]
pub type W = crate::W<u32, super::DSTATAR>;
#[doc = "Register DSTATAR `reset()`'s with value 0"]
impl crate::ResetValue for super::DSTATAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DSTATAR`"]
pub type DSTATAR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DSTATAR`"]
pub struct DSTATAR_W<'a> {
    w: &'a mut W,
}
impl<'a> DSTATAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Destination Status Address"]
    #[inline(always)]
    pub fn dstatar(&self) -> DSTATAR_R {
        DSTATAR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Destination Status Address"]
    #[inline(always)]
    pub fn dstatar(&mut self) -> DSTATAR_W {
        DSTATAR_W { w: self }
    }
}
