#[doc = "Reader of register TIM1"]
pub type R = crate::R<u32, super::TIM1>;
#[doc = "Writer for register TIM1"]
pub type W = crate::W<u32, super::TIM1>;
#[doc = "Register TIM1 `reset()`'s with value 0"]
impl crate::ResetValue for super::TIM1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DAWE`"]
pub type DAWE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAWE`"]
pub struct DAWE_W<'a> {
    w: &'a mut W,
}
impl<'a> DAWE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `MO`"]
pub type MO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MO`"]
pub struct MO_W<'a> {
    w: &'a mut W,
}
impl<'a> MO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `YE`"]
pub type YE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `YE`"]
pub struct YE_W<'a> {
    w: &'a mut W,
}
impl<'a> YE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Days of Week Time Value"]
    #[inline(always)]
    pub fn dawe(&self) -> DAWE_R {
        DAWE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:11 - Month Time Value"]
    #[inline(always)]
    pub fn mo(&self) -> MO_R {
        MO_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - Year Time Value"]
    #[inline(always)]
    pub fn ye(&self) -> YE_R {
        YE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - Days of Week Time Value"]
    #[inline(always)]
    pub fn dawe(&mut self) -> DAWE_W {
        DAWE_W { w: self }
    }
    #[doc = "Bits 8:11 - Month Time Value"]
    #[inline(always)]
    pub fn mo(&mut self) -> MO_W {
        MO_W { w: self }
    }
    #[doc = "Bits 16:31 - Year Time Value"]
    #[inline(always)]
    pub fn ye(&mut self) -> YE_W {
        YE_W { w: self }
    }
}
