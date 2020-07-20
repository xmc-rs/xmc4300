#[doc = "Reader of register SCTR"]
pub type R = crate::R<u32, super::SCTR>;
#[doc = "Writer for register SCTR"]
pub type W = crate::W<u32, super::SCTR>;
#[doc = "Register SCTR `reset()`'s with value 0"]
impl crate::ResetValue for super::SCTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Shift Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDIR_A {
    #[doc = "0: Shift LSB first. The first data bit of a data word is located at bit position 0."]
    VALUE1 = 0,
    #[doc = "1: Shift MSB first. The first data bit of a data word is located at the bit position given by bit field SCTR.WLE."]
    VALUE2 = 1,
}
impl From<SDIR_A> for bool {
    #[inline(always)]
    fn from(variant: SDIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SDIR`"]
pub type SDIR_R = crate::R<bool, SDIR_A>;
impl SDIR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDIR_A {
        match self.bits {
            false => SDIR_A::VALUE1,
            true => SDIR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SDIR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SDIR_A::VALUE2
    }
}
#[doc = "Write proxy for field `SDIR`"]
pub struct SDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDIR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Shift LSB first. The first data bit of a data word is located at bit position 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SDIR_A::VALUE1)
    }
    #[doc = "Shift MSB first. The first data bit of a data word is located at the bit position given by bit field SCTR.WLE."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SDIR_A::VALUE2)
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
#[doc = "Passive Data Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDL_A {
    #[doc = "0: The passive data level is 0."]
    VALUE1 = 0,
    #[doc = "1: The passive data level is 1."]
    VALUE2 = 1,
}
impl From<PDL_A> for bool {
    #[inline(always)]
    fn from(variant: PDL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDL`"]
pub type PDL_R = crate::R<bool, PDL_A>;
impl PDL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDL_A {
        match self.bits {
            false => PDL_A::VALUE1,
            true => PDL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PDL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PDL_A::VALUE2
    }
}
#[doc = "Write proxy for field `PDL`"]
pub struct PDL_W<'a> {
    w: &'a mut W,
}
impl<'a> PDL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The passive data level is 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDL_A::VALUE1)
    }
    #[doc = "The passive data level is 1."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDL_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Data Shift Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DSM_A {
    #[doc = "0: Receive and transmit data is shifted in and out one bit at a time through DX0 and DOUT0."]
    VALUE1 = 0,
    #[doc = "2: Receive and transmit data is shifted in and out two bits at a time through two input stages (DX0 and DX3) and DOUT\\[1:0\\]
respectively."]
    VALUE3 = 2,
    #[doc = "3: Receive and transmit data is shifted in and out four bits at a time through four input stages (DX0, DX\\[5:3\\]) and DOUT\\[3:0\\]
