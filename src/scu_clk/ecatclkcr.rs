#[doc = "Register `ECATCLKCR` reader"]
pub struct R(crate::R<ECATCLKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECATCLKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECATCLKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECATCLKCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECATCLKCR` writer"]
pub struct W(crate::W<ECATCLKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECATCLKCR_SPEC>;
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
impl From<crate::W<ECATCLKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECATCLKCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ECADIV` reader - EtherCAT Clock Divider Value"]
pub type ECADIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ECADIV` writer - EtherCAT Clock Divider Value"]
pub type ECADIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ECATCLKCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `ECATSEL` reader - EtherCAT Clock Selection Value"]
pub type ECATSEL_R = crate::BitReader<ECATSEL_A>;
#[doc = "EtherCAT Clock Selection Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECATSEL_A {
    #[doc = "0: fPLLUSB clock"]
    CONST_0 = 0,
    #[doc = "1: fPLL clock"]
    CONST_1 = 1,
}
impl From<ECATSEL_A> for bool {
    #[inline(always)]
    fn from(variant: ECATSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl ECATSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECATSEL_A {
        match self.bits {
            false => ECATSEL_A::CONST_0,
            true => ECATSEL_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == ECATSEL_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == ECATSEL_A::CONST_1
    }
}
#[doc = "Field `ECATSEL` writer - EtherCAT Clock Selection Value"]
pub type ECATSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECATCLKCR_SPEC, ECATSEL_A, O>;
impl<'a, const O: u8> ECATSEL_W<'a, O> {
    #[doc = "fPLLUSB clock"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ECATSEL_A::CONST_0)
    }
    #[doc = "fPLL clock"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ECATSEL_A::CONST_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - EtherCAT Clock Divider Value"]
    #[inline(always)]
    pub fn ecadiv(&self) -> ECADIV_R {
        ECADIV_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 16 - EtherCAT Clock Selection Value"]
    #[inline(always)]
    pub fn ecatsel(&self) -> ECATSEL_R {
        ECATSEL_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - EtherCAT Clock Divider Value"]
    #[inline(always)]
    #[must_use]
    pub fn ecadiv(&mut self) -> ECADIV_W<0> {
        ECADIV_W::new(self)
    }
    #[doc = "Bit 16 - EtherCAT Clock Selection Value"]
    #[inline(always)]
    #[must_use]
    pub fn ecatsel(&mut self) -> ECATSEL_W<16> {
        ECATSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EtherCAT Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecatclkcr](index.html) module"]
pub struct ECATCLKCR_SPEC;
impl crate::RegisterSpec for ECATCLKCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecatclkcr::R](R) reader structure"]
impl crate::Readable for ECATCLKCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ecatclkcr::W](W) writer structure"]
impl crate::Writable for ECATCLKCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECATCLKCR to value 0"]
impl crate::Resettable for ECATCLKCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
