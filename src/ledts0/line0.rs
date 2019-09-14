#[doc = "Reader of register LINE0"]
pub type R = crate::R<u32, super::LINE0>;
#[doc = "Writer for register LINE0"]
pub type W = crate::W<u32, super::LINE0>;
#[doc = "Register LINE0 `reset()`'s with value 0"]
impl crate::ResetValue for super::LINE0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LINE_0`"]
pub type LINE_0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LINE_0`"]
pub struct LINE_0_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `LINE_1`"]
pub type LINE_1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LINE_1`"]
pub struct LINE_1_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `LINE_2`"]
pub type LINE_2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LINE_2`"]
pub struct LINE_2_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `LINE_3`"]
pub type LINE_3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LINE_3`"]
pub struct LINE_3_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE_3_W<'a> {
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
    pub fn line_0(&self) -> LINE_0_R {
        LINE_0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Output on LINE\\[x\\]"]
    #[inline(always)]
    pub fn line_1(&self) -> LINE_1_R {
        LINE_1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Output on LINE\\[x\\]"]
    #[inline(always)]
    pub fn line_2(&self) -> LINE_2_R {
        LINE_2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Output on LINE\\[x\\]"]
    #[inline(always)]
    pub fn line_3(&self) -> LINE_3_R {
        LINE_3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Output on LINE\\[x\\]"]
    #[inline(always)]
    pub fn line_0(&mut self) -> LINE_0_W {
        LINE_0_W { w: self }
    }
    #[doc = "Bits 8:15 - Output on LINE\\[x\\]"]
    #[inline(always)]
    pub fn line_1(&mut self) -> LINE_1_W {
        LINE_1_W { w: self }
    }
    #[doc = "Bits 16:23 - Output on LINE\\[x\\]"]
    #[inline(always)]
    pub fn line_2(&mut self) -> LINE_2_W {
        LINE_2_W { w: self }
    }
    #[doc = "Bits 24:31 - Output on LINE\\[x\\]"]
    #[inline(always)]
    pub fn line_3(&mut self) -> LINE_3_W {
        LINE_3_W { w: self }
    }
}
