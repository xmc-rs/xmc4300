#[doc = "Register `SWS` writer"]
pub type W = crate::W<SWS_SPEC>;
#[doc = "Field `SPM` writer - Period match while counting up set"]
pub type SPM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOM` writer - One match while counting down set"]
pub type SOM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SCM1U` writer - Channel 1 Compare match while counting up set"]
pub type SCM1U_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SCM1D` writer - Channel 1 Compare match while counting down set"]
pub type SCM1D_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SCM2U` writer - Compare match while counting up set"]
pub type SCM2U_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SCM2D` writer - Compare match while counting down set"]
pub type SCM2D_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SE0A` writer - Event 0 detection set"]
pub type SE0A_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SE1A` writer - Event 1 detection set"]
pub type SE1A_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SE2A` writer - Event 2 detection set"]
pub type SE2A_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STRPF` writer - Trap Flag status set"]
pub type STRPF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Period match while counting up set"]
    #[inline(always)]
    #[must_use]
    pub fn spm(&mut self) -> SPM_W<SWS_SPEC, 0> {
        SPM_W::new(self)
    }
    #[doc = "Bit 1 - One match while counting down set"]
    #[inline(always)]
    #[must_use]
    pub fn som(&mut self) -> SOM_W<SWS_SPEC, 1> {
        SOM_W::new(self)
    }
    #[doc = "Bit 2 - Channel 1 Compare match while counting up set"]
    #[inline(always)]
    #[must_use]
    pub fn scm1u(&mut self) -> SCM1U_W<SWS_SPEC, 2> {
        SCM1U_W::new(self)
    }
    #[doc = "Bit 3 - Channel 1 Compare match while counting down set"]
    #[inline(always)]
    #[must_use]
    pub fn scm1d(&mut self) -> SCM1D_W<SWS_SPEC, 3> {
        SCM1D_W::new(self)
    }
    #[doc = "Bit 4 - Compare match while counting up set"]
    #[inline(always)]
    #[must_use]
    pub fn scm2u(&mut self) -> SCM2U_W<SWS_SPEC, 4> {
        SCM2U_W::new(self)
    }
    #[doc = "Bit 5 - Compare match while counting down set"]
    #[inline(always)]
    #[must_use]
    pub fn scm2d(&mut self) -> SCM2D_W<SWS_SPEC, 5> {
        SCM2D_W::new(self)
    }
    #[doc = "Bit 8 - Event 0 detection set"]
    #[inline(always)]
    #[must_use]
    pub fn se0a(&mut self) -> SE0A_W<SWS_SPEC, 8> {
        SE0A_W::new(self)
    }
    #[doc = "Bit 9 - Event 1 detection set"]
    #[inline(always)]
    #[must_use]
    pub fn se1a(&mut self) -> SE1A_W<SWS_SPEC, 9> {
        SE1A_W::new(self)
    }
    #[doc = "Bit 10 - Event 2 detection set"]
    #[inline(always)]
    #[must_use]
    pub fn se2a(&mut self) -> SE2A_W<SWS_SPEC, 10> {
        SE2A_W::new(self)
    }
    #[doc = "Bit 11 - Trap Flag status set"]
    #[inline(always)]
    #[must_use]
    pub fn strpf(&mut self) -> STRPF_W<SWS_SPEC, 11> {
        STRPF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Status Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sws::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWS_SPEC;
impl crate::RegisterSpec for SWS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sws::W`](W) writer structure"]
impl crate::Writable for SWS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWS to value 0"]
impl crate::Resettable for SWS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
