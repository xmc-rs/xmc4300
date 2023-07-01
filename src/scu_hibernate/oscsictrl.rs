#[doc = "Register `OSCSICTRL` reader"]
pub struct R(crate::R<OSCSICTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSCSICTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSCSICTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSCSICTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSCSICTRL` writer"]
pub struct W(crate::W<OSCSICTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSCSICTRL_SPEC>;
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
impl From<crate::W<OSCSICTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSCSICTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWD` reader - Turn OFF the fOSI Clock Source"]
pub type PWD_R = crate::BitReader<PWD_A>;
#[doc = "Turn OFF the fOSI Clock Source\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWD_A {
    #[doc = "0: Enabled"]
    CONST_0 = 0,
    #[doc = "1: Disabled"]
    CONST_1 = 1,
}
impl From<PWD_A> for bool {
    #[inline(always)]
    fn from(variant: PWD_A) -> Self {
        variant as u8 != 0
    }
}
impl PWD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWD_A {
        match self.bits {
            false => PWD_A::CONST_0,
            true => PWD_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PWD_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PWD_A::CONST_1
    }
}
#[doc = "Field `PWD` writer - Turn OFF the fOSI Clock Source"]
pub type PWD_W<'a, const O: u8> = crate::BitWriter<'a, OSCSICTRL_SPEC, O, PWD_A>;
impl<'a, const O: u8> PWD_W<'a, O> {
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PWD_A::CONST_0)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PWD_A::CONST_1)
    }
}
impl R {
    #[doc = "Bit 0 - Turn OFF the fOSI Clock Source"]
    #[inline(always)]
    pub fn pwd(&self) -> PWD_R {
        PWD_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Turn OFF the fOSI Clock Source"]
    #[inline(always)]
    #[must_use]
    pub fn pwd(&mut self) -> PWD_W<0> {
        PWD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "fOSI Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oscsictrl](index.html) module"]
pub struct OSCSICTRL_SPEC;
impl crate::RegisterSpec for OSCSICTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oscsictrl::R](R) reader structure"]
impl crate::Readable for OSCSICTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oscsictrl::W](W) writer structure"]
impl crate::Writable for OSCSICTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSCSICTRL to value 0x01"]
impl crate::Resettable for OSCSICTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
