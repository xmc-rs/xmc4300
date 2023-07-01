#[doc = "Register `TCCLR` writer"]
pub struct W(crate::W<TCCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCLR_SPEC>;
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
impl From<crate::W<TCCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRBC` writer - Timer Run Bit Clear"]
pub type TRBC_W<'a, const O: u8> = crate::BitWriter<'a, TCCLR_SPEC, O>;
#[doc = "Field `TCC` writer - Timer Clear"]
pub type TCC_W<'a, const O: u8> = crate::BitWriter<'a, TCCLR_SPEC, O>;
#[doc = "Field `DITC` writer - Dither Counter Clear"]
pub type DITC_W<'a, const O: u8> = crate::BitWriter<'a, TCCLR_SPEC, O>;
#[doc = "Field `DTC1C` writer - Dead Time Counter 1 Clear"]
pub type DTC1C_W<'a, const O: u8> = crate::BitWriter<'a, TCCLR_SPEC, O>;
#[doc = "Field `DTC2C` writer - Dead Time Counter 2 Clear"]
pub type DTC2C_W<'a, const O: u8> = crate::BitWriter<'a, TCCLR_SPEC, O>;
impl W {
    #[doc = "Bit 0 - Timer Run Bit Clear"]
    #[inline(always)]
    #[must_use]
    pub fn trbc(&mut self) -> TRBC_W<0> {
        TRBC_W::new(self)
    }
    #[doc = "Bit 1 - Timer Clear"]
    #[inline(always)]
    #[must_use]
    pub fn tcc(&mut self) -> TCC_W<1> {
        TCC_W::new(self)
    }
    #[doc = "Bit 2 - Dither Counter Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ditc(&mut self) -> DITC_W<2> {
        DITC_W::new(self)
    }
    #[doc = "Bit 3 - Dead Time Counter 1 Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dtc1c(&mut self) -> DTC1C_W<3> {
        DTC1C_W::new(self)
    }
    #[doc = "Bit 4 - Dead Time Counter 2 Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dtc2c(&mut self) -> DTC2C_W<4> {
        DTC2C_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slice Timer Clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcclr](index.html) module"]
pub struct TCCLR_SPEC;
impl crate::RegisterSpec for TCCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tcclr::W](W) writer structure"]
impl crate::Writable for TCCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCLR to value 0"]
impl crate::Resettable for TCCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
