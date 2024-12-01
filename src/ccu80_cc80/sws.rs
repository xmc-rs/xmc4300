#[doc = "Register `SWS` writer"]
pub type W = crate::W<SWS_SPEC>;
#[doc = "Field `SPM` writer - Period match while counting up set"]
pub type SPM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOM` writer - One match while counting down set"]
pub type SOM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCM1U` writer - Channel 1 Compare match while counting up set"]
pub type SCM1U_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCM1D` writer - Channel 1 Compare match while counting down set"]
pub type SCM1D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCM2U` writer - Compare match while counting up set"]
pub type SCM2U_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCM2D` writer - Compare match while counting down set"]
pub type SCM2D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SE0A` writer - Event 0 detection set"]
pub type SE0A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SE1A` writer - Event 1 detection set"]
pub type SE1A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SE2A` writer - Event 2 detection set"]
pub type SE2A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRPF` writer - Trap Flag status set"]
pub type STRPF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Period match while counting up set"]
    #[inline(always)]
    pub fn spm(&mut self) -> SPM_W<SWS_SPEC> {
        SPM_W::new(self, 0)
    }
    #[doc = "Bit 1 - One match while counting down set"]
    #[inline(always)]
    pub fn som(&mut self) -> SOM_W<SWS_SPEC> {
        SOM_W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 1 Compare match while counting up set"]
    #[inline(always)]
    pub fn scm1u(&mut self) -> SCM1U_W<SWS_SPEC> {
        SCM1U_W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 1 Compare match while counting down set"]
    #[inline(always)]
    pub fn scm1d(&mut self) -> SCM1D_W<SWS_SPEC> {
        SCM1D_W::new(self, 3)
    }
    #[doc = "Bit 4 - Compare match while counting up set"]
    #[inline(always)]
    pub fn scm2u(&mut self) -> SCM2U_W<SWS_SPEC> {
        SCM2U_W::new(self, 4)
    }
    #[doc = "Bit 5 - Compare match while counting down set"]
    #[inline(always)]
    pub fn scm2d(&mut self) -> SCM2D_W<SWS_SPEC> {
        SCM2D_W::new(self, 5)
    }
    #[doc = "Bit 8 - Event 0 detection set"]
    #[inline(always)]
    pub fn se0a(&mut self) -> SE0A_W<SWS_SPEC> {
        SE0A_W::new(self, 8)
    }
    #[doc = "Bit 9 - Event 1 detection set"]
    #[inline(always)]
    pub fn se1a(&mut self) -> SE1A_W<SWS_SPEC> {
        SE1A_W::new(self, 9)
    }
    #[doc = "Bit 10 - Event 2 detection set"]
    #[inline(always)]
    pub fn se2a(&mut self) -> SE2A_W<SWS_SPEC> {
        SE2A_W::new(self, 10)
    }
    #[doc = "Bit 11 - Trap Flag status set"]
    #[inline(always)]
    pub fn strpf(&mut self) -> STRPF_W<SWS_SPEC> {
        STRPF_W::new(self, 11)
    }
}
#[doc = "Interrupt Status Set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sws::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWS_SPEC;
impl crate::RegisterSpec for SWS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sws::W`](W) writer structure"]
impl crate::Writable for SWS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWS to value 0"]
impl crate::Resettable for SWS_SPEC {
    const RESET_VALUE: u32 = 0;
}
