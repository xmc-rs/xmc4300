#[doc = "Register `GLOBEFLAG` reader"]
pub struct R(crate::R<GLOBEFLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GLOBEFLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GLOBEFLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GLOBEFLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GLOBEFLAG` writer"]
pub struct W(crate::W<GLOBEFLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GLOBEFLAG_SPEC>;
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
impl From<crate::W<GLOBEFLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GLOBEFLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEVGLB` reader - Source Event (Background)"]
pub type SEVGLB_R = crate::BitReader<SEVGLB_A>;
#[doc = "Source Event (Background)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEVGLB_A {
    #[doc = "0: No source event"]
    VALUE1 = 0,
    #[doc = "1: A source event has occurred"]
    VALUE2 = 1,
}
impl From<SEVGLB_A> for bool {
    #[inline(always)]
    fn from(variant: SEVGLB_A) -> Self {
        variant as u8 != 0
    }
}
impl SEVGLB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEVGLB_A {
        match self.bits {
            false => SEVGLB_A::VALUE1,
            true => SEVGLB_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SEVGLB_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SEVGLB_A::VALUE2
    }
}
#[doc = "Field `SEVGLB` writer - Source Event (Background)"]
pub type SEVGLB_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLOBEFLAG_SPEC, SEVGLB_A, O>;
impl<'a, const O: u8> SEVGLB_W<'a, O> {
    #[doc = "No source event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SEVGLB_A::VALUE1)
    }
    #[doc = "A source event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SEVGLB_A::VALUE2)
    }
}
#[doc = "Field `REVGLB` reader - Global Result Event"]
pub type REVGLB_R = crate::BitReader<REVGLB_A>;
#[doc = "Global Result Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REVGLB_A {
    #[doc = "0: No result event"]
    VALUE1 = 0,
    #[doc = "1: New result was stored in register GLOBRES"]
    VALUE2 = 1,
}
impl From<REVGLB_A> for bool {
    #[inline(always)]
    fn from(variant: REVGLB_A) -> Self {
        variant as u8 != 0
    }
}
impl REVGLB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REVGLB_A {
        match self.bits {
            false => REVGLB_A::VALUE1,
            true => REVGLB_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == REVGLB_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REVGLB_A::VALUE2
    }
}
#[doc = "Field `REVGLB` writer - Global Result Event"]
pub type REVGLB_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLOBEFLAG_SPEC, REVGLB_A, O>;
impl<'a, const O: u8> REVGLB_W<'a, O> {
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REVGLB_A::VALUE1)
    }
    #[doc = "New result was stored in register GLOBRES"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REVGLB_A::VALUE2)
    }
}
#[doc = "Clear Source Event (Background)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEVGLBCLR_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the source event flag SEVGLB"]
    VALUE2 = 1,
}
impl From<SEVGLBCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: SEVGLBCLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEVGLBCLR` writer - Clear Source Event (Background)"]
pub type SEVGLBCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLOBEFLAG_SPEC, SEVGLBCLR_AW, O>;
impl<'a, const O: u8> SEVGLBCLR_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SEVGLBCLR_AW::VALUE1)
    }
    #[doc = "Clear the source event flag SEVGLB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SEVGLBCLR_AW::VALUE2)
    }
}
#[doc = "Clear Global Result Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REVGLBCLR_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag REVGLB"]
    VALUE2 = 1,
}
impl From<REVGLBCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: REVGLBCLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REVGLBCLR` writer - Clear Global Result Event"]
pub type REVGLBCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLOBEFLAG_SPEC, REVGLBCLR_AW, O>;
impl<'a, const O: u8> REVGLBCLR_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REVGLBCLR_AW::VALUE1)
    }
    #[doc = "Clear the result event flag REVGLB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REVGLBCLR_AW::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Source Event (Background)"]
    #[inline(always)]
    pub fn sevglb(&self) -> SEVGLB_R {
        SEVGLB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Global Result Event"]
    #[inline(always)]
    pub fn revglb(&self) -> REVGLB_R {
        REVGLB_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Source Event (Background)"]
    #[inline(always)]
    #[must_use]
    pub fn sevglb(&mut self) -> SEVGLB_W<0> {
        SEVGLB_W::new(self)
    }
    #[doc = "Bit 8 - Global Result Event"]
    #[inline(always)]
    #[must_use]
    pub fn revglb(&mut self) -> REVGLB_W<8> {
        REVGLB_W::new(self)
    }
    #[doc = "Bit 16 - Clear Source Event (Background)"]
    #[inline(always)]
    #[must_use]
    pub fn sevglbclr(&mut self) -> SEVGLBCLR_W<16> {
        SEVGLBCLR_W::new(self)
    }
    #[doc = "Bit 24 - Clear Global Result Event"]
    #[inline(always)]
    #[must_use]
    pub fn revglbclr(&mut self) -> REVGLBCLR_W<24> {
        REVGLBCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Event Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [globeflag](index.html) module"]
pub struct GLOBEFLAG_SPEC;
impl crate::RegisterSpec for GLOBEFLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [globeflag::R](R) reader structure"]
impl crate::Readable for GLOBEFLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [globeflag::W](W) writer structure"]
impl crate::Writable for GLOBEFLAG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GLOBEFLAG to value 0"]
impl crate::Resettable for GLOBEFLAG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
