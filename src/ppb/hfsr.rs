#[doc = "Register `HFSR` reader"]
pub type R = crate::R<HfsrSpec>;
#[doc = "Register `HFSR` writer"]
pub type W = crate::W<HfsrSpec>;
#[doc = "BusFault on vector table read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vecttbl {
    #[doc = "0: no BusFault on vector table read"]
    Value1 = 0,
    #[doc = "1: BusFault on vector table read"]
    Value2 = 1,
}
impl From<Vecttbl> for bool {
    #[inline(always)]
    fn from(variant: Vecttbl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VECTTBL` reader - BusFault on vector table read"]
pub type VecttblR = crate::BitReader<Vecttbl>;
impl VecttblR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vecttbl {
        match self.bits {
            false => Vecttbl::Value1,
            true => Vecttbl::Value2,
        }
    }
    #[doc = "no BusFault on vector table read"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Vecttbl::Value1
    }
    #[doc = "BusFault on vector table read"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Vecttbl::Value2
    }
}
#[doc = "Field `VECTTBL` writer - BusFault on vector table read"]
pub type VecttblW<'a, REG> = crate::BitWriter<'a, REG, Vecttbl>;
impl<'a, REG> VecttblW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no BusFault on vector table read"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Vecttbl::Value1)
    }
    #[doc = "BusFault on vector table read"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Vecttbl::Value2)
    }
}
#[doc = "Forced HardFault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Forced {
    #[doc = "0: no forced HardFault"]
    Value1 = 0,
    #[doc = "1: forced HardFault."]
    Value2 = 1,
}
impl From<Forced> for bool {
    #[inline(always)]
    fn from(variant: Forced) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCED` reader - Forced HardFault"]
pub type ForcedR = crate::BitReader<Forced>;
impl ForcedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Forced {
        match self.bits {
            false => Forced::Value1,
            true => Forced::Value2,
        }
    }
    #[doc = "no forced HardFault"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Forced::Value1
    }
    #[doc = "forced HardFault."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Forced::Value2
    }
}
#[doc = "Field `FORCED` writer - Forced HardFault"]
pub type ForcedW<'a, REG> = crate::BitWriter<'a, REG, Forced>;
impl<'a, REG> ForcedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no forced HardFault"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Forced::Value1)
    }
    #[doc = "forced HardFault."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Forced::Value2)
    }
}
#[doc = "Field `DEBUGEVT` reader - Reserved for Debug use"]
pub type DebugevtR = crate::BitReader;
#[doc = "Field `DEBUGEVT` writer - Reserved for Debug use"]
pub type DebugevtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - BusFault on vector table read"]
    #[inline(always)]
    pub fn vecttbl(&self) -> VecttblR {
        VecttblR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 30 - Forced HardFault"]
    #[inline(always)]
    pub fn forced(&self) -> ForcedR {
        ForcedR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Reserved for Debug use"]
    #[inline(always)]
    pub fn debugevt(&self) -> DebugevtR {
        DebugevtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - BusFault on vector table read"]
    #[inline(always)]
    #[must_use]
    pub fn vecttbl(&mut self) -> VecttblW<HfsrSpec> {
        VecttblW::new(self, 1)
    }
    #[doc = "Bit 30 - Forced HardFault"]
    #[inline(always)]
    #[must_use]
    pub fn forced(&mut self) -> ForcedW<HfsrSpec> {
        ForcedW::new(self, 30)
    }
    #[doc = "Bit 31 - Reserved for Debug use"]
    #[inline(always)]
    #[must_use]
    pub fn debugevt(&mut self) -> DebugevtW<HfsrSpec> {
        DebugevtW::new(self, 31)
    }
}
#[doc = "HardFault Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfsrSpec;
impl crate::RegisterSpec for HfsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfsr::R`](R) reader structure"]
impl crate::Readable for HfsrSpec {}
#[doc = "`write(|w| ..)` method takes [`hfsr::W`](W) writer structure"]
impl crate::Writable for HfsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HFSR to value 0"]
impl crate::Resettable for HfsrSpec {
    const RESET_VALUE: u32 = 0;
}
