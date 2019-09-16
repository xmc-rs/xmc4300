#[doc = "Reader of register LINE1"]
pub type R = crate::R<u32, super::LINE1>;
#[doc = "Writer for register LINE1"]
pub type W = crate::W<u32, super::LINE1>;
#[doc = "Register LINE1 `reset()`'s with value 0"]
impl crate::ResetValue for super::LINE1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LINE_4`"]
pub type LINE_4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LINE_4`"]
pub struct LINE_4_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE_4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `LINE_5`"]
pub type LINE_5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LINE_5`"]
pub struct LINE_5_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE_5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `LINE_6`"]
pub type LINE_6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LINE_6`"]
pub struct LINE_6_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE_6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `LINE_A`"]
pub type LINE_A_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LINE_A`"]
pub struct LINE_A_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE_A_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Output on LINE\\[x\\]"]
    #[inline(always)]
    pub fn line_4(&self) -> LINE_4_R {
        LINE_4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Output on LINE\\[x\\]"]
    #[inline(always)]
    pub fn line_5(&self) -> LINE_5_R {
        LINE_5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Output on LINE\\[x\\]"]
    #[inline(always)]
    pub fn line_6(&self) -> LINE_6_R {
        LINE_6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Output on LINE\\[x\\]"]
    #[inline(always)]
    pub fn line_a(&self) -> LINE_A_R {
        LINE_A_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Output on LINE\\[x\\]"]
    #[inline(always)]
    pub fn line_4(&mut self) -> LINE_4_W {
        LINE_4_W { w: self }
    }
    #[doc = "Bits 8:15 - Output on LINE\\[x\\]"]
    #[inline(always)]
    pub fn line_5(&mut self) -> LINE_5_W {
        LINE_5_W { w: self }
    }
    #[doc = "Bits 16:23 - Output on LINE\\[x\\]"]
    #[inline(always)]
    pub fn line_6(&mut self) -> LINE_6_W {
        LINE_6_W { w: self }
    }
    #[doc = "Bits 24:31 - Output on LINE\\[x\\]"]
    #[inline(always)]
    pub fn line_a(&mut self) -> LINE_A_W {
        LINE_A_W { w: self }
    }
}
