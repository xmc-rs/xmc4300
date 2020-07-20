#[doc = "Reader of register CHCTR[%s]"]
pub type R = crate::R<u32, super::CHCTR>;
#[doc = "Writer for register CHCTR[%s]"]
pub type W = crate::W<u32, super::CHCTR>;
#[doc = "Register CHCTR[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::CHCTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Input Class Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ICLSEL_A {
    #[doc = "0: Use group-specific class 0"]
    VALUE1 = 0,
    #[doc = "1: Use group-specific class 1"]
    VALUE2 = 1,
    #[doc = "2: Use global class 0"]
    VALUE3 = 2,
    #[doc = "3: Use global class 1"]
    VALUE4 = 3,
}
impl From<ICLSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ICLSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ICLSEL`"]
pub type ICLSEL_R = crate::R<u8, ICLSEL_A>;
impl ICLSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICLSEL_A {
        match self.bits {
            0 => ICLSEL_A::VALUE1,
            1 => ICLSEL_A::VALUE2,
            2 => ICLSEL_A::VALUE3,
            3 => ICLSEL_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ICLSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ICLSEL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ICLSEL_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == ICLSEL_A::VALUE4
    }
}
#[doc = "Write proxy for field `ICLSEL`"]
pub struct ICLSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ICLSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICLSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Use group-specific class 0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ICLSEL_A::VALUE1)
    }
    #[doc = "Use group-specific class 1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ICLSEL_A::VALUE2)
    }
    #[doc = "Use global class 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(ICLSEL_A::VALUE3)
    }
    #[doc = "Use global class 1"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(ICLSEL_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Lower Boundary Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BNDSELL_A {
    #[doc = "0: Use group-specific boundary 0"]
    VALUE1 = 0,
    #[doc = "1: Use group-specific boundary 1"]
    VALUE2 = 1,
    #[doc = "2: Use global boundary 0"]
    VALUE3 = 2,
    #[doc = "3: Use global boundary 1"]
    VALUE4 = 3,
}
impl From<BNDSELL_A> for u8 {
    #[inline(always)]
    fn from(variant: BNDSELL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BNDSELL`"]
