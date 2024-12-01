#[doc = "Register `BRSPND[%s]` reader"]
pub type R = crate::R<BRSPND_SPEC>;
#[doc = "Register `BRSPND[%s]` writer"]
pub type W = crate::W<BRSPND_SPEC>;
#[doc = "Channels Pending Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHPNDG0_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: Request conversion of this channel"]
    VALUE2 = 1,
}
impl From<CHPNDG0_A> for bool {
    #[inline(always)]
    fn from(variant: CHPNDG0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHPNDG0` reader - Channels Pending Group x"]
pub type CHPNDG0_R = crate::BitReader<CHPNDG0_A>;
impl CHPNDG0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHPNDG0_A {
        match self.bits {
            false => CHPNDG0_A::VALUE1,
            true => CHPNDG0_A::VALUE2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPNDG0_A::VALUE1
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPNDG0_A::VALUE2
    }
}
#[doc = "Field `CHPNDG0` writer - Channels Pending Group x"]
pub type CHPNDG0_W<'a, REG> = crate::BitWriter<'a, REG, CHPNDG0_A>;
impl<'a, REG> CHPNDG0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CHPNDG0_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CHPNDG0_A::VALUE2)
    }
}
#[doc = "Channels Pending Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHPNDG1_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: Request conversion of this channel"]
    VALUE2 = 1,
}
impl From<CHPNDG1_A> for bool {
    #[inline(always)]
    fn from(variant: CHPNDG1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHPNDG1` reader - Channels Pending Group x"]
pub type CHPNDG1_R = crate::BitReader<CHPNDG1_A>;
impl CHPNDG1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHPNDG1_A {
        match self.bits {
            false => CHPNDG1_A::VALUE1,
            true => CHPNDG1_A::VALUE2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPNDG1_A::VALUE1
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPNDG1_A::VALUE2
    }
}
#[doc = "Field `CHPNDG1` writer - Channels Pending Group x"]
pub type CHPNDG1_W<'a, REG> = crate::BitWriter<'a, REG, CHPNDG1_A>;
impl<'a, REG> CHPNDG1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CHPNDG1_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CHPNDG1_A::VALUE2)
    }
}
#[doc = "Channels Pending Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHPNDG2_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: Request conversion of this channel"]
    VALUE2 = 1,
}
impl From<CHPNDG2_A> for bool {
    #[inline(always)]
    fn from(variant: CHPNDG2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHPNDG2` reader - Channels Pending Group x"]
pub type CHPNDG2_R = crate::BitReader<CHPNDG2_A>;
impl CHPNDG2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHPNDG2_A {
        match self.bits {
            false => CHPNDG2_A::VALUE1,
            true => CHPNDG2_A::VALUE2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPNDG2_A::VALUE1
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPNDG2_A::VALUE2
    }
}
#[doc = "Field `CHPNDG2` writer - Channels Pending Group x"]
pub type CHPNDG2_W<'a, REG> = crate::BitWriter<'a, REG, CHPNDG2_A>;
impl<'a, REG> CHPNDG2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CHPNDG2_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CHPNDG2_A::VALUE2)
    }
}
#[doc = "Channels Pending Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHPNDG3_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: Request conversion of this channel"]
    VALUE2 = 1,
}
impl From<CHPNDG3_A> for bool {
    #[inline(always)]
    fn from(variant: CHPNDG3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHPNDG3` reader - Channels Pending Group x"]
pub type CHPNDG3_R = crate::BitReader<CHPNDG3_A>;
impl CHPNDG3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHPNDG3_A {
        match self.bits {
            false => CHPNDG3_A::VALUE1,
            true => CHPNDG3_A::VALUE2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPNDG3_A::VALUE1
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPNDG3_A::VALUE2
    }
}
#[doc = "Field `CHPNDG3` writer - Channels Pending Group x"]
pub type CHPNDG3_W<'a, REG> = crate::BitWriter<'a, REG, CHPNDG3_A>;
impl<'a, REG> CHPNDG3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CHPNDG3_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CHPNDG3_A::VALUE2)
    }
}
#[doc = "Channels Pending Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHPNDG4_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: Request conversion of this channel"]
    VALUE2 = 1,
}
impl From<CHPNDG4_A> for bool {
    #[inline(always)]
    fn from(variant: CHPNDG4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHPNDG4` reader - Channels Pending Group x"]
pub type CHPNDG4_R = crate::BitReader<CHPNDG4_A>;
impl CHPNDG4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHPNDG4_A {
        match self.bits {
            false => CHPNDG4_A::VALUE1,
            true => CHPNDG4_A::VALUE2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPNDG4_A::VALUE1
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPNDG4_A::VALUE2
    }
}
#[doc = "Field `CHPNDG4` writer - Channels Pending Group x"]
pub type CHPNDG4_W<'a, REG> = crate::BitWriter<'a, REG, CHPNDG4_A>;
impl<'a, REG> CHPNDG4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CHPNDG4_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CHPNDG4_A::VALUE2)
    }
}
#[doc = "Channels Pending Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHPNDG5_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: Request conversion of this channel"]
    VALUE2 = 1,
}
impl From<CHPNDG5_A> for bool {
    #[inline(always)]
    fn from(variant: CHPNDG5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHPNDG5` reader - Channels Pending Group x"]
pub type CHPNDG5_R = crate::BitReader<CHPNDG5_A>;
impl CHPNDG5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHPNDG5_A {
        match self.bits {
            false => CHPNDG5_A::VALUE1,
            true => CHPNDG5_A::VALUE2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPNDG5_A::VALUE1
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPNDG5_A::VALUE2
    }
}
#[doc = "Field `CHPNDG5` writer - Channels Pending Group x"]
pub type CHPNDG5_W<'a, REG> = crate::BitWriter<'a, REG, CHPNDG5_A>;
impl<'a, REG> CHPNDG5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CHPNDG5_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CHPNDG5_A::VALUE2)
    }
}
#[doc = "Channels Pending Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHPNDG6_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: Request conversion of this channel"]
    VALUE2 = 1,
}
impl From<CHPNDG6_A> for bool {
    #[inline(always)]
    fn from(variant: CHPNDG6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHPNDG6` reader - Channels Pending Group x"]
pub type CHPNDG6_R = crate::BitReader<CHPNDG6_A>;
impl CHPNDG6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHPNDG6_A {
        match self.bits {
            false => CHPNDG6_A::VALUE1,
            true => CHPNDG6_A::VALUE2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPNDG6_A::VALUE1
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPNDG6_A::VALUE2
    }
}
#[doc = "Field `CHPNDG6` writer - Channels Pending Group x"]
pub type CHPNDG6_W<'a, REG> = crate::BitWriter<'a, REG, CHPNDG6_A>;
impl<'a, REG> CHPNDG6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CHPNDG6_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CHPNDG6_A::VALUE2)
    }
}
#[doc = "Channels Pending Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHPNDG7_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: Request conversion of this channel"]
    VALUE2 = 1,
}
impl From<CHPNDG7_A> for bool {
    #[inline(always)]
    fn from(variant: CHPNDG7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHPNDG7` reader - Channels Pending Group x"]
pub type CHPNDG7_R = crate::BitReader<CHPNDG7_A>;
impl CHPNDG7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHPNDG7_A {
        match self.bits {
            false => CHPNDG7_A::VALUE1,
            true => CHPNDG7_A::VALUE2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPNDG7_A::VALUE1
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPNDG7_A::VALUE2
    }
}
#[doc = "Field `CHPNDG7` writer - Channels Pending Group x"]
pub type CHPNDG7_W<'a, REG> = crate::BitWriter<'a, REG, CHPNDG7_A>;
impl<'a, REG> CHPNDG7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CHPNDG7_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CHPNDG7_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg0(&self) -> CHPNDG0_R {
        CHPNDG0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg1(&self) -> CHPNDG1_R {
        CHPNDG1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg2(&self) -> CHPNDG2_R {
        CHPNDG2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg3(&self) -> CHPNDG3_R {
        CHPNDG3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg4(&self) -> CHPNDG4_R {
        CHPNDG4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg5(&self) -> CHPNDG5_R {
        CHPNDG5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg6(&self) -> CHPNDG6_R {
        CHPNDG6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg7(&self) -> CHPNDG7_R {
        CHPNDG7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg0(&mut self) -> CHPNDG0_W<BRSPND_SPEC> {
        CHPNDG0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg1(&mut self) -> CHPNDG1_W<BRSPND_SPEC> {
        CHPNDG1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg2(&mut self) -> CHPNDG2_W<BRSPND_SPEC> {
        CHPNDG2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg3(&mut self) -> CHPNDG3_W<BRSPND_SPEC> {
        CHPNDG3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg4(&mut self) -> CHPNDG4_W<BRSPND_SPEC> {
        CHPNDG4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg5(&mut self) -> CHPNDG5_W<BRSPND_SPEC> {
        CHPNDG5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg6(&mut self) -> CHPNDG6_W<BRSPND_SPEC> {
        CHPNDG6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg7(&mut self) -> CHPNDG7_W<BRSPND_SPEC> {
        CHPNDG7_W::new(self, 7)
    }
}
#[doc = "Background Request Source Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`brspnd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brspnd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BRSPND_SPEC;
impl crate::RegisterSpec for BRSPND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brspnd::R`](R) reader structure"]
impl crate::Readable for BRSPND_SPEC {}
#[doc = "`write(|w| ..)` method takes [`brspnd::W`](W) writer structure"]
impl crate::Writable for BRSPND_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BRSPND[%s]
to value 0"]
impl crate::Resettable for BRSPND_SPEC {
    const RESET_VALUE: u32 = 0;
}
