#[doc = "Register `CCUCLKCR` reader"]
pub struct R(crate::R<CCUCLKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCUCLKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCUCLKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCUCLKCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCUCLKCR` writer"]
pub struct W(crate::W<CCUCLKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCUCLKCR_SPEC>;
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
impl From<crate::W<CCUCLKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCUCLKCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CCU Clock Divider Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCUDIV_A {
    #[doc = "0: fCCU = fSYS"]
    CONST_0 = 0,
    #[doc = "1: fCCU = fSYS / 2"]
    CONST_1 = 1,
}
impl From<CCUDIV_A> for bool {
    #[inline(always)]
    fn from(variant: CCUDIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCUDIV` reader - CCU Clock Divider Enable"]
pub struct CCUDIV_R(crate::FieldReader<bool, CCUDIV_A>);
impl CCUDIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCUDIV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCUDIV_A {
        match self.bits {
            false => CCUDIV_A::CONST_0,
            true => CCUDIV_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == CCUDIV_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == CCUDIV_A::CONST_1
    }
}
impl core::ops::Deref for CCUDIV_R {
    type Target = crate::FieldReader<bool, CCUDIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCUDIV` writer - CCU Clock Divider Enable"]
pub struct CCUDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CCUDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCUDIV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "fCCU = fSYS"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(CCUDIV_A::CONST_0)
    }
    #[doc = "fCCU = fSYS / 2"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(CCUDIV_A::CONST_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - CCU Clock Divider Enable"]
    #[inline(always)]
    pub fn ccudiv(&self) -> CCUDIV_R {
        CCUDIV_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CCU Clock Divider Enable"]
    #[inline(always)]
    pub fn ccudiv(&mut self) -> CCUDIV_W {
        CCUDIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CCU Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccuclkcr](index.html) module"]
pub struct CCUCLKCR_SPEC;
impl crate::RegisterSpec for CCUCLKCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccuclkcr::R](R) reader structure"]
impl crate::Readable for CCUCLKCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccuclkcr::W](W) writer structure"]
impl crate::Writable for CCUCLKCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCUCLKCR to value 0"]
impl crate::Resettable for CCUCLKCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
