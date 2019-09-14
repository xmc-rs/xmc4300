#[doc = "Reader of register HFNUM"]
pub type R = crate::R<u32, super::HFNUM>;
#[doc = "Writer for register HFNUM"]
pub type W = crate::W<u32, super::HFNUM>;
#[doc = "Register HFNUM `reset()`'s with value 0x3fff"]
impl crate::ResetValue for super::HFNUM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3fff
    }
}
#[doc = "Reader of field `FrNum`"]
pub type FRNUM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FrNum`"]
pub struct FRNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> FRNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `FrRem`"]
pub type FRREM_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Frame Number"]
    #[inline(always)]
    pub fn fr_num(&self) -> FRNUM_R {
        FRNUM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Frame Time Remaining"]
    #[inline(always)]
    pub fn fr_rem(&self) -> FRREM_R {
        FRREM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Frame Number"]
    #[inline(always)]
    pub fn fr_num(&mut self) -> FRNUM_W {
        FRNUM_W { w: self }
    }
}
