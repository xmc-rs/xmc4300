#[doc = "Register `CPUCLKCR` reader"]
pub struct R(crate::R<CPUCLKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPUCLKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPUCLKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPUCLKCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPUCLKCR` writer"]
pub struct W(crate::W<CPUCLKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPUCLKCR_SPEC>;
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
impl From<crate::W<CPUCLKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPUCLKCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPUDIV` reader - CPU Clock Divider Enable"]
pub type CPUDIV_R = crate::BitReader<CPUDIV_A>;
#[doc = "CPU Clock Divider Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPUDIV_A {
    #[doc = "0: fCPU = fSYS"]
    CONST_0 = 0,
    #[doc = "1: fCPU = fSYS / 2"]
    CONST_1 = 1,
}
impl From<CPUDIV_A> for bool {
    #[inline(always)]
    fn from(variant: CPUDIV_A) -> Self {
        variant as u8 != 0
    }
}
impl CPUDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPUDIV_A {
        match self.bits {
            false => CPUDIV_A::CONST_0,
            true => CPUDIV_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == CPUDIV_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == CPUDIV_A::CONST_1
    }
}
#[doc = "Field `CPUDIV` writer - CPU Clock Divider Enable"]
pub type CPUDIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPUCLKCR_SPEC, CPUDIV_A, O>;
impl<'a, const O: u8> CPUDIV_W<'a, O> {
    #[doc = "fCPU = fSYS"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(CPUDIV_A::CONST_0)
    }
    #[doc = "fCPU = fSYS / 2"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(CPUDIV_A::CONST_1)
    }
}
impl R {
    #[doc = "Bit 0 - CPU Clock Divider Enable"]
    #[inline(always)]
    pub fn cpudiv(&self) -> CPUDIV_R {
        CPUDIV_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU Clock Divider Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cpudiv(&mut self) -> CPUDIV_W<0> {
        CPUDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuclkcr](index.html) module"]
pub struct CPUCLKCR_SPEC;
impl crate::RegisterSpec for CPUCLKCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpuclkcr::R](R) reader structure"]
impl crate::Readable for CPUCLKCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpuclkcr::W](W) writer structure"]
impl crate::Writable for CPUCLKCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPUCLKCR to value 0"]
impl crate::Resettable for CPUCLKCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
