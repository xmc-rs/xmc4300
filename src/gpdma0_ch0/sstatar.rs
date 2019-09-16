#[doc = "Reader of register SSTATAR"]
pub type R = crate::R<u32, super::SSTATAR>;
#[doc = "Writer for register SSTATAR"]
pub type W = crate::W<u32, super::SSTATAR>;
#[doc = "Register SSTATAR `reset()`'s with value 0"]
impl crate::ResetValue for super::SSTATAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SSTATAR`"]
pub type SSTATAR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SSTATAR`"]
pub struct SSTATAR_W<'a> {
    w: &'a mut W,
}
impl<'a> SSTATAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Source Status Address"]
    #[inline(always)]
    pub fn sstatar(&self) -> SSTATAR_R {
        SSTATAR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Source Status Address"]
    #[inline(always)]
    pub fn sstatar(&mut self) -> SSTATAR_W {
        SSTATAR_W { w: self }
    }
}
