#[doc = "Register `TIM1` reader"]
pub struct R(crate::R<TIM1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM1` writer"]
pub struct W(crate::W<TIM1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM1_SPEC>;
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
impl From<crate::W<TIM1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAWE` reader - Days of Week Time Value"]
pub type DAWE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAWE` writer - Days of Week Time Value"]
pub type DAWE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM1_SPEC, u8, u8, 3, O>;
#[doc = "Field `MO` reader - Month Time Value"]
pub type MO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MO` writer - Month Time Value"]
pub type MO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM1_SPEC, u8, u8, 4, O>;
#[doc = "Field `YE` reader - Year Time Value"]
pub type YE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `YE` writer - Year Time Value"]
pub type YE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:2 - Days of Week Time Value"]
    #[inline(always)]
    pub fn dawe(&self) -> DAWE_R {
        DAWE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:11 - Month Time Value"]
    #[inline(always)]
    pub fn mo(&self) -> MO_R {
        MO_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - Year Time Value"]
    #[inline(always)]
    pub fn ye(&self) -> YE_R {
        YE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - Days of Week Time Value"]
    #[inline(always)]
    #[must_use]
    pub fn dawe(&mut self) -> DAWE_W<0> {
        DAWE_W::new(self)
    }
    #[doc = "Bits 8:11 - Month Time Value"]
    #[inline(always)]
    #[must_use]
    pub fn mo(&mut self) -> MO_W<8> {
        MO_W::new(self)
    }
    #[doc = "Bits 16:31 - Year Time Value"]
    #[inline(always)]
    #[must_use]
    pub fn ye(&mut self) -> YE_W<16> {
        YE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Time Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1](index.html) module"]
pub struct TIM1_SPEC;
impl crate::RegisterSpec for TIM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim1::R](R) reader structure"]
impl crate::Readable for TIM1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim1::W](W) writer structure"]
impl crate::Writable for TIM1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM1 to value 0"]
impl crate::Resettable for TIM1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
