#[doc = "Register `SCR` reader"]
pub type R = crate::R<SCR_SPEC>;
#[doc = "Register `SCR` writer"]
pub type W = crate::W<SCR_SPEC>;
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
    pub const fn variant(&self) -> SLEEPONEXIT_A {
        match self.bits {
            false => SLEEPONEXIT_A::VALUE1,
            true => SLEEPONEXIT_A::VALUE2,
        }
    }
    #[doc = "do not sleep when returning to Thread mode."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SLEEPONEXIT_A::VALUE1
    }
    #[doc = "enter sleep, or deep sleep, on return from an ISR."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SLEEPONEXIT_A::VALUE2
    }
}
#[doc = "Field `SLEEPONEXIT` writer - Sleep on Exit"]
pub type SLEEPONEXIT_W<'a, REG> = crate::BitWriter<'a, REG, SLEEPONEXIT_A>;
impl<'a, REG> SLEEPONEXIT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "do not sleep when returning to Thread mode."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SLEEPONEXIT_A::VALUE1)
    }
    #[doc = "enter sleep, or deep sleep, on return from an ISR."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> SLEEPDEEP_A {
        match self.bits {
            false => SLEEPDEEP_A::VALUE1,
            true => SLEEPDEEP_A::VALUE2,
        }
    }
    #[doc = "sleep"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SLEEPDEEP_A::VALUE1
    }
    #[doc = "deep sleep"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SLEEPDEEP_A::VALUE2
    }
}
#[doc = "Field `SLEEPDEEP` writer - Sleep or Deep Sleep"]
pub type SLEEPDEEP_W<'a, REG> = crate::BitWriter<'a, REG, SLEEPDEEP_A>;
impl<'a, REG> SLEEPDEEP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "sleep"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SLEEPDEEP_A::VALUE1)
    }
    #[doc = "deep sleep"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> SEVONPEND_A {
        match self.bits {
            false => SEVONPEND_A::VALUE1,
            true => SEVONPEND_A::VALUE2,
        }
    }
    #[doc = "only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SEVONPEND_A::VALUE1
    }
    #[doc = "enabled events and all interrupts, including disabled interrupts, can wakeup the processor."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SEVONPEND_A::VALUE2
    }
}
#[doc = "Field `SEVONPEND` writer - Send Event on Pending bit:"]
pub type SEVONPEND_W<'a, REG> = crate::BitWriter<'a, REG, SEVONPEND_A>;
impl<'a, REG> SEVONPEND_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SEVONPEND_A::VALUE1)
    }
    #[doc = "enabled events and all interrupts, including disabled interrupts, can wakeup the processor."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
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
    pub fn sleeponexit(&mut self) -> SLEEPONEXIT_W<SCR_SPEC> {
        SLEEPONEXIT_W::new(self, 1)
    }
    #[doc = "Bit 2 - Sleep or Deep Sleep"]
    #[inline(always)]
    #[must_use]
    pub fn sleepdeep(&mut self) -> SLEEPDEEP_W<SCR_SPEC> {
        SLEEPDEEP_W::new(self, 2)
    }
    #[doc = "Bit 4 - Send Event on Pending bit:"]
    #[inline(always)]
    #[must_use]
    pub fn sevonpend(&mut self) -> SEVONPEND_W<SCR_SPEC> {
        SEVONPEND_W::new(self, 4)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "System Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scr::R`](R) reader structure"]
impl crate::Readable for SCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scr::W`](W) writer structure"]
impl crate::Writable for SCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for SCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
