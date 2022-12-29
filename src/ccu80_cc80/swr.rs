#[doc = "Register `SWR` writer"]
pub struct W(crate::W<SWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWR_SPEC>;
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
impl From<crate::W<SWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RPM` writer - Period match while counting up clear"]
pub type RPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWR_SPEC, bool, O>;
#[doc = "Field `ROM` writer - One match while counting down clear"]
pub type ROM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWR_SPEC, bool, O>;
#[doc = "Field `RCM1U` writer - Channel 1 Compare match while counting up clear"]
pub type RCM1U_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWR_SPEC, bool, O>;
#[doc = "Field `RCM1D` writer - Channel 1 Compare match while counting down clear"]
pub type RCM1D_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWR_SPEC, bool, O>;
#[doc = "Field `RCM2U` writer - Channel 2 Compare match while counting up clear"]
pub type RCM2U_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWR_SPEC, bool, O>;
#[doc = "Field `RCM2D` writer - Channel 2 Compare match while counting down clear"]
pub type RCM2D_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWR_SPEC, bool, O>;
#[doc = "Field `RE0A` writer - Event 0 detection clear"]
pub type RE0A_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWR_SPEC, bool, O>;
#[doc = "Field `RE1A` writer - Event 1 detection clear"]
pub type RE1A_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWR_SPEC, bool, O>;
#[doc = "Field `RE2A` writer - Event 2 detection clear"]
pub type RE2A_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWR_SPEC, bool, O>;
#[doc = "Field `RTRPF` writer - Trap Flag status clear"]
pub type RTRPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Period match while counting up clear"]
    #[inline(always)]
    #[must_use]
    pub fn rpm(&mut self) -> RPM_W<0> {
        RPM_W::new(self)
    }
    #[doc = "Bit 1 - One match while counting down clear"]
    #[inline(always)]
    #[must_use]
    pub fn rom(&mut self) -> ROM_W<1> {
        ROM_W::new(self)
    }
    #[doc = "Bit 2 - Channel 1 Compare match while counting up clear"]
    #[inline(always)]
    #[must_use]
    pub fn rcm1u(&mut self) -> RCM1U_W<2> {
        RCM1U_W::new(self)
    }
    #[doc = "Bit 3 - Channel 1 Compare match while counting down clear"]
    #[inline(always)]
    #[must_use]
    pub fn rcm1d(&mut self) -> RCM1D_W<3> {
        RCM1D_W::new(self)
    }
    #[doc = "Bit 4 - Channel 2 Compare match while counting up clear"]
    #[inline(always)]
    #[must_use]
    pub fn rcm2u(&mut self) -> RCM2U_W<4> {
        RCM2U_W::new(self)
    }
    #[doc = "Bit 5 - Channel 2 Compare match while counting down clear"]
    #[inline(always)]
    #[must_use]
    pub fn rcm2d(&mut self) -> RCM2D_W<5> {
        RCM2D_W::new(self)
    }
    #[doc = "Bit 8 - Event 0 detection clear"]
    #[inline(always)]
    #[must_use]
    pub fn re0a(&mut self) -> RE0A_W<8> {
        RE0A_W::new(self)
    }
    #[doc = "Bit 9 - Event 1 detection clear"]
    #[inline(always)]
    #[must_use]
    pub fn re1a(&mut self) -> RE1A_W<9> {
        RE1A_W::new(self)
    }
    #[doc = "Bit 10 - Event 2 detection clear"]
    #[inline(always)]
    #[must_use]
    pub fn re2a(&mut self) -> RE2A_W<10> {
        RE2A_W::new(self)
    }
    #[doc = "Bit 11 - Trap Flag status clear"]
    #[inline(always)]
    #[must_use]
    pub fn rtrpf(&mut self) -> RTRPF_W<11> {
        RTRPF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Status Clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swr](index.html) module"]
pub struct SWR_SPEC;
impl crate::RegisterSpec for SWR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [swr::W](W) writer structure"]
impl crate::Writable for SWR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWR to value 0"]
impl crate::Resettable for SWR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
