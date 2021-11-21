#[doc = "Register `DAC1PATH` reader"]
pub struct R(crate::R<DAC1PATH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC1PATH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC1PATH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC1PATH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC1PATH` writer"]
pub struct W(crate::W<DAC1PATH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC1PATH_SPEC>;
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
impl From<crate::W<DAC1PATH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC1PATH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAT6` reader - Pattern Number 6 for PATGEN of DAC1"]
pub struct PAT6_R(crate::FieldReader<u8, u8>);
impl PAT6_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAT6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAT6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAT6` writer - Pattern Number 6 for PATGEN of DAC1"]
pub struct PAT6_W<'a> {
    w: &'a mut W,
}
impl<'a> PAT6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `PAT7` reader - Pattern Number 7 for PATGEN of DAC1"]
pub struct PAT7_R(crate::FieldReader<u8, u8>);
impl PAT7_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAT7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAT7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAT7` writer - Pattern Number 7 for PATGEN of DAC1"]
pub struct PAT7_W<'a> {
    w: &'a mut W,
}
impl<'a> PAT7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | ((value as u32 & 0x1f) << 5);
        self.w
    }
}
#[doc = "Field `PAT8` reader - Pattern Number 8 for PATGEN of DAC1"]
pub struct PAT8_R(crate::FieldReader<u8, u8>);
impl PAT8_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAT8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAT8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAT8` writer - Pattern Number 8 for PATGEN of DAC1"]
pub struct PAT8_W<'a> {
    w: &'a mut W,
}
impl<'a> PAT8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | ((value as u32 & 0x1f) << 10);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC1 Higher Pattern Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac1path](index.html) module"]
pub struct DAC1PATH_SPEC;
impl crate::RegisterSpec for DAC1PATH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac1path::R](R) reader structure"]
impl crate::Readable for DAC1PATH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac1path::W](W) writer structure"]
impl crate::Writable for DAC1PATH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DAC1PATH to value 0x7fdd"]
impl crate::Resettable for DAC1PATH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7fdd
    }
}
