#[doc = "Register `TCCLR` writer"]
pub type W = crate::W<TCCLR_SPEC>;
#[doc = "Field `TRBC` writer - Timer Run Bit Clear"]
pub type TRBC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCC` writer - Timer Clear"]
pub type TCC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DITC` writer - Dither Counter Clear"]
pub type DITC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Timer Run Bit Clear"]
    #[inline(always)]
    #[must_use]
    pub fn trbc(&mut self) -> TRBC_W<TCCLR_SPEC> {
        TRBC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Timer Clear"]
    #[inline(always)]
    #[must_use]
    pub fn tcc(&mut self) -> TCC_W<TCCLR_SPEC> {
        TCC_W::new(self, 1)
    }
    #[doc = "Bit 2 - Dither Counter Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ditc(&mut self) -> DITC_W<TCCLR_SPEC> {
        DITC_W::new(self, 2)
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
#[doc = "Slice Timer Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCCLR_SPEC;
impl crate::RegisterSpec for TCCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tcclr::W`](W) writer structure"]
impl crate::Writable for TCCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TCCLR to value 0"]
impl crate::Resettable for TCCLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
