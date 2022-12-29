#[doc = "Register `GIDLS` writer"]
pub struct W(crate::W<GIDLS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GIDLS_SPEC>;
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
impl From<crate::W<GIDLS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GIDLS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SS0I` writer - CC80 IDLE mode set"]
pub type SS0I_W<'a, const O: u8> = crate::BitWriter<'a, u32, GIDLS_SPEC, bool, O>;
#[doc = "Field `SS1I` writer - CC81 IDLE mode set"]
pub type SS1I_W<'a, const O: u8> = crate::BitWriter<'a, u32, GIDLS_SPEC, bool, O>;
#[doc = "Field `SS2I` writer - CC82 IDLE mode set"]
pub type SS2I_W<'a, const O: u8> = crate::BitWriter<'a, u32, GIDLS_SPEC, bool, O>;
#[doc = "Field `SS3I` writer - CC83 IDLE mode set"]
pub type SS3I_W<'a, const O: u8> = crate::BitWriter<'a, u32, GIDLS_SPEC, bool, O>;
#[doc = "Field `CPRB` writer - Prescaler# Run Bit Clear"]
pub type CPRB_W<'a, const O: u8> = crate::BitWriter<'a, u32, GIDLS_SPEC, bool, O>;
#[doc = "Field `PSIC` writer - Prescaler clear"]
pub type PSIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GIDLS_SPEC, bool, O>;
#[doc = "Field `CPCH` writer - Parity Checker Run bit clear"]
pub type CPCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, GIDLS_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - CC80 IDLE mode set"]
    #[inline(always)]
    #[must_use]
    pub fn ss0i(&mut self) -> SS0I_W<0> {
        SS0I_W::new(self)
    }
    #[doc = "Bit 1 - CC81 IDLE mode set"]
    #[inline(always)]
    #[must_use]
    pub fn ss1i(&mut self) -> SS1I_W<1> {
        SS1I_W::new(self)
    }
    #[doc = "Bit 2 - CC82 IDLE mode set"]
    #[inline(always)]
    #[must_use]
    pub fn ss2i(&mut self) -> SS2I_W<2> {
        SS2I_W::new(self)
    }
    #[doc = "Bit 3 - CC83 IDLE mode set"]
    #[inline(always)]
    #[must_use]
    pub fn ss3i(&mut self) -> SS3I_W<3> {
        SS3I_W::new(self)
    }
    #[doc = "Bit 8 - Prescaler# Run Bit Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cprb(&mut self) -> CPRB_W<8> {
        CPRB_W::new(self)
    }
    #[doc = "Bit 9 - Prescaler clear"]
    #[inline(always)]
    #[must_use]
    pub fn psic(&mut self) -> PSIC_W<9> {
        PSIC_W::new(self)
    }
    #[doc = "Bit 10 - Parity Checker Run bit clear"]
    #[inline(always)]
    #[must_use]
    pub fn cpch(&mut self) -> CPCH_W<10> {
        CPCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Idle Set\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gidls](index.html) module"]
pub struct GIDLS_SPEC;
impl crate::RegisterSpec for GIDLS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [gidls::W](W) writer structure"]
impl crate::Writable for GIDLS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GIDLS to value 0"]
impl crate::Resettable for GIDLS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
