#[doc = "Reader of register PLLCON1"]
pub type R = crate::R<u32, super::PLLCON1>;
#[doc = "Writer for register PLLCON1"]
pub type W = crate::W<u32, super::PLLCON1>;
#[doc = "Register PLLCON1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PLLCON1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `K1DIV`"]
pub type K1DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `K1DIV`"]
pub struct K1DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> K1DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `NDIV`"]
pub type NDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NDIV`"]
pub struct NDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> NDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Reader of field `K2DIV`"]
pub type K2DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `K2DIV`"]
pub struct K2DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> K2DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `PDIV`"]
pub type PDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PDIV`"]
pub struct PDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - K1-Divider Value"]
    #[inline(always)]
    pub fn k1div(&self) -> K1DIV_R {
        K1DIV_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - N-Divider Value"]
    #[inline(always)]
    pub fn ndiv(&self) -> NDIV_R {
        NDIV_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - K2-Divider Value"]
    #[inline(always)]
    pub fn k2div(&self) -> K2DIV_R {
        K2DIV_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:27 - P-Divider Value"]
    #[inline(always)]
    pub fn pdiv(&self) -> PDIV_R {
        PDIV_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - K1-Divider Value"]
    #[inline(always)]
    pub fn k1div(&mut self) -> K1DIV_W {
        K1DIV_W { w: self }
    }
    #[doc = "Bits 8:14 - N-Divider Value"]
    #[inline(always)]
    pub fn ndiv(&mut self) -> NDIV_W {
        NDIV_W { w: self }
    }
    #[doc = "Bits 16:22 - K2-Divider Value"]
    #[inline(always)]
    pub fn k2div(&mut self) -> K2DIV_W {
        K2DIV_W { w: self }
    }
    #[doc = "Bits 24:27 - P-Divider Value"]
    #[inline(always)]
    pub fn pdiv(&mut self) -> PDIV_W {
        PDIV_W { w: self }
    }
}
