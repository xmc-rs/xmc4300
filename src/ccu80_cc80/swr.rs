#[doc = "Register `SWR` writer"]
pub type W = crate::W<SwrSpec>;
#[doc = "Field `RPM` writer - Period match while counting up clear"]
pub type RpmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROM` writer - One match while counting down clear"]
pub type RomW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCM1U` writer - Channel 1 Compare match while counting up clear"]
pub type Rcm1uW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCM1D` writer - Channel 1 Compare match while counting down clear"]
pub type Rcm1dW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCM2U` writer - Channel 2 Compare match while counting up clear"]
pub type Rcm2uW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCM2D` writer - Channel 2 Compare match while counting down clear"]
pub type Rcm2dW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RE0A` writer - Event 0 detection clear"]
pub type Re0aW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RE1A` writer - Event 1 detection clear"]
pub type Re1aW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RE2A` writer - Event 2 detection clear"]
pub type Re2aW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTRPF` writer - Trap Flag status clear"]
pub type RtrpfW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Period match while counting up clear"]
    #[inline(always)]
    #[must_use]
    pub fn rpm(&mut self) -> RpmW<SwrSpec> {
        RpmW::new(self, 0)
    }
    #[doc = "Bit 1 - One match while counting down clear"]
    #[inline(always)]
    #[must_use]
    pub fn rom(&mut self) -> RomW<SwrSpec> {
        RomW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 1 Compare match while counting up clear"]
    #[inline(always)]
    #[must_use]
    pub fn rcm1u(&mut self) -> Rcm1uW<SwrSpec> {
        Rcm1uW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 1 Compare match while counting down clear"]
    #[inline(always)]
    #[must_use]
    pub fn rcm1d(&mut self) -> Rcm1dW<SwrSpec> {
        Rcm1dW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 2 Compare match while counting up clear"]
    #[inline(always)]
    #[must_use]
    pub fn rcm2u(&mut self) -> Rcm2uW<SwrSpec> {
        Rcm2uW::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 2 Compare match while counting down clear"]
    #[inline(always)]
    #[must_use]
    pub fn rcm2d(&mut self) -> Rcm2dW<SwrSpec> {
        Rcm2dW::new(self, 5)
    }
    #[doc = "Bit 8 - Event 0 detection clear"]
    #[inline(always)]
    #[must_use]
    pub fn re0a(&mut self) -> Re0aW<SwrSpec> {
        Re0aW::new(self, 8)
    }
    #[doc = "Bit 9 - Event 1 detection clear"]
    #[inline(always)]
    #[must_use]
    pub fn re1a(&mut self) -> Re1aW<SwrSpec> {
        Re1aW::new(self, 9)
    }
    #[doc = "Bit 10 - Event 2 detection clear"]
    #[inline(always)]
    #[must_use]
    pub fn re2a(&mut self) -> Re2aW<SwrSpec> {
        Re2aW::new(self, 10)
    }
    #[doc = "Bit 11 - Trap Flag status clear"]
    #[inline(always)]
    #[must_use]
    pub fn rtrpf(&mut self) -> RtrpfW<SwrSpec> {
        RtrpfW::new(self, 11)
    }
}
#[doc = "Interrupt Status Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwrSpec;
impl crate::RegisterSpec for SwrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swr::W`](W) writer structure"]
impl crate::Writable for SwrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWR to value 0"]
impl crate::Resettable for SwrSpec {
    const RESET_VALUE: u32 = 0;
}
