#[doc = "Reader of register SRSEL0"]
pub type R = crate::R<u32, super::SRSEL0>;
#[doc = "Writer for register SRSEL0"]
pub type W = crate::W<u32, super::SRSEL0>;
#[doc = "Register SRSEL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SRSEL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RS0`"]
pub type RS0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RS0`"]
pub struct RS0_W<'a> {
    w: &'a mut W,
}
impl<'a> RS0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `RS1`"]
pub type RS1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RS1`"]
pub struct RS1_W<'a> {
    w: &'a mut W,
}
impl<'a> RS1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `RS2`"]
pub type RS2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RS2`"]
pub struct RS2_W<'a> {
    w: &'a mut W,
}
impl<'a> RS2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `RS3`"]
pub type RS3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RS3`"]
pub struct RS3_W<'a> {
    w: &'a mut W,
}
impl<'a> RS3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `RS4`"]
pub type RS4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RS4`"]
pub struct RS4_W<'a> {
    w: &'a mut W,
}
impl<'a> RS4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `RS5`"]
pub type RS5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RS5`"]
pub struct RS5_W<'a> {
    w: &'a mut W,
}
impl<'a> RS5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `RS6`"]
pub type RS6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RS6`"]
pub struct RS6_W<'a> {
    w: &'a mut W,
}
impl<'a> RS6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `RS7`"]
pub type RS7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RS7`"]
pub struct RS7_W<'a> {
    w: &'a mut W,
}
impl<'a> RS7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Request Source for Line 0"]
    #[inline(always)]
    pub fn rs0(&self) -> RS0_R {
        RS0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Request Source for Line 1"]
    #[inline(always)]
    pub fn rs1(&self) -> RS1_R {
        RS1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Request Source for Line 2"]
    #[inline(always)]
    pub fn rs2(&self) -> RS2_R {
        RS2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Request Source for Line 3"]
    #[inline(always)]
    pub fn rs3(&self) -> RS3_R {
        RS3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Request Source for Line 4"]
    #[inline(always)]
    pub fn rs4(&self) -> RS4_R {
        RS4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Request Source for Line 5"]
    #[inline(always)]
    pub fn rs5(&self) -> RS5_R {
        RS5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Request Source for Line 6"]
    #[inline(always)]
    pub fn rs6(&self) -> RS6_R {
        RS6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Request Source for Line 7"]
    #[inline(always)]
    pub fn rs7(&self) -> RS7_R {
        RS7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Request Source for Line 0"]
    #[inline(always)]
    pub fn rs0(&mut self) -> RS0_W {
        RS0_W { w: self }
    }
    #[doc = "Bits 4:7 - Request Source for Line 1"]
    #[inline(always)]
    pub fn rs1(&mut self) -> RS1_W {
        RS1_W { w: self }
    }
    #[doc = "Bits 8:11 - Request Source for Line 2"]
    #[inline(always)]
    pub fn rs2(&mut self) -> RS2_W {
        RS2_W { w: self }
    }
    #[doc = "Bits 12:15 - Request Source for Line 3"]
    #[inline(always)]
    pub fn rs3(&mut self) -> RS3_W {
        RS3_W { w: self }
    }
    #[doc = "Bits 16:19 - Request Source for Line 4"]
    #[inline(always)]
    pub fn rs4(&mut self) -> RS4_W {
        RS4_W { w: self }
    }
    #[doc = "Bits 20:23 - Request Source for Line 5"]
    #[inline(always)]
    pub fn rs5(&mut self) -> RS5_W {
        RS5_W { w: self }
    }
    #[doc = "Bits 24:27 - Request Source for Line 6"]
    #[inline(always)]
    pub fn rs6(&mut self) -> RS6_W {
        RS6_W { w: self }
    }
    #[doc = "Bits 28:31 - Request Source for Line 7"]
    #[inline(always)]
    pub fn rs7(&mut self) -> RS7_W {
        RS7_W { w: self }
    }
}
