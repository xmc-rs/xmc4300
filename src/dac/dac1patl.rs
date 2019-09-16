#[doc = "Reader of register DAC1PATL"]
pub type R = crate::R<u32, super::DAC1PATL>;
#[doc = "Writer for register DAC1PATL"]
pub type W = crate::W<u32, super::DAC1PATL>;
#[doc = "Register DAC1PATL `reset()`'s with value 0x3568_b0c0"]
impl crate::ResetValue for super::DAC1PATL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3568_b0c0
    }
}
#[doc = "Reader of field `PAT0`"]
pub type PAT0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PAT0`"]
pub struct PAT0_W<'a> {
    w: &'a mut W,
}
impl<'a> PAT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `PAT1`"]
pub type PAT1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PAT1`"]
pub struct PAT1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | (((value as u32) & 0x1f) << 5);
        self.w
    }
}
#[doc = "Reader of field `PAT2`"]
pub type PAT2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PAT2`"]
pub struct PAT2_W<'a> {
    w: &'a mut W,
}
impl<'a> PAT2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | (((value as u32) & 0x1f) << 10);
        self.w
    }
}
#[doc = "Reader of field `PAT3`"]
pub type PAT3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PAT3`"]
pub struct PAT3_W<'a> {
    w: &'a mut W,
}
impl<'a> PAT3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 15)) | (((value as u32) & 0x1f) << 15);
        self.w
    }
}
#[doc = "Reader of field `PAT4`"]
pub type PAT4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PAT4`"]
pub struct PAT4_W<'a> {
    w: &'a mut W,
}
impl<'a> PAT4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | (((value as u32) & 0x1f) << 20);
        self.w
    }
}
#[doc = "Reader of field `PAT5`"]
pub type PAT5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PAT5`"]
pub struct PAT5_W<'a> {
    w: &'a mut W,
}
impl<'a> PAT5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 25)) | (((value as u32) & 0x1f) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Pattern Number 0 for PATGEN of DAC1"]
    #[inline(always)]
    pub fn pat0(&self) -> PAT0_R {
        PAT0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Pattern Number 1 for PATGEN of DAC1"]
    #[inline(always)]
    pub fn pat1(&self) -> PAT1_R {
        PAT1_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Pattern Number 2 for PATGEN of DAC1"]
    #[inline(always)]
    pub fn pat2(&self) -> PAT2_R {
        PAT2_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - Pattern Number 3 for PATGEN of DAC1"]
    #[inline(always)]
    pub fn pat3(&self) -> PAT3_R {
        PAT3_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - Pattern Number 4 for PATGEN of DAC1"]
    #[inline(always)]
    pub fn pat4(&self) -> PAT4_R {
        PAT4_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - Pattern Number 5 for PATGEN of DAC1"]
    #[inline(always)]
    pub fn pat5(&self) -> PAT5_R {
        PAT5_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Pattern Number 0 for PATGEN of DAC1"]
    #[inline(always)]
    pub fn pat0(&mut self) -> PAT0_W {
        PAT0_W { w: self }
    }
    #[doc = "Bits 5:9 - Pattern Number 1 for PATGEN of DAC1"]
    #[inline(always)]
    pub fn pat1(&mut self) -> PAT1_W {
        PAT1_W { w: self }
    }
    #[doc = "Bits 10:14 - Pattern Number 2 for PATGEN of DAC1"]
    #[inline(always)]
    pub fn pat2(&mut self) -> PAT2_W {
        PAT2_W { w: self }
    }
    #[doc = "Bits 15:19 - Pattern Number 3 for PATGEN of DAC1"]
    #[inline(always)]
    pub fn pat3(&mut self) -> PAT3_W {
        PAT3_W { w: self }
    }
    #[doc = "Bits 20:24 - Pattern Number 4 for PATGEN of DAC1"]
    #[inline(always)]
    pub fn pat4(&mut self) -> PAT4_W {
        PAT4_W { w: self }
    }
    #[doc = "Bits 25:29 - Pattern Number 5 for PATGEN of DAC1"]
    #[inline(always)]
    pub fn pat5(&mut self) -> PAT5_W {
        PAT5_W { w: self }
    }
}
