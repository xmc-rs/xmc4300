#[doc = "Register `DC_ACT` reader"]
pub type R = crate::R<DC_ACT_SPEC>;
#[doc = "Register `DC_ACT` writer"]
pub type W = crate::W<DC_ACT_SPEC>;
#[doc = "Sync Out Unit activation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNC_OUT_A {
    #[doc = "0: Deactivated"]
    VALUE1 = 0,
    #[doc = "1: Activated"]
    VALUE2 = 1,
}
impl From<SYNC_OUT_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC_OUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNC_OUT` reader - Sync Out Unit activation"]
pub type SYNC_OUT_R = crate::BitReader<SYNC_OUT_A>;
impl SYNC_OUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYNC_OUT_A {
        match self.bits {
            false => SYNC_OUT_A::VALUE1,
            true => SYNC_OUT_A::VALUE2,
        }
    }
    #[doc = "Deactivated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SYNC_OUT_A::VALUE1
    }
    #[doc = "Activated"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SYNC_OUT_A::VALUE2
    }
}
#[doc = "Field `SYNC_OUT` writer - Sync Out Unit activation"]
pub type SYNC_OUT_W<'a, REG> = crate::BitWriter<'a, REG, SYNC_OUT_A>;
impl<'a, REG> SYNC_OUT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Deactivated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_OUT_A::VALUE1)
    }
    #[doc = "Activated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_OUT_A::VALUE2)
    }
}
#[doc = "SYNC0 generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNC_0_A {
    #[doc = "0: Deactivated"]
    VALUE1 = 0,
    #[doc = "1: SYNC0 pulse is generated"]
    VALUE2 = 1,
}
impl From<SYNC_0_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC_0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNC_0` reader - SYNC0 generation"]
pub type SYNC_0_R = crate::BitReader<SYNC_0_A>;
impl SYNC_0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYNC_0_A {
        match self.bits {
            false => SYNC_0_A::VALUE1,
            true => SYNC_0_A::VALUE2,
        }
    }
    #[doc = "Deactivated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SYNC_0_A::VALUE1
    }
    #[doc = "SYNC0 pulse is generated"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SYNC_0_A::VALUE2
    }
}
#[doc = "Field `SYNC_0` writer - SYNC0 generation"]
pub type SYNC_0_W<'a, REG> = crate::BitWriter<'a, REG, SYNC_0_A>;
impl<'a, REG> SYNC_0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Deactivated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_0_A::VALUE1)
    }
    #[doc = "SYNC0 pulse is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_0_A::VALUE2)
    }
}
#[doc = "SYNC1 generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNC_1_A {
    #[doc = "0: Deactivated"]
    VALUE1 = 0,
    #[doc = "1: SYNC1 pulse is generated"]
    VALUE2 = 1,
}
impl From<SYNC_1_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC_1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNC_1` reader - SYNC1 generation"]
pub type SYNC_1_R = crate::BitReader<SYNC_1_A>;
impl SYNC_1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYNC_1_A {
        match self.bits {
            false => SYNC_1_A::VALUE1,
            true => SYNC_1_A::VALUE2,
        }
    }
    #[doc = "Deactivated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SYNC_1_A::VALUE1
    }
    #[doc = "SYNC1 pulse is generated"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SYNC_1_A::VALUE2
    }
}
#[doc = "Field `SYNC_1` writer - SYNC1 generation"]
pub type SYNC_1_W<'a, REG> = crate::BitWriter<'a, REG, SYNC_1_A>;
impl<'a, REG> SYNC_1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Deactivated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_1_A::VALUE1)
    }
    #[doc = "SYNC1 pulse is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_1_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Sync Out Unit activation"]
    #[inline(always)]
    pub fn sync_out(&self) -> SYNC_OUT_R {
        SYNC_OUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SYNC0 generation"]
    #[inline(always)]
    pub fn sync_0(&self) -> SYNC_0_R {
        SYNC_0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SYNC1 generation"]
    #[inline(always)]
    pub fn sync_1(&self) -> SYNC_1_R {
        SYNC_1_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sync Out Unit activation"]
    #[inline(always)]
    #[must_use]
    pub fn sync_out(&mut self) -> SYNC_OUT_W<DC_ACT_SPEC> {
        SYNC_OUT_W::new(self, 0)
    }
    #[doc = "Bit 1 - SYNC0 generation"]
    #[inline(always)]
    #[must_use]
    pub fn sync_0(&mut self) -> SYNC_0_W<DC_ACT_SPEC> {
        SYNC_0_W::new(self, 1)
    }
    #[doc = "Bit 2 - SYNC1 generation"]
    #[inline(always)]
    #[must_use]
    pub fn sync_1(&mut self) -> SYNC_1_W<DC_ACT_SPEC> {
        SYNC_1_W::new(self, 2)
    }
}
#[doc = "Activation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_act::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc_act::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DC_ACT_SPEC;
impl crate::RegisterSpec for DC_ACT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dc_act::R`](R) reader structure"]
impl crate::Readable for DC_ACT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dc_act::W`](W) writer structure"]
impl crate::Writable for DC_ACT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DC_ACT to value 0"]
impl crate::Resettable for DC_ACT_SPEC {
    const RESET_VALUE: u8 = 0;
}
