#[doc = "Register `SEFLAG` reader"]
pub type R = crate::R<SeflagSpec>;
#[doc = "Register `SEFLAG` writer"]
pub type W = crate::W<SeflagSpec>;
#[doc = "Source Event 0/1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sev0 {
    #[doc = "0: No source event"]
    Value1 = 0,
    #[doc = "1: A source event has occurred"]
    Value2 = 1,
}
impl From<Sev0> for bool {
    #[inline(always)]
    fn from(variant: Sev0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEV0` reader - Source Event 0/1"]
pub type Sev0R = crate::BitReader<Sev0>;
impl Sev0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sev0 {
        match self.bits {
            false => Sev0::Value1,
            true => Sev0::Value2,
        }
    }
    #[doc = "No source event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sev0::Value1
    }
    #[doc = "A source event has occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sev0::Value2
    }
}
#[doc = "Field `SEV0` writer - Source Event 0/1"]
pub type Sev0W<'a, REG> = crate::BitWriter<'a, REG, Sev0>;
impl<'a, REG> Sev0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No source event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Sev0::Value1)
    }
    #[doc = "A source event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Sev0::Value2)
    }
}
#[doc = "Source Event 0/1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sev1 {
    #[doc = "0: No source event"]
    Value1 = 0,
    #[doc = "1: A source event has occurred"]
    Value2 = 1,
}
impl From<Sev1> for bool {
    #[inline(always)]
    fn from(variant: Sev1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEV1` reader - Source Event 0/1"]
pub type Sev1R = crate::BitReader<Sev1>;
impl Sev1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sev1 {
        match self.bits {
            false => Sev1::Value1,
            true => Sev1::Value2,
        }
    }
    #[doc = "No source event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sev1::Value1
    }
    #[doc = "A source event has occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sev1::Value2
    }
}
#[doc = "Field `SEV1` writer - Source Event 0/1"]
pub type Sev1W<'a, REG> = crate::BitWriter<'a, REG, Sev1>;
impl<'a, REG> Sev1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No source event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Sev1::Value1)
    }
    #[doc = "A source event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Sev1::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Source Event 0/1"]
    #[inline(always)]
    pub fn sev0(&self) -> Sev0R {
        Sev0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Source Event 0/1"]
    #[inline(always)]
    pub fn sev1(&self) -> Sev1R {
        Sev1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Source Event 0/1"]
    #[inline(always)]
    #[must_use]
    pub fn sev0(&mut self) -> Sev0W<SeflagSpec> {
        Sev0W::new(self, 0)
    }
    #[doc = "Bit 1 - Source Event 0/1"]
    #[inline(always)]
    #[must_use]
    pub fn sev1(&mut self) -> Sev1W<SeflagSpec> {
        Sev1W::new(self, 1)
    }
}
#[doc = "Source Event Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seflag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seflag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeflagSpec;
impl crate::RegisterSpec for SeflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seflag::R`](R) reader structure"]
impl crate::Readable for SeflagSpec {}
#[doc = "`write(|w| ..)` method takes [`seflag::W`](W) writer structure"]
impl crate::Writable for SeflagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEFLAG to value 0"]
impl crate::Resettable for SeflagSpec {
    const RESET_VALUE: u32 = 0;
}
