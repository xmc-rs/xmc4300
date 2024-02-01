#[doc = "Register `PCON` reader"]
pub type R = crate::R<PCON_SPEC>;
#[doc = "Register `PCON` writer"]
pub type W = crate::W<PCON_SPEC>;
#[doc = "Field `IBYP` reader - Instruction Prefetch Buffer Bypass"]
pub type IBYP_R = crate::BitReader<IBYP_A>;
#[doc = "Instruction Prefetch Buffer Bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IBYP_A {
    #[doc = "0: Instruction prefetch buffer not bypassed."]
    CONST_0 = 0,
    #[doc = "1: Instruction prefetch buffer bypassed."]
    CONST_1 = 1,
}
impl From<IBYP_A> for bool {
    #[inline(always)]
    fn from(variant: IBYP_A) -> Self {
        variant as u8 != 0
    }
}
impl IBYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IBYP_A {
        match self.bits {
            false => IBYP_A::CONST_0,
            true => IBYP_A::CONST_1,
        }
    }
    #[doc = "Instruction prefetch buffer not bypassed."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == IBYP_A::CONST_0
    }
    #[doc = "Instruction prefetch buffer bypassed."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == IBYP_A::CONST_1
    }
}
#[doc = "Field `IBYP` writer - Instruction Prefetch Buffer Bypass"]
pub type IBYP_W<'a, REG> = crate::BitWriter<'a, REG, IBYP_A>;
impl<'a, REG> IBYP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Instruction prefetch buffer not bypassed."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(IBYP_A::CONST_0)
    }
    #[doc = "Instruction prefetch buffer bypassed."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(IBYP_A::CONST_1)
    }
}
#[doc = "Instruction Prefetch Buffer Invalidate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IINV_AW {
    #[doc = "0: No effect."]
    CONST_0 = 0,
    #[doc = "1: Initiate invalidation of entire instruction cache."]
    CONST_1 = 1,
}
impl From<IINV_AW> for bool {
    #[inline(always)]
    fn from(variant: IINV_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IINV` writer - Instruction Prefetch Buffer Invalidate"]
pub type IINV_W<'a, REG> = crate::BitWriter<'a, REG, IINV_AW>;
impl<'a, REG> IINV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(IINV_AW::CONST_0)
    }
    #[doc = "Initiate invalidation of entire instruction cache."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(IINV_AW::CONST_1)
    }
}
#[doc = "Field `DBYP` reader - Data Buffer Bypass"]
pub type DBYP_R = crate::BitReader<DBYP_A>;
#[doc = "Data Buffer Bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBYP_A {
    #[doc = "0: Prefetch Data buffer not bypassed."]
    VALUE1 = 0,
    #[doc = "1: Prefetch Data buffer bypassed."]
    VALUE2 = 1,
}
impl From<DBYP_A> for bool {
    #[inline(always)]
    fn from(variant: DBYP_A) -> Self {
        variant as u8 != 0
    }
}
impl DBYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBYP_A {
        match self.bits {
            false => DBYP_A::VALUE1,
            true => DBYP_A::VALUE2,
        }
    }
    #[doc = "Prefetch Data buffer not bypassed."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DBYP_A::VALUE1
    }
    #[doc = "Prefetch Data buffer bypassed."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DBYP_A::VALUE2
    }
}
#[doc = "Field `DBYP` writer - Data Buffer Bypass"]
pub type DBYP_W<'a, REG> = crate::BitWriter<'a, REG, DBYP_A>;
impl<'a, REG> DBYP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Prefetch Data buffer not bypassed."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DBYP_A::VALUE1)
    }
    #[doc = "Prefetch Data buffer bypassed."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DBYP_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Instruction Prefetch Buffer Bypass"]
    #[inline(always)]
    pub fn ibyp(&self) -> IBYP_R {
        IBYP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Data Buffer Bypass"]
    #[inline(always)]
    pub fn dbyp(&self) -> DBYP_R {
        DBYP_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Instruction Prefetch Buffer Bypass"]
    #[inline(always)]
    #[must_use]
    pub fn ibyp(&mut self) -> IBYP_W<PCON_SPEC> {
        IBYP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Instruction Prefetch Buffer Invalidate"]
    #[inline(always)]
    #[must_use]
    pub fn iinv(&mut self) -> IINV_W<PCON_SPEC> {
        IINV_W::new(self, 1)
    }
    #[doc = "Bit 4 - Data Buffer Bypass"]
    #[inline(always)]
    #[must_use]
    pub fn dbyp(&mut self) -> DBYP_W<PCON_SPEC> {
        DBYP_W::new(self, 4)
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
#[doc = "Prefetch Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcon::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcon::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCON_SPEC;
impl crate::RegisterSpec for PCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcon::R`](R) reader structure"]
impl crate::Readable for PCON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcon::W`](W) writer structure"]
impl crate::Writable for PCON_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCON to value 0"]
impl crate::Resettable for PCON_SPEC {
    const RESET_VALUE: u32 = 0;
}