respectively."]
    VALUE4 = 3,
}
impl From<DSM_A> for u8 {
    #[inline(always)]
    fn from(variant: DSM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DSM`"]
pub type DSM_R = crate::R<u8, DSM_A>;
impl DSM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DSM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DSM_A::VALUE1),
            2 => Val(DSM_A::VALUE3),
            3 => Val(DSM_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DSM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == DSM_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == DSM_A::VALUE4
    }
}
#[doc = "Write proxy for field `DSM`"]
pub struct DSM_W<'a> {
    w: &'a mut W,
}
impl<'a> DSM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Receive and transmit data is shifted in and out one bit at a time through DX0 and DOUT0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DSM_A::VALUE1)
    }
    #[doc = "Receive and transmit data is shifted in and out two bits at a time through two input stages (DX0 and DX3) and DOUT\\[1:0\\]
respectively."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(DSM_A::VALUE3)
    }
    #[doc = "Receive and transmit data is shifted in and out four bits at a time through four input stages (DX0, DX\\[5:3\\]) and DOUT\\[3:0\\]
respectively."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(DSM_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Port Control Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPCDIR_A {
    #[doc = "0: The pin(s) with hardware pin control enabled are selected to be in input mode."]
    VALUE1 = 0,
    #[doc = "1: The pin(s) with hardware pin control enabled are selected to be in output mode."]
    VALUE2 = 1,
}
impl From<HPCDIR_A> for bool {
    #[inline(always)]
    fn from(variant: HPCDIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HPCDIR`"]
pub type HPCDIR_R = crate::R<bool, HPCDIR_A>;
impl HPCDIR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HPCDIR_A {
        match self.bits {
            false => HPCDIR_A::VALUE1,
            true => HPCDIR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HPCDIR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HPCDIR_A::VALUE2
    }
}
#[doc = "Write proxy for field `HPCDIR`"]
pub struct HPCDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> HPCDIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HPCDIR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The pin(s) with hardware pin control enabled are selected to be in input mode."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HPCDIR_A::VALUE1)
    }
    #[doc = "The pin(s) with hardware pin control enabled are selected to be in output mode."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HPCDIR_A::VALUE2)
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
#[doc = "Data Output Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DOCFG_A {
    #[doc = "0: DOUTx = shift data value"]
    VALUE1 = 0,
    #[doc = "1: DOUTx = inverted shift data value"]
    VALUE2 = 1,
}
impl From<DOCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: DOCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DOCFG`"]
pub type DOCFG_R = crate::R<u8, DOCFG_A>;
impl DOCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DOCFG_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DOCFG_A::VALUE1),
            1 => Val(DOCFG_A::VALUE2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DOCFG_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DOCFG_A::VALUE2
    }
}
#[doc = "Write proxy for field `DOCFG`"]
pub struct DOCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> DOCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOCFG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "DOUTx = shift data value"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DOCFG_A::VALUE1)
    }
    #[doc = "DOUTx = inverted shift data value"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DOCFG_A::VALUE2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Transmission Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRM_A {
    #[doc = "0: The shift control signal is considered as inactive and data frame transfers are not possible."]
    VALUE1 = 0,
    #[doc = "1: The shift control signal is considered active if it is at 1-level. This is the setting to be programmed to allow data transfers."]
    VALUE2 = 1,
    #[doc = "2: The shift control signal is considered active if it is at 0-level. It is recommended to avoid this setting and to use the inversion in the DX2 stage in case of a low-active signal."]
    VALUE3 = 2,
    #[doc = "3: The shift control signal is considered active without referring to the actual signal level. Data frame transfer is possible after each edge of the signal."]
    VALUE4 = 3,
}
impl From<TRM_A> for u8 {
    #[inline(always)]
    fn from(variant: TRM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TRM`"]
pub type TRM_R = crate::R<u8, TRM_A>;
impl TRM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRM_A {
        match self.bits {
            0 => TRM_A::VALUE1,
            1 => TRM_A::VALUE2,
            2 => TRM_A::VALUE3,
            3 => TRM_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TRM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TRM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TRM_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == TRM_A::VALUE4
    }
}
#[doc = "Write proxy for field `TRM`"]
pub struct TRM_W<'a> {
    w: &'a mut W,
}
impl<'a> TRM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "The shift control signal is considered as inactive and data frame transfers are not possible."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TRM_A::VALUE1)
    }
    #[doc = "The shift control signal is considered active if it is at 1-level. This is the setting to be programmed to allow data transfers."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TRM_A::VALUE2)
    }
    #[doc = "The shift control signal is considered active if it is at 0-level. It is recommended to avoid this setting and to use the inversion in the DX2 stage in case of a low-active signal."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(TRM_A::VALUE3)
    }
    #[doc = "The shift control signal is considered active without referring to the actual signal level. Data frame transfer is possible after each edge of the signal."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(TRM_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `FLE`"]
