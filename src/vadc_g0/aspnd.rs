#[doc = "Register `ASPND` reader"]
pub type R = crate::R<ASPND_SPEC>;
#[doc = "Register `ASPND` writer"]
pub type W = crate::W<ASPND_SPEC>;
#[doc = "Field `CHPND0` reader - Channels Pending"]
pub type CHPND0_R = crate::BitReader<CHPND0_A>;
#[doc = "Channels Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHPND0_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: Request conversion of this channel"]
    VALUE2 = 1,
}
impl From<CHPND0_A> for bool {
    #[inline(always)]
    fn from(variant: CHPND0_A) -> Self {
        variant as u8 != 0
    }
}
impl CHPND0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHPND0_A {
        match self.bits {
            false => CHPND0_A::VALUE1,
            true => CHPND0_A::VALUE2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPND0_A::VALUE1
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPND0_A::VALUE2
    }
}
#[doc = "Field `CHPND0` writer - Channels Pending"]
pub type CHPND0_W<'a, REG> = crate::BitWriter<'a, REG, CHPND0_A>;
impl<'a, REG> CHPND0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CHPND0_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CHPND0_A::VALUE2)
    }
}
#[doc = "Field `CHPND1` reader - Channels Pending"]
pub type CHPND1_R = crate::BitReader<CHPND1_A>;
#[doc = "Channels Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHPND1_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: Request conversion of this channel"]
    VALUE2 = 1,
}
impl From<CHPND1_A> for bool {
    #[inline(always)]
    fn from(variant: CHPND1_A) -> Self {
        variant as u8 != 0
    }
}
impl CHPND1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHPND1_A {
        match self.bits {
            false => CHPND1_A::VALUE1,
            true => CHPND1_A::VALUE2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPND1_A::VALUE1
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPND1_A::VALUE2
    }
}
#[doc = "Field `CHPND1` writer - Channels Pending"]
pub type CHPND1_W<'a, REG> = crate::BitWriter<'a, REG, CHPND1_A>;
impl<'a, REG> CHPND1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CHPND1_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CHPND1_A::VALUE2)
    }
}
#[doc = "Field `CHPND2` reader - Channels Pending"]
pub type CHPND2_R = crate::BitReader<CHPND2_A>;
#[doc = "Channels Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHPND2_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: Request conversion of this channel"]
    VALUE2 = 1,
}
impl From<CHPND2_A> for bool {
    #[inline(always)]
    fn from(variant: CHPND2_A) -> Self {
        variant as u8 != 0
    }
}
impl CHPND2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHPND2_A {
        match self.bits {
            false => CHPND2_A::VALUE1,
            true => CHPND2_A::VALUE2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPND2_A::VALUE1
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPND2_A::VALUE2
    }
}
#[doc = "Field `CHPND2` writer - Channels Pending"]
pub type CHPND2_W<'a, REG> = crate::BitWriter<'a, REG, CHPND2_A>;
impl<'a, REG> CHPND2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CHPND2_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CHPND2_A::VALUE2)
    }
}
#[doc = "Field `CHPND3` reader - Channels Pending"]
pub type CHPND3_R = crate::BitReader<CHPND3_A>;
#[doc = "Channels Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHPND3_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: Request conversion of this channel"]
    VALUE2 = 1,
}
impl From<CHPND3_A> for bool {
    #[inline(always)]
    fn from(variant: CHPND3_A) -> Self {
        variant as u8 != 0
    }
}
impl CHPND3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHPND3_A {
        match self.bits {
            false => CHPND3_A::VALUE1,
            true => CHPND3_A::VALUE2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPND3_A::VALUE1
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPND3_A::VALUE2
    }
}
#[doc = "Field `CHPND3` writer - Channels Pending"]
pub type CHPND3_W<'a, REG> = crate::BitWriter<'a, REG, CHPND3_A>;
impl<'a, REG> CHPND3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CHPND3_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CHPND3_A::VALUE2)
    }
}
#[doc = "Field `CHPND4` reader - Channels Pending"]
pub type CHPND4_R = crate::BitReader<CHPND4_A>;
#[doc = "Channels Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHPND4_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: Request conversion of this channel"]
    VALUE2 = 1,
}
impl From<CHPND4_A> for bool {
    #[inline(always)]
    fn from(variant: CHPND4_A) -> Self {
        variant as u8 != 0
    }
}
impl CHPND4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHPND4_A {
        match self.bits {
            false => CHPND4_A::VALUE1,
            true => CHPND4_A::VALUE2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPND4_A::VALUE1
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPND4_A::VALUE2
    }
}
#[doc = "Field `CHPND4` writer - Channels Pending"]
pub type CHPND4_W<'a, REG> = crate::BitWriter<'a, REG, CHPND4_A>;
impl<'a, REG> CHPND4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CHPND4_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CHPND4_A::VALUE2)
    }
}
#[doc = "Field `CHPND5` reader - Channels Pending"]
pub type CHPND5_R = crate::BitReader<CHPND5_A>;
#[doc = "Channels Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHPND5_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: Request conversion of this channel"]
    VALUE2 = 1,
}
impl From<CHPND5_A> for bool {
    #[inline(always)]
    fn from(variant: CHPND5_A) -> Self {
        variant as u8 != 0
    }
}
impl CHPND5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHPND5_A {
        match self.bits {
            false => CHPND5_A::VALUE1,
            true => CHPND5_A::VALUE2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPND5_A::VALUE1
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPND5_A::VALUE2
    }
}
#[doc = "Field `CHPND5` writer - Channels Pending"]
pub type CHPND5_W<'a, REG> = crate::BitWriter<'a, REG, CHPND5_A>;
impl<'a, REG> CHPND5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CHPND5_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CHPND5_A::VALUE2)
    }
}
#[doc = "Field `CHPND6` reader - Channels Pending"]
pub type CHPND6_R = crate::BitReader<CHPND6_A>;
#[doc = "Channels Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHPND6_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: Request conversion of this channel"]
    VALUE2 = 1,
}
impl From<CHPND6_A> for bool {
    #[inline(always)]
    fn from(variant: CHPND6_A) -> Self {
        variant as u8 != 0
    }
}
impl CHPND6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHPND6_A {
        match self.bits {
            false => CHPND6_A::VALUE1,
            true => CHPND6_A::VALUE2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPND6_A::VALUE1
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPND6_A::VALUE2
    }
}
#[doc = "Field `CHPND6` writer - Channels Pending"]
pub type CHPND6_W<'a, REG> = crate::BitWriter<'a, REG, CHPND6_A>;
impl<'a, REG> CHPND6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CHPND6_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CHPND6_A::VALUE2)
    }
}
#[doc = "Field `CHPND7` reader - Channels Pending"]
pub type CHPND7_R = crate::BitReader<CHPND7_A>;
#[doc = "Channels Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHPND7_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: Request conversion of this channel"]
    VALUE2 = 1,
}
impl From<CHPND7_A> for bool {
    #[inline(always)]
    fn from(variant: CHPND7_A) -> Self {
        variant as u8 != 0
    }
}
impl CHPND7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHPND7_A {
        match self.bits {
            false => CHPND7_A::VALUE1,
            true => CHPND7_A::VALUE2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPND7_A::VALUE1
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPND7_A::VALUE2
    }
}
#[doc = "Field `CHPND7` writer - Channels Pending"]
pub type CHPND7_W<'a, REG> = crate::BitWriter<'a, REG, CHPND7_A>;
impl<'a, REG> CHPND7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CHPND7_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CHPND7_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Channels Pending"]
    #[inline(always)]
    pub fn chpnd0(&self) -> CHPND0_R {
        CHPND0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channels Pending"]
    #[inline(always)]
    pub fn chpnd1(&self) -> CHPND1_R {
        CHPND1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channels Pending"]
    #[inline(always)]
    pub fn chpnd2(&self) -> CHPND2_R {
        CHPND2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channels Pending"]
    #[inline(always)]
    pub fn chpnd3(&self) -> CHPND3_R {
        CHPND3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channels Pending"]
    #[inline(always)]
    pub fn chpnd4(&self) -> CHPND4_R {
        CHPND4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channels Pending"]
    #[inline(always)]
    pub fn chpnd5(&self) -> CHPND5_R {
        CHPND5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channels Pending"]
    #[inline(always)]
    pub fn chpnd6(&self) -> CHPND6_R {
        CHPND6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channels Pending"]
    #[inline(always)]
    pub fn chpnd7(&self) -> CHPND7_R {
        CHPND7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channels Pending"]
    #[inline(always)]
    #[must_use]
    pub fn chpnd0(&mut self) -> CHPND0_W<ASPND_SPEC> {
        CHPND0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channels Pending"]
    #[inline(always)]
    #[must_use]
    pub fn chpnd1(&mut self) -> CHPND1_W<ASPND_SPEC> {
        CHPND1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Channels Pending"]
    #[inline(always)]
    #[must_use]
    pub fn chpnd2(&mut self) -> CHPND2_W<ASPND_SPEC> {
        CHPND2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Channels Pending"]
    #[inline(always)]
    #[must_use]
    pub fn chpnd3(&mut self) -> CHPND3_W<ASPND_SPEC> {
        CHPND3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Channels Pending"]
    #[inline(always)]
    #[must_use]
    pub fn chpnd4(&mut self) -> CHPND4_W<ASPND_SPEC> {
        CHPND4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Channels Pending"]
    #[inline(always)]
    #[must_use]
    pub fn chpnd5(&mut self) -> CHPND5_W<ASPND_SPEC> {
        CHPND5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Channels Pending"]
    #[inline(always)]
    #[must_use]
    pub fn chpnd6(&mut self) -> CHPND6_W<ASPND_SPEC> {
        CHPND6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Channels Pending"]
    #[inline(always)]
    #[must_use]
    pub fn chpnd7(&mut self) -> CHPND7_W<ASPND_SPEC> {
        CHPND7_W::new(self, 7)
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
#[doc = "Autoscan Source Pending Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aspnd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aspnd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ASPND_SPEC;
impl crate::RegisterSpec for ASPND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aspnd::R`](R) reader structure"]
impl crate::Readable for ASPND_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aspnd::W`](W) writer structure"]
impl crate::Writable for ASPND_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ASPND to value 0"]
impl crate::Resettable for ASPND_SPEC {
    const RESET_VALUE: u32 = 0;
}
