#[doc = "Reader of register GLOBTF"]
pub type R = crate::R<u32, super::GLOBTF>;
#[doc = "Writer for register GLOBTF"]
pub type W = crate::W<u32, super::GLOBTF>;
#[doc = "Register GLOBTF `reset()`'s with value 0"]
impl crate::ResetValue for super::GLOBTF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CDGR`"]
pub type CDGR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CDGR`"]
pub struct CDGR_W<'a> {
    w: &'a mut W,
}
impl<'a> CDGR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Converter Diagnostics Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDEN_A {
    #[doc = "0: All diagnostic pull devices are disconnected"]
    VALUE1 = 0,
    #[doc = "1: Diagnostic pull devices connected as selected by bitfield CDSEL"]
    VALUE2 = 1,
}
impl From<CDEN_A> for bool {
    #[inline(always)]
    fn from(variant: CDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CDEN`"]
pub type CDEN_R = crate::R<bool, CDEN_A>;
impl CDEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDEN_A {
        match self.bits {
            false => CDEN_A::VALUE1,
            true => CDEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CDEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CDEN_A::VALUE2
    }
}
#[doc = "Write proxy for field `CDEN`"]
pub struct CDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CDEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "All diagnostic pull devices are disconnected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CDEN_A::VALUE1)
    }
    #[doc = "Diagnostic pull devices connected as selected by bitfield CDSEL"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CDEN_A::VALUE2)
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
#[doc = "Converter Diagnostics Pull-Devices Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CDSEL_A {
    #[doc = "0: Connected to VAREF"]
    VALUE1 = 0,
    #[doc = "1: Connected to VAGND"]
    VALUE2 = 1,
    #[doc = "2: Connected to 1/3rd VAREF"]
    VALUE3 = 2,
    #[doc = "3: Connected to 2/3rd VAREF"]
    VALUE4 = 3,
}
impl From<CDSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CDSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CDSEL`"]
pub type CDSEL_R = crate::R<u8, CDSEL_A>;
impl CDSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDSEL_A {
        match self.bits {
            0 => CDSEL_A::VALUE1,
            1 => CDSEL_A::VALUE2,
            2 => CDSEL_A::VALUE3,
            3 => CDSEL_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CDSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CDSEL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CDSEL_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CDSEL_A::VALUE4
    }
}
#[doc = "Write proxy for field `CDSEL`"]
pub struct CDSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CDSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CDSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Connected to VAREF"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CDSEL_A::VALUE1)
    }
    #[doc = "Connected to VAGND"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CDSEL_A::VALUE2)
    }
    #[doc = "Connected to 1/3rd VAREF"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CDSEL_A::VALUE3)
    }
    #[doc = "Connected to 2/3rd VAREF"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CDSEL_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "Write Control for Conversion Diagnostics\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDWC_AW {
    #[doc = "0: No write access to parameters"]
    VALUE1 = 0,
    #[doc = "1: Bitfields CDSEL, CDEN, CDGR can be written"]
    VALUE2 = 1,
}
impl From<CDWC_AW> for bool {
    #[inline(always)]
    fn from(variant: CDWC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CDWC`"]
pub struct CDWC_W<'a> {
    w: &'a mut W,
}
impl<'a> CDWC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CDWC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No write access to parameters"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CDWC_AW::VALUE1)
    }
    #[doc = "Bitfields CDSEL, CDEN, CDGR can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CDWC_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Pull-Down Diagnostics Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD_A {
    #[doc = "0: Disconnected"]
    VALUE1 = 0,
    #[doc = "1: The pull-down diagnostics device is active"]
    VALUE2 = 1,
}
impl From<PDD_A> for bool {
    #[inline(always)]
    fn from(variant: PDD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDD`"]
pub type PDD_R = crate::R<bool, PDD_A>;
impl PDD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD_A {
        match self.bits {
            false => PDD_A::VALUE1,
            true => PDD_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PDD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PDD_A::VALUE2
    }
}
#[doc = "Write proxy for field `PDD`"]
pub struct PDD_W<'a> {
    w: &'a mut W,
}
impl<'a> PDD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disconnected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDD_A::VALUE1)
    }
    #[doc = "The pull-down diagnostics device is active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDD_A::VALUE2)
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
#[doc = "Write Control for Multiplexer Diagnostics\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDWC_AW {
    #[doc = "0: No write access to parameters"]
    VALUE1 = 0,
    #[doc = "1: Bitfield PDD can be written"]
    VALUE2 = 1,
}
impl From<MDWC_AW> for bool {
    #[inline(always)]
    fn from(variant: MDWC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `MDWC`"]
pub struct MDWC_W<'a> {
    w: &'a mut W,
}
impl<'a> MDWC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MDWC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No write access to parameters"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MDWC_AW::VALUE1)
    }
    #[doc = "Bitfield PDD can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MDWC_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:7 - Converter Diagnostics Group"]
    #[inline(always)]
    pub fn cdgr(&self) -> CDGR_R {
        CDGR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Converter Diagnostics Enable"]
    #[inline(always)]
    pub fn cden(&self) -> CDEN_R {
        CDEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - Converter Diagnostics Pull-Devices Select"]
    #[inline(always)]
    pub fn cdsel(&self) -> CDSEL_R {
        CDSEL_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Pull-Down Diagnostics Enable"]
    #[inline(always)]
    pub fn pdd(&self) -> PDD_R {
        PDD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 4:7 - Converter Diagnostics Group"]
    #[inline(always)]
    pub fn cdgr(&mut self) -> CDGR_W {
        CDGR_W { w: self }
    }
    #[doc = "Bit 8 - Converter Diagnostics Enable"]
    #[inline(always)]
    pub fn cden(&mut self) -> CDEN_W {
        CDEN_W { w: self }
    }
    #[doc = "Bits 9:10 - Converter Diagnostics Pull-Devices Select"]
    #[inline(always)]
    pub fn cdsel(&mut self) -> CDSEL_W {
        CDSEL_W { w: self }
    }
    #[doc = "Bit 15 - Write Control for Conversion Diagnostics"]
    #[inline(always)]
    pub fn cdwc(&mut self) -> CDWC_W {
        CDWC_W { w: self }
    }
    #[doc = "Bit 16 - Pull-Down Diagnostics Enable"]
    #[inline(always)]
    pub fn pdd(&mut self) -> PDD_W {
        PDD_W { w: self }
    }
    #[doc = "Bit 23 - Write Control for Multiplexer Diagnostics"]
    #[inline(always)]
    pub fn mdwc(&mut self) -> MDWC_W {
        MDWC_W { w: self }
    }
}
