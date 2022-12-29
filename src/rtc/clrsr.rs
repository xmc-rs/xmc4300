#[doc = "Register `CLRSR` writer"]
pub struct W(crate::W<CLRSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLRSR_SPEC>;
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
impl From<crate::W<CLRSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLRSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RPSE` writer - Periodic Seconds Interrupt Clear"]
pub type RPSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRSR_SPEC, bool, O>;
#[doc = "Field `RPMI` writer - Periodic Minutes Interrupt Clear"]
pub type RPMI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRSR_SPEC, bool, O>;
#[doc = "Field `RPHO` writer - Periodic Hours Interrupt Clear"]
pub type RPHO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRSR_SPEC, bool, O>;
#[doc = "Field `RPDA` writer - Periodic Days Interrupt Clear"]
pub type RPDA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRSR_SPEC, bool, O>;
#[doc = "Field `RPMO` writer - Periodic Months Interrupt Clear"]
pub type RPMO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRSR_SPEC, bool, O>;
#[doc = "Field `RPYE` writer - Periodic Years Interrupt Clear"]
pub type RPYE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRSR_SPEC, bool, O>;
#[doc = "Field `RAI` writer - Alarm Interrupt Clear"]
pub type RAI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRSR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Periodic Seconds Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rpse(&mut self) -> RPSE_W<0> {
        RPSE_W::new(self)
    }
    #[doc = "Bit 1 - Periodic Minutes Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rpmi(&mut self) -> RPMI_W<1> {
        RPMI_W::new(self)
    }
    #[doc = "Bit 2 - Periodic Hours Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rpho(&mut self) -> RPHO_W<2> {
        RPHO_W::new(self)
    }
    #[doc = "Bit 3 - Periodic Days Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rpda(&mut self) -> RPDA_W<3> {
        RPDA_W::new(self)
    }
    #[doc = "Bit 5 - Periodic Months Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rpmo(&mut self) -> RPMO_W<5> {
        RPMO_W::new(self)
    }
    #[doc = "Bit 6 - Periodic Years Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rpye(&mut self) -> RPYE_W<6> {
        RPYE_W::new(self)
    }
    #[doc = "Bit 8 - Alarm Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rai(&mut self) -> RAI_W<8> {
        RAI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Clear Service Request Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clrsr](index.html) module"]
pub struct CLRSR_SPEC;
impl crate::RegisterSpec for CLRSR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [clrsr::W](W) writer structure"]
impl crate::Writable for CLRSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLRSR to value 0"]
impl crate::Resettable for CLRSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
