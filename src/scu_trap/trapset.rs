#[doc = "Writer for register TRAPSET"]
pub type W = crate::W<u32, super::TRAPSET>;
#[doc = "Register TRAPSET `reset()`'s with value 0"]
impl crate::ResetValue for super::TRAPSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "OSC_HP Oscillator Watchdog Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOSCWDGT_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Set trap request"]
    CONST_1 = 1,
}
impl From<SOSCWDGT_AW> for bool {
    #[inline(always)]
    fn from(variant: SOSCWDGT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SOSCWDGT`"]
pub struct SOSCWDGT_W<'a> {
    w: &'a mut W,
}
impl<'a> SOSCWDGT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOSCWDGT_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(SOSCWDGT_AW::CONST_0)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(SOSCWDGT_AW::CONST_1)
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
#[doc = "System VCO Lock Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVCOLCKT_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Set trap request"]
    CONST_1 = 1,
}
impl From<SVCOLCKT_AW> for bool {
    #[inline(always)]
    fn from(variant: SVCOLCKT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SVCOLCKT`"]
pub struct SVCOLCKT_W<'a> {
    w: &'a mut W,
}
impl<'a> SVCOLCKT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SVCOLCKT_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(SVCOLCKT_AW::CONST_0)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(SVCOLCKT_AW::CONST_1)
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
#[doc = "USB VCO Lock Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UVCOLCKT_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Set trap request"]
    CONST_1 = 1,
}
impl From<UVCOLCKT_AW> for bool {
    #[inline(always)]
    fn from(variant: UVCOLCKT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `UVCOLCKT`"]
pub struct UVCOLCKT_W<'a> {
    w: &'a mut W,
}
impl<'a> UVCOLCKT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UVCOLCKT_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(UVCOLCKT_AW::CONST_0)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(UVCOLCKT_AW::CONST_1)
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
#[doc = "Parity Error Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PET_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Set trap request"]
    CONST_1 = 1,
}
impl From<PET_AW> for bool {
    #[inline(always)]
    fn from(variant: PET_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `PET`"]
pub struct PET_W<'a> {
    w: &'a mut W,
}
impl<'a> PET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PET_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PET_AW::CONST_0)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PET_AW::CONST_1)
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
#[doc = "Brown Out Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRWNT_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Set trap request"]
    CONST_1 = 1,
}
impl From<BRWNT_AW> for bool {
    #[inline(always)]
    fn from(variant: BRWNT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `BRWNT`"]
pub struct BRWNT_W<'a> {
    w: &'a mut W,
}
impl<'a> BRWNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRWNT_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(BRWNT_AW::CONST_0)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(BRWNT_AW::CONST_1)
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
#[doc = "OSC_ULP Oscillator Watchdog Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ULPWDT_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Set trap request"]
    CONST_1 = 1,
}
impl From<ULPWDT_AW> for bool {
    #[inline(always)]
    fn from(variant: ULPWDT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ULPWDT`"]
pub struct ULPWDT_W<'a> {
    w: &'a mut W,
}
impl<'a> ULPWDT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ULPWDT_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ULPWDT_AW::CONST_0)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ULPWDT_AW::CONST_1)
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
#[doc = "Peripheral Bridge 0 Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWERR0T_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Set trap request"]
    CONST_1 = 1,
}
impl From<BWERR0T_AW> for bool {
    #[inline(always)]
    fn from(variant: BWERR0T_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `BWERR0T`"]
pub struct BWERR0T_W<'a> {
    w: &'a mut W,
}
impl<'a> BWERR0T_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BWERR0T_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(BWERR0T_AW::CONST_0)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(BWERR0T_AW::CONST_1)
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
#[doc = "Peripheral Bridge 1 Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWERR1T_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Set trap request"]
    CONST_1 = 1,
}
impl From<BWERR1T_AW> for bool {
    #[inline(always)]
    fn from(variant: BWERR1T_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `BWERR1T`"]
pub struct BWERR1T_W<'a> {
    w: &'a mut W,
}
impl<'a> BWERR1T_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BWERR1T_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(BWERR1T_AW::CONST_0)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(BWERR1T_AW::CONST_1)
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
#[doc = "EtherCat Reset 0 Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECAT0RST_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Set trap request"]
    CONST_1 = 1,
}
impl From<ECAT0RST_AW> for bool {
    #[inline(always)]
    fn from(variant: ECAT0RST_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ECAT0RST`"]
pub struct ECAT0RST_W<'a> {
    w: &'a mut W,
}
impl<'a> ECAT0RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECAT0RST_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ECAT0RST_AW::CONST_0)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ECAT0RST_AW::CONST_1)
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
    #[doc = "Bit 0 - OSC_HP Oscillator Watchdog Trap Set"]
    #[inline(always)]
    pub fn soscwdgt(&mut self) -> SOSCWDGT_W {
        SOSCWDGT_W { w: self }
    }
    #[doc = "Bit 2 - System VCO Lock Trap Set"]
    #[inline(always)]
    pub fn svcolckt(&mut self) -> SVCOLCKT_W {
        SVCOLCKT_W { w: self }
    }
    #[doc = "Bit 3 - USB VCO Lock Trap Set"]
    #[inline(always)]
    pub fn uvcolckt(&mut self) -> UVCOLCKT_W {
        UVCOLCKT_W { w: self }
    }
    #[doc = "Bit 4 - Parity Error Trap Set"]
    #[inline(always)]
    pub fn pet(&mut self) -> PET_W {
        PET_W { w: self }
    }
    #[doc = "Bit 5 - Brown Out Trap Set"]
    #[inline(always)]
    pub fn brwnt(&mut self) -> BRWNT_W {
        BRWNT_W { w: self }
    }
    #[doc = "Bit 6 - OSC_ULP Oscillator Watchdog Trap Set"]
    #[inline(always)]
    pub fn ulpwdt(&mut self) -> ULPWDT_W {
        ULPWDT_W { w: self }
    }
    #[doc = "Bit 7 - Peripheral Bridge 0 Trap Set"]
    #[inline(always)]
    pub fn bwerr0t(&mut self) -> BWERR0T_W {
        BWERR0T_W { w: self }
    }
    #[doc = "Bit 8 - Peripheral Bridge 1 Trap Set"]
    #[inline(always)]
    pub fn bwerr1t(&mut self) -> BWERR1T_W {
        BWERR1T_W { w: self }
    }
    #[doc = "Bit 16 - EtherCat Reset 0 Trap Set"]
    #[inline(always)]
    pub fn ecat0rst(&mut self) -> ECAT0RST_W {
        ECAT0RST_W { w: self }
    }
}
