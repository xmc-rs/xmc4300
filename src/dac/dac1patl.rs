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
pub type PAT0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAT0` writer - Pattern Number 0 for PATGEN of DAC1"]
pub type PAT0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAC1PATL_SPEC, u8, u8, 5, O>;
#[doc = "Field `PAT1` reader - Pattern Number 1 for PATGEN of DAC1"]
pub type PAT1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAT1` writer - Pattern Number 1 for PATGEN of DAC1"]
pub type PAT1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAC1PATL_SPEC, u8, u8, 5, O>;
#[doc = "Field `PAT2` reader - Pattern Number 2 for PATGEN of DAC1"]
pub type PAT2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAT2` writer - Pattern Number 2 for PATGEN of DAC1"]
pub type PAT2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAC1PATL_SPEC, u8, u8, 5, O>;
#[doc = "Field `PAT3` reader - Pattern Number 3 for PATGEN of DAC1"]
pub type PAT3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAT3` writer - Pattern Number 3 for PATGEN of DAC1"]
pub type PAT3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAC1PATL_SPEC, u8, u8, 5, O>;
#[doc = "Field `PAT4` reader - Pattern Number 4 for PATGEN of DAC1"]
pub type PAT4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAT4` writer - Pattern Number 4 for PATGEN of DAC1"]
pub type PAT4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAC1PATL_SPEC, u8, u8, 5, O>;
#[doc = "Field `PAT5` reader - Pattern Number 5 for PATGEN of DAC1"]
pub type PAT5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAT5` writer - Pattern Number 5 for PATGEN of DAC1"]
pub type PAT5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAC1PATL_SPEC, u8, u8, 5, O>;
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
    #[must_use]
    pub fn pat0(&mut self) -> PAT0_W<0> {
        PAT0_W::new(self)
    }
    #[doc = "Bits 5:9 - Pattern Number 1 for PATGEN of DAC1"]
    #[inline(always)]
    #[must_use]
    pub fn pat1(&mut self) -> PAT1_W<5> {
        PAT1_W::new(self)
    }
    #[doc = "Bits 10:14 - Pattern Number 2 for PATGEN of DAC1"]
    #[inline(always)]
    #[must_use]
    pub fn pat2(&mut self) -> PAT2_W<10> {
        PAT2_W::new(self)
    }
    #[doc = "Bits 15:19 - Pattern Number 3 for PATGEN of DAC1"]
    #[inline(always)]
    #[must_use]
    pub fn pat3(&mut self) -> PAT3_W<15> {
        PAT3_W::new(self)
    }
    #[doc = "Bits 20:24 - Pattern Number 4 for PATGEN of DAC1"]
    #[inline(always)]
    #[must_use]
    pub fn pat4(&mut self) -> PAT4_W<20> {
        PAT4_W::new(self)
    }
    #[doc = "Bits 25:29 - Pattern Number 5 for PATGEN of DAC1"]
    #[inline(always)]
    #[must_use]
    pub fn pat5(&mut self) -> PAT5_W<25> {
        PAT5_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAC1PATL to value 0x3568_b0c0"]
impl crate::Resettable for DAC1PATL_SPEC {
    const RESET_VALUE: Self::Ux = 0x3568_b0c0;
}
