#[doc = "Register `CGATCLR0` writer"]
pub struct W(crate::W<CGATCLR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CGATCLR0_SPEC>;
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
impl From<crate::W<CGATCLR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CGATCLR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "VADC Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VADC_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Disable gating"]
    CONST_1 = 1,
}
impl From<VADC_AW> for bool {
    #[inline(always)]
    fn from(variant: VADC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VADC` writer - VADC Gating Clear"]
pub type VADC_W<'a, const O: u8> = crate::BitWriter<'a, CGATCLR0_SPEC, O, VADC_AW>;
impl<'a, const O: u8> VADC_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(VADC_AW::CONST_0)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(VADC_AW::CONST_1)
    }
}
#[doc = "CCU40 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU40_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Disable gating"]
    CONST_1 = 1,
}
impl From<CCU40_AW> for bool {
    #[inline(always)]
    fn from(variant: CCU40_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU40` writer - CCU40 Gating Clear"]
pub type CCU40_W<'a, const O: u8> = crate::BitWriter<'a, CGATCLR0_SPEC, O, CCU40_AW>;
impl<'a, const O: u8> CCU40_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(CCU40_AW::CONST_0)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(CCU40_AW::CONST_1)
    }
}
#[doc = "CCU41 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU41_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Disable gating"]
    CONST_1 = 1,
}
impl From<CCU41_AW> for bool {
    #[inline(always)]
    fn from(variant: CCU41_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU41` writer - CCU41 Gating Clear"]
pub type CCU41_W<'a, const O: u8> = crate::BitWriter<'a, CGATCLR0_SPEC, O, CCU41_AW>;
impl<'a, const O: u8> CCU41_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(CCU41_AW::CONST_0)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(CCU41_AW::CONST_1)
    }
}
#[doc = "CCU80 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU80_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Disable gating"]
    CONST_1 = 1,
}
impl From<CCU80_AW> for bool {
    #[inline(always)]
    fn from(variant: CCU80_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU80` writer - CCU80 Gating Clear"]
pub type CCU80_W<'a, const O: u8> = crate::BitWriter<'a, CGATCLR0_SPEC, O, CCU80_AW>;
impl<'a, const O: u8> CCU80_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(CCU80_AW::CONST_0)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(CCU80_AW::CONST_1)
    }
}
#[doc = "POSIF0 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POSIF0_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Disable gating"]
    CONST_1 = 1,
}
impl From<POSIF0_AW> for bool {
    #[inline(always)]
    fn from(variant: POSIF0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSIF0` writer - POSIF0 Gating Clear"]
pub type POSIF0_W<'a, const O: u8> = crate::BitWriter<'a, CGATCLR0_SPEC, O, POSIF0_AW>;
impl<'a, const O: u8> POSIF0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(POSIF0_AW::CONST_0)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(POSIF0_AW::CONST_1)
    }
}
#[doc = "USIC0 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USIC0_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Disable gating"]
    CONST_1 = 1,
}
impl From<USIC0_AW> for bool {
    #[inline(always)]
    fn from(variant: USIC0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC0` writer - USIC0 Gating Clear"]
pub type USIC0_W<'a, const O: u8> = crate::BitWriter<'a, CGATCLR0_SPEC, O, USIC0_AW>;
impl<'a, const O: u8> USIC0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(USIC0_AW::CONST_0)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(USIC0_AW::CONST_1)
    }
}
#[doc = "ERU1 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERU1_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Disable gating"]
    CONST_1 = 1,
}
impl From<ERU1_AW> for bool {
    #[inline(always)]
    fn from(variant: ERU1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERU1` writer - ERU1 Gating Clear"]
pub type ERU1_W<'a, const O: u8> = crate::BitWriter<'a, CGATCLR0_SPEC, O, ERU1_AW>;
impl<'a, const O: u8> ERU1_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ERU1_AW::CONST_0)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ERU1_AW::CONST_1)
    }
}
impl W {
    #[doc = "Bit 0 - VADC Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn vadc(&mut self) -> VADC_W<0> {
        VADC_W::new(self)
    }
    #[doc = "Bit 2 - CCU40 Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ccu40(&mut self) -> CCU40_W<2> {
        CCU40_W::new(self)
    }
    #[doc = "Bit 3 - CCU41 Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ccu41(&mut self) -> CCU41_W<3> {
        CCU41_W::new(self)
    }
    #[doc = "Bit 7 - CCU80 Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ccu80(&mut self) -> CCU80_W<7> {
        CCU80_W::new(self)
    }
    #[doc = "Bit 9 - POSIF0 Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn posif0(&mut self) -> POSIF0_W<9> {
        POSIF0_W::new(self)
    }
    #[doc = "Bit 11 - USIC0 Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn usic0(&mut self) -> USIC0_W<11> {
        USIC0_W::new(self)
    }
    #[doc = "Bit 16 - ERU1 Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn eru1(&mut self) -> ERU1_W<16> {
        ERU1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral 0 Clock Gating Clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgatclr0](index.html) module"]
pub struct CGATCLR0_SPEC;
impl crate::RegisterSpec for CGATCLR0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cgatclr0::W](W) writer structure"]
impl crate::Writable for CGATCLR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CGATCLR0 to value 0"]
impl crate::Resettable for CGATCLR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