pub type FLE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FLE`"]
pub struct FLE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Word Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WLE_A {
    #[doc = "0: The data word contains 1 data bit located at bit position 0."]
    VALUE1 = 0,
    #[doc = "1: The data word contains 2 data bits located at bit positions \\[1:0\\]."]
    VALUE2 = 1,
    #[doc = "14: The data word contains 15 data bits located at bit positions \\[14:0\\]."]
    VALUE3 = 14,
    #[doc = "15: The data word contains 16 data bits located at bit positions \\[15:0\\]."]
    VALUE4 = 15,
}
impl From<WLE_A> for u8 {
    #[inline(always)]
    fn from(variant: WLE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WLE`"]
pub type WLE_R = crate::R<u8, WLE_A>;
impl WLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WLE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WLE_A::VALUE1),
            1 => Val(WLE_A::VALUE2),
            14 => Val(WLE_A::VALUE3),
            15 => Val(WLE_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WLE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WLE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == WLE_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == WLE_A::VALUE4
    }
}
#[doc = "Write proxy for field `WLE`"]
pub struct WLE_W<'a> {
    w: &'a mut W,
}
impl<'a> WLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WLE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The data word contains 1 data bit located at bit position 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WLE_A::VALUE1)
    }
    #[doc = "The data word contains 2 data bits located at bit positions \\[1:0\\]."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WLE_A::VALUE2)
    }
    #[doc = "The data word contains 15 data bits located at bit positions \\[14:0\\]."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(WLE_A::VALUE3)
    }
    #[doc = "The data word contains 16 data bits located at bit positions \\[15:0\\]."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(WLE_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Shift Direction"]
    #[inline(always)]
    pub fn sdir(&self) -> SDIR_R {
        SDIR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Passive Data Level"]
    #[inline(always)]
    pub fn pdl(&self) -> PDL_R {
        PDL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Data Shift Mode"]
    #[inline(always)]
    pub fn dsm(&self) -> DSM_R {
        DSM_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Port Control Direction"]
    #[inline(always)]
    pub fn hpcdir(&self) -> HPCDIR_R {
        HPCDIR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Data Output Configuration"]
    #[inline(always)]
    pub fn docfg(&self) -> DOCFG_R {
        DOCFG_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Transmission Mode"]
    #[inline(always)]
    pub fn trm(&self) -> TRM_R {
        TRM_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:21 - Frame Length"]
    #[inline(always)]
    pub fn fle(&self) -> FLE_R {
        FLE_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:27 - Word Length"]
    #[inline(always)]
    pub fn wle(&self) -> WLE_R {
        WLE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Shift Direction"]
    #[inline(always)]
    pub fn sdir(&mut self) -> SDIR_W {
        SDIR_W { w: self }
    }
    #[doc = "Bit 1 - Passive Data Level"]
    #[inline(always)]
    pub fn pdl(&mut self) -> PDL_W {
        PDL_W { w: self }
    }
    #[doc = "Bits 2:3 - Data Shift Mode"]
    #[inline(always)]
    pub fn dsm(&mut self) -> DSM_W {
        DSM_W { w: self }
    }
    #[doc = "Bit 4 - Port Control Direction"]
    #[inline(always)]
    pub fn hpcdir(&mut self) -> HPCDIR_W {
        HPCDIR_W { w: self }
    }
    #[doc = "Bits 6:7 - Data Output Configuration"]
    #[inline(always)]
    pub fn docfg(&mut self) -> DOCFG_W {
        DOCFG_W { w: self }
    }
    #[doc = "Bits 8:9 - Transmission Mode"]
    #[inline(always)]
    pub fn trm(&mut self) -> TRM_W {
        TRM_W { w: self }
    }
    #[doc = "Bits 16:21 - Frame Length"]
    #[inline(always)]
    pub fn fle(&mut self) -> FLE_W {
        FLE_W { w: self }
    }
    #[doc = "Bits 24:27 - Word Length"]
    #[inline(always)]
    pub fn wle(&mut self) -> WLE_W {
        WLE_W { w: self }
    }
}
