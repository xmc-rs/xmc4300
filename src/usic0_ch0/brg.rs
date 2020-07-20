#[doc = "Reader of register BRG"]
pub type R = crate::R<u32, super::BRG>;
#[doc = "Writer for register BRG"]
pub type W = crate::W<u32, super::BRG>;
#[doc = "Register BRG `reset()`'s with value 0"]
impl crate::ResetValue for super::BRG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "0: The fractional divider frequency fFD is selected."]
    VALUE1 = 0,
    #[doc = "2: The trigger signal DX1T defines fPIN. Signal MCLK toggles with fPIN."]
    VALUE3 = 2,
    #[doc = "3: Signal MCLK corresponds to the DX1S signal and the frequency fPIN is derived from the rising edges of DX1S."]
    VALUE4 = 3,
}
impl From<CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLKSEL`"]
pub type CLKSEL_R = crate::R<u8, CLKSEL_A>;
impl CLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLKSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLKSEL_A::VALUE1),
            2 => Val(CLKSEL_A::VALUE3),
            3 => Val(CLKSEL_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CLKSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CLKSEL_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CLKSEL_A::VALUE4
    }
}
#[doc = "Write proxy for field `CLKSEL`"]
pub struct CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The fractional divider frequency fFD is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CLKSEL_A::VALUE1)
    }
    #[doc = "The trigger signal DX1T defines fPIN. Signal MCLK toggles with fPIN."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CLKSEL_A::VALUE3)
    }
    #[doc = "Signal MCLK corresponds to the DX1S signal and the frequency fPIN is derived from the rising edges of DX1S."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CLKSEL_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Timing Measurement Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMEN_A {
    #[doc = "0: Timing measurement is disabled: The trigger signals DX0T and DX1T are ignored."]
    VALUE1 = 0,
    #[doc = "1: Timing measurement is enabled: The 10-bit counter is incremented by 1 with fPPP and stops counting when reaching its maximum value. If one of the trigger signals DX0T or DX1T become active, the counter value is captured into bit field CTV, the counter is cleared and a transmit shift event is generated."]
    VALUE2 = 1,
}
impl From<TMEN_A> for bool {
    #[inline(always)]
    fn from(variant: TMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMEN`"]
pub type TMEN_R = crate::R<bool, TMEN_A>;
impl TMEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMEN_A {
        match self.bits {
            false => TMEN_A::VALUE1,
            true => TMEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TMEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TMEN_A::VALUE2
    }
}
#[doc = "Write proxy for field `TMEN`"]
pub struct TMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timing measurement is disabled: The trigger signals DX0T and DX1T are ignored."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TMEN_A::VALUE1)
    }
    #[doc = "Timing measurement is enabled: The 10-bit counter is incremented by 1 with fPPP and stops counting when reaching its maximum value. If one of the trigger signals DX0T or DX1T become active, the counter value is captured into bit field CTV, the counter is cleared and a transmit shift event is generated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TMEN_A::VALUE2)
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
#[doc = "Enable 2:1 Divider for fPPP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPPEN_A {
    #[doc = "0: The 2:1 divider for fPPP is disabled. fPPP = fPIN"]
    VALUE1 = 0,
    #[doc = "1: The 2:1 divider for fPPP is enabled. fPPP = fMCLK = fPIN / 2."]
    VALUE2 = 1,
}
impl From<PPPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PPPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PPPEN`"]
pub type PPPEN_R = crate::R<bool, PPPEN_A>;
impl PPPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPPEN_A {
        match self.bits {
            false => PPPEN_A::VALUE1,
            true => PPPEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PPPEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PPPEN_A::VALUE2
    }
}
#[doc = "Write proxy for field `PPPEN`"]
pub struct PPPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PPPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PPPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The 2:1 divider for fPPP is disabled. fPPP = fPIN"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PPPEN_A::VALUE1)
    }
    #[doc = "The 2:1 divider for fPPP is enabled. fPPP = fMCLK = fPIN / 2."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PPPEN_A::VALUE2)
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
#[doc = "Input Selection for CTQ\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CTQSEL_A {
    #[doc = "0: fCTQIN = fPDIV"]
    VALUE1 = 0,
    #[doc = "1: fCTQIN = fPPP"]
    VALUE2 = 1,
    #[doc = "2: fCTQIN = fSCLK"]
    VALUE3 = 2,
    #[doc = "3: fCTQIN = fMCLK"]
    VALUE4 = 3,
}
impl From<CTQSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CTQSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CTQSEL`"]
pub type CTQSEL_R = crate::R<u8, CTQSEL_A>;
impl CTQSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTQSEL_A {
        match self.bits {
            0 => CTQSEL_A::VALUE1,
            1 => CTQSEL_A::VALUE2,
            2 => CTQSEL_A::VALUE3,
            3 => CTQSEL_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CTQSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CTQSEL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CTQSEL_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CTQSEL_A::VALUE4
    }
}
#[doc = "Write proxy for field `CTQSEL`"]
pub struct CTQSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CTQSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTQSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "fCTQIN = fPDIV"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CTQSEL_A::VALUE1)
    }
    #[doc = "fCTQIN = fPPP"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CTQSEL_A::VALUE2)
    }
    #[doc = "fCTQIN = fSCLK"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CTQSEL_A::VALUE3)
    }
    #[doc = "fCTQIN = fMCLK"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CTQSEL_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `PCTQ`"]
