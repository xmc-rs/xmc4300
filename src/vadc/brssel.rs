#[doc = "Register `BRSSEL[%s]` reader"]
pub type R = crate::R<BRSSEL_SPEC>;
#[doc = "Register `BRSSEL[%s]` writer"]
pub type W = crate::W<BRSSEL_SPEC>;
#[doc = "Channel Selection Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSELG0_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    VALUE2 = 1,
}
impl From<CHSELG0_A> for bool {
    #[inline(always)]
    fn from(variant: CHSELG0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSELG0` reader - Channel Selection Group x"]
pub type CHSELG0_R = crate::BitReader<CHSELG0_A>;
impl CHSELG0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHSELG0_A {
        match self.bits {
            false => CHSELG0_A::VALUE1,
            true => CHSELG0_A::VALUE2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHSELG0_A::VALUE1
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHSELG0_A::VALUE2
    }
}
#[doc = "Field `CHSELG0` writer - Channel Selection Group x"]
pub type CHSELG0_W<'a, REG> = crate::BitWriter<'a, REG, CHSELG0_A>;
impl<'a, REG> CHSELG0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSELG0_A::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CHSELG0_A::VALUE2)
    }
}
#[doc = "Channel Selection Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSELG1_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    VALUE2 = 1,
}
impl From<CHSELG1_A> for bool {
    #[inline(always)]
    fn from(variant: CHSELG1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSELG1` reader - Channel Selection Group x"]
pub type CHSELG1_R = crate::BitReader<CHSELG1_A>;
impl CHSELG1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHSELG1_A {
        match self.bits {
            false => CHSELG1_A::VALUE1,
            true => CHSELG1_A::VALUE2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHSELG1_A::VALUE1
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHSELG1_A::VALUE2
    }
}
#[doc = "Field `CHSELG1` writer - Channel Selection Group x"]
pub type CHSELG1_W<'a, REG> = crate::BitWriter<'a, REG, CHSELG1_A>;
impl<'a, REG> CHSELG1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSELG1_A::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CHSELG1_A::VALUE2)
    }
}
#[doc = "Channel Selection Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSELG2_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    VALUE2 = 1,
}
impl From<CHSELG2_A> for bool {
    #[inline(always)]
    fn from(variant: CHSELG2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSELG2` reader - Channel Selection Group x"]
pub type CHSELG2_R = crate::BitReader<CHSELG2_A>;
impl CHSELG2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHSELG2_A {
        match self.bits {
            false => CHSELG2_A::VALUE1,
            true => CHSELG2_A::VALUE2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHSELG2_A::VALUE1
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHSELG2_A::VALUE2
    }
}
#[doc = "Field `CHSELG2` writer - Channel Selection Group x"]
pub type CHSELG2_W<'a, REG> = crate::BitWriter<'a, REG, CHSELG2_A>;
impl<'a, REG> CHSELG2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSELG2_A::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CHSELG2_A::VALUE2)
    }
}
#[doc = "Channel Selection Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSELG3_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    VALUE2 = 1,
}
impl From<CHSELG3_A> for bool {
    #[inline(always)]
    fn from(variant: CHSELG3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSELG3` reader - Channel Selection Group x"]
pub type CHSELG3_R = crate::BitReader<CHSELG3_A>;
impl CHSELG3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHSELG3_A {
        match self.bits {
            false => CHSELG3_A::VALUE1,
            true => CHSELG3_A::VALUE2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHSELG3_A::VALUE1
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHSELG3_A::VALUE2
    }
}
#[doc = "Field `CHSELG3` writer - Channel Selection Group x"]
pub type CHSELG3_W<'a, REG> = crate::BitWriter<'a, REG, CHSELG3_A>;
impl<'a, REG> CHSELG3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSELG3_A::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CHSELG3_A::VALUE2)
    }
}
#[doc = "Channel Selection Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSELG4_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    VALUE2 = 1,
}
impl From<CHSELG4_A> for bool {
    #[inline(always)]
    fn from(variant: CHSELG4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSELG4` reader - Channel Selection Group x"]
pub type CHSELG4_R = crate::BitReader<CHSELG4_A>;
impl CHSELG4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHSELG4_A {
        match self.bits {
            false => CHSELG4_A::VALUE1,
            true => CHSELG4_A::VALUE2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHSELG4_A::VALUE1
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHSELG4_A::VALUE2
    }
}
#[doc = "Field `CHSELG4` writer - Channel Selection Group x"]
pub type CHSELG4_W<'a, REG> = crate::BitWriter<'a, REG, CHSELG4_A>;
impl<'a, REG> CHSELG4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSELG4_A::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CHSELG4_A::VALUE2)
    }
}
#[doc = "Channel Selection Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSELG5_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    VALUE2 = 1,
}
impl From<CHSELG5_A> for bool {
    #[inline(always)]
    fn from(variant: CHSELG5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSELG5` reader - Channel Selection Group x"]
pub type CHSELG5_R = crate::BitReader<CHSELG5_A>;
impl CHSELG5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHSELG5_A {
        match self.bits {
            false => CHSELG5_A::VALUE1,
            true => CHSELG5_A::VALUE2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHSELG5_A::VALUE1
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHSELG5_A::VALUE2
    }
}
#[doc = "Field `CHSELG5` writer - Channel Selection Group x"]
pub type CHSELG5_W<'a, REG> = crate::BitWriter<'a, REG, CHSELG5_A>;
impl<'a, REG> CHSELG5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSELG5_A::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CHSELG5_A::VALUE2)
    }
}
#[doc = "Channel Selection Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSELG6_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    VALUE2 = 1,
}
impl From<CHSELG6_A> for bool {
    #[inline(always)]
    fn from(variant: CHSELG6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSELG6` reader - Channel Selection Group x"]
pub type CHSELG6_R = crate::BitReader<CHSELG6_A>;
impl CHSELG6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHSELG6_A {
        match self.bits {
            false => CHSELG6_A::VALUE1,
            true => CHSELG6_A::VALUE2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHSELG6_A::VALUE1
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHSELG6_A::VALUE2
    }
}
#[doc = "Field `CHSELG6` writer - Channel Selection Group x"]
pub type CHSELG6_W<'a, REG> = crate::BitWriter<'a, REG, CHSELG6_A>;
impl<'a, REG> CHSELG6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSELG6_A::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CHSELG6_A::VALUE2)
    }
}
#[doc = "Channel Selection Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSELG7_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    VALUE2 = 1,
}
impl From<CHSELG7_A> for bool {
    #[inline(always)]
    fn from(variant: CHSELG7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSELG7` reader - Channel Selection Group x"]
pub type CHSELG7_R = crate::BitReader<CHSELG7_A>;
impl CHSELG7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHSELG7_A {
        match self.bits {
            false => CHSELG7_A::VALUE1,
            true => CHSELG7_A::VALUE2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHSELG7_A::VALUE1
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHSELG7_A::VALUE2
    }
}
#[doc = "Field `CHSELG7` writer - Channel Selection Group x"]
pub type CHSELG7_W<'a, REG> = crate::BitWriter<'a, REG, CHSELG7_A>;
impl<'a, REG> CHSELG7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSELG7_A::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CHSELG7_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg0(&self) -> CHSELG0_R {
        CHSELG0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg1(&self) -> CHSELG1_R {
        CHSELG1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg2(&self) -> CHSELG2_R {
        CHSELG2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg3(&self) -> CHSELG3_R {
        CHSELG3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg4(&self) -> CHSELG4_R {
        CHSELG4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg5(&self) -> CHSELG5_R {
        CHSELG5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg6(&self) -> CHSELG6_R {
        CHSELG6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg7(&self) -> CHSELG7_R {
        CHSELG7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Selection Group x"]
    #[inline(always)]
    #[must_use]
    pub fn chselg0(&mut self) -> CHSELG0_W<BRSSEL_SPEC> {
        CHSELG0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel Selection Group x"]
    #[inline(always)]
    #[must_use]
    pub fn chselg1(&mut self) -> CHSELG1_W<BRSSEL_SPEC> {
        CHSELG1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel Selection Group x"]
    #[inline(always)]
    #[must_use]
    pub fn chselg2(&mut self) -> CHSELG2_W<BRSSEL_SPEC> {
        CHSELG2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel Selection Group x"]
    #[inline(always)]
    #[must_use]
    pub fn chselg3(&mut self) -> CHSELG3_W<BRSSEL_SPEC> {
        CHSELG3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel Selection Group x"]
    #[inline(always)]
    #[must_use]
    pub fn chselg4(&mut self) -> CHSELG4_W<BRSSEL_SPEC> {
        CHSELG4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel Selection Group x"]
    #[inline(always)]
    #[must_use]
    pub fn chselg5(&mut self) -> CHSELG5_W<BRSSEL_SPEC> {
        CHSELG5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Channel Selection Group x"]
    #[inline(always)]
    #[must_use]
    pub fn chselg6(&mut self) -> CHSELG6_W<BRSSEL_SPEC> {
        CHSELG6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Channel Selection Group x"]
    #[inline(always)]
    #[must_use]
    pub fn chselg7(&mut self) -> CHSELG7_W<BRSSEL_SPEC> {
        CHSELG7_W::new(self, 7)
    }
}
#[doc = "Background Request Source Channel Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brssel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brssel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BRSSEL_SPEC;
impl crate::RegisterSpec for BRSSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brssel::R`](R) reader structure"]
impl crate::Readable for BRSSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`brssel::W`](W) writer structure"]
impl crate::Writable for BRSSEL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BRSSEL[%s]
to value 0"]
impl crate::Resettable for BRSSEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
