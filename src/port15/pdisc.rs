#[doc = "Reader of register PDISC"]
pub type R = crate::R<u32, super::PDISC>;
#[doc = "Writer for register PDISC"]
pub type W = crate::W<u32, super::PDISC>;
#[doc = "Register PDISC `reset()`'s with value 0"]
impl crate::ResetValue for super::PDISC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pad Disable for Port n Pin 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS0_A {
    #[doc = "0: Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0 = 0,
    #[doc = "1: Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1 = 1,
}
impl From<PDIS0_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDIS0`"]
pub type PDIS0_R = crate::R<bool, PDIS0_A>;
impl PDIS0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDIS0_A {
        match self.bits {
            false => PDIS0_A::CONST_0,
            true => PDIS0_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS0_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS0_A::CONST_1
    }
}
#[doc = "Write proxy for field `PDIS0`"]
pub struct PDIS0_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIS0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDIS0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PDIS0_A::CONST_0)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PDIS0_A::CONST_1)
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
#[doc = "Pad Disable for Port n Pin 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS1_A {
    #[doc = "0: Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0 = 0,
    #[doc = "1: Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1 = 1,
}
impl From<PDIS1_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDIS1`"]
pub type PDIS1_R = crate::R<bool, PDIS1_A>;
impl PDIS1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDIS1_A {
        match self.bits {
            false => PDIS1_A::CONST_0,
            true => PDIS1_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS1_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS1_A::CONST_1
    }
}
#[doc = "Write proxy for field `PDIS1`"]
pub struct PDIS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDIS1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PDIS1_A::CONST_0)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PDIS1_A::CONST_1)
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
#[doc = "Pad Disable for Port n Pin 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS2_A {
    #[doc = "0: Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0 = 0,
    #[doc = "1: Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1 = 1,
}
impl From<PDIS2_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDIS2`"]
pub type PDIS2_R = crate::R<bool, PDIS2_A>;
impl PDIS2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDIS2_A {
        match self.bits {
            false => PDIS2_A::CONST_0,
            true => PDIS2_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS2_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS2_A::CONST_1
    }
}
#[doc = "Write proxy for field `PDIS2`"]
pub struct PDIS2_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIS2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDIS2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PDIS2_A::CONST_0)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PDIS2_A::CONST_1)
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
#[doc = "Pad Disable for Port n Pin 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS3_A {
    #[doc = "0: Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0 = 0,
    #[doc = "1: Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1 = 1,
}
impl From<PDIS3_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDIS3`"]
pub type PDIS3_R = crate::R<bool, PDIS3_A>;
impl PDIS3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDIS3_A {
        match self.bits {
            false => PDIS3_A::CONST_0,
            true => PDIS3_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS3_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS3_A::CONST_1
    }
}
#[doc = "Write proxy for field `PDIS3`"]
pub struct PDIS3_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIS3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDIS3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PDIS3_A::CONST_0)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PDIS3_A::CONST_1)
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
#[doc = "Pad Disable for Port n Pin 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS4_A {
    #[doc = "0: Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0 = 0,
    #[doc = "1: Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1 = 1,
}
impl From<PDIS4_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDIS4`"]
pub type PDIS4_R = crate::R<bool, PDIS4_A>;
impl PDIS4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDIS4_A {
        match self.bits {
            false => PDIS4_A::CONST_0,
            true => PDIS4_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS4_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS4_A::CONST_1
    }
}
#[doc = "Write proxy for field `PDIS4`"]
pub struct PDIS4_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIS4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDIS4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PDIS4_A::CONST_0)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PDIS4_A::CONST_1)
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
#[doc = "Pad Disable for Port n Pin 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS5_A {
    #[doc = "0: Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0 = 0,
    #[doc = "1: Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1 = 1,
}
impl From<PDIS5_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDIS5`"]
pub type PDIS5_R = crate::R<bool, PDIS5_A>;
impl PDIS5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDIS5_A {
        match self.bits {
            false => PDIS5_A::CONST_0,
            true => PDIS5_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS5_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS5_A::CONST_1
    }
}
#[doc = "Write proxy for field `PDIS5`"]
pub struct PDIS5_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIS5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDIS5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PDIS5_A::CONST_0)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PDIS5_A::CONST_1)
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
#[doc = "Pad Disable for Port n Pin 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS6_A {
    #[doc = "0: Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0 = 0,
    #[doc = "1: Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1 = 1,
}
impl From<PDIS6_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDIS6`"]
pub type PDIS6_R = crate::R<bool, PDIS6_A>;
impl PDIS6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDIS6_A {
        match self.bits {
            false => PDIS6_A::CONST_0,
            true => PDIS6_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS6_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS6_A::CONST_1
    }
}
#[doc = "Write proxy for field `PDIS6`"]
pub struct PDIS6_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIS6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDIS6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PDIS6_A::CONST_0)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PDIS6_A::CONST_1)
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
#[doc = "Pad Disable for Port n Pin 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS7_A {
    #[doc = "0: Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0 = 0,
    #[doc = "1: Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1 = 1,
}
impl From<PDIS7_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDIS7`"]
pub type PDIS7_R = crate::R<bool, PDIS7_A>;
impl PDIS7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDIS7_A {
        match self.bits {
            false => PDIS7_A::CONST_0,
            true => PDIS7_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS7_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS7_A::CONST_1
    }
}
#[doc = "Write proxy for field `PDIS7`"]
pub struct PDIS7_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIS7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDIS7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PDIS7_A::CONST_0)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PDIS7_A::CONST_1)
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
#[doc = "Pad Disable for Port n Pin 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS8_A {
    #[doc = "0: Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0 = 0,
    #[doc = "1: Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1 = 1,
}
impl From<PDIS8_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDIS8`"]
pub type PDIS8_R = crate::R<bool, PDIS8_A>;
impl PDIS8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDIS8_A {
        match self.bits {
            false => PDIS8_A::CONST_0,
            true => PDIS8_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS8_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS8_A::CONST_1
    }
}
#[doc = "Write proxy for field `PDIS8`"]
pub struct PDIS8_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIS8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDIS8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PDIS8_A::CONST_0)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PDIS8_A::CONST_1)
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
#[doc = "Pad Disable for Port n Pin 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS9_A {
    #[doc = "0: Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0 = 0,
    #[doc = "1: Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1 = 1,
}
impl From<PDIS9_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDIS9`"]
pub type PDIS9_R = crate::R<bool, PDIS9_A>;
impl PDIS9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDIS9_A {
        match self.bits {
            false => PDIS9_A::CONST_0,
            true => PDIS9_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS9_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS9_A::CONST_1
    }
}
#[doc = "Write proxy for field `PDIS9`"]
pub struct PDIS9_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIS9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDIS9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PDIS9_A::CONST_0)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PDIS9_A::CONST_1)
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
#[doc = "Pad Disable for Port n Pin 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS10_A {
    #[doc = "0: Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0 = 0,
    #[doc = "1: Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1 = 1,
}
impl From<PDIS10_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDIS10`"]
pub type PDIS10_R = crate::R<bool, PDIS10_A>;
impl PDIS10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDIS10_A {
        match self.bits {
            false => PDIS10_A::CONST_0,
            true => PDIS10_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS10_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS10_A::CONST_1
    }
}
#[doc = "Write proxy for field `PDIS10`"]
pub struct PDIS10_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIS10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDIS10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PDIS10_A::CONST_0)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PDIS10_A::CONST_1)
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
#[doc = "Pad Disable for Port n Pin 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS11_A {
    #[doc = "0: Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0 = 0,
    #[doc = "1: Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1 = 1,
}
impl From<PDIS11_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDIS11`"]
pub type PDIS11_R = crate::R<bool, PDIS11_A>;
impl PDIS11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDIS11_A {
        match self.bits {
            false => PDIS11_A::CONST_0,
            true => PDIS11_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS11_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS11_A::CONST_1
    }
}
#[doc = "Write proxy for field `PDIS11`"]
pub struct PDIS11_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIS11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDIS11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PDIS11_A::CONST_0)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PDIS11_A::CONST_1)
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
#[doc = "Pad Disable for Port n Pin 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS12_A {
    #[doc = "0: Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0 = 0,
    #[doc = "1: Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1 = 1,
}
impl From<PDIS12_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDIS12`"]
pub type PDIS12_R = crate::R<bool, PDIS12_A>;
impl PDIS12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDIS12_A {
        match self.bits {
            false => PDIS12_A::CONST_0,
            true => PDIS12_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS12_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS12_A::CONST_1
    }
}
#[doc = "Write proxy for field `PDIS12`"]
pub struct PDIS12_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIS12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDIS12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PDIS12_A::CONST_0)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PDIS12_A::CONST_1)
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
#[doc = "Pad Disable for Port n Pin 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS13_A {
    #[doc = "0: Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0 = 0,
    #[doc = "1: Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1 = 1,
}
impl From<PDIS13_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDIS13`"]
pub type PDIS13_R = crate::R<bool, PDIS13_A>;
impl PDIS13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDIS13_A {
        match self.bits {
            false => PDIS13_A::CONST_0,
            true => PDIS13_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS13_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS13_A::CONST_1
    }
}
#[doc = "Write proxy for field `PDIS13`"]
pub struct PDIS13_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIS13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDIS13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PDIS13_A::CONST_0)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PDIS13_A::CONST_1)
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
#[doc = "Pad Disable for Port n Pin 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS14_A {
    #[doc = "0: Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0 = 0,
    #[doc = "1: Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1 = 1,
}
impl From<PDIS14_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDIS14`"]
pub type PDIS14_R = crate::R<bool, PDIS14_A>;
impl PDIS14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDIS14_A {
        match self.bits {
            false => PDIS14_A::CONST_0,
            true => PDIS14_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS14_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS14_A::CONST_1
    }
}
#[doc = "Write proxy for field `PDIS14`"]
pub struct PDIS14_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIS14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDIS14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PDIS14_A::CONST_0)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PDIS14_A::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Pad Disable for Port n Pin 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS15_A {
    #[doc = "0: Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0 = 0,
    #[doc = "1: Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1 = 1,
}
impl From<PDIS15_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDIS15`"]
pub type PDIS15_R = crate::R<bool, PDIS15_A>;
impl PDIS15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDIS15_A {
        match self.bits {
            false => PDIS15_A::CONST_0,
            true => PDIS15_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS15_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS15_A::CONST_1
    }
}
#[doc = "Write proxy for field `PDIS15`"]
pub struct PDIS15_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIS15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDIS15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PDIS15_A::CONST_0)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PDIS15_A::CONST_1)
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
impl R {
    #[doc = "Bit 0 - Pad Disable for Port n Pin 0"]
    #[inline(always)]
    pub fn pdis0(&self) -> PDIS0_R {
        PDIS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pad Disable for Port n Pin 1"]
    #[inline(always)]
    pub fn pdis1(&self) -> PDIS1_R {
        PDIS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Pad Disable for Port n Pin 2"]
    #[inline(always)]
    pub fn pdis2(&self) -> PDIS2_R {
        PDIS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Pad Disable for Port n Pin 3"]
    #[inline(always)]
    pub fn pdis3(&self) -> PDIS3_R {
        PDIS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pad Disable for Port n Pin 4"]
    #[inline(always)]
    pub fn pdis4(&self) -> PDIS4_R {
        PDIS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Pad Disable for Port n Pin 5"]
    #[inline(always)]
    pub fn pdis5(&self) -> PDIS5_R {
        PDIS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Pad Disable for Port n Pin 6"]
    #[inline(always)]
    pub fn pdis6(&self) -> PDIS6_R {
        PDIS6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Pad Disable for Port n Pin 7"]
    #[inline(always)]
    pub fn pdis7(&self) -> PDIS7_R {
        PDIS7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad Disable for Port n Pin 8"]
    #[inline(always)]
    pub fn pdis8(&self) -> PDIS8_R {
        PDIS8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pad Disable for Port n Pin 9"]
    #[inline(always)]
    pub fn pdis9(&self) -> PDIS9_R {
        PDIS9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Pad Disable for Port n Pin 10"]
    #[inline(always)]
    pub fn pdis10(&self) -> PDIS10_R {
        PDIS10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Pad Disable for Port n Pin 11"]
    #[inline(always)]
    pub fn pdis11(&self) -> PDIS11_R {
        PDIS11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pad Disable for Port n Pin 12"]
    #[inline(always)]
    pub fn pdis12(&self) -> PDIS12_R {
        PDIS12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Pad Disable for Port n Pin 13"]
    #[inline(always)]
    pub fn pdis13(&self) -> PDIS13_R {
        PDIS13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Pad Disable for Port n Pin 14"]
    #[inline(always)]
    pub fn pdis14(&self) -> PDIS14_R {
        PDIS14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Pad Disable for Port n Pin 15"]
    #[inline(always)]
    pub fn pdis15(&self) -> PDIS15_R {
        PDIS15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pad Disable for Port n Pin 0"]
    #[inline(always)]
    pub fn pdis0(&mut self) -> PDIS0_W {
        PDIS0_W { w: self }
    }
    #[doc = "Bit 1 - Pad Disable for Port n Pin 1"]
    #[inline(always)]
    pub fn pdis1(&mut self) -> PDIS1_W {
        PDIS1_W { w: self }
    }
    #[doc = "Bit 2 - Pad Disable for Port n Pin 2"]
    #[inline(always)]
    pub fn pdis2(&mut self) -> PDIS2_W {
        PDIS2_W { w: self }
    }
    #[doc = "Bit 3 - Pad Disable for Port n Pin 3"]
    #[inline(always)]
    pub fn pdis3(&mut self) -> PDIS3_W {
        PDIS3_W { w: self }
    }
    #[doc = "Bit 4 - Pad Disable for Port n Pin 4"]
    #[inline(always)]
    pub fn pdis4(&mut self) -> PDIS4_W {
        PDIS4_W { w: self }
    }
    #[doc = "Bit 5 - Pad Disable for Port n Pin 5"]
    #[inline(always)]
    pub fn pdis5(&mut self) -> PDIS5_W {
        PDIS5_W { w: self }
    }
    #[doc = "Bit 6 - Pad Disable for Port n Pin 6"]
    #[inline(always)]
    pub fn pdis6(&mut self) -> PDIS6_W {
        PDIS6_W { w: self }
    }
    #[doc = "Bit 7 - Pad Disable for Port n Pin 7"]
    #[inline(always)]
    pub fn pdis7(&mut self) -> PDIS7_W {
        PDIS7_W { w: self }
    }
    #[doc = "Bit 8 - Pad Disable for Port n Pin 8"]
    #[inline(always)]
    pub fn pdis8(&mut self) -> PDIS8_W {
        PDIS8_W { w: self }
    }
    #[doc = "Bit 9 - Pad Disable for Port n Pin 9"]
    #[inline(always)]
    pub fn pdis9(&mut self) -> PDIS9_W {
        PDIS9_W { w: self }
    }
    #[doc = "Bit 10 - Pad Disable for Port n Pin 10"]
    #[inline(always)]
    pub fn pdis10(&mut self) -> PDIS10_W {
        PDIS10_W { w: self }
    }
    #[doc = "Bit 11 - Pad Disable for Port n Pin 11"]
    #[inline(always)]
    pub fn pdis11(&mut self) -> PDIS11_W {
        PDIS11_W { w: self }
    }
    #[doc = "Bit 12 - Pad Disable for Port n Pin 12"]
    #[inline(always)]
    pub fn pdis12(&mut self) -> PDIS12_W {
        PDIS12_W { w: self }
    }
    #[doc = "Bit 13 - Pad Disable for Port n Pin 13"]
    #[inline(always)]
    pub fn pdis13(&mut self) -> PDIS13_W {
        PDIS13_W { w: self }
    }
    #[doc = "Bit 14 - Pad Disable for Port n Pin 14"]
    #[inline(always)]
    pub fn pdis14(&mut self) -> PDIS14_W {
        PDIS14_W { w: self }
    }
    #[doc = "Bit 15 - Pad Disable for Port n Pin 15"]
    #[inline(always)]
    pub fn pdis15(&mut self) -> PDIS15_W {
        PDIS15_W { w: self }
    }
}
