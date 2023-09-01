#[doc = "Register `DC_LATCH1_CONT` reader"]
pub type R = crate::R<DC_LATCH1_CONT_SPEC>;
#[doc = "Register `DC_LATCH1_CONT` writer"]
pub type W = crate::W<DC_LATCH1_CONT_SPEC>;
#[doc = "Field `L1_POS` reader - Latch1 positive edge"]
pub type L1_POS_R = crate::BitReader<L1_POS_A>;
#[doc = "Latch1 positive edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1_POS_A {
    #[doc = "0: Continuous Latch active"]
    VALUE1 = 0,
    #[doc = "1: Single event (only first event active)"]
    VALUE2 = 1,
}
impl From<L1_POS_A> for bool {
    #[inline(always)]
    fn from(variant: L1_POS_A) -> Self {
        variant as u8 != 0
    }
}
impl L1_POS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L1_POS_A {
        match self.bits {
            false => L1_POS_A::VALUE1,
            true => L1_POS_A::VALUE2,
        }
    }
    #[doc = "Continuous Latch active"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == L1_POS_A::VALUE1
    }
    #[doc = "Single event (only first event active)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == L1_POS_A::VALUE2
    }
}
#[doc = "Field `L1_POS` writer - Latch1 positive edge"]
pub type L1_POS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, L1_POS_A>;
impl<'a, REG, const O: u8> L1_POS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Continuous Latch active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(L1_POS_A::VALUE1)
    }
    #[doc = "Single event (only first event active)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(L1_POS_A::VALUE2)
    }
}
#[doc = "Field `L1_NEG` reader - Latch1 negative edge"]
pub type L1_NEG_R = crate::BitReader<L1_NEG_A>;
#[doc = "Latch1 negative edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1_NEG_A {
    #[doc = "0: Continuous Latch active"]
    VALUE1 = 0,
    #[doc = "1: Single event (only first event active)"]
    VALUE2 = 1,
}
impl From<L1_NEG_A> for bool {
    #[inline(always)]
    fn from(variant: L1_NEG_A) -> Self {
        variant as u8 != 0
    }
}
impl L1_NEG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L1_NEG_A {
        match self.bits {
            false => L1_NEG_A::VALUE1,
            true => L1_NEG_A::VALUE2,
        }
    }
    #[doc = "Continuous Latch active"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == L1_NEG_A::VALUE1
    }
    #[doc = "Single event (only first event active)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == L1_NEG_A::VALUE2
    }
}
#[doc = "Field `L1_NEG` writer - Latch1 negative edge"]
pub type L1_NEG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, L1_NEG_A>;
impl<'a, REG, const O: u8> L1_NEG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Continuous Latch active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(L1_NEG_A::VALUE1)
    }
    #[doc = "Single event (only first event active)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(L1_NEG_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Latch1 positive edge"]
    #[inline(always)]
    pub fn l1_pos(&self) -> L1_POS_R {
        L1_POS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Latch1 negative edge"]
    #[inline(always)]
    pub fn l1_neg(&self) -> L1_NEG_R {
        L1_NEG_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Latch1 positive edge"]
    #[inline(always)]
    #[must_use]
    pub fn l1_pos(&mut self) -> L1_POS_W<DC_LATCH1_CONT_SPEC, 0> {
        L1_POS_W::new(self)
    }
    #[doc = "Bit 1 - Latch1 negative edge"]
    #[inline(always)]
    #[must_use]
    pub fn l1_neg(&mut self) -> L1_NEG_W<DC_LATCH1_CONT_SPEC, 1> {
        L1_NEG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Latch1 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_latch1_cont::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc_latch1_cont::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DC_LATCH1_CONT_SPEC;
impl crate::RegisterSpec for DC_LATCH1_CONT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dc_latch1_cont::R`](R) reader structure"]
impl crate::Readable for DC_LATCH1_CONT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dc_latch1_cont::W`](W) writer structure"]
impl crate::Writable for DC_LATCH1_CONT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DC_LATCH1_CONT to value 0"]
impl crate::Resettable for DC_LATCH1_CONT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
