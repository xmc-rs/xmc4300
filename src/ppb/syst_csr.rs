#[doc = "Register `SYST_CSR` reader"]
pub type R = crate::R<SYST_CSR_SPEC>;
#[doc = "Register `SYST_CSR` writer"]
pub type W = crate::W<SYST_CSR_SPEC>;
#[doc = "Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE_A {
    #[doc = "0: counter disabled"]
    VALUE1 = 0,
    #[doc = "1: counter enabled."]
    VALUE2 = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader<ENABLE_A>;
impl ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::VALUE1,
            true => ENABLE_A::VALUE2,
        }
    }
    #[doc = "counter disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ENABLE_A::VALUE1
    }
    #[doc = "counter enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ENABLE_A::VALUE2
    }
}
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, ENABLE_A>;
impl<'a, REG> ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "counter disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLE_A::VALUE1)
    }
    #[doc = "counter enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLE_A::VALUE2)
    }
}
#[doc = "Tick Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TICKINT_A {
    #[doc = "0: counting down to zero does not assert the SysTick exception request"]
    VALUE1 = 0,
    #[doc = "1: counting down to zero to asserts the SysTick exception request."]
    VALUE2 = 1,
}
impl From<TICKINT_A> for bool {
    #[inline(always)]
    fn from(variant: TICKINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TICKINT` reader - Tick Interrupt Enable"]
pub type TICKINT_R = crate::BitReader<TICKINT_A>;
impl TICKINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TICKINT_A {
        match self.bits {
            false => TICKINT_A::VALUE1,
            true => TICKINT_A::VALUE2,
        }
    }
    #[doc = "counting down to zero does not assert the SysTick exception request"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TICKINT_A::VALUE1
    }
    #[doc = "counting down to zero to asserts the SysTick exception request."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TICKINT_A::VALUE2
    }
}
#[doc = "Field `TICKINT` writer - Tick Interrupt Enable"]
pub type TICKINT_W<'a, REG> = crate::BitWriter<'a, REG, TICKINT_A>;
impl<'a, REG> TICKINT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "counting down to zero does not assert the SysTick exception request"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TICKINT_A::VALUE1)
    }
    #[doc = "counting down to zero to asserts the SysTick exception request."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TICKINT_A::VALUE2)
    }
}
#[doc = "Indicates the clock source:\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKSOURCE_A {
    #[doc = "0: external clock"]
    VALUE1 = 0,
    #[doc = "1: processor clock."]
    VALUE2 = 1,
}
impl From<CLKSOURCE_A> for bool {
    #[inline(always)]
    fn from(variant: CLKSOURCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKSOURCE` reader - Indicates the clock source:"]
pub type CLKSOURCE_R = crate::BitReader<CLKSOURCE_A>;
impl CLKSOURCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLKSOURCE_A {
        match self.bits {
            false => CLKSOURCE_A::VALUE1,
            true => CLKSOURCE_A::VALUE2,
        }
    }
    #[doc = "external clock"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CLKSOURCE_A::VALUE1
    }
    #[doc = "processor clock."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CLKSOURCE_A::VALUE2
    }
}
#[doc = "Field `CLKSOURCE` writer - Indicates the clock source:"]
pub type CLKSOURCE_W<'a, REG> = crate::BitWriter<'a, REG, CLKSOURCE_A>;
impl<'a, REG> CLKSOURCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "external clock"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSOURCE_A::VALUE1)
    }
    #[doc = "processor clock."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSOURCE_A::VALUE2)
    }
}
#[doc = "Field `COUNTFLAG` reader - Counter Flag"]
pub type COUNTFLAG_R = crate::BitReader;
#[doc = "Field `COUNTFLAG` writer - Counter Flag"]
pub type COUNTFLAG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tick Interrupt Enable"]
    #[inline(always)]
    pub fn tickint(&self) -> TICKINT_R {
        TICKINT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates the clock source:"]
    #[inline(always)]
    pub fn clksource(&self) -> CLKSOURCE_R {
        CLKSOURCE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - Counter Flag"]
    #[inline(always)]
    pub fn countflag(&self) -> COUNTFLAG_R {
        COUNTFLAG_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<SYST_CSR_SPEC> {
        ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Tick Interrupt Enable"]
    #[inline(always)]
    pub fn tickint(&mut self) -> TICKINT_W<SYST_CSR_SPEC> {
        TICKINT_W::new(self, 1)
    }
    #[doc = "Bit 2 - Indicates the clock source:"]
    #[inline(always)]
    pub fn clksource(&mut self) -> CLKSOURCE_W<SYST_CSR_SPEC> {
        CLKSOURCE_W::new(self, 2)
    }
    #[doc = "Bit 16 - Counter Flag"]
    #[inline(always)]
    pub fn countflag(&mut self) -> COUNTFLAG_W<SYST_CSR_SPEC> {
        COUNTFLAG_W::new(self, 16)
    }
}
#[doc = "SysTick Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syst_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syst_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYST_CSR_SPEC;
impl crate::RegisterSpec for SYST_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syst_csr::R`](R) reader structure"]
impl crate::Readable for SYST_CSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`syst_csr::W`](W) writer structure"]
impl crate::Writable for SYST_CSR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYST_CSR to value 0x04"]
impl crate::Resettable for SYST_CSR_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
