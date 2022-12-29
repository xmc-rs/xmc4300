#[doc = "Register `SCR` reader"]
pub struct R(crate::R<SCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCR` writer"]
pub struct W(crate::W<SCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCR_SPEC>;
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
impl From<crate::W<SCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLEEPONEXIT` reader - Sleep on Exit"]
pub type SLEEPONEXIT_R = crate::BitReader<SLEEPONEXIT_A>;
#[doc = "Sleep on Exit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLEEPONEXIT_A {
    #[doc = "0: do not sleep when returning to Thread mode."]
    VALUE1 = 0,
    #[doc = "1: enter sleep, or deep sleep, on return from an ISR."]
    VALUE2 = 1,
}
impl From<SLEEPONEXIT_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEPONEXIT_A) -> Self {
        variant as u8 != 0
    }
}
impl SLEEPONEXIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEPONEXIT_A {
        match self.bits {
            false => SLEEPONEXIT_A::VALUE1,
            true => SLEEPONEXIT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SLEEPONEXIT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SLEEPONEXIT_A::VALUE2
    }
}
#[doc = "Field `SLEEPONEXIT` writer - Sleep on Exit"]
pub type SLEEPONEXIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, SLEEPONEXIT_A, O>;
impl<'a, const O: u8> SLEEPONEXIT_W<'a, O> {
    #[doc = "do not sleep when returning to Thread mode."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SLEEPONEXIT_A::VALUE1)
    }
    #[doc = "enter sleep, or deep sleep, on return from an ISR."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SLEEPONEXIT_A::VALUE2)
    }
}
#[doc = "Field `SLEEPDEEP` reader - Sleep or Deep Sleep"]
pub type SLEEPDEEP_R = crate::BitReader<SLEEPDEEP_A>;
#[doc = "Sleep or Deep Sleep\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLEEPDEEP_A {
    #[doc = "0: sleep"]
    VALUE1 = 0,
    #[doc = "1: deep sleep"]
    VALUE2 = 1,
}
impl From<SLEEPDEEP_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEPDEEP_A) -> Self {
        variant as u8 != 0
    }
}
impl SLEEPDEEP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEPDEEP_A {
        match self.bits {
            false => SLEEPDEEP_A::VALUE1,
            true => SLEEPDEEP_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SLEEPDEEP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SLEEPDEEP_A::VALUE2
    }
}
#[doc = "Field `SLEEPDEEP` writer - Sleep or Deep Sleep"]
pub type SLEEPDEEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, SLEEPDEEP_A, O>;
impl<'a, const O: u8> SLEEPDEEP_W<'a, O> {
    #[doc = "sleep"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SLEEPDEEP_A::VALUE1)
    }
    #[doc = "deep sleep"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SLEEPDEEP_A::VALUE2)
    }
}
#[doc = "Field `SEVONPEND` reader - Send Event on Pending bit:"]
pub type SEVONPEND_R = crate::BitReader<SEVONPEND_A>;
#[doc = "Send Event on Pending bit:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEVONPEND_A {
    #[doc = "0: only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded"]
    VALUE1 = 0,
    #[doc = "1: enabled events and all interrupts, including disabled interrupts, can wakeup the processor."]
    VALUE2 = 1,
}
impl From<SEVONPEND_A> for bool {
    #[inline(always)]
    fn from(variant: SEVONPEND_A) -> Self {
        variant as u8 != 0
    }
}
impl SEVONPEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEVONPEND_A {
        match self.bits {
            false => SEVONPEND_A::VALUE1,
            true => SEVONPEND_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SEVONPEND_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SEVONPEND_A::VALUE2
    }
}
#[doc = "Field `SEVONPEND` writer - Send Event on Pending bit:"]
pub type SEVONPEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, SEVONPEND_A, O>;
impl<'a, const O: u8> SEVONPEND_W<'a, O> {
    #[doc = "only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SEVONPEND_A::VALUE1)
    }
    #[doc = "enabled events and all interrupts, including disabled interrupts, can wakeup the processor."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SEVONPEND_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 1 - Sleep on Exit"]
    #[inline(always)]
    pub fn sleeponexit(&self) -> SLEEPONEXIT_R {
        SLEEPONEXIT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sleep or Deep Sleep"]
    #[inline(always)]
    pub fn sleepdeep(&self) -> SLEEPDEEP_R {
        SLEEPDEEP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Send Event on Pending bit:"]
    #[inline(always)]
    pub fn sevonpend(&self) -> SEVONPEND_R {
        SEVONPEND_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Sleep on Exit"]
    #[inline(always)]
    #[must_use]
    pub fn sleeponexit(&mut self) -> SLEEPONEXIT_W<1> {
        SLEEPONEXIT_W::new(self)
    }
    #[doc = "Bit 2 - Sleep or Deep Sleep"]
    #[inline(always)]
    #[must_use]
    pub fn sleepdeep(&mut self) -> SLEEPDEEP_W<2> {
        SLEEPDEEP_W::new(self)
    }
    #[doc = "Bit 4 - Send Event on Pending bit:"]
    #[inline(always)]
    #[must_use]
    pub fn sevonpend(&mut self) -> SEVONPEND_W<4> {
        SEVONPEND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](index.html) module"]
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scr::R](R) reader structure"]
impl crate::Readable for SCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scr::W](W) writer structure"]
impl crate::Writable for SCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for SCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
