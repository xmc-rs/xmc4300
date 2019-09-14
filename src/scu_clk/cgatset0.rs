#[doc = "Writer for register CGATSET0"]
pub type W = crate::W<u32, super::CGATSET0>;
#[doc = "Register CGATSET0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CGATSET0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "VADC Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VADC_AW {
    #[doc = "0: No effect"]
    CONST_0,
    #[doc = "1: Enable gating"]
    CONST_1,
}
impl From<VADC_AW> for bool {
    #[inline(always)]
    fn from(variant: VADC_AW) -> Self {
        match variant {
            VADC_AW::CONST_0 => false,
            VADC_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `VADC`"]
pub struct VADC_W<'a> {
    w: &'a mut W,
}
impl<'a> VADC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VADC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(VADC_AW::CONST_0)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(VADC_AW::CONST_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "CCU40 Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU40_AW {
    #[doc = "0: No effect"]
    CONST_0,
    #[doc = "1: Enable gating"]
    CONST_1,
}
impl From<CCU40_AW> for bool {
    #[inline(always)]
    fn from(variant: CCU40_AW) -> Self {
        match variant {
            CCU40_AW::CONST_0 => false,
            CCU40_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `CCU40`"]
pub struct CCU40_W<'a> {
    w: &'a mut W,
}
impl<'a> CCU40_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCU40_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(CCU40_AW::CONST_0)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(CCU40_AW::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "CCU41 Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU41_AW {
    #[doc = "0: No effect"]
    CONST_0,
    #[doc = "1: Enable gating"]
    CONST_1,
}
impl From<CCU41_AW> for bool {
    #[inline(always)]
    fn from(variant: CCU41_AW) -> Self {
        match variant {
            CCU41_AW::CONST_0 => false,
            CCU41_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `CCU41`"]
pub struct CCU41_W<'a> {
    w: &'a mut W,
}
impl<'a> CCU41_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCU41_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(CCU41_AW::CONST_0)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(CCU41_AW::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "CCU80 Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU80_AW {
    #[doc = "0: No effect"]
    CONST_0,
    #[doc = "1: Enable gating"]
    CONST_1,
}
impl From<CCU80_AW> for bool {
    #[inline(always)]
    fn from(variant: CCU80_AW) -> Self {
        match variant {
            CCU80_AW::CONST_0 => false,
            CCU80_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `CCU80`"]
pub struct CCU80_W<'a> {
    w: &'a mut W,
}
impl<'a> CCU80_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCU80_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(CCU80_AW::CONST_0)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(CCU80_AW::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "POSIF0 Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POSIF0_AW {
    #[doc = "0: No effect"]
    CONST_0,
    #[doc = "1: Enable gating"]
    CONST_1,
}
impl From<POSIF0_AW> for bool {
    #[inline(always)]
    fn from(variant: POSIF0_AW) -> Self {
        match variant {
            POSIF0_AW::CONST_0 => false,
            POSIF0_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `POSIF0`"]
pub struct POSIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> POSIF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POSIF0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(POSIF0_AW::CONST_0)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(POSIF0_AW::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "USIC0 Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USIC0_AW {
    #[doc = "0: No effect"]
    CONST_0,
    #[doc = "1: Enable gating"]
    CONST_1,
}
impl From<USIC0_AW> for bool {
    #[inline(always)]
    fn from(variant: USIC0_AW) -> Self {
        match variant {
            USIC0_AW::CONST_0 => false,
            USIC0_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `USIC0`"]
pub struct USIC0_W<'a> {
    w: &'a mut W,
}
impl<'a> USIC0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USIC0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(USIC0_AW::CONST_0)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(USIC0_AW::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "ERU1 Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERU1_AW {
    #[doc = "0: No effect"]
    CONST_0,
    #[doc = "1: Enable gating"]
    CONST_1,
}
impl From<ERU1_AW> for bool {
    #[inline(always)]
    fn from(variant: ERU1_AW) -> Self {
        match variant {
            ERU1_AW::CONST_0 => false,
            ERU1_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `ERU1`"]
pub struct ERU1_W<'a> {
    w: &'a mut W,
}
impl<'a> ERU1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERU1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ERU1_AW::CONST_0)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ERU1_AW::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - VADC Gating Set"]
    #[inline(always)]
    pub fn vadc(&mut self) -> VADC_W {
        VADC_W { w: self }
    }
    #[doc = "Bit 2 - CCU40 Gating Set"]
    #[inline(always)]
    pub fn ccu40(&mut self) -> CCU40_W {
        CCU40_W { w: self }
    }
    #[doc = "Bit 3 - CCU41 Gating Set"]
    #[inline(always)]
    pub fn ccu41(&mut self) -> CCU41_W {
        CCU41_W { w: self }
    }
    #[doc = "Bit 7 - CCU80 Gating Set"]
    #[inline(always)]
    pub fn ccu80(&mut self) -> CCU80_W {
        CCU80_W { w: self }
    }
    #[doc = "Bit 9 - POSIF0 Gating Set"]
    #[inline(always)]
    pub fn posif0(&mut self) -> POSIF0_W {
        POSIF0_W { w: self }
    }
    #[doc = "Bit 11 - USIC0 Gating Set"]
    #[inline(always)]
    pub fn usic0(&mut self) -> USIC0_W {
        USIC0_W { w: self }
    }
    #[doc = "Bit 16 - ERU1 Gating Set"]
    #[inline(always)]
    pub fn eru1(&mut self) -> ERU1_W {
        ERU1_W { w: self }
    }
}
