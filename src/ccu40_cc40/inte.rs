#[doc = "Register `INTE` reader"]
pub type R = crate::R<InteSpec>;
#[doc = "Register `INTE` writer"]
pub type W = crate::W<InteSpec>;
#[doc = "Period match while counting up enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pme {
    #[doc = "0: Period Match interrupt is disabled"]
    Value1 = 0,
    #[doc = "1: Period Match interrupt is enabled"]
    Value2 = 1,
}
impl From<Pme> for bool {
    #[inline(always)]
    fn from(variant: Pme) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PME` reader - Period match while counting up enable"]
pub type PmeR = crate::BitReader<Pme>;
impl PmeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pme {
        match self.bits {
            false => Pme::Value1,
            true => Pme::Value2,
        }
    }
    #[doc = "Period Match interrupt is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pme::Value1
    }
    #[doc = "Period Match interrupt is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pme::Value2
    }
}
#[doc = "Field `PME` writer - Period match while counting up enable"]
pub type PmeW<'a, REG> = crate::BitWriter<'a, REG, Pme>;
impl<'a, REG> PmeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Period Match interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pme::Value1)
    }
    #[doc = "Period Match interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pme::Value2)
    }
}
#[doc = "One match while counting down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ome {
    #[doc = "0: One Match interrupt is disabled"]
    Value1 = 0,
    #[doc = "1: One Match interrupt is enabled"]
    Value2 = 1,
}
impl From<Ome> for bool {
    #[inline(always)]
    fn from(variant: Ome) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OME` reader - One match while counting down enable"]
pub type OmeR = crate::BitReader<Ome>;
impl OmeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ome {
        match self.bits {
            false => Ome::Value1,
            true => Ome::Value2,
        }
    }
    #[doc = "One Match interrupt is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ome::Value1
    }
    #[doc = "One Match interrupt is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ome::Value2
    }
}
#[doc = "Field `OME` writer - One match while counting down enable"]
pub type OmeW<'a, REG> = crate::BitWriter<'a, REG, Ome>;
impl<'a, REG> OmeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "One Match interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ome::Value1)
    }
    #[doc = "One Match interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ome::Value2)
    }
}
#[doc = "Compare match while counting up enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmue {
    #[doc = "0: Compare Match while counting up interrupt is disabled"]
    Value1 = 0,
    #[doc = "1: Compare Match while counting up interrupt is enabled"]
    Value2 = 1,
}
impl From<Cmue> for bool {
    #[inline(always)]
    fn from(variant: Cmue) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMUE` reader - Compare match while counting up enable"]
pub type CmueR = crate::BitReader<Cmue>;
impl CmueR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmue {
        match self.bits {
            false => Cmue::Value1,
            true => Cmue::Value2,
        }
    }
    #[doc = "Compare Match while counting up interrupt is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cmue::Value1
    }
    #[doc = "Compare Match while counting up interrupt is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cmue::Value2
    }
}
#[doc = "Field `CMUE` writer - Compare match while counting up enable"]
pub type CmueW<'a, REG> = crate::BitWriter<'a, REG, Cmue>;
impl<'a, REG> CmueW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Compare Match while counting up interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmue::Value1)
    }
    #[doc = "Compare Match while counting up interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cmue::Value2)
    }
}
#[doc = "Compare match while counting down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmde {
    #[doc = "0: Compare Match while counting down interrupt is disabled"]
    Value1 = 0,
    #[doc = "1: Compare Match while counting down interrupt is enabled"]
    Value2 = 1,
}
impl From<Cmde> for bool {
    #[inline(always)]
    fn from(variant: Cmde) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDE` reader - Compare match while counting down enable"]
pub type CmdeR = crate::BitReader<Cmde>;
impl CmdeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmde {
        match self.bits {
            false => Cmde::Value1,
            true => Cmde::Value2,
        }
    }
    #[doc = "Compare Match while counting down interrupt is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cmde::Value1
    }
    #[doc = "Compare Match while counting down interrupt is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cmde::Value2
    }
}
#[doc = "Field `CMDE` writer - Compare match while counting down enable"]
pub type CmdeW<'a, REG> = crate::BitWriter<'a, REG, Cmde>;
impl<'a, REG> CmdeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Compare Match while counting down interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmde::Value1)
    }
    #[doc = "Compare Match while counting down interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cmde::Value2)
    }
}
#[doc = "Event 0 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum E0ae {
    #[doc = "0: Event 0 detection interrupt is disabled"]
    Value1 = 0,
    #[doc = "1: Event 0 detection interrupt is enabled"]
    Value2 = 1,
}
impl From<E0ae> for bool {
    #[inline(always)]
    fn from(variant: E0ae) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `E0AE` reader - Event 0 interrupt enable"]
pub type E0aeR = crate::BitReader<E0ae>;
impl E0aeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> E0ae {
        match self.bits {
            false => E0ae::Value1,
            true => E0ae::Value2,
        }
    }
    #[doc = "Event 0 detection interrupt is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == E0ae::Value1
    }
    #[doc = "Event 0 detection interrupt is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == E0ae::Value2
    }
}
#[doc = "Field `E0AE` writer - Event 0 interrupt enable"]
pub type E0aeW<'a, REG> = crate::BitWriter<'a, REG, E0ae>;
impl<'a, REG> E0aeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event 0 detection interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(E0ae::Value1)
    }
    #[doc = "Event 0 detection interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(E0ae::Value2)
    }
}
#[doc = "Event 1 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum E1ae {
    #[doc = "0: Event 1 detection interrupt is disabled"]
    Value1 = 0,
    #[doc = "1: Event 1 detection interrupt is enabled"]
    Value2 = 1,
}
impl From<E1ae> for bool {
    #[inline(always)]
    fn from(variant: E1ae) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `E1AE` reader - Event 1 interrupt enable"]
pub type E1aeR = crate::BitReader<E1ae>;
impl E1aeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> E1ae {
        match self.bits {
            false => E1ae::Value1,
            true => E1ae::Value2,
        }
    }
    #[doc = "Event 1 detection interrupt is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == E1ae::Value1
    }
    #[doc = "Event 1 detection interrupt is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == E1ae::Value2
    }
}
#[doc = "Field `E1AE` writer - Event 1 interrupt enable"]
pub type E1aeW<'a, REG> = crate::BitWriter<'a, REG, E1ae>;
impl<'a, REG> E1aeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event 1 detection interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(E1ae::Value1)
    }
    #[doc = "Event 1 detection interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(E1ae::Value2)
    }
}
#[doc = "Event 2 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum E2ae {
    #[doc = "0: Event 2 detection interrupt is disabled"]
    Value1 = 0,
    #[doc = "1: Event 2 detection interrupt is enabled"]
    Value2 = 1,
}
impl From<E2ae> for bool {
    #[inline(always)]
    fn from(variant: E2ae) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `E2AE` reader - Event 2 interrupt enable"]
pub type E2aeR = crate::BitReader<E2ae>;
impl E2aeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> E2ae {
        match self.bits {
            false => E2ae::Value1,
            true => E2ae::Value2,
        }
    }
    #[doc = "Event 2 detection interrupt is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == E2ae::Value1
    }
    #[doc = "Event 2 detection interrupt is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == E2ae::Value2
    }
}
#[doc = "Field `E2AE` writer - Event 2 interrupt enable"]
pub type E2aeW<'a, REG> = crate::BitWriter<'a, REG, E2ae>;
impl<'a, REG> E2aeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event 2 detection interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(E2ae::Value1)
    }
    #[doc = "Event 2 detection interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(E2ae::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Period match while counting up enable"]
    #[inline(always)]
    pub fn pme(&self) -> PmeR {
        PmeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - One match while counting down enable"]
    #[inline(always)]
    pub fn ome(&self) -> OmeR {
        OmeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Compare match while counting up enable"]
    #[inline(always)]
    pub fn cmue(&self) -> CmueR {
        CmueR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare match while counting down enable"]
    #[inline(always)]
    pub fn cmde(&self) -> CmdeR {
        CmdeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Event 0 interrupt enable"]
    #[inline(always)]
    pub fn e0ae(&self) -> E0aeR {
        E0aeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event 1 interrupt enable"]
    #[inline(always)]
    pub fn e1ae(&self) -> E1aeR {
        E1aeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Event 2 interrupt enable"]
    #[inline(always)]
    pub fn e2ae(&self) -> E2aeR {
        E2aeR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Period match while counting up enable"]
    #[inline(always)]
    #[must_use]
    pub fn pme(&mut self) -> PmeW<InteSpec> {
        PmeW::new(self, 0)
    }
    #[doc = "Bit 1 - One match while counting down enable"]
    #[inline(always)]
    #[must_use]
    pub fn ome(&mut self) -> OmeW<InteSpec> {
        OmeW::new(self, 1)
    }
    #[doc = "Bit 2 - Compare match while counting up enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmue(&mut self) -> CmueW<InteSpec> {
        CmueW::new(self, 2)
    }
    #[doc = "Bit 3 - Compare match while counting down enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmde(&mut self) -> CmdeW<InteSpec> {
        CmdeW::new(self, 3)
    }
    #[doc = "Bit 8 - Event 0 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn e0ae(&mut self) -> E0aeW<InteSpec> {
        E0aeW::new(self, 8)
    }
    #[doc = "Bit 9 - Event 1 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn e1ae(&mut self) -> E1aeW<InteSpec> {
        E1aeW::new(self, 9)
    }
    #[doc = "Bit 10 - Event 2 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn e2ae(&mut self) -> E2aeW<InteSpec> {
        E2aeW::new(self, 10)
    }
}
#[doc = "Interrupt Enable Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inte::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inte::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InteSpec;
impl crate::RegisterSpec for InteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inte::R`](R) reader structure"]
impl crate::Readable for InteSpec {}
#[doc = "`write(|w| ..)` method takes [`inte::W`](W) writer structure"]
impl crate::Writable for InteSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTE to value 0"]
impl crate::Resettable for InteSpec {
    const RESET_VALUE: u32 = 0;
}
