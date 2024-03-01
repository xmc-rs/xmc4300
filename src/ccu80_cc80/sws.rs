#[doc = "Register `SWS` writer"]
pub type W = crate::W<SwsSpec>;
#[doc = "Field `SPM` writer - Period match while counting up set"]
pub type SpmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOM` writer - One match while counting down set"]
pub type SomW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCM1U` writer - Channel 1 Compare match while counting up set"]
pub type Scm1uW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCM1D` writer - Channel 1 Compare match while counting down set"]
pub type Scm1dW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCM2U` writer - Compare match while counting up set"]
pub type Scm2uW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCM2D` writer - Compare match while counting down set"]
pub type Scm2dW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SE0A` writer - Event 0 detection set"]
pub type Se0aW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SE1A` writer - Event 1 detection set"]
pub type Se1aW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SE2A` writer - Event 2 detection set"]
pub type Se2aW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRPF` writer - Trap Flag status set"]
pub type StrpfW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Period match while counting up set"]
    #[inline(always)]
    #[must_use]
    pub fn spm(&mut self) -> SpmW<SwsSpec> {
        SpmW::new(self, 0)
    }
    #[doc = "Bit 1 - One match while counting down set"]
    #[inline(always)]
    #[must_use]
    pub fn som(&mut self) -> SomW<SwsSpec> {
        SomW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 1 Compare match while counting up set"]
    #[inline(always)]
    #[must_use]
    pub fn scm1u(&mut self) -> Scm1uW<SwsSpec> {
        Scm1uW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 1 Compare match while counting down set"]
    #[inline(always)]
    #[must_use]
    pub fn scm1d(&mut self) -> Scm1dW<SwsSpec> {
        Scm1dW::new(self, 3)
    }
    #[doc = "Bit 4 - Compare match while counting up set"]
    #[inline(always)]
    #[must_use]
    pub fn scm2u(&mut self) -> Scm2uW<SwsSpec> {
        Scm2uW::new(self, 4)
    }
    #[doc = "Bit 5 - Compare match while counting down set"]
    #[inline(always)]
    #[must_use]
    pub fn scm2d(&mut self) -> Scm2dW<SwsSpec> {
        Scm2dW::new(self, 5)
    }
    #[doc = "Bit 8 - Event 0 detection set"]
    #[inline(always)]
    #[must_use]
    pub fn se0a(&mut self) -> Se0aW<SwsSpec> {
        Se0aW::new(self, 8)
    }
    #[doc = "Bit 9 - Event 1 detection set"]
    #[inline(always)]
    #[must_use]
    pub fn se1a(&mut self) -> Se1aW<SwsSpec> {
        Se1aW::new(self, 9)
    }
    #[doc = "Bit 10 - Event 2 detection set"]
    #[inline(always)]
    #[must_use]
    pub fn se2a(&mut self) -> Se2aW<SwsSpec> {
        Se2aW::new(self, 10)
    }
    #[doc = "Bit 11 - Trap Flag status set"]
    #[inline(always)]
    #[must_use]
    pub fn strpf(&mut self) -> StrpfW<SwsSpec> {
        StrpfW::new(self, 11)
    }
}
#[doc = "Interrupt Status Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sws::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwsSpec;
impl crate::RegisterSpec for SwsSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sws::W`](W) writer structure"]
impl crate::Writable for SwsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWS to value 0"]
impl crate::Resettable for SwsSpec {
    const RESET_VALUE: u32 = 0;
}
