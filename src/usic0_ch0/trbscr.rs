#[doc = "Register `TRBSCR` writer"]
pub struct W(crate::W<TRBSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRBSCR_SPEC>;
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
impl From<crate::W<TRBSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRBSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clear Standard Receive Buffer Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSRBI_AW {
    #[doc = "0: No effect."]
    VALUE1 = 0,
    #[doc = "1: Clear TRBSR.SRBI."]
    VALUE2 = 1,
}
impl From<CSRBI_AW> for bool {
    #[inline(always)]
    fn from(variant: CSRBI_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSRBI` writer - Clear Standard Receive Buffer Event"]
pub type CSRBI_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRBSCR_SPEC, CSRBI_AW, O>;
impl<'a, const O: u8> CSRBI_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CSRBI_AW::VALUE1)
    }
    #[doc = "Clear TRBSR.SRBI."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CSRBI_AW::VALUE2)
    }
}
#[doc = "Clear Receive Buffer Error Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRBERI_AW {
    #[doc = "0: No effect."]
    VALUE1 = 0,
    #[doc = "1: Clear TRBSR.RBERI."]
    VALUE2 = 1,
}
impl From<CRBERI_AW> for bool {
    #[inline(always)]
    fn from(variant: CRBERI_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRBERI` writer - Clear Receive Buffer Error Event"]
pub type CRBERI_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRBSCR_SPEC, CRBERI_AW, O>;
impl<'a, const O: u8> CRBERI_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CRBERI_AW::VALUE1)
    }
    #[doc = "Clear TRBSR.RBERI."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CRBERI_AW::VALUE2)
    }
}
#[doc = "Clear Alternative Receive Buffer Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CARBI_AW {
    #[doc = "0: No effect."]
    VALUE1 = 0,
    #[doc = "1: Clear TRBSR.ARBI."]
    VALUE2 = 1,
}
impl From<CARBI_AW> for bool {
    #[inline(always)]
    fn from(variant: CARBI_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARBI` writer - Clear Alternative Receive Buffer Event"]
pub type CARBI_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRBSCR_SPEC, CARBI_AW, O>;
impl<'a, const O: u8> CARBI_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CARBI_AW::VALUE1)
    }
    #[doc = "Clear TRBSR.ARBI."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CARBI_AW::VALUE2)
    }
}
#[doc = "Clear Standard Transmit Buffer Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTBI_AW {
    #[doc = "0: No effect."]
    VALUE1 = 0,
    #[doc = "1: Clear TRBSR.STBI."]
    VALUE2 = 1,
}
impl From<CSTBI_AW> for bool {
    #[inline(always)]
    fn from(variant: CSTBI_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSTBI` writer - Clear Standard Transmit Buffer Event"]
pub type CSTBI_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRBSCR_SPEC, CSTBI_AW, O>;
impl<'a, const O: u8> CSTBI_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CSTBI_AW::VALUE1)
    }
    #[doc = "Clear TRBSR.STBI."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CSTBI_AW::VALUE2)
    }
}
#[doc = "Clear Transmit Buffer Error Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTBERI_AW {
    #[doc = "0: No effect."]
    VALUE1 = 0,
    #[doc = "1: Clear TRBSR.TBERI."]
    VALUE2 = 1,
}
impl From<CTBERI_AW> for bool {
    #[inline(always)]
    fn from(variant: CTBERI_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTBERI` writer - Clear Transmit Buffer Error Event"]
pub type CTBERI_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRBSCR_SPEC, CTBERI_AW, O>;
impl<'a, const O: u8> CTBERI_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CTBERI_AW::VALUE1)
    }
    #[doc = "Clear TRBSR.TBERI."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CTBERI_AW::VALUE2)
    }
}
#[doc = "Clear Bypass Data Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CBDV_AW {
    #[doc = "0: No effect."]
    VALUE1 = 0,
    #[doc = "1: Clear BYPCR.BDV."]
    VALUE2 = 1,
}
impl From<CBDV_AW> for bool {
    #[inline(always)]
    fn from(variant: CBDV_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CBDV` writer - Clear Bypass Data Valid"]
pub type CBDV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRBSCR_SPEC, CBDV_AW, O>;
impl<'a, const O: u8> CBDV_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CBDV_AW::VALUE1)
    }
    #[doc = "Clear BYPCR.BDV."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CBDV_AW::VALUE2)
    }
}
#[doc = "Flush Receive Buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLUSHRB_AW {
    #[doc = "0: No effect."]
    VALUE1 = 0,
    #[doc = "1: The receive FIFO buffer is cleared (filling level is cleared and output pointer is set to input pointer value). Should only be used while the FIFO buffer is not taking part in data traffic."]
    VALUE2 = 1,
}
impl From<FLUSHRB_AW> for bool {
    #[inline(always)]
    fn from(variant: FLUSHRB_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLUSHRB` writer - Flush Receive Buffer"]
pub type FLUSHRB_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRBSCR_SPEC, FLUSHRB_AW, O>;
impl<'a, const O: u8> FLUSHRB_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FLUSHRB_AW::VALUE1)
    }
    #[doc = "The receive FIFO buffer is cleared (filling level is cleared and output pointer is set to input pointer value). Should only be used while the FIFO buffer is not taking part in data traffic."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FLUSHRB_AW::VALUE2)
    }
}
#[doc = "Flush Transmit Buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLUSHTB_AW {
    #[doc = "0: No effect."]
    VALUE1 = 0,
    #[doc = "1: The transmit FIFO buffer is cleared (filling level is cleared and output pointer is set to input pointer value). Should only be used while the FIFO buffer is not taking part in data traffic."]
    VALUE2 = 1,
}
impl From<FLUSHTB_AW> for bool {
    #[inline(always)]
    fn from(variant: FLUSHTB_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLUSHTB` writer - Flush Transmit Buffer"]
pub type FLUSHTB_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRBSCR_SPEC, FLUSHTB_AW, O>;
impl<'a, const O: u8> FLUSHTB_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FLUSHTB_AW::VALUE1)
    }
    #[doc = "The transmit FIFO buffer is cleared (filling level is cleared and output pointer is set to input pointer value). Should only be used while the FIFO buffer is not taking part in data traffic."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FLUSHTB_AW::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Standard Receive Buffer Event"]
    #[inline(always)]
    #[must_use]
    pub fn csrbi(&mut self) -> CSRBI_W<0> {
        CSRBI_W::new(self)
    }
    #[doc = "Bit 1 - Clear Receive Buffer Error Event"]
    #[inline(always)]
    #[must_use]
    pub fn crberi(&mut self) -> CRBERI_W<1> {
        CRBERI_W::new(self)
    }
    #[doc = "Bit 2 - Clear Alternative Receive Buffer Event"]
    #[inline(always)]
    #[must_use]
    pub fn carbi(&mut self) -> CARBI_W<2> {
        CARBI_W::new(self)
    }
    #[doc = "Bit 8 - Clear Standard Transmit Buffer Event"]
    #[inline(always)]
    #[must_use]
    pub fn cstbi(&mut self) -> CSTBI_W<8> {
        CSTBI_W::new(self)
    }
    #[doc = "Bit 9 - Clear Transmit Buffer Error Event"]
    #[inline(always)]
    #[must_use]
    pub fn ctberi(&mut self) -> CTBERI_W<9> {
        CTBERI_W::new(self)
    }
    #[doc = "Bit 10 - Clear Bypass Data Valid"]
    #[inline(always)]
    #[must_use]
    pub fn cbdv(&mut self) -> CBDV_W<10> {
        CBDV_W::new(self)
    }
    #[doc = "Bit 14 - Flush Receive Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn flushrb(&mut self) -> FLUSHRB_W<14> {
        FLUSHRB_W::new(self)
    }
    #[doc = "Bit 15 - Flush Transmit Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn flushtb(&mut self) -> FLUSHTB_W<15> {
        FLUSHTB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit/Receive Buffer Status Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trbscr](index.html) module"]
pub struct TRBSCR_SPEC;
impl crate::RegisterSpec for TRBSCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [trbscr::W](W) writer structure"]
impl crate::Writable for TRBSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRBSCR to value 0"]
impl crate::Resettable for TRBSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
