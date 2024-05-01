#[doc = "Register `INTE` reader"]
pub type R = crate::R<INTE_SPEC>;
#[doc = "Register `INTE` writer"]
pub type W = crate::W<INTE_SPEC>;
#[doc = "Period match while counting up enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PME_A {
    #[doc = "0: Period Match interrupt is disabled"]
    VALUE1 = 0,
    #[doc = "1: Period Match interrupt is enabled"]
    VALUE2 = 1,
}
impl From<PME_A> for bool {
    #[inline(always)]
    fn from(variant: PME_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PME` reader - Period match while counting up enable"]
pub type PME_R = crate::BitReader<PME_A>;
impl PME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PME_A {
        match self.bits {
            false => PME_A::VALUE1,
            true => PME_A::VALUE2,
        }
    }
    #[doc = "Period Match interrupt is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PME_A::VALUE1
    }
    #[doc = "Period Match interrupt is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PME_A::VALUE2
    }
}
#[doc = "Field `PME` writer - Period match while counting up enable"]
pub type PME_W<'a, REG> = crate::BitWriter<'a, REG, PME_A>;
impl<'a, REG> PME_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Period Match interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PME_A::VALUE1)
    }
    #[doc = "Period Match interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PME_A::VALUE2)
    }
}
#[doc = "One match while counting down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OME_A {
    #[doc = "0: One Match interrupt is disabled"]
    VALUE1 = 0,
    #[doc = "1: One Match interrupt is enabled"]
    VALUE2 = 1,
}
impl From<OME_A> for bool {
    #[inline(always)]
    fn from(variant: OME_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OME` reader - One match while counting down enable"]
pub type OME_R = crate::BitReader<OME_A>;
impl OME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OME_A {
        match self.bits {
            false => OME_A::VALUE1,
            true => OME_A::VALUE2,
        }
    }
    #[doc = "One Match interrupt is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OME_A::VALUE1
    }
    #[doc = "One Match interrupt is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OME_A::VALUE2
    }
}
#[doc = "Field `OME` writer - One match while counting down enable"]
pub type OME_W<'a, REG> = crate::BitWriter<'a, REG, OME_A>;
impl<'a, REG> OME_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "One Match interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(OME_A::VALUE1)
    }
    #[doc = "One Match interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(OME_A::VALUE2)
    }
}
#[doc = "Channel 1 Compare match while counting up enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMU1E_A {
    #[doc = "0: Compare Match while counting up interrupt is disabled"]
    VALUE1 = 0,
    #[doc = "1: Compare Match while counting up interrupt is enabled"]
    VALUE2 = 1,
}
impl From<CMU1E_A> for bool {
    #[inline(always)]
    fn from(variant: CMU1E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMU1E` reader - Channel 1 Compare match while counting up enable"]
pub type CMU1E_R = crate::BitReader<CMU1E_A>;
impl CMU1E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMU1E_A {
        match self.bits {
            false => CMU1E_A::VALUE1,
            true => CMU1E_A::VALUE2,
        }
    }
    #[doc = "Compare Match while counting up interrupt is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMU1E_A::VALUE1
    }
    #[doc = "Compare Match while counting up interrupt is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMU1E_A::VALUE2
    }
}
#[doc = "Field `CMU1E` writer - Channel 1 Compare match while counting up enable"]
pub type CMU1E_W<'a, REG> = crate::BitWriter<'a, REG, CMU1E_A>;
impl<'a, REG> CMU1E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Compare Match while counting up interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CMU1E_A::VALUE1)
    }
    #[doc = "Compare Match while counting up interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CMU1E_A::VALUE2)
    }
}
#[doc = "Channel 1 Compare match while counting down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMD1E_A {
    #[doc = "0: Compare Match while counting down interrupt is disabled"]
    VALUE1 = 0,
    #[doc = "1: Compare Match while counting down interrupt is enabled"]
    VALUE2 = 1,
}
impl From<CMD1E_A> for bool {
    #[inline(always)]
    fn from(variant: CMD1E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMD1E` reader - Channel 1 Compare match while counting down enable"]
pub type CMD1E_R = crate::BitReader<CMD1E_A>;
impl CMD1E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMD1E_A {
        match self.bits {
            false => CMD1E_A::VALUE1,
            true => CMD1E_A::VALUE2,
        }
    }
    #[doc = "Compare Match while counting down interrupt is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMD1E_A::VALUE1
    }
    #[doc = "Compare Match while counting down interrupt is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMD1E_A::VALUE2
    }
}
#[doc = "Field `CMD1E` writer - Channel 1 Compare match while counting down enable"]
pub type CMD1E_W<'a, REG> = crate::BitWriter<'a, REG, CMD1E_A>;
impl<'a, REG> CMD1E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Compare Match while counting down interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CMD1E_A::VALUE1)
    }
    #[doc = "Compare Match while counting down interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CMD1E_A::VALUE2)
    }
}
#[doc = "Channel 2 Compare match while counting up enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMU2E_A {
    #[doc = "0: Compare Match while counting up interrupt is disabled"]
    VALUE1 = 0,
    #[doc = "1: Compare Match while counting up interrupt is enabled"]
    VALUE2 = 1,
}
impl From<CMU2E_A> for bool {
    #[inline(always)]
    fn from(variant: CMU2E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMU2E` reader - Channel 2 Compare match while counting up enable"]
pub type CMU2E_R = crate::BitReader<CMU2E_A>;
impl CMU2E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMU2E_A {
        match self.bits {
            false => CMU2E_A::VALUE1,
            true => CMU2E_A::VALUE2,
        }
    }
    #[doc = "Compare Match while counting up interrupt is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMU2E_A::VALUE1
    }
    #[doc = "Compare Match while counting up interrupt is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMU2E_A::VALUE2
    }
}
#[doc = "Field `CMU2E` writer - Channel 2 Compare match while counting up enable"]
pub type CMU2E_W<'a, REG> = crate::BitWriter<'a, REG, CMU2E_A>;
impl<'a, REG> CMU2E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Compare Match while counting up interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CMU2E_A::VALUE1)
    }
    #[doc = "Compare Match while counting up interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CMU2E_A::VALUE2)
    }
}
#[doc = "Channel 2 Compare match while counting down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMD2E_A {
    #[doc = "0: Compare Match while counting down interrupt is disabled"]
    VALUE1 = 0,
    #[doc = "1: Compare Match while counting down interrupt is enabled"]
    VALUE2 = 1,
}
impl From<CMD2E_A> for bool {
    #[inline(always)]
    fn from(variant: CMD2E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMD2E` reader - Channel 2 Compare match while counting down enable"]
pub type CMD2E_R = crate::BitReader<CMD2E_A>;
impl CMD2E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMD2E_A {
        match self.bits {
            false => CMD2E_A::VALUE1,
            true => CMD2E_A::VALUE2,
        }
    }
    #[doc = "Compare Match while counting down interrupt is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMD2E_A::VALUE1
    }
    #[doc = "Compare Match while counting down interrupt is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMD2E_A::VALUE2
    }
}
#[doc = "Field `CMD2E` writer - Channel 2 Compare match while counting down enable"]
pub type CMD2E_W<'a, REG> = crate::BitWriter<'a, REG, CMD2E_A>;
impl<'a, REG> CMD2E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Compare Match while counting down interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CMD2E_A::VALUE1)
    }
    #[doc = "Compare Match while counting down interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CMD2E_A::VALUE2)
    }
}
#[doc = "Event 0 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum E0AE_A {
    #[doc = "0: Event 0 detection interrupt is disabled"]
    VALUE1 = 0,
    #[doc = "1: Event 0 detection interrupt is enabled"]
    VALUE2 = 1,
}
impl From<E0AE_A> for bool {
    #[inline(always)]
    fn from(variant: E0AE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `E0AE` reader - Event 0 interrupt enable"]
pub type E0AE_R = crate::BitReader<E0AE_A>;
impl E0AE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> E0AE_A {
        match self.bits {
            false => E0AE_A::VALUE1,
            true => E0AE_A::VALUE2,
        }
    }
    #[doc = "Event 0 detection interrupt is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == E0AE_A::VALUE1
    }
    #[doc = "Event 0 detection interrupt is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == E0AE_A::VALUE2
    }
}
#[doc = "Field `E0AE` writer - Event 0 interrupt enable"]
pub type E0AE_W<'a, REG> = crate::BitWriter<'a, REG, E0AE_A>;
impl<'a, REG> E0AE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event 0 detection interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(E0AE_A::VALUE1)
    }
    #[doc = "Event 0 detection interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(E0AE_A::VALUE2)
    }
}
#[doc = "Event 1 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum E1AE_A {
    #[doc = "0: Event 1 detection interrupt is disabled"]
    VALUE1 = 0,
    #[doc = "1: Event 1 detection interrupt is enabled"]
    VALUE2 = 1,
}
impl From<E1AE_A> for bool {
    #[inline(always)]
    fn from(variant: E1AE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `E1AE` reader - Event 1 interrupt enable"]
pub type E1AE_R = crate::BitReader<E1AE_A>;
impl E1AE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> E1AE_A {
        match self.bits {
            false => E1AE_A::VALUE1,
            true => E1AE_A::VALUE2,
        }
    }
    #[doc = "Event 1 detection interrupt is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == E1AE_A::VALUE1
    }
    #[doc = "Event 1 detection interrupt is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == E1AE_A::VALUE2
    }
}
#[doc = "Field `E1AE` writer - Event 1 interrupt enable"]
pub type E1AE_W<'a, REG> = crate::BitWriter<'a, REG, E1AE_A>;
impl<'a, REG> E1AE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event 1 detection interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(E1AE_A::VALUE1)
    }
    #[doc = "Event 1 detection interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(E1AE_A::VALUE2)
    }
}
#[doc = "Event 2 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum E2AE_A {
    #[doc = "0: Event 2 detection interrupt is disabled"]
    VALUE1 = 0,
    #[doc = "1: Event 2 detection interrupt is enabled"]
    VALUE2 = 1,
}
impl From<E2AE_A> for bool {
    #[inline(always)]
    fn from(variant: E2AE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `E2AE` reader - Event 2 interrupt enable"]
pub type E2AE_R = crate::BitReader<E2AE_A>;
impl E2AE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> E2AE_A {
        match self.bits {
            false => E2AE_A::VALUE1,
            true => E2AE_A::VALUE2,
        }
    }
    #[doc = "Event 2 detection interrupt is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == E2AE_A::VALUE1
    }
    #[doc = "Event 2 detection interrupt is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == E2AE_A::VALUE2
    }
}
#[doc = "Field `E2AE` writer - Event 2 interrupt enable"]
pub type E2AE_W<'a, REG> = crate::BitWriter<'a, REG, E2AE_A>;
impl<'a, REG> E2AE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event 2 detection interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(E2AE_A::VALUE1)
    }
    #[doc = "Event 2 detection interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(E2AE_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Period match while counting up enable"]
    #[inline(always)]
    pub fn pme(&self) -> PME_R {
        PME_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - One match while counting down enable"]
    #[inline(always)]
    pub fn ome(&self) -> OME_R {
        OME_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 1 Compare match while counting up enable"]
    #[inline(always)]
    pub fn cmu1e(&self) -> CMU1E_R {
        CMU1E_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 1 Compare match while counting down enable"]
    #[inline(always)]
    pub fn cmd1e(&self) -> CMD1E_R {
        CMD1E_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 2 Compare match while counting up enable"]
    #[inline(always)]
    pub fn cmu2e(&self) -> CMU2E_R {
        CMU2E_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 2 Compare match while counting down enable"]
    #[inline(always)]
    pub fn cmd2e(&self) -> CMD2E_R {
        CMD2E_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Event 0 interrupt enable"]
    #[inline(always)]
    pub fn e0ae(&self) -> E0AE_R {
        E0AE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event 1 interrupt enable"]
    #[inline(always)]
    pub fn e1ae(&self) -> E1AE_R {
        E1AE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Event 2 interrupt enable"]
    #[inline(always)]
    pub fn e2ae(&self) -> E2AE_R {
        E2AE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Period match while counting up enable"]
    #[inline(always)]
    #[must_use]
    pub fn pme(&mut self) -> PME_W<INTE_SPEC> {
        PME_W::new(self, 0)
    }
    #[doc = "Bit 1 - One match while counting down enable"]
    #[inline(always)]
    #[must_use]
    pub fn ome(&mut self) -> OME_W<INTE_SPEC> {
        OME_W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 1 Compare match while counting up enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmu1e(&mut self) -> CMU1E_W<INTE_SPEC> {
        CMU1E_W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 1 Compare match while counting down enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmd1e(&mut self) -> CMD1E_W<INTE_SPEC> {
        CMD1E_W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 2 Compare match while counting up enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmu2e(&mut self) -> CMU2E_W<INTE_SPEC> {
        CMU2E_W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 2 Compare match while counting down enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmd2e(&mut self) -> CMD2E_W<INTE_SPEC> {
        CMD2E_W::new(self, 5)
    }
    #[doc = "Bit 8 - Event 0 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn e0ae(&mut self) -> E0AE_W<INTE_SPEC> {
        E0AE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Event 1 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn e1ae(&mut self) -> E1AE_W<INTE_SPEC> {
        E1AE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Event 2 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn e2ae(&mut self) -> E2AE_W<INTE_SPEC> {
        E2AE_W::new(self, 10)
    }
}
#[doc = "Interrupt Enable Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inte::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inte::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTE_SPEC;
impl crate::RegisterSpec for INTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inte::R`](R) reader structure"]
impl crate::Readable for INTE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inte::W`](W) writer structure"]
impl crate::Writable for INTE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTE to value 0"]
impl crate::Resettable for INTE_SPEC {
    const RESET_VALUE: u32 = 0;
}
