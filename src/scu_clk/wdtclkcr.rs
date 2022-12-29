#[doc = "Register `WDTCLKCR` reader"]
pub struct R(crate::R<WDTCLKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTCLKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTCLKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTCLKCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTCLKCR` writer"]
pub struct W(crate::W<WDTCLKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTCLKCR_SPEC>;
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
impl From<crate::W<WDTCLKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTCLKCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDTDIV` reader - WDT Clock Divider Value"]
pub type WDTDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WDTDIV` writer - WDT Clock Divider Value"]
pub type WDTDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WDTCLKCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `WDTSEL` reader - WDT Clock Selection Value"]
pub type WDTSEL_R = crate::FieldReader<u8, WDTSEL_A>;
#[doc = "WDT Clock Selection Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDTSEL_A {
    #[doc = "0: fOFI clock"]
    CONST_00 = 0,
    #[doc = "1: fSTDBY clock"]
    CONST_01 = 1,
    #[doc = "2: fPLL clock"]
    CONST_10 = 2,
}
impl From<WDTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: WDTSEL_A) -> Self {
        variant as _
    }
}
impl WDTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WDTSEL_A> {
        match self.bits {
            0 => Some(WDTSEL_A::CONST_00),
            1 => Some(WDTSEL_A::CONST_01),
            2 => Some(WDTSEL_A::CONST_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == WDTSEL_A::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == WDTSEL_A::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == WDTSEL_A::CONST_10
    }
}
#[doc = "Field `WDTSEL` writer - WDT Clock Selection Value"]
pub type WDTSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WDTCLKCR_SPEC, u8, WDTSEL_A, 2, O>;
impl<'a, const O: u8> WDTSEL_W<'a, O> {
    #[doc = "fOFI clock"]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut W {
        self.variant(WDTSEL_A::CONST_00)
    }
    #[doc = "fSTDBY clock"]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut W {
        self.variant(WDTSEL_A::CONST_01)
    }
    #[doc = "fPLL clock"]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut W {
        self.variant(WDTSEL_A::CONST_10)
    }
}
impl R {
    #[doc = "Bits 0:7 - WDT Clock Divider Value"]
    #[inline(always)]
    pub fn wdtdiv(&self) -> WDTDIV_R {
        WDTDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - WDT Clock Selection Value"]
    #[inline(always)]
    pub fn wdtsel(&self) -> WDTSEL_R {
        WDTSEL_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - WDT Clock Divider Value"]
    #[inline(always)]
    #[must_use]
    pub fn wdtdiv(&mut self) -> WDTDIV_W<0> {
        WDTDIV_W::new(self)
    }
    #[doc = "Bits 16:17 - WDT Clock Selection Value"]
    #[inline(always)]
    #[must_use]
    pub fn wdtsel(&mut self) -> WDTSEL_W<16> {
        WDTSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WDT Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtclkcr](index.html) module"]
pub struct WDTCLKCR_SPEC;
impl crate::RegisterSpec for WDTCLKCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdtclkcr::R](R) reader structure"]
impl crate::Readable for WDTCLKCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtclkcr::W](W) writer structure"]
impl crate::Writable for WDTCLKCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDTCLKCR to value 0"]
impl crate::Resettable for WDTCLKCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
