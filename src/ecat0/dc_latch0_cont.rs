#[doc = "Register `DC_LATCH0_CONT` reader"]
pub type R = crate::R<DC_LATCH0_CONT_SPEC>;
#[doc = "Register `DC_LATCH0_CONT` writer"]
pub type W = crate::W<DC_LATCH0_CONT_SPEC>;
#[doc = "Latch0 positive edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L0_POS_A {
    #[doc = "0: Continuous Latch active"]
    VALUE1 = 0,
    #[doc = "1: Single event (only first event active)"]
    VALUE2 = 1,
}
impl From<L0_POS_A> for bool {
    #[inline(always)]
    fn from(variant: L0_POS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `L0_POS` reader - Latch0 positive edge"]
pub type L0_POS_R = crate::BitReader<L0_POS_A>;
impl L0_POS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> L0_POS_A {
        match self.bits {
            false => L0_POS_A::VALUE1,
            true => L0_POS_A::VALUE2,
        }
    }
    #[doc = "Continuous Latch active"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == L0_POS_A::VALUE1
    }
    #[doc = "Single event (only first event active)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == L0_POS_A::VALUE2
    }
}
#[doc = "Field `L0_POS` writer - Latch0 positive edge"]
pub type L0_POS_W<'a, REG> = crate::BitWriter<'a, REG, L0_POS_A>;
impl<'a, REG> L0_POS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Continuous Latch active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(L0_POS_A::VALUE1)
    }
    #[doc = "Single event (only first event active)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(L0_POS_A::VALUE2)
    }
}
#[doc = "Latch0 negative edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L0_NEG_A {
    #[doc = "0: Continuous Latch active"]
    VALUE1 = 0,
    #[doc = "1: Single event (only first event active)"]
    VALUE2 = 1,
}
impl From<L0_NEG_A> for bool {
    #[inline(always)]
    fn from(variant: L0_NEG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `L0_NEG` reader - Latch0 negative edge"]
pub type L0_NEG_R = crate::BitReader<L0_NEG_A>;
impl L0_NEG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> L0_NEG_A {
        match self.bits {
            false => L0_NEG_A::VALUE1,
            true => L0_NEG_A::VALUE2,
        }
    }
    #[doc = "Continuous Latch active"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == L0_NEG_A::VALUE1
    }
    #[doc = "Single event (only first event active)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == L0_NEG_A::VALUE2
    }
}
#[doc = "Field `L0_NEG` writer - Latch0 negative edge"]
pub type L0_NEG_W<'a, REG> = crate::BitWriter<'a, REG, L0_NEG_A>;
impl<'a, REG> L0_NEG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Continuous Latch active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(L0_NEG_A::VALUE1)
    }
    #[doc = "Single event (only first event active)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(L0_NEG_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Latch0 positive edge"]
    #[inline(always)]
    pub fn l0_pos(&self) -> L0_POS_R {
        L0_POS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Latch0 negative edge"]
    #[inline(always)]
    pub fn l0_neg(&self) -> L0_NEG_R {
        L0_NEG_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Latch0 positive edge"]
    #[inline(always)]
    #[must_use]
    pub fn l0_pos(&mut self) -> L0_POS_W<DC_LATCH0_CONT_SPEC> {
        L0_POS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Latch0 negative edge"]
    #[inline(always)]
    #[must_use]
    pub fn l0_neg(&mut self) -> L0_NEG_W<DC_LATCH0_CONT_SPEC> {
        L0_NEG_W::new(self, 1)
    }
}
#[doc = "Latch0 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_latch0_cont::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc_latch0_cont::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DC_LATCH0_CONT_SPEC;
impl crate::RegisterSpec for DC_LATCH0_CONT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dc_latch0_cont::R`](R) reader structure"]
impl crate::Readable for DC_LATCH0_CONT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dc_latch0_cont::W`](W) writer structure"]
impl crate::Writable for DC_LATCH0_CONT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DC_LATCH0_CONT to value 0"]
impl crate::Resettable for DC_LATCH0_CONT_SPEC {
    const RESET_VALUE: u8 = 0;
}
