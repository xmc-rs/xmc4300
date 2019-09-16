#[doc = "Reader of register CMTR"]
pub type R = crate::R<u32, super::CMTR>;
#[doc = "Writer for register CMTR"]
pub type W = crate::W<u32, super::CMTR>;
#[doc = "Register CMTR `reset()`'s with value 0"]
impl crate::ResetValue for super::CMTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CTV`"]
pub type CTV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CTV`"]
pub struct CTV_W<'a> {
    w: &'a mut W,
}
impl<'a> CTV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Captured Timer Value"]
    #[inline(always)]
    pub fn ctv(&self) -> CTV_R {
        CTV_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Captured Timer Value"]
    #[inline(always)]
    pub fn ctv(&mut self) -> CTV_W {
        CTV_W { w: self }
    }
}
