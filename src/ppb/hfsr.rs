#[doc = "Register `HFSR` reader"]
pub type R = crate::R<HFSR_SPEC>;
#[doc = "Register `HFSR` writer"]
pub type W = crate::W<HFSR_SPEC>;
#[doc = "BusFault on vector table read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VECTTBL_A {
    #[doc = "0: no BusFault on vector table read"]
    VALUE1 = 0,
    #[doc = "1: BusFault on vector table read"]
    VALUE2 = 1,
}
impl From<VECTTBL_A> for bool {
    #[inline(always)]
    fn from(variant: VECTTBL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VECTTBL` reader - BusFault on vector table read"]
pub type VECTTBL_R = crate::BitReader<VECTTBL_A>;
impl VECTTBL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VECTTBL_A {
        match self.bits {
            false => VECTTBL_A::VALUE1,
            true => VECTTBL_A::VALUE2,
        }
    }
    #[doc = "no BusFault on vector table read"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VECTTBL_A::VALUE1
    }
    #[doc = "BusFault on vector table read"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VECTTBL_A::VALUE2
    }
}
#[doc = "Field `VECTTBL` writer - BusFault on vector table read"]
pub type VECTTBL_W<'a, REG> = crate::BitWriter<'a, REG, VECTTBL_A>;
impl<'a, REG> VECTTBL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no BusFault on vector table read"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VECTTBL_A::VALUE1)
    }
    #[doc = "BusFault on vector table read"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(VECTTBL_A::VALUE2)
    }
}
#[doc = "Forced HardFault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FORCED_A {
    #[doc = "0: no forced HardFault"]
    VALUE1 = 0,
    #[doc = "1: forced HardFault."]
    VALUE2 = 1,
}
impl From<FORCED_A> for bool {
    #[inline(always)]
    fn from(variant: FORCED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCED` reader - Forced HardFault"]
pub type FORCED_R = crate::BitReader<FORCED_A>;
impl FORCED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FORCED_A {
        match self.bits {
            false => FORCED_A::VALUE1,
            true => FORCED_A::VALUE2,
        }
    }
    #[doc = "no forced HardFault"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FORCED_A::VALUE1
    }
    #[doc = "forced HardFault."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FORCED_A::VALUE2
    }
}
#[doc = "Field `FORCED` writer - Forced HardFault"]
pub type FORCED_W<'a, REG> = crate::BitWriter<'a, REG, FORCED_A>;
impl<'a, REG> FORCED_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no forced HardFault"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FORCED_A::VALUE1)
    }
    #[doc = "forced HardFault."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FORCED_A::VALUE2)
    }
}
#[doc = "Field `DEBUGEVT` reader - Reserved for Debug use"]
pub type DEBUGEVT_R = crate::BitReader;
#[doc = "Field `DEBUGEVT` writer - Reserved for Debug use"]
pub type DEBUGEVT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - BusFault on vector table read"]
    #[inline(always)]
    pub fn vecttbl(&self) -> VECTTBL_R {
        VECTTBL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 30 - Forced HardFault"]
    #[inline(always)]
    pub fn forced(&self) -> FORCED_R {
        FORCED_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Reserved for Debug use"]
    #[inline(always)]
    pub fn debugevt(&self) -> DEBUGEVT_R {
        DEBUGEVT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - BusFault on vector table read"]
    #[inline(always)]
    #[must_use]
    pub fn vecttbl(&mut self) -> VECTTBL_W<HFSR_SPEC> {
        VECTTBL_W::new(self, 1)
    }
    #[doc = "Bit 30 - Forced HardFault"]
    #[inline(always)]
    #[must_use]
    pub fn forced(&mut self) -> FORCED_W<HFSR_SPEC> {
        FORCED_W::new(self, 30)
    }
    #[doc = "Bit 31 - Reserved for Debug use"]
    #[inline(always)]
    #[must_use]
    pub fn debugevt(&mut self) -> DEBUGEVT_W<HFSR_SPEC> {
        DEBUGEVT_W::new(self, 31)
    }
}
#[doc = "HardFault Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hfsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFSR_SPEC;
impl crate::RegisterSpec for HFSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfsr::R`](R) reader structure"]
impl crate::Readable for HFSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hfsr::W`](W) writer structure"]
impl crate::Writable for HFSR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HFSR to value 0"]
impl crate::Resettable for HFSR_SPEC {
    const RESET_VALUE: u32 = 0;
}
