#[doc = "Register `RSTSET` writer"]
pub type W = crate::W<RstsetSpec>;
#[doc = "Set Hibernate Wake-up Reset Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hibwk {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Assert reset status bit"]
    Const1 = 1,
}
impl From<Hibwk> for bool {
    #[inline(always)]
    fn from(variant: Hibwk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIBWK` writer - Set Hibernate Wake-up Reset Status"]
pub type HibwkW<'a, REG> = crate::BitWriter<'a, REG, Hibwk>;
impl<'a, REG> HibwkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Hibwk::Const0)
    }
    #[doc = "Assert reset status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Hibwk::Const1)
    }
}
#[doc = "Set Hibernate Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hibrs {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Assert reset"]
    Const1 = 1,
}
impl From<Hibrs> for bool {
    #[inline(always)]
    fn from(variant: Hibrs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIBRS` writer - Set Hibernate Reset"]
pub type HibrsW<'a, REG> = crate::BitWriter<'a, REG, Hibrs>;
impl<'a, REG> HibrsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Hibrs::Const0)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Hibrs::Const1)
    }
}
#[doc = "Enable Lockup Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcken {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Enable reset when Lockup gets asserted"]
    Const1 = 1,
}
impl From<Lcken> for bool {
    #[inline(always)]
    fn from(variant: Lcken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCKEN` writer - Enable Lockup Reset"]
pub type LckenW<'a, REG> = crate::BitWriter<'a, REG, Lcken>;
impl<'a, REG> LckenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcken::Const0)
    }
    #[doc = "Enable reset when Lockup gets asserted"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcken::Const1)
    }
}
#[doc = "ECAT0 Reset Status Information\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecat0rs {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Assert reset status bit"]
    Const1 = 1,
}
impl From<Ecat0rs> for bool {
    #[inline(always)]
    fn from(variant: Ecat0rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECAT0RS` writer - ECAT0 Reset Status Information"]
pub type Ecat0rsW<'a, REG> = crate::BitWriter<'a, REG, Ecat0rs>;
impl<'a, REG> Ecat0rsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ecat0rs::Const0)
    }
    #[doc = "Assert reset status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ecat0rs::Const1)
    }
}
impl W {
    #[doc = "Bit 8 - Set Hibernate Wake-up Reset Status"]
    #[inline(always)]
    #[must_use]
    pub fn hibwk(&mut self) -> HibwkW<RstsetSpec> {
        HibwkW::new(self, 8)
    }
    #[doc = "Bit 9 - Set Hibernate Reset"]
    #[inline(always)]
    #[must_use]
    pub fn hibrs(&mut self) -> HibrsW<RstsetSpec> {
        HibrsW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Lockup Reset"]
    #[inline(always)]
    #[must_use]
    pub fn lcken(&mut self) -> LckenW<RstsetSpec> {
        LckenW::new(self, 10)
    }
    #[doc = "Bit 12 - ECAT0 Reset Status Information"]
    #[inline(always)]
    #[must_use]
    pub fn ecat0rs(&mut self) -> Ecat0rsW<RstsetSpec> {
        Ecat0rsW::new(self, 12)
    }
}
#[doc = "RCU Reset Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstsetSpec;
impl crate::RegisterSpec for RstsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rstset::W`](W) writer structure"]
impl crate::Writable for RstsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSTSET to value 0"]
impl crate::Resettable for RstsetSpec {
    const RESET_VALUE: u32 = 0;
}
