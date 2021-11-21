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
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct VADCRS_W<'a> {
    w: &'a mut W,
}
impl<'a> VADCRS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VADCRS_AW) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "CCU40 Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct CCU40RS_W<'a> {
    w: &'a mut W,
}
impl<'a> CCU40RS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCU40RS_AW) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "CCU41 Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct CCU41RS_W<'a> {
    w: &'a mut W,
}
impl<'a> CCU41RS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCU41RS_AW) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "CCU80 Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct CCU80RS_W<'a> {
    w: &'a mut W,
}
impl<'a> CCU80RS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCU80RS_AW) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "USIC0 Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct USIC0RS_W<'a> {
    w: &'a mut W,
}
impl<'a> USIC0RS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USIC0RS_AW) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "ERU1 Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct ERU1RS_W<'a> {
    w: &'a mut W,
}
impl<'a> ERU1RS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERU1RS_AW) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - VADC Reset Clear"]
    #[inline(always)]
    pub fn vadcrs(&mut self) -> VADCRS_W {
        VADCRS_W { w: self }
    }
    #[doc = "Bit 2 - CCU40 Reset Clear"]
    #[inline(always)]
    pub fn ccu40rs(&mut self) -> CCU40RS_W {
        CCU40RS_W { w: self }
    }
    #[doc = "Bit 3 - CCU41 Reset Clear"]
    #[inline(always)]
    pub fn ccu41rs(&mut self) -> CCU41RS_W {
        CCU41RS_W { w: self }
    }
    #[doc = "Bit 7 - CCU80 Reset Clear"]
    #[inline(always)]
    pub fn ccu80rs(&mut self) -> CCU80RS_W {
        CCU80RS_W { w: self }
    }
    #[doc = "Bit 11 - USIC0 Reset Clear"]
    #[inline(always)]
    pub fn usic0rs(&mut self) -> USIC0RS_W {
        USIC0RS_W { w: self }
    }
    #[doc = "Bit 16 - ERU1 Reset Clear"]
    #[inline(always)]
    pub fn eru1rs(&mut self) -> ERU1RS_W {
        ERU1RS_W { w: self }
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
}
#[doc = "`reset()` method sets PRCLR0 to value 0"]
impl crate::Resettable for PRCLR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