pub type BNDSELL_R = crate::R<u8, BNDSELL_A>;
impl BNDSELL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNDSELL_A {
        match self.bits {
            0 => BNDSELL_A::VALUE1,
            1 => BNDSELL_A::VALUE2,
            2 => BNDSELL_A::VALUE3,
            3 => BNDSELL_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BNDSELL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BNDSELL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == BNDSELL_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == BNDSELL_A::VALUE4
    }
}
#[doc = "Write proxy for field `BNDSELL`"]
pub struct BNDSELL_W<'a> {
    w: &'a mut W,
}
impl<'a> BNDSELL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BNDSELL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Use group-specific boundary 0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BNDSELL_A::VALUE1)
    }
    #[doc = "Use group-specific boundary 1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BNDSELL_A::VALUE2)
    }
    #[doc = "Use global boundary 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(BNDSELL_A::VALUE3)
    }
    #[doc = "Use global boundary 1"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(BNDSELL_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Upper Boundary Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BNDSELU_A {
    #[doc = "0: Use group-specific boundary 0"]
    VALUE1 = 0,
    #[doc = "1: Use group-specific boundary 1"]
    VALUE2 = 1,
    #[doc = "2: Use global boundary 0"]
    VALUE3 = 2,
    #[doc = "3: Use global boundary 1"]
    VALUE4 = 3,
}
impl From<BNDSELU_A> for u8 {
    #[inline(always)]
    fn from(variant: BNDSELU_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BNDSELU`"]
pub type BNDSELU_R = crate::R<u8, BNDSELU_A>;
impl BNDSELU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNDSELU_A {
        match self.bits {
            0 => BNDSELU_A::VALUE1,
            1 => BNDSELU_A::VALUE2,
            2 => BNDSELU_A::VALUE3,
            3 => BNDSELU_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BNDSELU_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BNDSELU_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == BNDSELU_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == BNDSELU_A::VALUE4
    }
}
#[doc = "Write proxy for field `BNDSELU`"]
pub struct BNDSELU_W<'a> {
    w: &'a mut W,
}
impl<'a> BNDSELU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BNDSELU_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Use group-specific boundary 0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BNDSELU_A::VALUE1)
    }
    #[doc = "Use group-specific boundary 1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BNDSELU_A::VALUE2)
    }
    #[doc = "Use global boundary 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(BNDSELU_A::VALUE3)
    }
    #[doc = "Use global boundary 1"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(BNDSELU_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Channel Event Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHEVMODE_A {
    #[doc = "0: Never"]
    VALUE1 = 0,
    #[doc = "1: NCM: If result is inside the boundary band FCM: If result becomes high (above cmp. val.)"]
    VALUE2 = 1,
    #[doc = "2: NCM: If result is outside the boundary band FCM: If result becomes low (below cmp. val.)"]
    VALUE3 = 2,
    #[doc = "3: NCM: Always (ignore band) FCM: If result switches to either level"]
    VALUE4 = 3,
}
impl From<CHEVMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CHEVMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CHEVMODE`"]
pub type CHEVMODE_R = crate::R<u8, CHEVMODE_A>;
impl CHEVMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEVMODE_A {
        match self.bits {
            0 => CHEVMODE_A::VALUE1,
            1 => CHEVMODE_A::VALUE2,
            2 => CHEVMODE_A::VALUE3,
            3 => CHEVMODE_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHEVMODE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHEVMODE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CHEVMODE_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CHEVMODE_A::VALUE4
    }
}
#[doc = "Write proxy for field `CHEVMODE`"]
pub struct CHEVMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEVMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEVMODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Never"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHEVMODE_A::VALUE1)
    }
    #[doc = "NCM: If result is inside the boundary band FCM: If result becomes high (above cmp. val.)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHEVMODE_A::VALUE2)
    }
    #[doc = "NCM: If result is outside the boundary band FCM: If result becomes low (below cmp. val.)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CHEVMODE_A::VALUE3)
    }
    #[doc = "NCM: Always (ignore band) FCM: If result switches to either level"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CHEVMODE_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Synchronization Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC_A {
    #[doc = "0: No synchroniz. request, standalone operation"]
    VALUE1 = 0,
    #[doc = "1: Request a synchronized conversion of this channel (only taken into account for a master)"]
    VALUE2 = 1,
}
impl From<SYNC_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYNC`"]
pub type SYNC_R = crate::R<bool, SYNC_A>;
impl SYNC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNC_A {
        match self.bits {
            false => SYNC_A::VALUE1,
            true => SYNC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SYNC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SYNC_A::VALUE2
    }
}
#[doc = "Write proxy for field `SYNC`"]
pub struct SYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No synchroniz. request, standalone operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SYNC_A::VALUE1)
    }
    #[doc = "Request a synchronized conversion of this channel (only taken into account for a master)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SYNC_A::VALUE2)
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
#[doc = "Reference Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFSEL_A {
    #[doc = "0: Standard reference input VAREF"]
    VALUE1 = 0,
    #[doc = "1: Alternate reference input from CH0"]
    VALUE2 = 1,
}
impl From<REFSEL_A> for bool {
    #[inline(always)]
    fn from(variant: REFSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REFSEL`"]
