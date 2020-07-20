#[doc = "Reader of register GCTRL"]
pub type R = crate::R<u32, super::GCTRL>;
#[doc = "Writer for register GCTRL"]
pub type W = crate::W<u32, super::GCTRL>;
#[doc = "Register GCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::GCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Prescaler Clear Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRBC_A {
    #[doc = "0: SW only"]
    VALUE1 = 0,
    #[doc = "1: GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC80 is cleared."]
    VALUE2 = 1,
    #[doc = "2: GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC81 is cleared."]
    VALUE3 = 2,
    #[doc = "3: GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC82 is cleared."]
    VALUE4 = 3,
    #[doc = "4: GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC83 is cleared."]
    VALUE5 = 4,
}
impl From<PRBC_A> for u8 {
    #[inline(always)]
    fn from(variant: PRBC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRBC`"]
pub type PRBC_R = crate::R<u8, PRBC_A>;
impl PRBC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PRBC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PRBC_A::VALUE1),
            1 => Val(PRBC_A::VALUE2),
            2 => Val(PRBC_A::VALUE3),
            3 => Val(PRBC_A::VALUE4),
            4 => Val(PRBC_A::VALUE5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PRBC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PRBC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PRBC_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PRBC_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == PRBC_A::VALUE5
    }
}
#[doc = "Write proxy for field `PRBC`"]
pub struct PRBC_W<'a> {
    w: &'a mut W,
}
impl<'a> PRBC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRBC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SW only"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PRBC_A::VALUE1)
    }
    #[doc = "GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC80 is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRBC_A::VALUE2)
    }
    #[doc = "GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC81 is cleared."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PRBC_A::VALUE3)
    }
    #[doc = "GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC82 is cleared."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PRBC_A::VALUE4)
    }
    #[doc = "GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC83 is cleared."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(PRBC_A::VALUE5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Prescaler Input Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCIS_A {
    #[doc = "0: Module clock"]
    VALUE1 = 0,
    #[doc = "1: CCU8x.ECLKA"]
    VALUE2 = 1,
    #[doc = "2: CCU8x.ECLKB"]
    VALUE3 = 2,
    #[doc = "3: CCU8x.ECLKC"]
    VALUE4 = 3,
}
impl From<PCIS_A> for u8 {
    #[inline(always)]
    fn from(variant: PCIS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PCIS`"]
pub type PCIS_R = crate::R<u8, PCIS_A>;
impl PCIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCIS_A {
        match self.bits {
            0 => PCIS_A::VALUE1,
            1 => PCIS_A::VALUE2,
            2 => PCIS_A::VALUE3,
            3 => PCIS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PCIS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PCIS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PCIS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PCIS_A::VALUE4
    }
}
#[doc = "Write proxy for field `PCIS`"]
pub struct PCIS_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCIS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Module clock"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PCIS_A::VALUE1)
    }
    #[doc = "CCU8x.ECLKA"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PCIS_A::VALUE2)
    }
    #[doc = "CCU8x.ECLKB"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PCIS_A::VALUE3)
    }
    #[doc = "CCU8x.ECLKC"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PCIS_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Suspend Mode Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SUSCFG_A {
    #[doc = "0: Suspend request ignored. The module never enters in suspend"]
    VALUE1 = 0,
    #[doc = "1: Stops all the running slices immediately. Safe stop is not applied."]
    VALUE2 = 1,
    #[doc = "2: Stops the block immediately and clamps all the outputs to PASSIVE state. Safe stop is applied."]
    VALUE3 = 2,
    #[doc = "3: Waits for the roll over of each slice to stop and clamp the slices outputs. Safe stop is applied."]
    VALUE4 = 3,
}
impl From<SUSCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: SUSCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SUSCFG`"]
pub type SUSCFG_R = crate::R<u8, SUSCFG_A>;
impl SUSCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUSCFG_A {
        match self.bits {
            0 => SUSCFG_A::VALUE1,
            1 => SUSCFG_A::VALUE2,
            2 => SUSCFG_A::VALUE3,
            3 => SUSCFG_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SUSCFG_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SUSCFG_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SUSCFG_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SUSCFG_A::VALUE4
    }
}
#[doc = "Write proxy for field `SUSCFG`"]
pub struct SUSCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUSCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Suspend request ignored. The module never enters in suspend"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SUSCFG_A::VALUE1)
    }
    #[doc = "Stops all the running slices immediately. Safe stop is not applied."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SUSCFG_A::VALUE2)
    }
    #[doc = "Stops the block immediately and clamps all the outputs to PASSIVE state. Safe stop is applied."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SUSCFG_A::VALUE3)
    }
    #[doc = "Waits for the roll over of each slice to stop and clamp the slices outputs. Safe stop is applied."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SUSCFG_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Slice 0 Multi Channel shadow transfer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSE0_A {
    #[doc = "0: Shadow transfer can only be requested by SW"]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer can be requested via SW and via the CCU8x.MCSS input."]
    VALUE2 = 1,
}
impl From<MSE0_A> for bool {
    #[inline(always)]
    fn from(variant: MSE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MSE0`"]
