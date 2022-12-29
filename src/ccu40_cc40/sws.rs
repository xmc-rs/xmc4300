#[doc = "Register `SWS` writer"]
pub struct W(crate::W<SWS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SWS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPM` writer - Period match while counting up set"]
pub type SPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWS_SPEC, bool, O>;
#[doc = "Field `SOM` writer - One match while counting down set"]
pub type SOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWS_SPEC, bool, O>;
#[doc = "Field `SCMU` writer - Compare match while counting up set"]
pub type SCMU_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWS_SPEC, bool, O>;
#[doc = "Field `SCMD` writer - Compare match while counting down set"]
pub type SCMD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWS_SPEC, bool, O>;
#[doc = "Field `SE0A` writer - Event 0 detection set"]
pub type SE0A_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWS_SPEC, bool, O>;
#[doc = "Field `SE1A` writer - Event 1 detection set"]
pub type SE1A_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWS_SPEC, bool, O>;
#[doc = "Field `SE2A` writer - Event 2 detection set"]
pub type SE2A_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWS_SPEC, bool, O>;
#[doc = "Field `STRPF` writer - Trap Flag status set"]
pub type STRPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWS_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Period match while counting up set"]
    #[inline(always)]
    #[must_use]
    pub fn spm(&mut self) -> SPM_W<0> {
        SPM_W::new(self)
    }
    #[doc = "Bit 1 - One match while counting down set"]
    #[inline(always)]
    #[must_use]
    pub fn som(&mut self) -> SOM_W<1> {
        SOM_W::new(self)
    }
    #[doc = "Bit 2 - Compare match while counting up set"]
    #[inline(always)]
    #[must_use]
    pub fn scmu(&mut self) -> SCMU_W<2> {
        SCMU_W::new(self)
    }
    #[doc = "Bit 3 - Compare match while counting down set"]
    #[inline(always)]
    #[must_use]
    pub fn scmd(&mut self) -> SCMD_W<3> {
        SCMD_W::new(self)
    }
    #[doc = "Bit 8 - Event 0 detection set"]
    #[inline(always)]
    #[must_use]
    pub fn se0a(&mut self) -> SE0A_W<8> {
        SE0A_W::new(self)
    }
    #[doc = "Bit 9 - Event 1 detection set"]
    #[inline(always)]
    #[must_use]
    pub fn se1a(&mut self) -> SE1A_W<9> {
        SE1A_W::new(self)
    }
    #[doc = "Bit 10 - Event 2 detection set"]
    #[inline(always)]
    #[must_use]
    pub fn se2a(&mut self) -> SE2A_W<10> {
        SE2A_W::new(self)
    }
    #[doc = "Bit 11 - Trap Flag status set"]
    #[inline(always)]
    #[must_use]
    pub fn strpf(&mut self) -> STRPF_W<11> {
        STRPF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Status Set\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sws](index.html) module"]
pub struct SWS_SPEC;
impl crate::RegisterSpec for SWS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [sws::W](W) writer structure"]
impl crate::Writable for SWS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWS to value 0"]
impl crate::Resettable for SWS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
