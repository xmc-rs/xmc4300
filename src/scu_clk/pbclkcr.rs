#[doc = "Register `PBCLKCR` reader"]
pub struct R(crate::R<PBCLKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBCLKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBCLKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBCLKCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBCLKCR` writer"]
pub struct W(crate::W<PBCLKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBCLKCR_SPEC>;
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
impl From<crate::W<PBCLKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBCLKCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PBDIV` reader - PB Clock Divider Enable"]
pub type PBDIV_R = crate::BitReader<PBDIV_A>;
#[doc = "PB Clock Divider Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PBDIV_A {
    #[doc = "0: fPERIPH = fCPU"]
    CONST_0 = 0,
    #[doc = "1: fPERIPH = fCPU / 2"]
    CONST_1 = 1,
}
impl From<PBDIV_A> for bool {
    #[inline(always)]
    fn from(variant: PBDIV_A) -> Self {
        variant as u8 != 0
    }
}
impl PBDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PBDIV_A {
        match self.bits {
            false => PBDIV_A::CONST_0,
            true => PBDIV_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PBDIV_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PBDIV_A::CONST_1
    }
}
#[doc = "Field `PBDIV` writer - PB Clock Divider Enable"]
pub type PBDIV_W<'a, const O: u8> = crate::BitWriter<'a, PBCLKCR_SPEC, O, PBDIV_A>;
impl<'a, const O: u8> PBDIV_W<'a, O> {
    #[doc = "fPERIPH = fCPU"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PBDIV_A::CONST_0)
    }
    #[doc = "fPERIPH = fCPU / 2"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PBDIV_A::CONST_1)
    }
}
impl R {
    #[doc = "Bit 0 - PB Clock Divider Enable"]
    #[inline(always)]
    pub fn pbdiv(&self) -> PBDIV_R {
        PBDIV_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PB Clock Divider Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pbdiv(&mut self) -> PBDIV_W<0> {
        PBDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Bus Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbclkcr](index.html) module"]
pub struct PBCLKCR_SPEC;
impl crate::RegisterSpec for PBCLKCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pbclkcr::R](R) reader structure"]
impl crate::Readable for PBCLKCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbclkcr::W](W) writer structure"]
impl crate::Writable for PBCLKCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PBCLKCR to value 0"]
impl crate::Resettable for PBCLKCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
