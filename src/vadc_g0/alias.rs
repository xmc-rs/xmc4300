#[doc = "Reader of register ALIAS"]
pub type R = crate::R<u32, super::ALIAS>;
#[doc = "Writer for register ALIAS"]
pub type W = crate::W<u32, super::ALIAS>;
#[doc = "Register ALIAS `reset()`'s with value 0x0100"]
impl crate::ResetValue for super::ALIAS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100
    }
}
#[doc = "Reader of field `ALIAS0`"]
pub type ALIAS0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ALIAS0`"]
pub struct ALIAS0_W<'a> {
    w: &'a mut W,
}
impl<'a> ALIAS0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `ALIAS1`"]
pub type ALIAS1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ALIAS1`"]
pub struct ALIAS1_W<'a> {
    w: &'a mut W,
}
impl<'a> ALIAS1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Alias Value for CH0 Conversion Requests"]
    #[inline(always)]
    pub fn alias0(&self) -> ALIAS0_R {
        ALIAS0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Alias Value for CH1 Conversion Requests"]
    #[inline(always)]
    pub fn alias1(&self) -> ALIAS1_R {
        ALIAS1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Alias Value for CH0 Conversion Requests"]
    #[inline(always)]
    pub fn alias0(&mut self) -> ALIAS0_W {
        ALIAS0_W { w: self }
    }
    #[doc = "Bits 8:12 - Alias Value for CH1 Conversion Requests"]
    #[inline(always)]
    pub fn alias1(&mut self) -> ALIAS1_W {
        ALIAS1_W { w: self }
    }
}
