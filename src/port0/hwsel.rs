#[doc = "Reader of register HWSEL"]
pub type R = crate::R<u32, super::HWSEL>;
#[doc = "Writer for register HWSEL"]
pub type W = crate::W<u32, super::HWSEL>;
#[doc = "Register HWSEL `reset()`'s with value 0x0001_4000"]
impl crate::ResetValue for super::HWSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0001_4000
    }
}
#[doc = "Port n Pin Hardware Select Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW0_A {
    #[doc = "0: Software control only."]
    CONST_00,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
}
impl From<HW0_A> for u8 {
    #[inline(always)]
    fn from(variant: HW0_A) -> Self {
        match variant {
            HW0_A::CONST_00 => 0,
            HW0_A::CONST_01 => 1,
            HW0_A::CONST_10 => 2,
        }
    }
}
#[doc = "Reader of field `HW0`"]
pub type HW0_R = crate::R<u8, HW0_A>;
impl HW0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HW0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HW0_A::CONST_00),
            1 => Val(HW0_A::CONST_01),
            2 => Val(HW0_A::CONST_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == HW0_A::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == HW0_A::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == HW0_A::CONST_10
    }
}
#[doc = "Write proxy for field `HW0`"]
pub struct HW0_W<'a> {
    w: &'a mut W,
}
impl<'a> HW0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HW0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut W {
        self.variant(HW0_A::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut W {
        self.variant(HW0_A::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut W {
        self.variant(HW0_A::CONST_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Port n Pin Hardware Select Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW1_A {
    #[doc = "0: Software control only."]
    CONST_00,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
}
impl From<HW1_A> for u8 {
    #[inline(always)]
    fn from(variant: HW1_A) -> Self {
        match variant {
            HW1_A::CONST_00 => 0,
            HW1_A::CONST_01 => 1,
            HW1_A::CONST_10 => 2,
        }
    }
}
#[doc = "Reader of field `HW1`"]
pub type HW1_R = crate::R<u8, HW1_A>;
impl HW1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HW1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HW1_A::CONST_00),
            1 => Val(HW1_A::CONST_01),
            2 => Val(HW1_A::CONST_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == HW1_A::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == HW1_A::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == HW1_A::CONST_10
    }
}
#[doc = "Write proxy for field `HW1`"]
pub struct HW1_W<'a> {
    w: &'a mut W,
}
impl<'a> HW1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HW1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut W {
        self.variant(HW1_A::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut W {
        self.variant(HW1_A::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut W {
        self.variant(HW1_A::CONST_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Port n Pin Hardware Select Bit 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW2_A {
    #[doc = "0: Software control only."]
    CONST_00,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
}
impl From<HW2_A> for u8 {
    #[inline(always)]
    fn from(variant: HW2_A) -> Self {
        match variant {
            HW2_A::CONST_00 => 0,
            HW2_A::CONST_01 => 1,
            HW2_A::CONST_10 => 2,
        }
    }
}
#[doc = "Reader of field `HW2`"]
pub type HW2_R = crate::R<u8, HW2_A>;
impl HW2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HW2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HW2_A::CONST_00),
            1 => Val(HW2_A::CONST_01),
            2 => Val(HW2_A::CONST_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == HW2_A::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == HW2_A::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == HW2_A::CONST_10
    }
}
#[doc = "Write proxy for field `HW2`"]
pub struct HW2_W<'a> {
    w: &'a mut W,
}
impl<'a> HW2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HW2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut W {
        self.variant(HW2_A::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut W {
        self.variant(HW2_A::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut W {
        self.variant(HW2_A::CONST_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Port n Pin Hardware Select Bit 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW3_A {
    #[doc = "0: Software control only."]
    CONST_00,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
}
impl From<HW3_A> for u8 {
    #[inline(always)]
    fn from(variant: HW3_A) -> Self {
        match variant {
            HW3_A::CONST_00 => 0,
            HW3_A::CONST_01 => 1,
            HW3_A::CONST_10 => 2,
        }
    }
}
#[doc = "Reader of field `HW3`"]
pub type HW3_R = crate::R<u8, HW3_A>;
impl HW3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HW3_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HW3_A::CONST_00),
            1 => Val(HW3_A::CONST_01),
            2 => Val(HW3_A::CONST_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == HW3_A::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == HW3_A::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == HW3_A::CONST_10
    }
}
#[doc = "Write proxy for field `HW3`"]
pub struct HW3_W<'a> {
    w: &'a mut W,
}
impl<'a> HW3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HW3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut W {
        self.variant(HW3_A::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut W {
        self.variant(HW3_A::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut W {
        self.variant(HW3_A::CONST_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Port n Pin Hardware Select Bit 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW4_A {
    #[doc = "0: Software control only."]
    CONST_00,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
}
impl From<HW4_A> for u8 {
    #[inline(always)]
    fn from(variant: HW4_A) -> Self {
        match variant {
            HW4_A::CONST_00 => 0,
            HW4_A::CONST_01 => 1,
            HW4_A::CONST_10 => 2,
        }
    }
}
#[doc = "Reader of field `HW4`"]
pub type HW4_R = crate::R<u8, HW4_A>;
impl HW4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HW4_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HW4_A::CONST_00),
            1 => Val(HW4_A::CONST_01),
            2 => Val(HW4_A::CONST_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == HW4_A::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == HW4_A::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == HW4_A::CONST_10
    }
}
#[doc = "Write proxy for field `HW4`"]
pub struct HW4_W<'a> {
    w: &'a mut W,
}
impl<'a> HW4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HW4_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut W {
        self.variant(HW4_A::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut W {
        self.variant(HW4_A::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut W {
        self.variant(HW4_A::CONST_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Port n Pin Hardware Select Bit 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW5_A {
    #[doc = "0: Software control only."]
    CONST_00,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
}
impl From<HW5_A> for u8 {
    #[inline(always)]
    fn from(variant: HW5_A) -> Self {
        match variant {
            HW5_A::CONST_00 => 0,
            HW5_A::CONST_01 => 1,
            HW5_A::CONST_10 => 2,
        }
    }
}
#[doc = "Reader of field `HW5`"]
pub type HW5_R = crate::R<u8, HW5_A>;
impl HW5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HW5_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HW5_A::CONST_00),
            1 => Val(HW5_A::CONST_01),
            2 => Val(HW5_A::CONST_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == HW5_A::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == HW5_A::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == HW5_A::CONST_10
    }
}
#[doc = "Write proxy for field `HW5`"]
pub struct HW5_W<'a> {
    w: &'a mut W,
}
impl<'a> HW5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HW5_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut W {
        self.variant(HW5_A::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut W {
        self.variant(HW5_A::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut W {
        self.variant(HW5_A::CONST_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Port n Pin Hardware Select Bit 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW6_A {
    #[doc = "0: Software control only."]
    CONST_00,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
}
impl From<HW6_A> for u8 {
    #[inline(always)]
    fn from(variant: HW6_A) -> Self {
        match variant {
            HW6_A::CONST_00 => 0,
            HW6_A::CONST_01 => 1,
            HW6_A::CONST_10 => 2,
        }
    }
}
#[doc = "Reader of field `HW6`"]
pub type HW6_R = crate::R<u8, HW6_A>;
impl HW6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HW6_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HW6_A::CONST_00),
            1 => Val(HW6_A::CONST_01),
            2 => Val(HW6_A::CONST_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == HW6_A::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == HW6_A::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == HW6_A::CONST_10
    }
}
#[doc = "Write proxy for field `HW6`"]
pub struct HW6_W<'a> {
    w: &'a mut W,
}
impl<'a> HW6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HW6_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut W {
        self.variant(HW6_A::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut W {
        self.variant(HW6_A::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut W {
        self.variant(HW6_A::CONST_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Port n Pin Hardware Select Bit 7\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW7_A {
    #[doc = "0: Software control only."]
    CONST_00,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
}
impl From<HW7_A> for u8 {
    #[inline(always)]
    fn from(variant: HW7_A) -> Self {
        match variant {
            HW7_A::CONST_00 => 0,
            HW7_A::CONST_01 => 1,
            HW7_A::CONST_10 => 2,
        }
    }
}
#[doc = "Reader of field `HW7`"]
pub type HW7_R = crate::R<u8, HW7_A>;
impl HW7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HW7_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HW7_A::CONST_00),
            1 => Val(HW7_A::CONST_01),
            2 => Val(HW7_A::CONST_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == HW7_A::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == HW7_A::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == HW7_A::CONST_10
    }
}
#[doc = "Write proxy for field `HW7`"]
pub struct HW7_W<'a> {
    w: &'a mut W,
}
impl<'a> HW7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HW7_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut W {
        self.variant(HW7_A::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut W {
        self.variant(HW7_A::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut W {
        self.variant(HW7_A::CONST_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Port n Pin Hardware Select Bit 8\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW8_A {
    #[doc = "0: Software control only."]
    CONST_00,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
}
impl From<HW8_A> for u8 {
    #[inline(always)]
    fn from(variant: HW8_A) -> Self {
        match variant {
            HW8_A::CONST_00 => 0,
            HW8_A::CONST_01 => 1,
            HW8_A::CONST_10 => 2,
        }
    }
}
#[doc = "Reader of field `HW8`"]
pub type HW8_R = crate::R<u8, HW8_A>;
impl HW8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HW8_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HW8_A::CONST_00),
            1 => Val(HW8_A::CONST_01),
            2 => Val(HW8_A::CONST_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == HW8_A::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == HW8_A::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == HW8_A::CONST_10
    }
}
#[doc = "Write proxy for field `HW8`"]
pub struct HW8_W<'a> {
    w: &'a mut W,
}
impl<'a> HW8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HW8_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut W {
        self.variant(HW8_A::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut W {
        self.variant(HW8_A::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut W {
        self.variant(HW8_A::CONST_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Port n Pin Hardware Select Bit 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW9_A {
    #[doc = "0: Software control only."]
    CONST_00,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
}
impl From<HW9_A> for u8 {
    #[inline(always)]
    fn from(variant: HW9_A) -> Self {
        match variant {
            HW9_A::CONST_00 => 0,
            HW9_A::CONST_01 => 1,
            HW9_A::CONST_10 => 2,
        }
    }
}
#[doc = "Reader of field `HW9`"]
pub type HW9_R = crate::R<u8, HW9_A>;
impl HW9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HW9_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HW9_A::CONST_00),
            1 => Val(HW9_A::CONST_01),
            2 => Val(HW9_A::CONST_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == HW9_A::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == HW9_A::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == HW9_A::CONST_10
    }
}
#[doc = "Write proxy for field `HW9`"]
pub struct HW9_W<'a> {
    w: &'a mut W,
}
impl<'a> HW9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HW9_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut W {
        self.variant(HW9_A::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut W {
        self.variant(HW9_A::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut W {
        self.variant(HW9_A::CONST_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Port n Pin Hardware Select Bit 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW10_A {
    #[doc = "0: Software control only."]
    CONST_00,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
}
impl From<HW10_A> for u8 {
    #[inline(always)]
    fn from(variant: HW10_A) -> Self {
        match variant {
            HW10_A::CONST_00 => 0,
            HW10_A::CONST_01 => 1,
            HW10_A::CONST_10 => 2,
        }
    }
}
#[doc = "Reader of field `HW10`"]
pub type HW10_R = crate::R<u8, HW10_A>;
impl HW10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HW10_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HW10_A::CONST_00),
            1 => Val(HW10_A::CONST_01),
            2 => Val(HW10_A::CONST_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == HW10_A::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == HW10_A::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == HW10_A::CONST_10
    }
}
#[doc = "Write proxy for field `HW10`"]
pub struct HW10_W<'a> {
    w: &'a mut W,
}
impl<'a> HW10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HW10_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut W {
        self.variant(HW10_A::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut W {
        self.variant(HW10_A::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut W {
        self.variant(HW10_A::CONST_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Port n Pin Hardware Select Bit 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW11_A {
    #[doc = "0: Software control only."]
    CONST_00,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
}
impl From<HW11_A> for u8 {
    #[inline(always)]
    fn from(variant: HW11_A) -> Self {
        match variant {
            HW11_A::CONST_00 => 0,
            HW11_A::CONST_01 => 1,
            HW11_A::CONST_10 => 2,
        }
    }
}
#[doc = "Reader of field `HW11`"]
pub type HW11_R = crate::R<u8, HW11_A>;
impl HW11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HW11_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HW11_A::CONST_00),
            1 => Val(HW11_A::CONST_01),
            2 => Val(HW11_A::CONST_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == HW11_A::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == HW11_A::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == HW11_A::CONST_10
    }
}
#[doc = "Write proxy for field `HW11`"]
pub struct HW11_W<'a> {
    w: &'a mut W,
}
impl<'a> HW11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HW11_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut W {
        self.variant(HW11_A::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut W {
        self.variant(HW11_A::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut W {
        self.variant(HW11_A::CONST_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Port n Pin Hardware Select Bit 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW12_A {
    #[doc = "0: Software control only."]
    CONST_00,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
}
impl From<HW12_A> for u8 {
    #[inline(always)]
    fn from(variant: HW12_A) -> Self {
        match variant {
            HW12_A::CONST_00 => 0,
            HW12_A::CONST_01 => 1,
            HW12_A::CONST_10 => 2,
        }
    }
}
#[doc = "Reader of field `HW12`"]
pub type HW12_R = crate::R<u8, HW12_A>;
impl HW12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HW12_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HW12_A::CONST_00),
            1 => Val(HW12_A::CONST_01),
            2 => Val(HW12_A::CONST_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == HW12_A::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == HW12_A::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == HW12_A::CONST_10
    }
}
#[doc = "Write proxy for field `HW12`"]
pub struct HW12_W<'a> {
    w: &'a mut W,
}
impl<'a> HW12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HW12_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut W {
        self.variant(HW12_A::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut W {
        self.variant(HW12_A::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut W {
        self.variant(HW12_A::CONST_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Port n Pin Hardware Select Bit 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW13_A {
    #[doc = "0: Software control only."]
    CONST_00,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
}
impl From<HW13_A> for u8 {
    #[inline(always)]
    fn from(variant: HW13_A) -> Self {
        match variant {
            HW13_A::CONST_00 => 0,
            HW13_A::CONST_01 => 1,
            HW13_A::CONST_10 => 2,
        }
    }
}
#[doc = "Reader of field `HW13`"]
pub type HW13_R = crate::R<u8, HW13_A>;
impl HW13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HW13_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HW13_A::CONST_00),
            1 => Val(HW13_A::CONST_01),
            2 => Val(HW13_A::CONST_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == HW13_A::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == HW13_A::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == HW13_A::CONST_10
    }
}
#[doc = "Write proxy for field `HW13`"]
pub struct HW13_W<'a> {
    w: &'a mut W,
}
impl<'a> HW13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HW13_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut W {
        self.variant(HW13_A::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut W {
        self.variant(HW13_A::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut W {
        self.variant(HW13_A::CONST_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Port n Pin Hardware Select Bit 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW14_A {
    #[doc = "0: Software control only."]
    CONST_00,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
}
impl From<HW14_A> for u8 {
    #[inline(always)]
    fn from(variant: HW14_A) -> Self {
        match variant {
            HW14_A::CONST_00 => 0,
            HW14_A::CONST_01 => 1,
            HW14_A::CONST_10 => 2,
        }
    }
}
#[doc = "Reader of field `HW14`"]
pub type HW14_R = crate::R<u8, HW14_A>;
impl HW14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HW14_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HW14_A::CONST_00),
            1 => Val(HW14_A::CONST_01),
            2 => Val(HW14_A::CONST_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == HW14_A::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == HW14_A::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == HW14_A::CONST_10
    }
}
#[doc = "Write proxy for field `HW14`"]
pub struct HW14_W<'a> {
    w: &'a mut W,
}
impl<'a> HW14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HW14_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut W {
        self.variant(HW14_A::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut W {
        self.variant(HW14_A::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut W {
        self.variant(HW14_A::CONST_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Port n Pin Hardware Select Bit 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW15_A {
    #[doc = "0: Software control only."]
    CONST_00,
    #[doc = "1: HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "2: HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
}
impl From<HW15_A> for u8 {
    #[inline(always)]
    fn from(variant: HW15_A) -> Self {
        match variant {
            HW15_A::CONST_00 => 0,
            HW15_A::CONST_01 => 1,
            HW15_A::CONST_10 => 2,
        }
    }
}
#[doc = "Reader of field `HW15`"]
pub type HW15_R = crate::R<u8, HW15_A>;
impl HW15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HW15_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HW15_A::CONST_00),
            1 => Val(HW15_A::CONST_01),
            2 => Val(HW15_A::CONST_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == HW15_A::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == HW15_A::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == HW15_A::CONST_10
    }
}
#[doc = "Write proxy for field `HW15`"]
pub struct HW15_W<'a> {
    w: &'a mut W,
}
impl<'a> HW15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HW15_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut W {
        self.variant(HW15_A::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut W {
        self.variant(HW15_A::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut W {
        self.variant(HW15_A::CONST_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Port n Pin Hardware Select Bit 0"]
    #[inline(always)]
    pub fn hw0(&self) -> HW0_R {
        HW0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Port n Pin Hardware Select Bit 1"]
    #[inline(always)]
    pub fn hw1(&self) -> HW1_R {
        HW1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Port n Pin Hardware Select Bit 2"]
    #[inline(always)]
    pub fn hw2(&self) -> HW2_R {
        HW2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Port n Pin Hardware Select Bit 3"]
    #[inline(always)]
    pub fn hw3(&self) -> HW3_R {
        HW3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Port n Pin Hardware Select Bit 4"]
    #[inline(always)]
    pub fn hw4(&self) -> HW4_R {
        HW4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Port n Pin Hardware Select Bit 5"]
    #[inline(always)]
    pub fn hw5(&self) -> HW5_R {
        HW5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Port n Pin Hardware Select Bit 6"]
    #[inline(always)]
    pub fn hw6(&self) -> HW6_R {
        HW6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Port n Pin Hardware Select Bit 7"]
    #[inline(always)]
    pub fn hw7(&self) -> HW7_R {
        HW7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Port n Pin Hardware Select Bit 8"]
    #[inline(always)]
    pub fn hw8(&self) -> HW8_R {
        HW8_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Port n Pin Hardware Select Bit 9"]
    #[inline(always)]
    pub fn hw9(&self) -> HW9_R {
        HW9_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Port n Pin Hardware Select Bit 10"]
    #[inline(always)]
    pub fn hw10(&self) -> HW10_R {
        HW10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Port n Pin Hardware Select Bit 11"]
    #[inline(always)]
    pub fn hw11(&self) -> HW11_R {
        HW11_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Port n Pin Hardware Select Bit 12"]
    #[inline(always)]
    pub fn hw12(&self) -> HW12_R {
        HW12_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Port n Pin Hardware Select Bit 13"]
    #[inline(always)]
    pub fn hw13(&self) -> HW13_R {
        HW13_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Port n Pin Hardware Select Bit 14"]
    #[inline(always)]
    pub fn hw14(&self) -> HW14_R {
        HW14_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Port n Pin Hardware Select Bit 15"]
    #[inline(always)]
    pub fn hw15(&self) -> HW15_R {
        HW15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port n Pin Hardware Select Bit 0"]
    #[inline(always)]
    pub fn hw0(&mut self) -> HW0_W {
        HW0_W { w: self }
    }
    #[doc = "Bits 2:3 - Port n Pin Hardware Select Bit 1"]
    #[inline(always)]
    pub fn hw1(&mut self) -> HW1_W {
        HW1_W { w: self }
    }
    #[doc = "Bits 4:5 - Port n Pin Hardware Select Bit 2"]
    #[inline(always)]
    pub fn hw2(&mut self) -> HW2_W {
        HW2_W { w: self }
    }
    #[doc = "Bits 6:7 - Port n Pin Hardware Select Bit 3"]
    #[inline(always)]
    pub fn hw3(&mut self) -> HW3_W {
        HW3_W { w: self }
    }
    #[doc = "Bits 8:9 - Port n Pin Hardware Select Bit 4"]
    #[inline(always)]
    pub fn hw4(&mut self) -> HW4_W {
        HW4_W { w: self }
    }
    #[doc = "Bits 10:11 - Port n Pin Hardware Select Bit 5"]
    #[inline(always)]
    pub fn hw5(&mut self) -> HW5_W {
        HW5_W { w: self }
    }
    #[doc = "Bits 12:13 - Port n Pin Hardware Select Bit 6"]
    #[inline(always)]
    pub fn hw6(&mut self) -> HW6_W {
        HW6_W { w: self }
    }
    #[doc = "Bits 14:15 - Port n Pin Hardware Select Bit 7"]
    #[inline(always)]
    pub fn hw7(&mut self) -> HW7_W {
        HW7_W { w: self }
    }
    #[doc = "Bits 16:17 - Port n Pin Hardware Select Bit 8"]
    #[inline(always)]
    pub fn hw8(&mut self) -> HW8_W {
        HW8_W { w: self }
    }
    #[doc = "Bits 18:19 - Port n Pin Hardware Select Bit 9"]
    #[inline(always)]
    pub fn hw9(&mut self) -> HW9_W {
        HW9_W { w: self }
    }
    #[doc = "Bits 20:21 - Port n Pin Hardware Select Bit 10"]
    #[inline(always)]
    pub fn hw10(&mut self) -> HW10_W {
        HW10_W { w: self }
    }
    #[doc = "Bits 22:23 - Port n Pin Hardware Select Bit 11"]
    #[inline(always)]
    pub fn hw11(&mut self) -> HW11_W {
        HW11_W { w: self }
    }
    #[doc = "Bits 24:25 - Port n Pin Hardware Select Bit 12"]
    #[inline(always)]
    pub fn hw12(&mut self) -> HW12_W {
        HW12_W { w: self }
    }
    #[doc = "Bits 26:27 - Port n Pin Hardware Select Bit 13"]
    #[inline(always)]
    pub fn hw13(&mut self) -> HW13_W {
        HW13_W { w: self }
    }
    #[doc = "Bits 28:29 - Port n Pin Hardware Select Bit 14"]
    #[inline(always)]
    pub fn hw14(&mut self) -> HW14_W {
        HW14_W { w: self }
    }
    #[doc = "Bits 30:31 - Port n Pin Hardware Select Bit 15"]
    #[inline(always)]
    pub fn hw15(&mut self) -> HW15_W {
        HW15_W { w: self }
    }
}
