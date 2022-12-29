#[doc = "Register `NVIC_ISER0` reader"]
pub struct R(crate::R<NVIC_ISER0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVIC_ISER0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVIC_ISER0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVIC_ISER0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVIC_ISER0` writer"]
pub struct W(crate::W<NVIC_ISER0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVIC_ISER0_SPEC>;
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
impl From<crate::W<NVIC_ISER0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVIC_ISER0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SETENA` reader - Interrupt set-enable bits"]
pub type SETENA_R = crate::FieldReader<u32, SETENA_A>;
#[doc = "Interrupt set-enable bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum SETENA_A {
    #[doc = "0: interrupt disabled"]
    VALUE3 = 0,
    #[doc = "1: interrupt enabled."]
    VALUE4 = 1,
}
impl From<SETENA_A> for u32 {
    #[inline(always)]
    fn from(variant: SETENA_A) -> Self {
        variant as _
    }
}
impl SETENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SETENA_A> {
        match self.bits {
            0 => Some(SETENA_A::VALUE3),
            1 => Some(SETENA_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SETENA_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SETENA_A::VALUE4
    }
}
#[doc = "Field `SETENA` writer - Interrupt set-enable bits"]
pub type SETENA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_ISER0_SPEC, u32, SETENA_A, 32, O>;
impl<'a, const O: u8> SETENA_W<'a, O> {
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SETENA_A::VALUE3)
    }
    #[doc = "interrupt enabled."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SETENA_A::VALUE4)
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt set-enable bits"]
    #[inline(always)]
    pub fn setena(&self) -> SETENA_R {
        SETENA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt set-enable bits"]
    #[inline(always)]
    #[must_use]
    pub fn setena(&mut self) -> SETENA_W<0> {
        SETENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Set-enable Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_iser0](index.html) module"]
pub struct NVIC_ISER0_SPEC;
impl crate::RegisterSpec for NVIC_ISER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvic_iser0::R](R) reader structure"]
impl crate::Readable for NVIC_ISER0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvic_iser0::W](W) writer structure"]
impl crate::Writable for NVIC_ISER0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NVIC_ISER0 to value 0"]
impl crate::Resettable for NVIC_ISER0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
