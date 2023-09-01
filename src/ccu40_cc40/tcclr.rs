#[doc = "Register `TCCLR` writer"]
pub type W = crate::W<TCCLR_SPEC>;
#[doc = "Field `TRBC` writer - Timer Run Bit Clear"]
pub type TRBC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TCC` writer - Timer Clear"]
pub type TCC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DITC` writer - Dither Counter Clear"]
pub type DITC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Timer Run Bit Clear"]
    #[inline(always)]
    #[must_use]
    pub fn trbc(&mut self) -> TRBC_W<TCCLR_SPEC, 0> {
        TRBC_W::new(self)
    }
    #[doc = "Bit 1 - Timer Clear"]
    #[inline(always)]
    #[must_use]
    pub fn tcc(&mut self) -> TCC_W<TCCLR_SPEC, 1> {
        TCC_W::new(self)
    }
    #[doc = "Bit 2 - Dither Counter Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ditc(&mut self) -> DITC_W<TCCLR_SPEC, 2> {
        DITC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Slice Timer Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCCLR_SPEC;
impl crate::RegisterSpec for TCCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tcclr::W`](W) writer structure"]
impl crate::Writable for TCCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCLR to value 0"]
impl crate::Resettable for TCCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
