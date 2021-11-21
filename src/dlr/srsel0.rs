#[doc = "Register `SRSEL0` reader"]
pub struct R(crate::R<SRSEL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRSEL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRSEL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRSEL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRSEL0` writer"]
pub struct W(crate::W<SRSEL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRSEL0_SPEC>;
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
impl From<crate::W<SRSEL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRSEL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RS0` reader - Request Source for Line 0"]
pub struct RS0_R(crate::FieldReader<u8, u8>);
impl RS0_R {
    pub(crate) fn new(bits: u8) -> Self {
        RS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS0` writer - Request Source for Line 0"]
pub struct RS0_W<'a> {
    w: &'a mut W,
}
impl<'a> RS0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `RS1` reader - Request Source for Line 1"]
pub struct RS1_R(crate::FieldReader<u8, u8>);
impl RS1_R {
    pub(crate) fn new(bits: u8) -> Self {
        RS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS1` writer - Request Source for Line 1"]
pub struct RS1_W<'a> {
    w: &'a mut W,
}
impl<'a> RS1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `RS2` reader - Request Source for Line 2"]
pub struct RS2_R(crate::FieldReader<u8, u8>);
impl RS2_R {
    pub(crate) fn new(bits: u8) -> Self {
        RS2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS2` writer - Request Source for Line 2"]
pub struct RS2_W<'a> {
    w: &'a mut W,
}
impl<'a> RS2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `RS3` reader - Request Source for Line 3"]
pub struct RS3_R(crate::FieldReader<u8, u8>);
impl RS3_R {
    pub(crate) fn new(bits: u8) -> Self {
        RS3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS3` writer - Request Source for Line 3"]
pub struct RS3_W<'a> {
    w: &'a mut W,
}
impl<'a> RS3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `RS4` reader - Request Source for Line 4"]
pub struct RS4_R(crate::FieldReader<u8, u8>);
impl RS4_R {
    pub(crate) fn new(bits: u8) -> Self {
        RS4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS4` writer - Request Source for Line 4"]
pub struct RS4_W<'a> {
    w: &'a mut W,
}
impl<'a> RS4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `RS5` reader - Request Source for Line 5"]
pub struct RS5_R(crate::FieldReader<u8, u8>);
impl RS5_R {
    pub(crate) fn new(bits: u8) -> Self {
        RS5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS5` writer - Request Source for Line 5"]
pub struct RS5_W<'a> {
    w: &'a mut W,
}
impl<'a> RS5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `RS6` reader - Request Source for Line 6"]
pub struct RS6_R(crate::FieldReader<u8, u8>);
impl RS6_R {
    pub(crate) fn new(bits: u8) -> Self {
        RS6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS6` writer - Request Source for Line 6"]
pub struct RS6_W<'a> {
    w: &'a mut W,
}
impl<'a> RS6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `RS7` reader - Request Source for Line 7"]
pub struct RS7_R(crate::FieldReader<u8, u8>);
impl RS7_R {
    pub(crate) fn new(bits: u8) -> Self {
        RS7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS7` writer - Request Source for Line 7"]
pub struct RS7_W<'a> {
    w: &'a mut W,
}
impl<'a> RS7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Service Request Selection 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srsel0](index.html) module"]
pub struct SRSEL0_SPEC;
impl crate::RegisterSpec for SRSEL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srsel0::R](R) reader structure"]
impl crate::Readable for SRSEL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srsel0::W](W) writer structure"]
impl crate::Writable for SRSEL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRSEL0 to value 0"]
impl crate::Resettable for SRSEL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
