#[doc = "Reader of register GLOBCFG"]
pub type R = crate::R<u32, super::GLOBCFG>;
#[doc = "Writer for register GLOBCFG"]
pub type W = crate::W<u32, super::GLOBCFG>;
#[doc = "Register GLOBCFG `reset()`'s with value 0x0f"]
impl crate::ResetValue for super::GLOBCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f
    }
}
#[doc = "Divider Factor for the Analog Internal Clock\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIVA_A {
    #[doc = "0: fADCI = fADC / 2"]
    VALUE1 = 0,
    #[doc = "1: fADCI = fADC / 2"]
    VALUE2 = 1,
    #[doc = "2: fADCI = fADC / 3"]
    VALUE3 = 2,
    #[doc = "31: fADCI = fADC / 32"]
    VALUE4 = 31,
}
impl From<DIVA_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVA_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DIVA`"]
pub type DIVA_R = crate::R<u8, DIVA_A>;
impl DIVA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DIVA_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DIVA_A::VALUE1),
            1 => Val(DIVA_A::VALUE2),
            2 => Val(DIVA_A::VALUE3),
            31 => Val(DIVA_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DIVA_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DIVA_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == DIVA_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == DIVA_A::VALUE4
    }
}
#[doc = "Write proxy for field `DIVA`"]
pub struct DIVA_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVA_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "fADCI = fADC / 2"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DIVA_A::VALUE1)
    }
    #[doc = "fADCI = fADC / 2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DIVA_A::VALUE2)
    }
    #[doc = "fADCI = fADC / 3"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(DIVA_A::VALUE3)
    }
    #[doc = "fADCI = fADC / 32"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(DIVA_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Double Clock for the MSB Conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCMSB_A {
    #[doc = "0: 1 clock cycles for the MSB (standard)"]
    VALUE1 = 0,
    #[doc = "1: 2 clock cycles for the MSB (fADCI > 20 MHz)"]
    VALUE2 = 1,
}
impl From<DCMSB_A> for bool {
    #[inline(always)]
    fn from(variant: DCMSB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DCMSB`"]
pub type DCMSB_R = crate::R<bool, DCMSB_A>;
impl DCMSB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCMSB_A {
        match self.bits {
            false => DCMSB_A::VALUE1,
            true => DCMSB_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DCMSB_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DCMSB_A::VALUE2
    }
}
#[doc = "Write proxy for field `DCMSB`"]
pub struct DCMSB_W<'a> {
    w: &'a mut W,
}
impl<'a> DCMSB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCMSB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "1 clock cycles for the MSB (standard)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DCMSB_A::VALUE1)
    }
    #[doc = "2 clock cycles for the MSB (fADCI > 20 MHz)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DCMSB_A::VALUE2)
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
#[doc = "Divider Factor for the Arbiter Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIVD_A {
    #[doc = "0: fADCD = fADC"]
    VALUE1 = 0,
    #[doc = "1: fADCD = fADC / 2"]
    VALUE2 = 1,
    #[doc = "2: fADCD = fADC / 3"]
    VALUE3 = 2,
    #[doc = "3: fADCD = fADC / 4"]
    VALUE4 = 3,
}
impl From<DIVD_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DIVD`"]
pub type DIVD_R = crate::R<u8, DIVD_A>;
impl DIVD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIVD_A {
        match self.bits {
            0 => DIVD_A::VALUE1,
            1 => DIVD_A::VALUE2,
            2 => DIVD_A::VALUE3,
            3 => DIVD_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DIVD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DIVD_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == DIVD_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == DIVD_A::VALUE4
    }
}
#[doc = "Write proxy for field `DIVD`"]
pub struct DIVD_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "fADCD = fADC"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DIVD_A::VALUE1)
    }
    #[doc = "fADCD = fADC / 2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DIVD_A::VALUE2)
    }
    #[doc = "fADCD = fADC / 3"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(DIVD_A::VALUE3)
    }
    #[doc = "fADCD = fADC / 4"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(DIVD_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Write Control for Divider Parameters\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVWC_AW {
    #[doc = "0: No write access to divider parameters"]
    VALUE1 = 0,
    #[doc = "1: Bitfields DIVA, DCMSB, DIVD can be written"]
    VALUE2 = 1,
}
impl From<DIVWC_AW> for bool {
    #[inline(always)]
    fn from(variant: DIVWC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DIVWC`"]