pub type MSE0_R = crate::R<bool, MSE0_A>;
impl MSE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSE0_A {
        match self.bits {
            false => MSE0_A::VALUE1,
            true => MSE0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MSE0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MSE0_A::VALUE2
    }
}
#[doc = "Write proxy for field `MSE0`"]
pub struct MSE0_W<'a> {
    w: &'a mut W,
}
impl<'a> MSE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSE0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Shadow transfer can only be requested by SW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MSE0_A::VALUE1)
    }
    #[doc = "Shadow transfer can be requested via SW and via the CCU8x.MCSS input."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MSE0_A::VALUE2)
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
#[doc = "Slice 1 Multi Channel shadow transfer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSE1_A {
    #[doc = "0: Shadow transfer can only be requested by SW"]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer can be requested via SW and via the CCU8x.MCSS input."]
    VALUE2 = 1,
}
impl From<MSE1_A> for bool {
    #[inline(always)]
    fn from(variant: MSE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MSE1`"]
pub type MSE1_R = crate::R<bool, MSE1_A>;
impl MSE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSE1_A {
        match self.bits {
            false => MSE1_A::VALUE1,
            true => MSE1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MSE1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MSE1_A::VALUE2
    }
}
#[doc = "Write proxy for field `MSE1`"]
pub struct MSE1_W<'a> {
    w: &'a mut W,
}
impl<'a> MSE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSE1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Shadow transfer can only be requested by SW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MSE1_A::VALUE1)
    }
    #[doc = "Shadow transfer can be requested via SW and via the CCU8x.MCSS input."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MSE1_A::VALUE2)
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
#[doc = "Slice 2 Multi Channel shadow transfer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSE2_A {
    #[doc = "0: Shadow transfer can only be requested by SW"]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer can be requested via SW and via the CCU8xMCSS input."]
    VALUE2 = 1,
}
impl From<MSE2_A> for bool {
    #[inline(always)]
    fn from(variant: MSE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MSE2`"]
pub type MSE2_R = crate::R<bool, MSE2_A>;
impl MSE2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSE2_A {
        match self.bits {
            false => MSE2_A::VALUE1,
            true => MSE2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MSE2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MSE2_A::VALUE2
    }
}
#[doc = "Write proxy for field `MSE2`"]
pub struct MSE2_W<'a> {
    w: &'a mut W,
}
impl<'a> MSE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSE2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Shadow transfer can only be requested by SW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MSE2_A::VALUE1)
    }
    #[doc = "Shadow transfer can be requested via SW and via the CCU8xMCSS input."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MSE2_A::VALUE2)
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
#[doc = "Slice 3 Multi Channel shadow transfer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSE3_A {
    #[doc = "0: Shadow transfer can only be requested by SW"]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer can be requested via SW and via the CCU8x.MCSS input."]
    VALUE2 = 1,
}
impl From<MSE3_A> for bool {
    #[inline(always)]
    fn from(variant: MSE3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MSE3`"]
pub type MSE3_R = crate::R<bool, MSE3_A>;
impl MSE3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSE3_A {
        match self.bits {
            false => MSE3_A::VALUE1,
            true => MSE3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MSE3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MSE3_A::VALUE2
    }
}
#[doc = "Write proxy for field `MSE3`"]
pub struct MSE3_W<'a> {
    w: &'a mut W,
}
impl<'a> MSE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSE3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Shadow transfer can only be requested by SW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MSE3_A::VALUE1)
    }
    #[doc = "Shadow transfer can be requested via SW and via the CCU8x.MCSS input."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MSE3_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Multi Channel shadow transfer request configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MSDE_A {
    #[doc = "0: Only the shadow transfer for period and compare values is requested"]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer for the compare, period and prescaler compare values is requested"]
    VALUE2 = 1,
    #[doc = "3: Shadow transfer for the compare, period, prescaler and dither compare values is requested"]
    VALUE4 = 3,
}
impl From<MSDE_A> for u8 {
    #[inline(always)]
    fn from(variant: MSDE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MSDE`"]
pub type MSDE_R = crate::R<u8, MSDE_A>;
impl MSDE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MSDE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MSDE_A::VALUE1),
            1 => Val(MSDE_A::VALUE2),
            3 => Val(MSDE_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MSDE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MSDE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == MSDE_A::VALUE4
    }
}
#[doc = "Write proxy for field `MSDE`"]
pub struct MSDE_W<'a> {
    w: &'a mut W,
}
impl<'a> MSDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSDE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Only the shadow transfer for period and compare values is requested"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MSDE_A::VALUE1)
    }
    #[doc = "Shadow transfer for the compare, period and prescaler compare values is requested"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MSDE_A::VALUE2)
    }
    #[doc = "Shadow transfer for the compare, period, prescaler and dither compare values is requested"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(MSDE_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Prescaler Clear Configuration"]
    #[inline(always)]
    pub fn prbc(&self) -> PRBC_R {
        PRBC_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:5 - Prescaler Input Clock Selection"]
    #[inline(always)]
    pub fn pcis(&self) -> PCIS_R {
        PCIS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Suspend Mode Configuration"]
    #[inline(always)]
    pub fn suscfg(&self) -> SUSCFG_R {
        SUSCFG_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Slice 0 Multi Channel shadow transfer enable"]
    #[inline(always)]
    pub fn mse0(&self) -> MSE0_R {
        MSE0_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Slice 1 Multi Channel shadow transfer enable"]
    #[inline(always)]
    pub fn mse1(&self) -> MSE1_R {
        MSE1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Slice 2 Multi Channel shadow transfer enable"]
    #[inline(always)]
    pub fn mse2(&self) -> MSE2_R {
        MSE2_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Slice 3 Multi Channel shadow transfer enable"]
    #[inline(always)]
    pub fn mse3(&self) -> MSE3_R {
        MSE3_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Multi Channel shadow transfer request configuration"]
    #[inline(always)]
    pub fn msde(&self) -> MSDE_R {
        MSDE_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Prescaler Clear Configuration"]
    #[inline(always)]
    pub fn prbc(&mut self) -> PRBC_W {
        PRBC_W { w: self }
    }
    #[doc = "Bits 4:5 - Prescaler Input Clock Selection"]
    #[inline(always)]
    pub fn pcis(&mut self) -> PCIS_W {
        PCIS_W { w: self }
    }
    #[doc = "Bits 8:9 - Suspend Mode Configuration"]
    #[inline(always)]
    pub fn suscfg(&mut self) -> SUSCFG_W {
        SUSCFG_W { w: self }
    }
    #[doc = "Bit 10 - Slice 0 Multi Channel shadow transfer enable"]
    #[inline(always)]
    pub fn mse0(&mut self) -> MSE0_W {
        MSE0_W { w: self }
    }
    #[doc = "Bit 11 - Slice 1 Multi Channel shadow transfer enable"]
    #[inline(always)]
    pub fn mse1(&mut self) -> MSE1_W {
        MSE1_W { w: self }
    }
    #[doc = "Bit 12 - Slice 2 Multi Channel shadow transfer enable"]
    #[inline(always)]
    pub fn mse2(&mut self) -> MSE2_W {
        MSE2_W { w: self }
    }
    #[doc = "Bit 13 - Slice 3 Multi Channel shadow transfer enable"]
    #[inline(always)]
    pub fn mse3(&mut self) -> MSE3_W {
        MSE3_W { w: self }
    }
    #[doc = "Bits 14:15 - Multi Channel shadow transfer request configuration"]
    #[inline(always)]
    pub fn msde(&mut self) -> MSDE_W {
        MSDE_W { w: self }
    }
}