pub type PCTQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCTQ`"]
pub struct PCTQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PCTQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `DCTQ`"]
pub type DCTQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DCTQ`"]
pub struct DCTQ_W<'a> {
    w: &'a mut W,
}
impl<'a> DCTQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | (((value as u32) & 0x1f) << 10);
        self.w
    }
}
#[doc = "Reader of field `PDIV`"]
pub type PDIV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PDIV`"]
pub struct PDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Shift Clock Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCLKOSEL_A {
    #[doc = "0: SCLK from the baud rate generator is selected as the SCLKOUT input source."]
    VALUE1 = 0,
    #[doc = "1: The transmit shift clock from DX1 input stage is selected as the SCLKOUT input source."]
    VALUE2 = 1,
}
impl From<SCLKOSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SCLKOSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SCLKOSEL`"]
pub type SCLKOSEL_R = crate::R<bool, SCLKOSEL_A>;
impl SCLKOSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCLKOSEL_A {
        match self.bits {
            false => SCLKOSEL_A::VALUE1,
            true => SCLKOSEL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SCLKOSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SCLKOSEL_A::VALUE2
    }
}
#[doc = "Write proxy for field `SCLKOSEL`"]
pub struct SCLKOSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLKOSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCLKOSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SCLK from the baud rate generator is selected as the SCLKOUT input source."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SCLKOSEL_A::VALUE1)
    }
    #[doc = "The transmit shift clock from DX1 input stage is selected as the SCLKOUT input source."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SCLKOSEL_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Master Clock Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCLKCFG_A {
    #[doc = "0: The passive level is 0."]
    VALUE1 = 0,
    #[doc = "1: The passive level is 1."]
    VALUE2 = 1,
}
impl From<MCLKCFG_A> for bool {
    #[inline(always)]
    fn from(variant: MCLKCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MCLKCFG`"]
pub type MCLKCFG_R = crate::R<bool, MCLKCFG_A>;
impl MCLKCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCLKCFG_A {
        match self.bits {
            false => MCLKCFG_A::VALUE1,
            true => MCLKCFG_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MCLKCFG_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MCLKCFG_A::VALUE2
    }
}
#[doc = "Write proxy for field `MCLKCFG`"]
pub struct MCLKCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> MCLKCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCLKCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The passive level is 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MCLKCFG_A::VALUE1)
    }
    #[doc = "The passive level is 1."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MCLKCFG_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Shift Clock Output Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SCLKCFG_A {
    #[doc = "0: The passive level is 0 and the delay is disabled."]
    VALUE1 = 0,
    #[doc = "1: The passive level is 1 and the delay is disabled."]
    VALUE2 = 1,
    #[doc = "2: The passive level is 0 and the delay is enabled."]
    VALUE3 = 2,
    #[doc = "3: The passive level is 1 and the delay is enabled."]
    VALUE4 = 3,
}
impl From<SCLKCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: SCLKCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SCLKCFG`"]
pub type SCLKCFG_R = crate::R<u8, SCLKCFG_A>;
impl SCLKCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCLKCFG_A {
        match self.bits {
            0 => SCLKCFG_A::VALUE1,
            1 => SCLKCFG_A::VALUE2,
            2 => SCLKCFG_A::VALUE3,
            3 => SCLKCFG_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SCLKCFG_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SCLKCFG_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SCLKCFG_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SCLKCFG_A::VALUE4
    }
}
#[doc = "Write proxy for field `SCLKCFG`"]
pub struct SCLKCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLKCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCLKCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "The passive level is 0 and the delay is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SCLKCFG_A::VALUE1)
    }
    #[doc = "The passive level is 1 and the delay is disabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SCLKCFG_A::VALUE2)
    }
    #[doc = "The passive level is 0 and the delay is enabled."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SCLKCFG_A::VALUE3)
    }
    #[doc = "The passive level is 1 and the delay is enabled."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SCLKCFG_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock Selection"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 3 - Timing Measurement Enable"]
    #[inline(always)]
    pub fn tmen(&self) -> TMEN_R {
        TMEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable 2:1 Divider for fPPP"]
    #[inline(always)]
    pub fn pppen(&self) -> PPPEN_R {
        PPPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Input Selection for CTQ"]
    #[inline(always)]
    pub fn ctqsel(&self) -> CTQSEL_R {
        CTQSEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Pre-Divider for Time Quanta Counter"]
    #[inline(always)]
    pub fn pctq(&self) -> PCTQ_R {
        PCTQ_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:14 - Denominator for Time Quanta Counter"]
    #[inline(always)]
    pub fn dctq(&self) -> DCTQ_R {
        DCTQ_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 16:25 - Divider Mode: Divider Factor to Generate fPDIV"]
    #[inline(always)]
    pub fn pdiv(&self) -> PDIV_R {
        PDIV_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 28 - Shift Clock Output Select"]
    #[inline(always)]
    pub fn sclkosel(&self) -> SCLKOSEL_R {
        SCLKOSEL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Master Clock Configuration"]
    #[inline(always)]
    pub fn mclkcfg(&self) -> MCLKCFG_R {
        MCLKCFG_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 30:31 - Shift Clock Output Configuration"]
    #[inline(always)]
    pub fn sclkcfg(&self) -> SCLKCFG_R {
        SCLKCFG_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock Selection"]
    #[inline(always)]
    pub fn clksel(&mut self) -> CLKSEL_W {
        CLKSEL_W { w: self }
    }
    #[doc = "Bit 3 - Timing Measurement Enable"]
    #[inline(always)]
    pub fn tmen(&mut self) -> TMEN_W {
        TMEN_W { w: self }
    }
    #[doc = "Bit 4 - Enable 2:1 Divider for fPPP"]
    #[inline(always)]
    pub fn pppen(&mut self) -> PPPEN_W {
        PPPEN_W { w: self }
    }
    #[doc = "Bits 6:7 - Input Selection for CTQ"]
    #[inline(always)]
    pub fn ctqsel(&mut self) -> CTQSEL_W {
        CTQSEL_W { w: self }
    }
    #[doc = "Bits 8:9 - Pre-Divider for Time Quanta Counter"]
    #[inline(always)]
    pub fn pctq(&mut self) -> PCTQ_W {
        PCTQ_W { w: self }
    }
    #[doc = "Bits 10:14 - Denominator for Time Quanta Counter"]
    #[inline(always)]
    pub fn dctq(&mut self) -> DCTQ_W {
        DCTQ_W { w: self }
    }
    #[doc = "Bits 16:25 - Divider Mode: Divider Factor to Generate fPDIV"]
    #[inline(always)]
    pub fn pdiv(&mut self) -> PDIV_W {
        PDIV_W { w: self }
    }
    #[doc = "Bit 28 - Shift Clock Output Select"]
    #[inline(always)]
    pub fn sclkosel(&mut self) -> SCLKOSEL_W {
        SCLKOSEL_W { w: self }
    }
    #[doc = "Bit 29 - Master Clock Configuration"]
    #[inline(always)]
    pub fn mclkcfg(&mut self) -> MCLKCFG_W {
        MCLKCFG_W { w: self }
    }
    #[doc = "Bits 30:31 - Shift Clock Output Configuration"]
    #[inline(always)]
    pub fn sclkcfg(&mut self) -> SCLKCFG_W {
        SCLKCFG_W { w: self }
    }
}
