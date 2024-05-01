#[doc = "Register `CHASS` reader"]
pub type R = crate::R<CHASS_SPEC>;
#[doc = "Register `CHASS` writer"]
pub type W = crate::W<CHASS_SPEC>;
#[doc = "Assignment for Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASSCH0_A {
    #[doc = "0: Channel y can be a background channel converted with lowest priority"]
    VALUE1 = 0,
    #[doc = "1: Channel y is a priority channel within group x"]
    VALUE2 = 1,
}
impl From<ASSCH0_A> for bool {
    #[inline(always)]
    fn from(variant: ASSCH0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASSCH0` reader - Assignment for Channel 0"]
pub type ASSCH0_R = crate::BitReader<ASSCH0_A>;
impl ASSCH0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ASSCH0_A {
        match self.bits {
            false => ASSCH0_A::VALUE1,
            true => ASSCH0_A::VALUE2,
        }
    }
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ASSCH0_A::VALUE1
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ASSCH0_A::VALUE2
    }
}
#[doc = "Field `ASSCH0` writer - Assignment for Channel 0"]
pub type ASSCH0_W<'a, REG> = crate::BitWriter<'a, REG, ASSCH0_A>;
impl<'a, REG> ASSCH0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ASSCH0_A::VALUE1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ASSCH0_A::VALUE2)
    }
}
#[doc = "Assignment for Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASSCH1_A {
    #[doc = "0: Channel y can be a background channel converted with lowest priority"]
    VALUE1 = 0,
    #[doc = "1: Channel y is a priority channel within group x"]
    VALUE2 = 1,
}
impl From<ASSCH1_A> for bool {
    #[inline(always)]
    fn from(variant: ASSCH1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASSCH1` reader - Assignment for Channel 1"]
pub type ASSCH1_R = crate::BitReader<ASSCH1_A>;
impl ASSCH1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ASSCH1_A {
        match self.bits {
            false => ASSCH1_A::VALUE1,
            true => ASSCH1_A::VALUE2,
        }
    }
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ASSCH1_A::VALUE1
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ASSCH1_A::VALUE2
    }
}
#[doc = "Field `ASSCH1` writer - Assignment for Channel 1"]
pub type ASSCH1_W<'a, REG> = crate::BitWriter<'a, REG, ASSCH1_A>;
impl<'a, REG> ASSCH1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ASSCH1_A::VALUE1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ASSCH1_A::VALUE2)
    }
}
#[doc = "Assignment for Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASSCH2_A {
    #[doc = "0: Channel y can be a background channel converted with lowest priority"]
    VALUE1 = 0,
    #[doc = "1: Channel y is a priority channel within group x"]
    VALUE2 = 1,
}
impl From<ASSCH2_A> for bool {
    #[inline(always)]
    fn from(variant: ASSCH2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASSCH2` reader - Assignment for Channel 2"]
pub type ASSCH2_R = crate::BitReader<ASSCH2_A>;
impl ASSCH2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ASSCH2_A {
        match self.bits {
            false => ASSCH2_A::VALUE1,
            true => ASSCH2_A::VALUE2,
        }
    }
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ASSCH2_A::VALUE1
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ASSCH2_A::VALUE2
    }
}
#[doc = "Field `ASSCH2` writer - Assignment for Channel 2"]
pub type ASSCH2_W<'a, REG> = crate::BitWriter<'a, REG, ASSCH2_A>;
impl<'a, REG> ASSCH2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ASSCH2_A::VALUE1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ASSCH2_A::VALUE2)
    }
}
#[doc = "Assignment for Channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASSCH3_A {
    #[doc = "0: Channel y can be a background channel converted with lowest priority"]
    VALUE1 = 0,
    #[doc = "1: Channel y is a priority channel within group x"]
    VALUE2 = 1,
}
impl From<ASSCH3_A> for bool {
    #[inline(always)]
    fn from(variant: ASSCH3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASSCH3` reader - Assignment for Channel 3"]
pub type ASSCH3_R = crate::BitReader<ASSCH3_A>;
impl ASSCH3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ASSCH3_A {
        match self.bits {
            false => ASSCH3_A::VALUE1,
            true => ASSCH3_A::VALUE2,
        }
    }
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ASSCH3_A::VALUE1
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ASSCH3_A::VALUE2
    }
}
#[doc = "Field `ASSCH3` writer - Assignment for Channel 3"]
pub type ASSCH3_W<'a, REG> = crate::BitWriter<'a, REG, ASSCH3_A>;
impl<'a, REG> ASSCH3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ASSCH3_A::VALUE1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ASSCH3_A::VALUE2)
    }
}
#[doc = "Assignment for Channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASSCH4_A {
    #[doc = "0: Channel y can be a background channel converted with lowest priority"]
    VALUE1 = 0,
    #[doc = "1: Channel y is a priority channel within group x"]
    VALUE2 = 1,
}
impl From<ASSCH4_A> for bool {
    #[inline(always)]
    fn from(variant: ASSCH4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASSCH4` reader - Assignment for Channel 4"]
pub type ASSCH4_R = crate::BitReader<ASSCH4_A>;
impl ASSCH4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ASSCH4_A {
        match self.bits {
            false => ASSCH4_A::VALUE1,
            true => ASSCH4_A::VALUE2,
        }
    }
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ASSCH4_A::VALUE1
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ASSCH4_A::VALUE2
    }
}
#[doc = "Field `ASSCH4` writer - Assignment for Channel 4"]
pub type ASSCH4_W<'a, REG> = crate::BitWriter<'a, REG, ASSCH4_A>;
impl<'a, REG> ASSCH4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ASSCH4_A::VALUE1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ASSCH4_A::VALUE2)
    }
}
#[doc = "Assignment for Channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASSCH5_A {
    #[doc = "0: Channel y can be a background channel converted with lowest priority"]
    VALUE1 = 0,
    #[doc = "1: Channel y is a priority channel within group x"]
    VALUE2 = 1,
}
impl From<ASSCH5_A> for bool {
    #[inline(always)]
    fn from(variant: ASSCH5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASSCH5` reader - Assignment for Channel 5"]
pub type ASSCH5_R = crate::BitReader<ASSCH5_A>;
impl ASSCH5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ASSCH5_A {
        match self.bits {
            false => ASSCH5_A::VALUE1,
            true => ASSCH5_A::VALUE2,
        }
    }
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ASSCH5_A::VALUE1
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ASSCH5_A::VALUE2
    }
}
#[doc = "Field `ASSCH5` writer - Assignment for Channel 5"]
pub type ASSCH5_W<'a, REG> = crate::BitWriter<'a, REG, ASSCH5_A>;
impl<'a, REG> ASSCH5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ASSCH5_A::VALUE1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ASSCH5_A::VALUE2)
    }
}
#[doc = "Assignment for Channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASSCH6_A {
    #[doc = "0: Channel y can be a background channel converted with lowest priority"]
    VALUE1 = 0,
    #[doc = "1: Channel y is a priority channel within group x"]
    VALUE2 = 1,
}
impl From<ASSCH6_A> for bool {
    #[inline(always)]
    fn from(variant: ASSCH6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASSCH6` reader - Assignment for Channel 6"]
pub type ASSCH6_R = crate::BitReader<ASSCH6_A>;
impl ASSCH6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ASSCH6_A {
        match self.bits {
            false => ASSCH6_A::VALUE1,
            true => ASSCH6_A::VALUE2,
        }
    }
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ASSCH6_A::VALUE1
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ASSCH6_A::VALUE2
    }
}
#[doc = "Field `ASSCH6` writer - Assignment for Channel 6"]
pub type ASSCH6_W<'a, REG> = crate::BitWriter<'a, REG, ASSCH6_A>;
impl<'a, REG> ASSCH6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ASSCH6_A::VALUE1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ASSCH6_A::VALUE2)
    }
}
#[doc = "Assignment for Channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASSCH7_A {
    #[doc = "0: Channel y can be a background channel converted with lowest priority"]
    VALUE1 = 0,
    #[doc = "1: Channel y is a priority channel within group x"]
    VALUE2 = 1,
}
impl From<ASSCH7_A> for bool {
    #[inline(always)]
    fn from(variant: ASSCH7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASSCH7` reader - Assignment for Channel 7"]
pub type ASSCH7_R = crate::BitReader<ASSCH7_A>;
impl ASSCH7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ASSCH7_A {
        match self.bits {
            false => ASSCH7_A::VALUE1,
            true => ASSCH7_A::VALUE2,
        }
    }
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ASSCH7_A::VALUE1
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ASSCH7_A::VALUE2
    }
}
#[doc = "Field `ASSCH7` writer - Assignment for Channel 7"]
pub type ASSCH7_W<'a, REG> = crate::BitWriter<'a, REG, ASSCH7_A>;
impl<'a, REG> ASSCH7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ASSCH7_A::VALUE1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ASSCH7_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Assignment for Channel 0"]
    #[inline(always)]
    pub fn assch0(&self) -> ASSCH0_R {
        ASSCH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Assignment for Channel 1"]
    #[inline(always)]
    pub fn assch1(&self) -> ASSCH1_R {
        ASSCH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Assignment for Channel 2"]
    #[inline(always)]
    pub fn assch2(&self) -> ASSCH2_R {
        ASSCH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Assignment for Channel 3"]
    #[inline(always)]
    pub fn assch3(&self) -> ASSCH3_R {
        ASSCH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Assignment for Channel 4"]
    #[inline(always)]
    pub fn assch4(&self) -> ASSCH4_R {
        ASSCH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Assignment for Channel 5"]
    #[inline(always)]
    pub fn assch5(&self) -> ASSCH5_R {
        ASSCH5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Assignment for Channel 6"]
    #[inline(always)]
    pub fn assch6(&self) -> ASSCH6_R {
        ASSCH6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Assignment for Channel 7"]
    #[inline(always)]
    pub fn assch7(&self) -> ASSCH7_R {
        ASSCH7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Assignment for Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn assch0(&mut self) -> ASSCH0_W<CHASS_SPEC> {
        ASSCH0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Assignment for Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn assch1(&mut self) -> ASSCH1_W<CHASS_SPEC> {
        ASSCH1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Assignment for Channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn assch2(&mut self) -> ASSCH2_W<CHASS_SPEC> {
        ASSCH2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Assignment for Channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn assch3(&mut self) -> ASSCH3_W<CHASS_SPEC> {
        ASSCH3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Assignment for Channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn assch4(&mut self) -> ASSCH4_W<CHASS_SPEC> {
        ASSCH4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Assignment for Channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn assch5(&mut self) -> ASSCH5_W<CHASS_SPEC> {
        ASSCH5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Assignment for Channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn assch6(&mut self) -> ASSCH6_W<CHASS_SPEC> {
        ASSCH6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Assignment for Channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn assch7(&mut self) -> ASSCH7_W<CHASS_SPEC> {
        ASSCH7_W::new(self, 7)
    }
}
#[doc = "Channel Assignment Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chass::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chass::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHASS_SPEC;
impl crate::RegisterSpec for CHASS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chass::R`](R) reader structure"]
impl crate::Readable for CHASS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chass::W`](W) writer structure"]
impl crate::Writable for CHASS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHASS to value 0"]
impl crate::Resettable for CHASS_SPEC {
    const RESET_VALUE: u32 = 0;
}
