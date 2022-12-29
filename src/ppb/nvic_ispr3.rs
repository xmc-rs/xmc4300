#[doc = "Register `NVIC_ISPR3` reader"]
pub struct R(crate::R<NVIC_ISPR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVIC_ISPR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVIC_ISPR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVIC_ISPR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVIC_ISPR3` writer"]
pub struct W(crate::W<NVIC_ISPR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVIC_ISPR3_SPEC>;
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
impl From<crate::W<NVIC_ISPR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVIC_ISPR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SETPEND` reader - Interrupt set-pending bits."]
pub type SETPEND_R = crate::FieldReader<u32, SETPEND_A>;
#[doc = "Interrupt set-pending bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum SETPEND_A {
    #[doc = "0: interrupt is not pending"]
    VALUE3 = 0,
    #[doc = "1: interrupt is pending."]
    VALUE4 = 1,
}
impl From<SETPEND_A> for u32 {
    #[inline(always)]
    fn from(variant: SETPEND_A) -> Self {
        variant as _
    }
}
impl SETPEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SETPEND_A> {
        match self.bits {
            0 => Some(SETPEND_A::VALUE3),
            1 => Some(SETPEND_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SETPEND_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SETPEND_A::VALUE4
    }
}
#[doc = "Field `SETPEND` writer - Interrupt set-pending bits."]
pub type SETPEND_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_ISPR3_SPEC, u32, SETPEND_A, 32, O>;
impl<'a, const O: u8> SETPEND_W<'a, O> {
    #[doc = "interrupt is not pending"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SETPEND_A::VALUE3)
    }
    #[doc = "interrupt is pending."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SETPEND_A::VALUE4)
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt set-pending bits."]
    #[inline(always)]
    pub fn setpend(&self) -> SETPEND_R {
        SETPEND_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt set-pending bits."]
    #[inline(always)]
    #[must_use]
    pub fn setpend(&mut self) -> SETPEND_W<0> {
        SETPEND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Set-pending Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ispr3](index.html) module"]
pub struct NVIC_ISPR3_SPEC;
impl crate::RegisterSpec for NVIC_ISPR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvic_ispr3::R](R) reader structure"]
impl crate::Readable for NVIC_ISPR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvic_ispr3::W](W) writer structure"]
impl crate::Writable for NVIC_ISPR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NVIC_ISPR3 to value 0"]
impl crate::Resettable for NVIC_ISPR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
