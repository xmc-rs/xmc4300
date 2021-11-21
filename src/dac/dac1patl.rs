#[doc = "Register `DAC1PATL` reader"]
pub struct R(crate::R<DAC1PATL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC1PATL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC1PATL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC1PATL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC1PATL` writer"]
pub struct W(crate::W<DAC1PATL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC1PATL_SPEC>;
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
impl From<crate::W<DAC1PATL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC1PATL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAT0` reader - Pattern Number 0 for PATGEN of DAC1"]
pub struct PAT0_R(crate::FieldReader<u8, u8>);
impl PAT0_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAT0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAT0` writer - Pattern Number 0 for PATGEN of DAC1"]
pub struct PAT0_W<'a> {
    w: &'a mut W,
}
impl<'a> PAT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `PAT1` reader - Pattern Number 1 for PATGEN of DAC1"]
pub struct PAT1_R(crate::FieldReader<u8, u8>);
impl PAT1_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAT1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAT1` writer - Pattern Number 1 for PATGEN of DAC1"]
pub struct PAT1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | ((value as u32 & 0x1f) << 5);
        self.w
    }
}
#[doc = "Field `PAT2` reader - Pattern Number 2 for PATGEN of DAC1"]
pub struct PAT2_R(crate::FieldReader<u8, u8>);
impl PAT2_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAT2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAT2` writer - Pattern Number 2 for PATGEN of DAC1"]
pub struct PAT2_W<'a> {
    w: &'a mut W,
}
impl<'a> PAT2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | ((value as u32 & 0x1f) << 10);
        self.w
    }
}
#[doc = "Field `PAT3` reader - Pattern Number 3 for PATGEN of DAC1"]
pub struct PAT3_R(crate::FieldReader<u8, u8>);
impl PAT3_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAT3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAT3` writer - Pattern Number 3 for PATGEN of DAC1"]
pub struct PAT3_W<'a> {
    w: &'a mut W,
}
impl<'a> PAT3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 15)) | ((value as u32 & 0x1f) << 15);
        self.w
    }
}
#[doc = "Field `PAT4` reader - Pattern Number 4 for PATGEN of DAC1"]
pub struct PAT4_R(crate::FieldReader<u8, u8>);
impl PAT4_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAT4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAT4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAT4` writer - Pattern Number 4 for PATGEN of DAC1"]
pub struct PAT4_W<'a> {
    w: &'a mut W,
}
impl<'a> PAT4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | ((value as u32 & 0x1f) << 20);
        self.w
    }
}
#[doc = "Field `PAT5` reader - Pattern Number 5 for PATGEN of DAC1"]
pub struct PAT5_R(crate::FieldReader<u8, u8>);
impl PAT5_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAT5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAT5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAT5` writer - Pattern Number 5 for PATGEN of DAC1"]
pub struct PAT5_W<'a> {
    w: &'a mut W,
}
impl<'a> PAT5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 25)) | ((value as u32 & 0x1f) << 25);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC1 Lower Pattern Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac1patl](index.html) module"]
pub struct DAC1PATL_SPEC;
impl crate::RegisterSpec for DAC1PATL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac1patl::R](R) reader structure"]
impl crate::Readable for DAC1PATL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac1patl::W](W) writer structure"]
impl crate::Writable for DAC1PATL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DAC1PATL to value 0x3568_b0c0"]
impl crate::Resettable for DAC1PATL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3568_b0c0
    }
}
