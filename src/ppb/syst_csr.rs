#[doc = "Register `SYST_CSR` reader"]
pub type R = crate::R<SystCsrSpec>;
#[doc = "Register `SYST_CSR` writer"]
pub type W = crate::W<SystCsrSpec>;
#[doc = "Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: counter disabled"]
    Value1 = 0,
    #[doc = "1: counter enabled."]
    Value2 = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Enable"]
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            false => Enable::Value1,
            true => Enable::Value2,
        }
    }
    #[doc = "counter disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Enable::Value1
    }
    #[doc = "counter enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Enable::Value2
    }
}
#[doc = "Field `ENABLE` writer - Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "counter disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Value1)
    }
    #[doc = "counter enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Value2)
    }
}
#[doc = "Tick Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tickint {
    #[doc = "0: counting down to zero does not assert the SysTick exception request"]
    Value1 = 0,
    #[doc = "1: counting down to zero to asserts the SysTick exception request."]
    Value2 = 1,
}
impl From<Tickint> for bool {
    #[inline(always)]
    fn from(variant: Tickint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TICKINT` reader - Tick Interrupt Enable"]
pub type TickintR = crate::BitReader<Tickint>;
impl TickintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tickint {
        match self.bits {
            false => Tickint::Value1,
            true => Tickint::Value2,
        }
    }
    #[doc = "counting down to zero does not assert the SysTick exception request"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Tickint::Value1
    }
    #[doc = "counting down to zero to asserts the SysTick exception request."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Tickint::Value2
    }
}
#[doc = "Field `TICKINT` writer - Tick Interrupt Enable"]
pub type TickintW<'a, REG> = crate::BitWriter<'a, REG, Tickint>;
impl<'a, REG> TickintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "counting down to zero does not assert the SysTick exception request"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Tickint::Value1)
    }
    #[doc = "counting down to zero to asserts the SysTick exception request."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Tickint::Value2)
    }
}
#[doc = "Indicates the clock source:\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clksource {
    #[doc = "0: external clock"]
    Value1 = 0,
    #[doc = "1: processor clock."]
    Value2 = 1,
}
impl From<Clksource> for bool {
    #[inline(always)]
    fn from(variant: Clksource) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKSOURCE` reader - Indicates the clock source:"]
pub type ClksourceR = crate::BitReader<Clksource>;
impl ClksourceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clksource {
        match self.bits {
            false => Clksource::Value1,
            true => Clksource::Value2,
        }
    }
    #[doc = "external clock"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Clksource::Value1
    }
    #[doc = "processor clock."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Clksource::Value2
    }
}
#[doc = "Field `CLKSOURCE` writer - Indicates the clock source:"]
pub type ClksourceW<'a, REG> = crate::BitWriter<'a, REG, Clksource>;
impl<'a, REG> ClksourceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "external clock"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Clksource::Value1)
    }
    #[doc = "processor clock."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Clksource::Value2)
    }
}
#[doc = "Field `COUNTFLAG` reader - Counter Flag"]
pub type CountflagR = crate::BitReader;
#[doc = "Field `COUNTFLAG` writer - Counter Flag"]
pub type CountflagW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tick Interrupt Enable"]
    #[inline(always)]
    pub fn tickint(&self) -> TickintR {
        TickintR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates the clock source:"]
    #[inline(always)]
    pub fn clksource(&self) -> ClksourceR {
        ClksourceR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - Counter Flag"]
    #[inline(always)]
    pub fn countflag(&self) -> CountflagR {
        CountflagR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<SystCsrSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Tick Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tickint(&mut self) -> TickintW<SystCsrSpec> {
        TickintW::new(self, 1)
    }
    #[doc = "Bit 2 - Indicates the clock source:"]
    #[inline(always)]
    #[must_use]
    pub fn clksource(&mut self) -> ClksourceW<SystCsrSpec> {
        ClksourceW::new(self, 2)
    }
    #[doc = "Bit 16 - Counter Flag"]
    #[inline(always)]
    #[must_use]
    pub fn countflag(&mut self) -> CountflagW<SystCsrSpec> {
        CountflagW::new(self, 16)
    }
}
#[doc = "SysTick Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syst_csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syst_csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SystCsrSpec;
impl crate::RegisterSpec for SystCsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syst_csr::R`](R) reader structure"]
impl crate::Readable for SystCsrSpec {}
#[doc = "`write(|w| ..)` method takes [`syst_csr::W`](W) writer structure"]
impl crate::Writable for SystCsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYST_CSR to value 0x04"]
impl crate::Resettable for SystCsrSpec {
    const RESET_VALUE: u32 = 0x04;
}
