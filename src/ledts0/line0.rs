#[doc = "Register `LINE0` reader"]
pub struct R(crate::R<LINE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LINE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LINE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LINE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LINE0` writer"]
pub struct W(crate::W<LINE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LINE0_SPEC>;
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
impl From<crate::W<LINE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LINE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LINE_0` reader - Output on LINE\\[x\\]"]
pub struct LINE_0_R(crate::FieldReader<u8, u8>);
impl LINE_0_R {
    pub(crate) fn new(bits: u8) -> Self {
        LINE_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINE_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINE_0` writer - Output on LINE\\[x\\]"]
pub struct LINE_0_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `LINE_1` reader - Output on LINE\\[x\\]"]
pub struct LINE_1_R(crate::FieldReader<u8, u8>);
impl LINE_1_R {
    pub(crate) fn new(bits: u8) -> Self {
        LINE_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINE_1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINE_1` writer - Output on LINE\\[x\\]"]
pub struct LINE_1_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `LINE_2` reader - Output on LINE\\[x\\]"]
pub struct LINE_2_R(crate::FieldReader<u8, u8>);
impl LINE_2_R {
    pub(crate) fn new(bits: u8) -> Self {
        LINE_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINE_2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINE_2` writer - Output on LINE\\[x\\]"]
pub struct LINE_2_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `LINE_3` reader - Output on LINE\\[x\\]"]
pub struct LINE_3_R(crate::FieldReader<u8, u8>);
impl LINE_3_R {
    pub(crate) fn new(bits: u8) -> Self {
        LINE_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINE_3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINE_3` writer - Output on LINE\\[x\\]"]
pub struct LINE_3_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE_3_W<'a> {
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Line Pattern Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [line0](index.html) module"]
pub struct LINE0_SPEC;
impl crate::RegisterSpec for LINE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [line0::R](R) reader structure"]
impl crate::Readable for LINE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [line0::W](W) writer structure"]
impl crate::Writable for LINE0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LINE0 to value 0"]
impl crate::Resettable for LINE0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
