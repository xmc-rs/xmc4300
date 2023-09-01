#[doc = "Register `WDTCLR` writer"]
pub type W = crate::W<WDTCLR_SPEC>;
#[doc = "Field `ALMC` writer - Pre-warning Alarm"]
pub type ALMC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Pre-warning Alarm"]
    #[inline(always)]
    #[must_use]
    pub fn almc(&mut self) -> ALMC_W<WDTCLR_SPEC, 0> {
        ALMC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "WDT Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDTCLR_SPEC;
impl crate::RegisterSpec for WDTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wdtclr::W`](W) writer structure"]
impl crate::Writable for WDTCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDTCLR to value 0"]
impl crate::Resettable for WDTCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
