#[doc = "Writer for register RSTCLR"]
pub type W = crate::W<u32, super::RSTCLR>;
#[doc = "Register RSTCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::RSTCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clear Reset Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSCLR_AW {
    #[doc = "0: No effect"]
    CONST_0,
    #[doc = "1: Clears field RSTSTAT.RSTSTAT"]
    CONST_1,
}
impl From<RSCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: RSCLR_AW) -> Self {
        match variant {
            RSCLR_AW::CONST_0 => false,
            RSCLR_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `RSCLR`"]
pub struct RSCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RSCLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSCLR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
#[doc = "Clear Hibernate Wake-up Reset Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIBWK_AW {
    #[doc = "0: No effect"]
    CONST_0,
    #[doc = "1: De-assert reset status bit"]
    CONST_1,
}
impl From<HIBWK_AW> for bool {
    #[inline(always)]
    fn from(variant: HIBWK_AW) -> Self {
        match variant {
            HIBWK_AW::CONST_0 => false,
            HIBWK_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `HIBWK`"]
pub struct HIBWK_W<'a> {
    w: &'a mut W,
}
impl<'a> HIBWK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HIBWK_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Clear Hibernate Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIBRS_AW {
    #[doc = "0: No effect"]
    CONST_0,
    #[doc = "1: De-assert reset"]
    CONST_1,
}
impl From<HIBRS_AW> for bool {
    #[inline(always)]
    fn from(variant: HIBRS_AW) -> Self {
        match variant {
            HIBRS_AW::CONST_0 => false,
            HIBRS_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `HIBRS`"]
pub struct HIBRS_W<'a> {
    w: &'a mut W,
}
impl<'a> HIBRS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HIBRS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
#[doc = "Enable Lockup Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCKEN_AW {
    #[doc = "0: No effect"]
    CONST_0,
    #[doc = "1: Disable reset when Lockup gets asserted"]
    CONST_1,
}
impl From<LCKEN_AW> for bool {
    #[inline(always)]
    fn from(variant: LCKEN_AW) -> Self {
        match variant {
            LCKEN_AW::CONST_0 => false,
            LCKEN_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `LCKEN`"]
pub struct LCKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LCKEN_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "ECAT0 Reset Status Information\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECAT0RS_AW {
    #[doc = "0: No effect"]
    CONST_0,
    #[doc = "1: De-assert reset status"]
    CONST_1,
}
impl From<ECAT0RS_AW> for bool {
    #[inline(always)]
    fn from(variant: ECAT0RS_AW) -> Self {
        match variant {
            ECAT0RS_AW::CONST_0 => false,
            ECAT0RS_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `ECAT0RS`"]
pub struct ECAT0RS_W<'a> {
    w: &'a mut W,
}
impl<'a> ECAT0RS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECAT0RS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Clear Reset Status"]
    #[inline(always)]
    pub fn rsclr(&mut self) -> RSCLR_W {
        RSCLR_W { w: self }
    }
    #[doc = "Bit 8 - Clear Hibernate Wake-up Reset Status"]
    #[inline(always)]
    pub fn hibwk(&mut self) -> HIBWK_W {
        HIBWK_W { w: self }
    }
    #[doc = "Bit 9 - Clear Hibernate Reset"]
    #[inline(always)]
    pub fn hibrs(&mut self) -> HIBRS_W {
        HIBRS_W { w: self }
    }
    #[doc = "Bit 10 - Enable Lockup Reset"]
    #[inline(always)]
    pub fn lcken(&mut self) -> LCKEN_W {
        LCKEN_W { w: self }
    }
    #[doc = "Bit 12 - ECAT0 Reset Status Information"]
    #[inline(always)]
    pub fn ecat0rs(&mut self) -> ECAT0RS_W {
        ECAT0RS_W { w: self }
    }
}
