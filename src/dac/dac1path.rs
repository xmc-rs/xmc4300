#[doc = "Reader of register DAC1PATH"]
pub type R = crate::R<u32, super::DAC1PATH>;
#[doc = "Writer for register DAC1PATH"]
pub type W = crate::W<u32, super::DAC1PATH>;
#[doc = "Register DAC1PATH `reset()`'s with value 0x7fdd"]
impl crate::ResetValue for super::DAC1PATH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x7fdd
    }
}
#[doc = "Reader of field `PAT6`"]
pub type PAT6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PAT6`"]
pub struct PAT6_W<'a> {
    w: &'a mut W,
}
impl<'a> PAT6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `PAT7`"]
pub type PAT7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PAT7`"]
pub struct PAT7_W<'a> {
    w: &'a mut W,
}
impl<'a> PAT7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | (((value as u32) & 0x1f) << 5);
        self.w
    }
}
#[doc = "Reader of field `PAT8`"]
pub type PAT8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PAT8`"]
pub struct PAT8_W<'a> {
    w: &'a mut W,
}
impl<'a> PAT8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | (((value as u32) & 0x1f) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Pattern Number 6 for PATGEN of DAC1"]
    #[inline(always)]
    pub fn pat6(&self) -> PAT6_R {
        PAT6_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Pattern Number 7 for PATGEN of DAC1"]
    #[inline(always)]
    pub fn pat7(&self) -> PAT7_R {
        PAT7_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Pattern Number 8 for PATGEN of DAC1"]
    #[inline(always)]
    pub fn pat8(&self) -> PAT8_R {
        PAT8_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Pattern Number 6 for PATGEN of DAC1"]
    #[inline(always)]
    pub fn pat6(&mut self) -> PAT6_W {
        PAT6_W { w: self }
    }
    #[doc = "Bits 5:9 - Pattern Number 7 for PATGEN of DAC1"]
    #[inline(always)]
    pub fn pat7(&mut self) -> PAT7_W {
        PAT7_W { w: self }
    }
    #[doc = "Bits 10:14 - Pattern Number 8 for PATGEN of DAC1"]
    #[inline(always)]
    pub fn pat8(&mut self) -> PAT8_W {
        PAT8_W { w: self }
    }
}
