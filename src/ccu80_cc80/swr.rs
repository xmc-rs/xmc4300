#[doc = "Register `SWR` writer"]
pub type W = crate::W<SWR_SPEC>;
#[doc = "Field `RPM` writer - Period match while counting up clear"]
pub type RPM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROM` writer - One match while counting down clear"]
pub type ROM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCM1U` writer - Channel 1 Compare match while counting up clear"]
pub type RCM1U_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCM1D` writer - Channel 1 Compare match while counting down clear"]
pub type RCM1D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCM2U` writer - Channel 2 Compare match while counting up clear"]
pub type RCM2U_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCM2D` writer - Channel 2 Compare match while counting down clear"]
pub type RCM2D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RE0A` writer - Event 0 detection clear"]
pub type RE0A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RE1A` writer - Event 1 detection clear"]
pub type RE1A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RE2A` writer - Event 2 detection clear"]
pub type RE2A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTRPF` writer - Trap Flag status clear"]
pub type RTRPF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Period match while counting up clear"]
    #[inline(always)]
    #[must_use]
    pub fn rpm(&mut self) -> RPM_W<SWR_SPEC> {
        RPM_W::new(self, 0)
    }
    #[doc = "Bit 1 - One match while counting down clear"]
    #[inline(always)]
    #[must_use]
    pub fn rom(&mut self) -> ROM_W<SWR_SPEC> {
        ROM_W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 1 Compare match while counting up clear"]
    #[inline(always)]
    #[must_use]
    pub fn rcm1u(&mut self) -> RCM1U_W<SWR_SPEC> {
        RCM1U_W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 1 Compare match while counting down clear"]
    #[inline(always)]
    #[must_use]
    pub fn rcm1d(&mut self) -> RCM1D_W<SWR_SPEC> {
        RCM1D_W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 2 Compare match while counting up clear"]
    #[inline(always)]
    #[must_use]
    pub fn rcm2u(&mut self) -> RCM2U_W<SWR_SPEC> {
        RCM2U_W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 2 Compare match while counting down clear"]
    #[inline(always)]
    #[must_use]
    pub fn rcm2d(&mut self) -> RCM2D_W<SWR_SPEC> {
        RCM2D_W::new(self, 5)
    }
    #[doc = "Bit 8 - Event 0 detection clear"]
    #[inline(always)]
    #[must_use]
    pub fn re0a(&mut self) -> RE0A_W<SWR_SPEC> {
        RE0A_W::new(self, 8)
    }
    #[doc = "Bit 9 - Event 1 detection clear"]
    #[inline(always)]
    #[must_use]
    pub fn re1a(&mut self) -> RE1A_W<SWR_SPEC> {
        RE1A_W::new(self, 9)
    }
    #[doc = "Bit 10 - Event 2 detection clear"]
    #[inline(always)]
    #[must_use]
    pub fn re2a(&mut self) -> RE2A_W<SWR_SPEC> {
        RE2A_W::new(self, 10)
    }
    #[doc = "Bit 11 - Trap Flag status clear"]
    #[inline(always)]
    #[must_use]
    pub fn rtrpf(&mut self) -> RTRPF_W<SWR_SPEC> {
        RTRPF_W::new(self, 11)
    }
}
#[doc = "Interrupt Status Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWR_SPEC;
impl crate::RegisterSpec for SWR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swr::W`](W) writer structure"]
impl crate::Writable for SWR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWR to value 0"]
impl crate::Resettable for SWR_SPEC {
    const RESET_VALUE: u32 = 0;
}
