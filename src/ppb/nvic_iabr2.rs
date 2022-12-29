#[doc = "Register `NVIC_IABR2` reader"]
pub struct R(crate::R<NVIC_IABR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVIC_IABR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVIC_IABR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVIC_IABR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVIC_IABR2` writer"]
pub struct W(crate::W<NVIC_IABR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVIC_IABR2_SPEC>;
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
impl From<crate::W<NVIC_IABR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVIC_IABR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACTIVE` reader - Interrupt active flags:"]
pub type ACTIVE_R = crate::FieldReader<u32, ACTIVE_A>;
#[doc = "Interrupt active flags:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum ACTIVE_A {
    #[doc = "0: interrupt not active"]
    VALUE1 = 0,
    #[doc = "1: interrupt active"]
    VALUE2 = 1,
}
impl From<ACTIVE_A> for u32 {
    #[inline(always)]
    fn from(variant: ACTIVE_A) -> Self {
        variant as _
    }
}
impl ACTIVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ACTIVE_A> {
        match self.bits {
            0 => Some(ACTIVE_A::VALUE1),
            1 => Some(ACTIVE_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ACTIVE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ACTIVE_A::VALUE2
    }
}
#[doc = "Field `ACTIVE` writer - Interrupt active flags:"]
pub type ACTIVE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IABR2_SPEC, u32, ACTIVE_A, 32, O>;
impl<'a, const O: u8> ACTIVE_W<'a, O> {
    #[doc = "interrupt not active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ACTIVE_A::VALUE1)
    }
    #[doc = "interrupt active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ACTIVE_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt active flags:"]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt active flags:"]
    #[inline(always)]
    #[must_use]
    pub fn active(&mut self) -> ACTIVE_W<0> {
        ACTIVE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Active Bit Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_iabr2](index.html) module"]
pub struct NVIC_IABR2_SPEC;
impl crate::RegisterSpec for NVIC_IABR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvic_iabr2::R](R) reader structure"]
impl crate::Readable for NVIC_IABR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvic_iabr2::W](W) writer structure"]
impl crate::Writable for NVIC_IABR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NVIC_IABR2 to value 0"]
impl crate::Resettable for NVIC_IABR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
