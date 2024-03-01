#[doc = "Register `PCON` reader"]
pub type R = crate::R<PconSpec>;
#[doc = "Register `PCON` writer"]
pub type W = crate::W<PconSpec>;
#[doc = "Instruction Prefetch Buffer Bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ibyp {
    #[doc = "0: Instruction prefetch buffer not bypassed."]
    Const0 = 0,
    #[doc = "1: Instruction prefetch buffer bypassed."]
    Const1 = 1,
}
impl From<Ibyp> for bool {
    #[inline(always)]
    fn from(variant: Ibyp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IBYP` reader - Instruction Prefetch Buffer Bypass"]
pub type IbypR = crate::BitReader<Ibyp>;
impl IbypR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ibyp {
        match self.bits {
            false => Ibyp::Const0,
            true => Ibyp::Const1,
        }
    }
    #[doc = "Instruction prefetch buffer not bypassed."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Ibyp::Const0
    }
    #[doc = "Instruction prefetch buffer bypassed."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Ibyp::Const1
    }
}
#[doc = "Field `IBYP` writer - Instruction Prefetch Buffer Bypass"]
pub type IbypW<'a, REG> = crate::BitWriter<'a, REG, Ibyp>;
impl<'a, REG> IbypW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Instruction prefetch buffer not bypassed."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ibyp::Const0)
    }
    #[doc = "Instruction prefetch buffer bypassed."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ibyp::Const1)
    }
}
#[doc = "Instruction Prefetch Buffer Invalidate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iinv {
    #[doc = "0: No effect."]
    Const0 = 0,
    #[doc = "1: Initiate invalidation of entire instruction cache."]
    Const1 = 1,
}
impl From<Iinv> for bool {
    #[inline(always)]
    fn from(variant: Iinv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IINV` writer - Instruction Prefetch Buffer Invalidate"]
pub type IinvW<'a, REG> = crate::BitWriter<'a, REG, Iinv>;
impl<'a, REG> IinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Iinv::Const0)
    }
    #[doc = "Initiate invalidation of entire instruction cache."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Iinv::Const1)
    }
}
#[doc = "Data Buffer Bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbyp {
    #[doc = "0: Prefetch Data buffer not bypassed."]
    Value1 = 0,
    #[doc = "1: Prefetch Data buffer bypassed."]
    Value2 = 1,
}
impl From<Dbyp> for bool {
    #[inline(always)]
    fn from(variant: Dbyp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBYP` reader - Data Buffer Bypass"]
pub type DbypR = crate::BitReader<Dbyp>;
impl DbypR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbyp {
        match self.bits {
            false => Dbyp::Value1,
            true => Dbyp::Value2,
        }
    }
    #[doc = "Prefetch Data buffer not bypassed."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dbyp::Value1
    }
    #[doc = "Prefetch Data buffer bypassed."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dbyp::Value2
    }
}
#[doc = "Field `DBYP` writer - Data Buffer Bypass"]
pub type DbypW<'a, REG> = crate::BitWriter<'a, REG, Dbyp>;
impl<'a, REG> DbypW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Prefetch Data buffer not bypassed."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dbyp::Value1)
    }
    #[doc = "Prefetch Data buffer bypassed."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dbyp::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Instruction Prefetch Buffer Bypass"]
    #[inline(always)]
    pub fn ibyp(&self) -> IbypR {
        IbypR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Data Buffer Bypass"]
    #[inline(always)]
    pub fn dbyp(&self) -> DbypR {
        DbypR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Instruction Prefetch Buffer Bypass"]
    #[inline(always)]
    #[must_use]
    pub fn ibyp(&mut self) -> IbypW<PconSpec> {
        IbypW::new(self, 0)
    }
    #[doc = "Bit 1 - Instruction Prefetch Buffer Invalidate"]
    #[inline(always)]
    #[must_use]
    pub fn iinv(&mut self) -> IinvW<PconSpec> {
        IinvW::new(self, 1)
    }
    #[doc = "Bit 4 - Data Buffer Bypass"]
    #[inline(always)]
    #[must_use]
    pub fn dbyp(&mut self) -> DbypW<PconSpec> {
        DbypW::new(self, 4)
    }
}
#[doc = "Prefetch Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcon::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcon::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PconSpec;
impl crate::RegisterSpec for PconSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcon::R`](R) reader structure"]
impl crate::Readable for PconSpec {}
#[doc = "`write(|w| ..)` method takes [`pcon::W`](W) writer structure"]
impl crate::Writable for PconSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCON to value 0"]
impl crate::Resettable for PconSpec {
    const RESET_VALUE: u32 = 0;
}
