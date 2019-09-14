#[doc = "Reader of register ATIM1"]
pub type R = crate::R<u32, super::ATIM1>;
#[doc = "Writer for register ATIM1"]
pub type W = crate::W<u32, super::ATIM1>;
#[doc = "Register ATIM1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ATIM1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AMO`"]
pub type AMO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AMO`"]
pub struct AMO_W<'a> {
    w: &'a mut W,
}
impl<'a> AMO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `AYE`"]
pub type AYE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `AYE`"]
pub struct AYE_W<'a> {
    w: &'a mut W,
}
impl<'a> AYE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:11 - Alarm Month Compare Value"]
    #[inline(always)]
    pub fn amo(&self) -> AMO_R {
        AMO_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - Alarm Year Compare Value"]
    #[inline(always)]
    pub fn aye(&self) -> AYE_R {
        AYE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:11 - Alarm Month Compare Value"]
    #[inline(always)]
    pub fn amo(&mut self) -> AMO_W {
        AMO_W { w: self }
    }
    #[doc = "Bits 16:31 - Alarm Year Compare Value"]
    #[inline(always)]
    pub fn aye(&mut self) -> AYE_W {
        AYE_W { w: self }
    }
}
