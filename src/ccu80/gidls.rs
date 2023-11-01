#[doc = "Register `GIDLS` writer"]
pub type W = crate::W<GIDLS_SPEC>;
#[doc = "Field `SS0I` writer - CC80 IDLE mode set"]
pub type SS0I_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SS1I` writer - CC81 IDLE mode set"]
pub type SS1I_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SS2I` writer - CC82 IDLE mode set"]
pub type SS2I_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SS3I` writer - CC83 IDLE mode set"]
pub type SS3I_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CPRB` writer - Prescaler# Run Bit Clear"]
pub type CPRB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PSIC` writer - Prescaler clear"]
pub type PSIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CPCH` writer - Parity Checker Run bit clear"]
pub type CPCH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - CC80 IDLE mode set"]
    #[inline(always)]
    #[must_use]
    pub fn ss0i(&mut self) -> SS0I_W<GIDLS_SPEC, 0> {
        SS0I_W::new(self)
    }
    #[doc = "Bit 1 - CC81 IDLE mode set"]
    #[inline(always)]
    #[must_use]
    pub fn ss1i(&mut self) -> SS1I_W<GIDLS_SPEC, 1> {
        SS1I_W::new(self)
    }
    #[doc = "Bit 2 - CC82 IDLE mode set"]
    #[inline(always)]
    #[must_use]
    pub fn ss2i(&mut self) -> SS2I_W<GIDLS_SPEC, 2> {
        SS2I_W::new(self)
    }
    #[doc = "Bit 3 - CC83 IDLE mode set"]
    #[inline(always)]
    #[must_use]
    pub fn ss3i(&mut self) -> SS3I_W<GIDLS_SPEC, 3> {
        SS3I_W::new(self)
    }
    #[doc = "Bit 8 - Prescaler# Run Bit Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cprb(&mut self) -> CPRB_W<GIDLS_SPEC, 8> {
        CPRB_W::new(self)
    }
    #[doc = "Bit 9 - Prescaler clear"]
    #[inline(always)]
    #[must_use]
    pub fn psic(&mut self) -> PSIC_W<GIDLS_SPEC, 9> {
        PSIC_W::new(self)
    }
    #[doc = "Bit 10 - Parity Checker Run bit clear"]
    #[inline(always)]
    #[must_use]
    pub fn cpch(&mut self) -> CPCH_W<GIDLS_SPEC, 10> {
        CPCH_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Global Idle Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gidls::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GIDLS_SPEC;
impl crate::RegisterSpec for GIDLS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gidls::W`](W) writer structure"]
impl crate::Writable for GIDLS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GIDLS to value 0"]
impl crate::Resettable for GIDLS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