pub type REFSEL_R = crate::R<bool, REFSEL_A>;
impl REFSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFSEL_A {
        match self.bits {
            false => REFSEL_A::VALUE1,
            true => REFSEL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == REFSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REFSEL_A::VALUE2
    }
}
#[doc = "Write proxy for field `REFSEL`"]
pub struct REFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REFSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Standard reference input VAREF"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REFSEL_A::VALUE1)
    }
    #[doc = "Alternate reference input from CH0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REFSEL_A::VALUE2)
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
#[doc = "Result Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RESREG_A {
    #[doc = "0: Store result in group result register GxRES0"]
    VALUE1 = 0,
    #[doc = "15: Store result in group result register GxRES15"]
    VALUE2 = 15,
}
impl From<RESREG_A> for u8 {
    #[inline(always)]
    fn from(variant: RESREG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RESREG`"]
pub type RESREG_R = crate::R<u8, RESREG_A>;
impl RESREG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RESREG_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RESREG_A::VALUE1),
            15 => Val(RESREG_A::VALUE2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RESREG_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RESREG_A::VALUE2
    }
}
#[doc = "Write proxy for field `RESREG`"]
pub struct RESREG_W<'a> {
    w: &'a mut W,
}
impl<'a> RESREG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESREG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Store result in group result register GxRES0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RESREG_A::VALUE1)
    }
    #[doc = "Store result in group result register GxRES15"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RESREG_A::VALUE2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Result Target for Background Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESTBS_A {
    #[doc = "0: Store results in the selected group result register"]
    VALUE1 = 0,
    #[doc = "1: Store results in the global result register"]
    VALUE2 = 1,
}
impl From<RESTBS_A> for bool {
    #[inline(always)]
    fn from(variant: RESTBS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RESTBS`"]
pub type RESTBS_R = crate::R<bool, RESTBS_A>;
impl RESTBS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESTBS_A {
        match self.bits {
            false => RESTBS_A::VALUE1,
            true => RESTBS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RESTBS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RESTBS_A::VALUE2
    }
}
#[doc = "Write proxy for field `RESTBS`"]
pub struct RESTBS_W<'a> {
    w: &'a mut W,
}
impl<'a> RESTBS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESTBS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Store results in the selected group result register"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RESTBS_A::VALUE1)
    }
    #[doc = "Store results in the global result register"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RESTBS_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Result Position\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESPOS_A {
    #[doc = "0: Store results left-aligned"]
    VALUE1 = 0,
    #[doc = "1: Store results right-aligned"]
    VALUE2 = 1,
}
impl From<RESPOS_A> for bool {
    #[inline(always)]
    fn from(variant: RESPOS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RESPOS`"]
pub type RESPOS_R = crate::R<bool, RESPOS_A>;
impl RESPOS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESPOS_A {
        match self.bits {
            false => RESPOS_A::VALUE1,
            true => RESPOS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RESPOS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RESPOS_A::VALUE2
    }
}
#[doc = "Write proxy for field `RESPOS`"]
pub struct RESPOS_W<'a> {
    w: &'a mut W,
}
impl<'a> RESPOS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESPOS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Store results left-aligned"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RESPOS_A::VALUE1)
    }
    #[doc = "Store results right-aligned"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RESPOS_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Broken Wire Detection Channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BWDCH_A {
    #[doc = "0: Select VAGND"]
    VALUE1 = 0,
    #[doc = "1: Select VAREF"]
    VALUE2 = 1,
}
impl From<BWDCH_A> for u8 {
    #[inline(always)]
    fn from(variant: BWDCH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BWDCH`"]
pub type BWDCH_R = crate::R<u8, BWDCH_A>;
impl BWDCH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BWDCH_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BWDCH_A::VALUE1),
            1 => Val(BWDCH_A::VALUE2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BWDCH_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BWDCH_A::VALUE2
    }
}
#[doc = "Write proxy for field `BWDCH`"]
pub struct BWDCH_W<'a> {
    w: &'a mut W,
}
impl<'a> BWDCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BWDCH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select VAGND"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BWDCH_A::VALUE1)
    }
    #[doc = "Select VAREF"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BWDCH_A::VALUE2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Broken Wire Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWDEN_A {
    #[doc = "0: Normal operation"]
    VALUE1 = 0,
    #[doc = "1: Additional preparation phase is enabled"]
    VALUE2 = 1,
}
impl From<BWDEN_A> for bool {
    #[inline(always)]
    fn from(variant: BWDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BWDEN`"]
pub type BWDEN_R = crate::R<bool, BWDEN_A>;
impl BWDEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWDEN_A {
        match self.bits {
            false => BWDEN_A::VALUE1,
            true => BWDEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BWDEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BWDEN_A::VALUE2
    }
}
#[doc = "Write proxy for field `BWDEN`"]
pub struct BWDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BWDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BWDEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BWDEN_A::VALUE1)
    }
    #[doc = "Additional preparation phase is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BWDEN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Input Class Select"]
    #[inline(always)]
    pub fn iclsel(&self) -> ICLSEL_R {
        ICLSEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Lower Boundary Select"]
    #[inline(always)]
    pub fn bndsell(&self) -> BNDSELL_R {
        BNDSELL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Upper Boundary Select"]
    #[inline(always)]
    pub fn bndselu(&self) -> BNDSELU_R {
        BNDSELU_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Channel Event Mode"]
    #[inline(always)]
    pub fn chevmode(&self) -> CHEVMODE_R {
        CHEVMODE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Synchronization Request"]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Reference Input Selection"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Result Register"]
    #[inline(always)]
    pub fn resreg(&self) -> RESREG_R {
        RESREG_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Result Target for Background Source"]
    #[inline(always)]
    pub fn restbs(&self) -> RESTBS_R {
        RESTBS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Result Position"]
    #[inline(always)]
    pub fn respos(&self) -> RESPOS_R {
        RESPOS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - Broken Wire Detection Channel"]
    #[inline(always)]
    pub fn bwdch(&self) -> BWDCH_R {
        BWDCH_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 30 - Broken Wire Detection Enable"]
    #[inline(always)]
    pub fn bwden(&self) -> BWDEN_R {
        BWDEN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Input Class Select"]
    #[inline(always)]
    pub fn iclsel(&mut self) -> ICLSEL_W {
        ICLSEL_W { w: self }
    }
    #[doc = "Bits 4:5 - Lower Boundary Select"]
    #[inline(always)]
    pub fn bndsell(&mut self) -> BNDSELL_W {
        BNDSELL_W { w: self }
    }
    #[doc = "Bits 6:7 - Upper Boundary Select"]
    #[inline(always)]
    pub fn bndselu(&mut self) -> BNDSELU_W {
        BNDSELU_W { w: self }
    }
    #[doc = "Bits 8:9 - Channel Event Mode"]
    #[inline(always)]
    pub fn chevmode(&mut self) -> CHEVMODE_W {
        CHEVMODE_W { w: self }
    }
    #[doc = "Bit 10 - Synchronization Request"]
    #[inline(always)]
    pub fn sync(&mut self) -> SYNC_W {
        SYNC_W { w: self }
    }
    #[doc = "Bit 11 - Reference Input Selection"]
    #[inline(always)]
    pub fn refsel(&mut self) -> REFSEL_W {
        REFSEL_W { w: self }
    }
    #[doc = "Bits 16:19 - Result Register"]
    #[inline(always)]
    pub fn resreg(&mut self) -> RESREG_W {
        RESREG_W { w: self }
    }
    #[doc = "Bit 20 - Result Target for Background Source"]
    #[inline(always)]
    pub fn restbs(&mut self) -> RESTBS_W {
        RESTBS_W { w: self }
    }
    #[doc = "Bit 21 - Result Position"]
    #[inline(always)]
    pub fn respos(&mut self) -> RESPOS_W {
        RESPOS_W { w: self }
    }
    #[doc = "Bits 28:29 - Broken Wire Detection Channel"]
    #[inline(always)]
    pub fn bwdch(&mut self) -> BWDCH_W {
        BWDCH_W { w: self }
    }
    #[doc = "Bit 30 - Broken Wire Detection Enable"]
    #[inline(always)]
    pub fn bwden(&mut self) -> BWDEN_W {
        BWDEN_W { w: self }
    }
}
