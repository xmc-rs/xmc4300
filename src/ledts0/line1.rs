#[doc = "Register `LINE1` reader"]
pub struct R(crate::R<LINE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LINE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LINE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LINE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LINE1` writer"]
pub struct W(crate::W<LINE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LINE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<LINE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LINE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LINE_4` reader - Output on LINE\\[x\\]"]
pub struct LINE_4_R(crate::FieldReader<u8, u8>);
impl LINE_4_R {
    pub(crate) fn new(bits: u8) -> Self {
        LINE_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINE_4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINE_4` writer - Output on LINE\\[x\\]"]
pub struct LINE_4_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE_4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `LINE_5` reader - Output on LINE\\[x\\]"]
pub struct LINE_5_R(crate::FieldReader<u8, u8>);
impl LINE_5_R {
    pub(crate) fn new(bits: u8) -> Self {
        LINE_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINE_5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINE_5` writer - Output on LINE\\[x\\]"]
pub struct LINE_5_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE_5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `LINE_6` reader - Output on LINE\\[x\\]"]
pub struct LINE_6_R(crate::FieldReader<u8, u8>);
impl LINE_6_R {
    pub(crate) fn new(bits: u8) -> Self {
        LINE_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINE_6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINE_6` writer - Output on LINE\\[x\\]"]
pub struct LINE_6_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE_6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `LINE_A` reader - Output on LINE\\[x\\]"]
pub struct LINE_A_R(crate::FieldReader<u8, u8>);
impl LINE_A_R {
    pub(crate) fn new(bits: u8) -> Self {
        LINE_A_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINE_A_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINE_A` writer - Output on LINE\\[x\\]"]
pub struct LINE_A_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE_A_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Line Pattern Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [line1](index.html) module"]
pub struct LINE1_SPEC;
impl crate::RegisterSpec for LINE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [line1::R](R) reader structure"]
impl crate::Readable for LINE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [line1::W](W) writer structure"]
impl crate::Writable for LINE1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LINE1 to value 0"]
impl crate::Resettable for LINE1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
