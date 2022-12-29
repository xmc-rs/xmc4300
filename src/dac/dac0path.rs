#[doc = "Register `DAC0PATH` reader"]
pub struct R(crate::R<DAC0PATH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC0PATH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC0PATH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC0PATH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC0PATH` writer"]
pub struct W(crate::W<DAC0PATH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC0PATH_SPEC>;
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
impl From<crate::W<DAC0PATH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC0PATH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAT6` reader - Pattern Number 6 for PATGEN of DAC0"]
pub type PAT6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAT6` writer - Pattern Number 6 for PATGEN of DAC0"]
pub type PAT6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAC0PATH_SPEC, u8, u8, 5, O>;
#[doc = "Field `PAT7` reader - Pattern Number 7 for PATGEN of DAC0"]
pub type PAT7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAT7` writer - Pattern Number 7 for PATGEN of DAC0"]
pub type PAT7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAC0PATH_SPEC, u8, u8, 5, O>;
#[doc = "Field `PAT8` reader - Pattern Number 8 for PATGEN of DAC0"]
pub type PAT8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAT8` writer - Pattern Number 8 for PATGEN of DAC0"]
pub type PAT8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAC0PATH_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Pattern Number 6 for PATGEN of DAC0"]
    #[inline(always)]
    pub fn pat6(&self) -> PAT6_R {
        PAT6_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Pattern Number 7 for PATGEN of DAC0"]
    #[inline(always)]
    pub fn pat7(&self) -> PAT7_R {
        PAT7_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Pattern Number 8 for PATGEN of DAC0"]
    #[inline(always)]
    pub fn pat8(&self) -> PAT8_R {
        PAT8_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Pattern Number 6 for PATGEN of DAC0"]
    #[inline(always)]
    #[must_use]
    pub fn pat6(&mut self) -> PAT6_W<0> {
        PAT6_W::new(self)
    }
    #[doc = "Bits 5:9 - Pattern Number 7 for PATGEN of DAC0"]
    #[inline(always)]
    #[must_use]
    pub fn pat7(&mut self) -> PAT7_W<5> {
        PAT7_W::new(self)
    }
    #[doc = "Bits 10:14 - Pattern Number 8 for PATGEN of DAC0"]
    #[inline(always)]
    #[must_use]
    pub fn pat8(&mut self) -> PAT8_W<10> {
        PAT8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC0 Higher Pattern Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac0path](index.html) module"]
pub struct DAC0PATH_SPEC;
impl crate::RegisterSpec for DAC0PATH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac0path::R](R) reader structure"]
impl crate::Readable for DAC0PATH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac0path::W](W) writer structure"]
impl crate::Writable for DAC0PATH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAC0PATH to value 0x7fdd"]
impl crate::Resettable for DAC0PATH_SPEC {
    const RESET_VALUE: Self::Ux = 0x7fdd;
}
