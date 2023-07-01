#[doc = "Register `RSTCLR` writer"]
pub struct W(crate::W<RSTCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTCLR_SPEC>;
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
impl From<crate::W<RSTCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clear Reset Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSCLR_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Clears field RSTSTAT.RSTSTAT"]
    CONST_1 = 1,
}
impl From<RSCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: RSCLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSCLR` writer - Clear Reset Status"]
pub type RSCLR_W<'a, const O: u8> = crate::BitWriter<'a, RSTCLR_SPEC, O, RSCLR_AW>;
impl<'a, const O: u8> RSCLR_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(RSCLR_AW::CONST_0)
    }
    #[doc = "Clears field RSTSTAT.RSTSTAT"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(RSCLR_AW::CONST_1)
    }
}
#[doc = "Clear Hibernate Wake-up Reset Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HIBWK_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: De-assert reset status bit"]
    CONST_1 = 1,
}
impl From<HIBWK_AW> for bool {
    #[inline(always)]
    fn from(variant: HIBWK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIBWK` writer - Clear Hibernate Wake-up Reset Status"]
pub type HIBWK_W<'a, const O: u8> = crate::BitWriter<'a, RSTCLR_SPEC, O, HIBWK_AW>;
impl<'a, const O: u8> HIBWK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(HIBWK_AW::CONST_0)
    }
    #[doc = "De-assert reset status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(HIBWK_AW::CONST_1)
    }
}
#[doc = "Clear Hibernate Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HIBRS_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: De-assert reset"]
    CONST_1 = 1,
}
impl From<HIBRS_AW> for bool {
    #[inline(always)]
    fn from(variant: HIBRS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIBRS` writer - Clear Hibernate Reset"]
pub type HIBRS_W<'a, const O: u8> = crate::BitWriter<'a, RSTCLR_SPEC, O, HIBRS_AW>;
impl<'a, const O: u8> HIBRS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(HIBRS_AW::CONST_0)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(HIBRS_AW::CONST_1)
    }
}
#[doc = "Enable Lockup Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCKEN_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Disable reset when Lockup gets asserted"]
    CONST_1 = 1,
}
impl From<LCKEN_AW> for bool {
    #[inline(always)]
    fn from(variant: LCKEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCKEN` writer - Enable Lockup Reset"]
pub type LCKEN_W<'a, const O: u8> = crate::BitWriter<'a, RSTCLR_SPEC, O, LCKEN_AW>;
impl<'a, const O: u8> LCKEN_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(LCKEN_AW::CONST_0)
    }
    #[doc = "Disable reset when Lockup gets asserted"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(LCKEN_AW::CONST_1)
    }
}
#[doc = "ECAT0 Reset Status Information\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECAT0RS_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: De-assert reset status"]
    CONST_1 = 1,
}
impl From<ECAT0RS_AW> for bool {
    #[inline(always)]
    fn from(variant: ECAT0RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECAT0RS` writer - ECAT0 Reset Status Information"]
pub type ECAT0RS_W<'a, const O: u8> = crate::BitWriter<'a, RSTCLR_SPEC, O, ECAT0RS_AW>;
impl<'a, const O: u8> ECAT0RS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ECAT0RS_AW::CONST_0)
    }
    #[doc = "De-assert reset status"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ECAT0RS_AW::CONST_1)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Reset Status"]
    #[inline(always)]
    #[must_use]
    pub fn rsclr(&mut self) -> RSCLR_W<0> {
        RSCLR_W::new(self)
    }
    #[doc = "Bit 8 - Clear Hibernate Wake-up Reset Status"]
    #[inline(always)]
    #[must_use]
    pub fn hibwk(&mut self) -> HIBWK_W<8> {
        HIBWK_W::new(self)
    }
    #[doc = "Bit 9 - Clear Hibernate Reset"]
    #[inline(always)]
    #[must_use]
    pub fn hibrs(&mut self) -> HIBRS_W<9> {
        HIBRS_W::new(self)
    }
    #[doc = "Bit 10 - Enable Lockup Reset"]
    #[inline(always)]
    #[must_use]
    pub fn lcken(&mut self) -> LCKEN_W<10> {
        LCKEN_W::new(self)
    }
    #[doc = "Bit 12 - ECAT0 Reset Status Information"]
    #[inline(always)]
    #[must_use]
    pub fn ecat0rs(&mut self) -> ECAT0RS_W<12> {
        ECAT0RS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCU Reset Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstclr](index.html) module"]
pub struct RSTCLR_SPEC;
impl crate::RegisterSpec for RSTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [rstclr::W](W) writer structure"]
impl crate::Writable for RSTCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSTCLR to value 0"]
impl crate::Resettable for RSTCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
