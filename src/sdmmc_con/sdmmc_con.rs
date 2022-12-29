#[doc = "Register `SDMMC_CON` reader"]
pub struct R(crate::R<SDMMC_CON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMMC_CON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMMC_CON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMMC_CON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDMMC_CON` writer"]
pub struct W(crate::W<SDMMC_CON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDMMC_CON_SPEC>;
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
impl From<crate::W<SDMMC_CON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDMMC_CON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WPSEL` reader - SDMMC Write Protection Input Multiplexer Control"]
pub type WPSEL_R = crate::BitReader<WPSEL_A>;
#[doc = "SDMMC Write Protection Input Multiplexer Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPSEL_A {
    #[doc = "0: P1.1 input pin selected"]
    VALUE1 = 0,
    #[doc = "1: Software bit WPVAL is selected"]
    VALUE2 = 1,
}
impl From<WPSEL_A> for bool {
    #[inline(always)]
    fn from(variant: WPSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl WPSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPSEL_A {
        match self.bits {
            false => WPSEL_A::VALUE1,
            true => WPSEL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WPSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WPSEL_A::VALUE2
    }
}
#[doc = "Field `WPSEL` writer - SDMMC Write Protection Input Multiplexer Control"]
pub type WPSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_CON_SPEC, WPSEL_A, O>;
impl<'a, const O: u8> WPSEL_W<'a, O> {
    #[doc = "P1.1 input pin selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WPSEL_A::VALUE1)
    }
    #[doc = "Software bit WPVAL is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WPSEL_A::VALUE2)
    }
}
#[doc = "Field `WPSVAL` reader - SDMMC Write Protect Software Control"]
pub type WPSVAL_R = crate::BitReader<WPSVAL_A>;
#[doc = "SDMMC Write Protect Software Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPSVAL_A {
    #[doc = "0: No write protection"]
    VALUE1 = 0,
    #[doc = "1: Write protection active"]
    VALUE2 = 1,
}
impl From<WPSVAL_A> for bool {
    #[inline(always)]
    fn from(variant: WPSVAL_A) -> Self {
        variant as u8 != 0
    }
}
impl WPSVAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPSVAL_A {
        match self.bits {
            false => WPSVAL_A::VALUE1,
            true => WPSVAL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WPSVAL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WPSVAL_A::VALUE2
    }
}
#[doc = "Field `WPSVAL` writer - SDMMC Write Protect Software Control"]
pub type WPSVAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_CON_SPEC, WPSVAL_A, O>;
impl<'a, const O: u8> WPSVAL_W<'a, O> {
    #[doc = "No write protection"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WPSVAL_A::VALUE1)
    }
    #[doc = "Write protection active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WPSVAL_A::VALUE2)
    }
}
#[doc = "Field `CDSEL` reader - SDMMC Card Detection Control"]
pub type CDSEL_R = crate::BitReader<CDSEL_A>;
#[doc = "SDMMC Card Detection Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDSEL_A {
    #[doc = "0: P1.10 input pin selected"]
    VALUE1 = 0,
    #[doc = "1: Software bit CDSVAL is selected"]
    VALUE2 = 1,
}
impl From<CDSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CDSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl CDSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDSEL_A {
        match self.bits {
            false => CDSEL_A::VALUE1,
            true => CDSEL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CDSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CDSEL_A::VALUE2
    }
}
#[doc = "Field `CDSEL` writer - SDMMC Card Detection Control"]
pub type CDSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_CON_SPEC, CDSEL_A, O>;
impl<'a, const O: u8> CDSEL_W<'a, O> {
    #[doc = "P1.10 input pin selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CDSEL_A::VALUE1)
    }
    #[doc = "Software bit CDSVAL is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CDSEL_A::VALUE2)
    }
}
#[doc = "Field `CDSVAL` reader - SDMMC Write Protect Software Control"]
pub type CDSVAL_R = crate::BitReader<CDSVAL_A>;
#[doc = "SDMMC Write Protect Software Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDSVAL_A {
    #[doc = "0: No card detected"]
    VALUE1 = 0,
    #[doc = "1: Card detected"]
    VALUE2 = 1,
}
impl From<CDSVAL_A> for bool {
    #[inline(always)]
    fn from(variant: CDSVAL_A) -> Self {
        variant as u8 != 0
    }
}
impl CDSVAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDSVAL_A {
        match self.bits {
            false => CDSVAL_A::VALUE1,
            true => CDSVAL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CDSVAL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CDSVAL_A::VALUE2
    }
}
#[doc = "Field `CDSVAL` writer - SDMMC Write Protect Software Control"]
pub type CDSVAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_CON_SPEC, CDSVAL_A, O>;
impl<'a, const O: u8> CDSVAL_W<'a, O> {
    #[doc = "No card detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CDSVAL_A::VALUE1)
    }
    #[doc = "Card detected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CDSVAL_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - SDMMC Write Protection Input Multiplexer Control"]
    #[inline(always)]
    pub fn wpsel(&self) -> WPSEL_R {
        WPSEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - SDMMC Write Protect Software Control"]
    #[inline(always)]
    pub fn wpsval(&self) -> WPSVAL_R {
        WPSVAL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - SDMMC Card Detection Control"]
    #[inline(always)]
    pub fn cdsel(&self) -> CDSEL_R {
        CDSEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - SDMMC Write Protect Software Control"]
    #[inline(always)]
    pub fn cdsval(&self) -> CDSVAL_R {
        CDSVAL_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SDMMC Write Protection Input Multiplexer Control"]
    #[inline(always)]
    #[must_use]
    pub fn wpsel(&mut self) -> WPSEL_W<0> {
        WPSEL_W::new(self)
    }
    #[doc = "Bit 4 - SDMMC Write Protect Software Control"]
    #[inline(always)]
    #[must_use]
    pub fn wpsval(&mut self) -> WPSVAL_W<4> {
        WPSVAL_W::new(self)
    }
    #[doc = "Bit 16 - SDMMC Card Detection Control"]
    #[inline(always)]
    #[must_use]
    pub fn cdsel(&mut self) -> CDSEL_W<16> {
        CDSEL_W::new(self)
    }
    #[doc = "Bit 20 - SDMMC Write Protect Software Control"]
    #[inline(always)]
    #[must_use]
    pub fn cdsval(&mut self) -> CDSVAL_W<20> {
        CDSVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDMMC Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_con](index.html) module"]
pub struct SDMMC_CON_SPEC;
impl crate::RegisterSpec for SDMMC_CON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdmmc_con::R](R) reader structure"]
impl crate::Readable for SDMMC_CON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdmmc_con::W](W) writer structure"]
impl crate::Writable for SDMMC_CON_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDMMC_CON to value 0"]
impl crate::Resettable for SDMMC_CON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
