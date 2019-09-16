#[doc = "Writer for register PRSET1"]
pub type W = crate::W<u32, super::PRSET1>;
#[doc = "Register PRSET1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PRSET1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "LEDTS Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEDTSCU0RS_AW {
    #[doc = "0: No effect"]
    CONST_0,
    #[doc = "1: Assert reset"]
    CONST_1,
}
impl From<LEDTSCU0RS_AW> for bool {
    #[inline(always)]
    fn from(variant: LEDTSCU0RS_AW) -> Self {
        match variant {
            LEDTSCU0RS_AW::CONST_0 => false,
            LEDTSCU0RS_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `LEDTSCU0RS`"]
pub struct LEDTSCU0RS_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDTSCU0RS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEDTSCU0RS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(LEDTSCU0RS_AW::CONST_0)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(LEDTSCU0RS_AW::CONST_1)
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
#[doc = "MultiCAN Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCAN0RS_AW {
    #[doc = "0: No effect"]
    CONST_0,
    #[doc = "1: Assert reset"]
    CONST_1,
}
impl From<MCAN0RS_AW> for bool {
    #[inline(always)]
    fn from(variant: MCAN0RS_AW) -> Self {
        match variant {
            MCAN0RS_AW::CONST_0 => false,
            MCAN0RS_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `MCAN0RS`"]
pub struct MCAN0RS_W<'a> {
    w: &'a mut W,
}
impl<'a> MCAN0RS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCAN0RS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(MCAN0RS_AW::CONST_0)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(MCAN0RS_AW::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "DAC Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACRS_AW {
    #[doc = "0: No effect"]
    CONST_0,
    #[doc = "1: Assert reset"]
    CONST_1,
}
impl From<DACRS_AW> for bool {
    #[inline(always)]
    fn from(variant: DACRS_AW) -> Self {
        match variant {
            DACRS_AW::CONST_0 => false,
            DACRS_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `DACRS`"]
pub struct DACRS_W<'a> {
    w: &'a mut W,
}
impl<'a> DACRS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACRS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(DACRS_AW::CONST_0)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(DACRS_AW::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "MMC Interface Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMCIRS_AW {
    #[doc = "0: No effect"]
    CONST_0,
    #[doc = "1: Assert reset"]
    CONST_1,
}
impl From<MMCIRS_AW> for bool {
    #[inline(always)]
    fn from(variant: MMCIRS_AW) -> Self {
        match variant {
            MMCIRS_AW::CONST_0 => false,
            MMCIRS_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `MMCIRS`"]
pub struct MMCIRS_W<'a> {
    w: &'a mut W,
}
impl<'a> MMCIRS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MMCIRS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(MMCIRS_AW::CONST_0)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(MMCIRS_AW::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "USIC1 Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USIC1RS_AW {
    #[doc = "0: No effect"]
    CONST_0,
    #[doc = "1: Assert reset"]
    CONST_1,
}
impl From<USIC1RS_AW> for bool {
    #[inline(always)]
    fn from(variant: USIC1RS_AW) -> Self {
        match variant {
            USIC1RS_AW::CONST_0 => false,
            USIC1RS_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `USIC1RS`"]
pub struct USIC1RS_W<'a> {
    w: &'a mut W,
}
impl<'a> USIC1RS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USIC1RS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(USIC1RS_AW::CONST_0)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(USIC1RS_AW::CONST_1)
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
#[doc = "PORTS Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPORTSRS_AW {
    #[doc = "0: No effect"]
    CONST_0,
    #[doc = "1: Assert reset"]
    CONST_1,
}
impl From<PPORTSRS_AW> for bool {
    #[inline(always)]
    fn from(variant: PPORTSRS_AW) -> Self {
        match variant {
            PPORTSRS_AW::CONST_0 => false,
            PPORTSRS_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PPORTSRS`"]
pub struct PPORTSRS_W<'a> {
    w: &'a mut W,
}
impl<'a> PPORTSRS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PPORTSRS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PPORTSRS_AW::CONST_0)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PPORTSRS_AW::CONST_1)
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
impl W {
    #[doc = "Bit 3 - LEDTS Reset Assert"]
    #[inline(always)]
    pub fn ledtscu0rs(&mut self) -> LEDTSCU0RS_W {
        LEDTSCU0RS_W { w: self }
    }
    #[doc = "Bit 4 - MultiCAN Reset Assert"]
    #[inline(always)]
    pub fn mcan0rs(&mut self) -> MCAN0RS_W {
        MCAN0RS_W { w: self }
    }
    #[doc = "Bit 5 - DAC Reset Assert"]
    #[inline(always)]
    pub fn dacrs(&mut self) -> DACRS_W {
        DACRS_W { w: self }
    }
    #[doc = "Bit 6 - MMC Interface Reset Assert"]
    #[inline(always)]
    pub fn mmcirs(&mut self) -> MMCIRS_W {
        MMCIRS_W { w: self }
    }
    #[doc = "Bit 7 - USIC1 Reset Assert"]
    #[inline(always)]
    pub fn usic1rs(&mut self) -> USIC1RS_W {
        USIC1RS_W { w: self }
    }
    #[doc = "Bit 9 - PORTS Reset Assert"]
    #[inline(always)]
    pub fn pportsrs(&mut self) -> PPORTSRS_W {
        PPORTSRS_W { w: self }
    }
}