pub struct DIVWC_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVWC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVWC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No write access to divider parameters"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DIVWC_AW::VALUE1)
    }
    #[doc = "Bitfields DIVA, DCMSB, DIVD can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DIVWC_AW::VALUE2)
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
#[doc = "Disable Post-Calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPCAL0_A {
    #[doc = "0: Automatic post-calibration after each conversion of group x"]
    VALUE1 = 0,
    #[doc = "1: No post-calibration"]
    VALUE2 = 1,
}
impl From<DPCAL0_A> for bool {
    #[inline(always)]
    fn from(variant: DPCAL0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DPCAL0`"]
pub type DPCAL0_R = crate::R<bool, DPCAL0_A>;
impl DPCAL0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPCAL0_A {
        match self.bits {
            false => DPCAL0_A::VALUE1,
            true => DPCAL0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DPCAL0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DPCAL0_A::VALUE2
    }
}
#[doc = "Write proxy for field `DPCAL0`"]
pub struct DPCAL0_W<'a> {
    w: &'a mut W,
}
impl<'a> DPCAL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DPCAL0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Automatic post-calibration after each conversion of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DPCAL0_A::VALUE1)
    }
    #[doc = "No post-calibration"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DPCAL0_A::VALUE2)
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
#[doc = "Disable Post-Calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPCAL1_A {
    #[doc = "0: Automatic post-calibration after each conversion of group x"]
    VALUE1 = 0,
    #[doc = "1: No post-calibration"]
    VALUE2 = 1,
}
impl From<DPCAL1_A> for bool {
    #[inline(always)]
    fn from(variant: DPCAL1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DPCAL1`"]
pub type DPCAL1_R = crate::R<bool, DPCAL1_A>;
impl DPCAL1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPCAL1_A {
        match self.bits {
            false => DPCAL1_A::VALUE1,
            true => DPCAL1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DPCAL1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DPCAL1_A::VALUE2
    }
}
#[doc = "Write proxy for field `DPCAL1`"]
pub struct DPCAL1_W<'a> {
    w: &'a mut W,
}
impl<'a> DPCAL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DPCAL1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Automatic post-calibration after each conversion of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DPCAL1_A::VALUE1)
    }
    #[doc = "No post-calibration"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DPCAL1_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Disable Post-Calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPCAL2_A {
    #[doc = "0: Automatic post-calibration after each conversion of group x"]
    VALUE1 = 0,
    #[doc = "1: No post-calibration"]
    VALUE2 = 1,
}
impl From<DPCAL2_A> for bool {
    #[inline(always)]
    fn from(variant: DPCAL2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DPCAL2`"]
pub type DPCAL2_R = crate::R<bool, DPCAL2_A>;
impl DPCAL2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPCAL2_A {
        match self.bits {
            false => DPCAL2_A::VALUE1,
            true => DPCAL2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DPCAL2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DPCAL2_A::VALUE2
    }
}
#[doc = "Write proxy for field `DPCAL2`"]
pub struct DPCAL2_W<'a> {
    w: &'a mut W,
}
impl<'a> DPCAL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DPCAL2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Automatic post-calibration after each conversion of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DPCAL2_A::VALUE1)
    }
    #[doc = "No post-calibration"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DPCAL2_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Disable Post-Calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPCAL3_A {
    #[doc = "0: Automatic post-calibration after each conversion of group x"]
    VALUE1 = 0,
    #[doc = "1: No post-calibration"]
    VALUE2 = 1,
}
impl From<DPCAL3_A> for bool {
    #[inline(always)]
    fn from(variant: DPCAL3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DPCAL3`"]
pub type DPCAL3_R = crate::R<bool, DPCAL3_A>;
impl DPCAL3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPCAL3_A {
        match self.bits {
            false => DPCAL3_A::VALUE1,
            true => DPCAL3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DPCAL3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DPCAL3_A::VALUE2
    }
}
#[doc = "Write proxy for field `DPCAL3`"]
pub struct DPCAL3_W<'a> {
    w: &'a mut W,
}
impl<'a> DPCAL3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DPCAL3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Automatic post-calibration after each conversion of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DPCAL3_A::VALUE1)
    }
    #[doc = "No post-calibration"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DPCAL3_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Start-Up Calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUCAL_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Initiate the start-up calibration phase (indication in bit GxARBCFG.CAL)"]
    VALUE2 = 1,
}
impl From<SUCAL_AW> for bool {
    #[inline(always)]
    fn from(variant: SUCAL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SUCAL`"]
pub struct SUCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SUCAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUCAL_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SUCAL_AW::VALUE1)
    }
    #[doc = "Initiate the start-up calibration phase (indication in bit GxARBCFG.CAL)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SUCAL_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Divider Factor for the Analog Internal Clock"]
    #[inline(always)]
    pub fn diva(&self) -> DIVA_R {
        DIVA_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Double Clock for the MSB Conversion"]
    #[inline(always)]
    pub fn dcmsb(&self) -> DCMSB_R {
        DCMSB_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Divider Factor for the Arbiter Clock"]
    #[inline(always)]
    pub fn divd(&self) -> DIVD_R {
        DIVD_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Disable Post-Calibration"]
    #[inline(always)]
    pub fn dpcal0(&self) -> DPCAL0_R {
        DPCAL0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Disable Post-Calibration"]
    #[inline(always)]
    pub fn dpcal1(&self) -> DPCAL1_R {
        DPCAL1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Disable Post-Calibration"]
    #[inline(always)]
    pub fn dpcal2(&self) -> DPCAL2_R {
        DPCAL2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Disable Post-Calibration"]
    #[inline(always)]
    pub fn dpcal3(&self) -> DPCAL3_R {
        DPCAL3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Divider Factor for the Analog Internal Clock"]
    #[inline(always)]
    pub fn diva(&mut self) -> DIVA_W {
        DIVA_W { w: self }
    }
    #[doc = "Bit 7 - Double Clock for the MSB Conversion"]
    #[inline(always)]
    pub fn dcmsb(&mut self) -> DCMSB_W {
        DCMSB_W { w: self }
    }
    #[doc = "Bits 8:9 - Divider Factor for the Arbiter Clock"]
    #[inline(always)]
    pub fn divd(&mut self) -> DIVD_W {
        DIVD_W { w: self }
    }
    #[doc = "Bit 15 - Write Control for Divider Parameters"]
    #[inline(always)]
    pub fn divwc(&mut self) -> DIVWC_W {
        DIVWC_W { w: self }
    }
    #[doc = "Bit 16 - Disable Post-Calibration"]
    #[inline(always)]
    pub fn dpcal0(&mut self) -> DPCAL0_W {
        DPCAL0_W { w: self }
    }
    #[doc = "Bit 17 - Disable Post-Calibration"]
    #[inline(always)]
    pub fn dpcal1(&mut self) -> DPCAL1_W {
        DPCAL1_W { w: self }
    }
    #[doc = "Bit 18 - Disable Post-Calibration"]
    #[inline(always)]
    pub fn dpcal2(&mut self) -> DPCAL2_W {
        DPCAL2_W { w: self }
    }
    #[doc = "Bit 19 - Disable Post-Calibration"]
    #[inline(always)]
    pub fn dpcal3(&mut self) -> DPCAL3_W {
        DPCAL3_W { w: self }
    }
    #[doc = "Bit 31 - Start-Up Calibration"]
    #[inline(always)]
    pub fn sucal(&mut self) -> SUCAL_W {
        SUCAL_W { w: self }
    }
}
