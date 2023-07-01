#[doc = "Register `PRCLR0` writer"]
pub struct W(crate::W<PRCLR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRCLR0_SPEC>;
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
impl From<crate::W<PRCLR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRCLR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "VADC Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VADCRS_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: De-assert reset"]
    CONST_1 = 1,
}
impl From<VADCRS_AW> for bool {
    #[inline(always)]
    fn from(variant: VADCRS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VADCRS` writer - VADC Reset Clear"]
pub type VADCRS_W<'a, const O: u8> = crate::BitWriter<'a, PRCLR0_SPEC, O, VADCRS_AW>;
impl<'a, const O: u8> VADCRS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(VADCRS_AW::CONST_0)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(VADCRS_AW::CONST_1)
    }
}
#[doc = "CCU40 Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU40RS_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: De-assert reset"]
    CONST_1 = 1,
}
impl From<CCU40RS_AW> for bool {
    #[inline(always)]
    fn from(variant: CCU40RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU40RS` writer - CCU40 Reset Clear"]
pub type CCU40RS_W<'a, const O: u8> = crate::BitWriter<'a, PRCLR0_SPEC, O, CCU40RS_AW>;
impl<'a, const O: u8> CCU40RS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(CCU40RS_AW::CONST_0)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(CCU40RS_AW::CONST_1)
    }
}
#[doc = "CCU41 Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU41RS_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: De-assert reset"]
    CONST_1 = 1,
}
impl From<CCU41RS_AW> for bool {
    #[inline(always)]
    fn from(variant: CCU41RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU41RS` writer - CCU41 Reset Clear"]
pub type CCU41RS_W<'a, const O: u8> = crate::BitWriter<'a, PRCLR0_SPEC, O, CCU41RS_AW>;
impl<'a, const O: u8> CCU41RS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(CCU41RS_AW::CONST_0)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(CCU41RS_AW::CONST_1)
    }
}
#[doc = "CCU80 Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU80RS_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: De-assert reset"]
    CONST_1 = 1,
}
impl From<CCU80RS_AW> for bool {
    #[inline(always)]
    fn from(variant: CCU80RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU80RS` writer - CCU80 Reset Clear"]
pub type CCU80RS_W<'a, const O: u8> = crate::BitWriter<'a, PRCLR0_SPEC, O, CCU80RS_AW>;
impl<'a, const O: u8> CCU80RS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(CCU80RS_AW::CONST_0)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(CCU80RS_AW::CONST_1)
    }
}
#[doc = "USIC0 Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USIC0RS_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: De-assert reset"]
    CONST_1 = 1,
}
impl From<USIC0RS_AW> for bool {
    #[inline(always)]
    fn from(variant: USIC0RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC0RS` writer - USIC0 Reset Clear"]
pub type USIC0RS_W<'a, const O: u8> = crate::BitWriter<'a, PRCLR0_SPEC, O, USIC0RS_AW>;
impl<'a, const O: u8> USIC0RS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(USIC0RS_AW::CONST_0)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(USIC0RS_AW::CONST_1)
    }
}
#[doc = "ERU1 Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERU1RS_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: De-assert reset"]
    CONST_1 = 1,
}
impl From<ERU1RS_AW> for bool {
    #[inline(always)]
    fn from(variant: ERU1RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERU1RS` writer - ERU1 Reset Clear"]
pub type ERU1RS_W<'a, const O: u8> = crate::BitWriter<'a, PRCLR0_SPEC, O, ERU1RS_AW>;
impl<'a, const O: u8> ERU1RS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ERU1RS_AW::CONST_0)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ERU1RS_AW::CONST_1)
    }
}
impl W {
    #[doc = "Bit 0 - VADC Reset Clear"]
    #[inline(always)]
    #[must_use]
    pub fn vadcrs(&mut self) -> VADCRS_W<0> {
        VADCRS_W::new(self)
    }
    #[doc = "Bit 2 - CCU40 Reset Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ccu40rs(&mut self) -> CCU40RS_W<2> {
        CCU40RS_W::new(self)
    }
    #[doc = "Bit 3 - CCU41 Reset Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ccu41rs(&mut self) -> CCU41RS_W<3> {
        CCU41RS_W::new(self)
    }
    #[doc = "Bit 7 - CCU80 Reset Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ccu80rs(&mut self) -> CCU80RS_W<7> {
        CCU80RS_W::new(self)
    }
    #[doc = "Bit 11 - USIC0 Reset Clear"]
    #[inline(always)]
    #[must_use]
    pub fn usic0rs(&mut self) -> USIC0RS_W<11> {
        USIC0RS_W::new(self)
    }
    #[doc = "Bit 16 - ERU1 Reset Clear"]
    #[inline(always)]
    #[must_use]
    pub fn eru1rs(&mut self) -> ERU1RS_W<16> {
        ERU1RS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCU Peripheral 0 Reset Clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prclr0](index.html) module"]
pub struct PRCLR0_SPEC;
impl crate::RegisterSpec for PRCLR0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [prclr0::W](W) writer structure"]
impl crate::Writable for PRCLR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRCLR0 to value 0"]
impl crate::Resettable for PRCLR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
