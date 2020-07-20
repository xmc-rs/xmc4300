#[doc = "Reader of register INS"]
pub type R = crate::R<u32, super::INS>;
#[doc = "Writer for register INS"]
pub type W = crate::W<u32, super::INS>;
#[doc = "Register INS `reset()`'s with value 0"]
impl crate::ResetValue for super::INS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Event 0 signal selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EV0IS_A {
    #[doc = "0: CCU4x.INyA"]
    VALUE1 = 0,
    #[doc = "1: CCU4x.INyB"]
    VALUE2 = 1,
    #[doc = "2: CCU4x.INyC"]
    VALUE3 = 2,
    #[doc = "3: CCU4x.INyD"]
    VALUE4 = 3,
    #[doc = "4: CCU4x.INyE"]
    VALUE5 = 4,
    #[doc = "5: CCU4x.INyF"]
    VALUE6 = 5,
    #[doc = "6: CCU4x.INyG"]
    VALUE7 = 6,
    #[doc = "7: CCU4x.INyH"]
    VALUE8 = 7,
    #[doc = "8: CCU4x.INyI"]
    VALUE9 = 8,
    #[doc = "9: CCU4x.INyJ"]
    VALUE10 = 9,
    #[doc = "10: CCU4x.INyK"]
    VALUE11 = 10,
    #[doc = "11: CCU4x.INyL"]
    VALUE12 = 11,
    #[doc = "12: CCU4x.INyM"]
    VALUE13 = 12,
    #[doc = "13: CCU4x.INyN"]
    VALUE14 = 13,
    #[doc = "14: CCU4x.INyO"]
    VALUE15 = 14,
    #[doc = "15: CCU4x.INyP"]
    VALUE16 = 15,
}
impl From<EV0IS_A> for u8 {
    #[inline(always)]
    fn from(variant: EV0IS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EV0IS`"]
pub type EV0IS_R = crate::R<u8, EV0IS_A>;
impl EV0IS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EV0IS_A {
        match self.bits {
            0 => EV0IS_A::VALUE1,
            1 => EV0IS_A::VALUE2,
            2 => EV0IS_A::VALUE3,
            3 => EV0IS_A::VALUE4,
            4 => EV0IS_A::VALUE5,
            5 => EV0IS_A::VALUE6,
            6 => EV0IS_A::VALUE7,
            7 => EV0IS_A::VALUE8,
            8 => EV0IS_A::VALUE9,
            9 => EV0IS_A::VALUE10,
            10 => EV0IS_A::VALUE11,
            11 => EV0IS_A::VALUE12,
            12 => EV0IS_A::VALUE13,
            13 => EV0IS_A::VALUE14,
            14 => EV0IS_A::VALUE15,
            15 => EV0IS_A::VALUE16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EV0IS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EV0IS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == EV0IS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == EV0IS_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == EV0IS_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == EV0IS_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == EV0IS_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == EV0IS_A::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == EV0IS_A::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == EV0IS_A::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == EV0IS_A::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == EV0IS_A::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == EV0IS_A::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == EV0IS_A::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == EV0IS_A::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == EV0IS_A::VALUE16
    }
}
#[doc = "Write proxy for field `EV0IS`"]
pub struct EV0IS_W<'a> {
    w: &'a mut W,
}
impl<'a> EV0IS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EV0IS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CCU4x.INyA"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EV0IS_A::VALUE1)
    }
    #[doc = "CCU4x.INyB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EV0IS_A::VALUE2)
    }
    #[doc = "CCU4x.INyC"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(EV0IS_A::VALUE3)
    }
    #[doc = "CCU4x.INyD"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(EV0IS_A::VALUE4)
    }
    #[doc = "CCU4x.INyE"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(EV0IS_A::VALUE5)
    }
    #[doc = "CCU4x.INyF"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(EV0IS_A::VALUE6)
    }
    #[doc = "CCU4x.INyG"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(EV0IS_A::VALUE7)
    }
    #[doc = "CCU4x.INyH"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut W {
        self.variant(EV0IS_A::VALUE8)
    }
    #[doc = "CCU4x.INyI"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut W {
        self.variant(EV0IS_A::VALUE9)
    }
    #[doc = "CCU4x.INyJ"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut W {
        self.variant(EV0IS_A::VALUE10)
    }
    #[doc = "CCU4x.INyK"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut W {
        self.variant(EV0IS_A::VALUE11)
    }
    #[doc = "CCU4x.INyL"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut W {
        self.variant(EV0IS_A::VALUE12)
    }
    #[doc = "CCU4x.INyM"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut W {
        self.variant(EV0IS_A::VALUE13)
    }
    #[doc = "CCU4x.INyN"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut W {
        self.variant(EV0IS_A::VALUE14)
    }
    #[doc = "CCU4x.INyO"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut W {
        self.variant(EV0IS_A::VALUE15)
    }
    #[doc = "CCU4x.INyP"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut W {
        self.variant(EV0IS_A::VALUE16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Event 1 signal selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EV1IS_A {
    #[doc = "0: CCU4x.INyA"]
    VALUE1 = 0,
    #[doc = "1: CCU4x.INyB"]
    VALUE2 = 1,
    #[doc = "2: CCU4x.INyC"]
    VALUE3 = 2,
    #[doc = "3: CCU4x.INyD"]
    VALUE4 = 3,
    #[doc = "4: CCU4x.INyE"]
    VALUE5 = 4,
    #[doc = "5: CCU4x.INyF"]
    VALUE6 = 5,
    #[doc = "6: CCU4x.INyG"]
    VALUE7 = 6,
    #[doc = "7: CCU4x.INyH"]
    VALUE8 = 7,
    #[doc = "8: CCU4x.INyI"]
    VALUE9 = 8,
    #[doc = "9: CCU4x.INyJ"]
    VALUE10 = 9,
    #[doc = "10: CCU4x.INyK"]
    VALUE11 = 10,
    #[doc = "11: CCU4x.INyL"]
    VALUE12 = 11,
    #[doc = "12: CCU4x.INyM"]
    VALUE13 = 12,
    #[doc = "13: CCU4x.INyN"]
    VALUE14 = 13,
    #[doc = "14: CCU4x.INyO"]
    VALUE15 = 14,
    #[doc = "15: CCU4x.INyP"]
    VALUE16 = 15,
}
impl From<EV1IS_A> for u8 {
    #[inline(always)]
    fn from(variant: EV1IS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EV1IS`"]
pub type EV1IS_R = crate::R<u8, EV1IS_A>;
impl EV1IS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EV1IS_A {
        match self.bits {
            0 => EV1IS_A::VALUE1,
            1 => EV1IS_A::VALUE2,
            2 => EV1IS_A::VALUE3,
            3 => EV1IS_A::VALUE4,
            4 => EV1IS_A::VALUE5,
            5 => EV1IS_A::VALUE6,
            6 => EV1IS_A::VALUE7,
            7 => EV1IS_A::VALUE8,
            8 => EV1IS_A::VALUE9,
            9 => EV1IS_A::VALUE10,
            10 => EV1IS_A::VALUE11,
            11 => EV1IS_A::VALUE12,
            12 => EV1IS_A::VALUE13,
            13 => EV1IS_A::VALUE14,
            14 => EV1IS_A::VALUE15,
            15 => EV1IS_A::VALUE16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EV1IS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EV1IS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == EV1IS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == EV1IS_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == EV1IS_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == EV1IS_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == EV1IS_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == EV1IS_A::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == EV1IS_A::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == EV1IS_A::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == EV1IS_A::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == EV1IS_A::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == EV1IS_A::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == EV1IS_A::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == EV1IS_A::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == EV1IS_A::VALUE16
    }
}
#[doc = "Write proxy for field `EV1IS`"]
pub struct EV1IS_W<'a> {
    w: &'a mut W,
}
impl<'a> EV1IS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EV1IS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CCU4x.INyA"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EV1IS_A::VALUE1)
    }
    #[doc = "CCU4x.INyB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EV1IS_A::VALUE2)
    }
    #[doc = "CCU4x.INyC"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(EV1IS_A::VALUE3)
    }
    #[doc = "CCU4x.INyD"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(EV1IS_A::VALUE4)
    }
    #[doc = "CCU4x.INyE"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(EV1IS_A::VALUE5)
    }
    #[doc = "CCU4x.INyF"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(EV1IS_A::VALUE6)
    }
    #[doc = "CCU4x.INyG"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(EV1IS_A::VALUE7)
    }
    #[doc = "CCU4x.INyH"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut W {
        self.variant(EV1IS_A::VALUE8)
    }
    #[doc = "CCU4x.INyI"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut W {
        self.variant(EV1IS_A::VALUE9)
    }
    #[doc = "CCU4x.INyJ"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut W {
        self.variant(EV1IS_A::VALUE10)
    }
    #[doc = "CCU4x.INyK"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut W {
        self.variant(EV1IS_A::VALUE11)
    }
    #[doc = "CCU4x.INyL"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut W {
        self.variant(EV1IS_A::VALUE12)
    }
    #[doc = "CCU4x.INyM"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut W {
        self.variant(EV1IS_A::VALUE13)
    }
    #[doc = "CCU4x.INyN"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut W {
        self.variant(EV1IS_A::VALUE14)
    }
    #[doc = "CCU4x.INyO"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut W {
        self.variant(EV1IS_A::VALUE15)
    }
    #[doc = "CCU4x.INyP"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut W {
        self.variant(EV1IS_A::VALUE16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Event 2 signal selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EV2IS_A {
    #[doc = "0: CCU4x.INyA"]
    VALUE1 = 0,
    #[doc = "1: CCU4x.INyB"]
    VALUE2 = 1,
    #[doc = "2: CCU4x.INyC"]
    VALUE3 = 2,
    #[doc = "3: CCU4x.INyD"]
    VALUE4 = 3,
    #[doc = "4: CCU4x.INyE"]
    VALUE5 = 4,
    #[doc = "5: CCU4x.INyF"]
    VALUE6 = 5,
    #[doc = "6: CCU4x.INyG"]
    VALUE7 = 6,
    #[doc = "7: CCU4x.INyH"]
    VALUE8 = 7,
    #[doc = "8: CCU4x.INyI"]
    VALUE9 = 8,
    #[doc = "9: CCU4x.INyJ"]
    VALUE10 = 9,
    #[doc = "10: CCU4x.INyK"]
    VALUE11 = 10,
    #[doc = "11: CCU4x.INyL"]
    VALUE12 = 11,
    #[doc = "12: CCU4x.INyM"]
    VALUE13 = 12,
    #[doc = "13: CCU4x.INyN"]
    VALUE14 = 13,
    #[doc = "14: CCU4x.INyO"]
    VALUE15 = 14,
    #[doc = "15: CCU4x.INyP"]
    VALUE16 = 15,
}
impl From<EV2IS_A> for u8 {
    #[inline(always)]
    fn from(variant: EV2IS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EV2IS`"]
pub type EV2IS_R = crate::R<u8, EV2IS_A>;
impl EV2IS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EV2IS_A {
        match self.bits {
            0 => EV2IS_A::VALUE1,
            1 => EV2IS_A::VALUE2,
            2 => EV2IS_A::VALUE3,
            3 => EV2IS_A::VALUE4,
            4 => EV2IS_A::VALUE5,
            5 => EV2IS_A::VALUE6,
            6 => EV2IS_A::VALUE7,
            7 => EV2IS_A::VALUE8,
            8 => EV2IS_A::VALUE9,
            9 => EV2IS_A::VALUE10,
            10 => EV2IS_A::VALUE11,
            11 => EV2IS_A::VALUE12,
            12 => EV2IS_A::VALUE13,
            13 => EV2IS_A::VALUE14,
            14 => EV2IS_A::VALUE15,
            15 => EV2IS_A::VALUE16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EV2IS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EV2IS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == EV2IS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == EV2IS_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == EV2IS_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == EV2IS_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == EV2IS_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == EV2IS_A::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == EV2IS_A::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == EV2IS_A::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == EV2IS_A::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == EV2IS_A::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == EV2IS_A::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == EV2IS_A::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == EV2IS_A::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == EV2IS_A::VALUE16
    }
}
#[doc = "Write proxy for field `EV2IS`"]
pub struct EV2IS_W<'a> {
    w: &'a mut W,
}
impl<'a> EV2IS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EV2IS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CCU4x.INyA"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EV2IS_A::VALUE1)
    }
    #[doc = "CCU4x.INyB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EV2IS_A::VALUE2)
    }
    #[doc = "CCU4x.INyC"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(EV2IS_A::VALUE3)
    }
    #[doc = "CCU4x.INyD"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(EV2IS_A::VALUE4)
    }
    #[doc = "CCU4x.INyE"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(EV2IS_A::VALUE5)
    }
    #[doc = "CCU4x.INyF"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(EV2IS_A::VALUE6)
    }
    #[doc = "CCU4x.INyG"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(EV2IS_A::VALUE7)
    }
    #[doc = "CCU4x.INyH"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut W {
        self.variant(EV2IS_A::VALUE8)
    }
    #[doc = "CCU4x.INyI"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut W {
        self.variant(EV2IS_A::VALUE9)
    }
    #[doc = "CCU4x.INyJ"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut W {
        self.variant(EV2IS_A::VALUE10)
    }
    #[doc = "CCU4x.INyK"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut W {
        self.variant(EV2IS_A::VALUE11)
    }
    #[doc = "CCU4x.INyL"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut W {
        self.variant(EV2IS_A::VALUE12)
    }
    #[doc = "CCU4x.INyM"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut W {
        self.variant(EV2IS_A::VALUE13)
    }
    #[doc = "CCU4x.INyN"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut W {
        self.variant(EV2IS_A::VALUE14)
    }
    #[doc = "CCU4x.INyO"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut W {
        self.variant(EV2IS_A::VALUE15)
    }
    #[doc = "CCU4x.INyP"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut W {
        self.variant(EV2IS_A::VALUE16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Event 0 Edge Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EV0EM_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Signal active on rising edge"]
    VALUE2 = 1,
    #[doc = "2: Signal active on falling edge"]
    VALUE3 = 2,
    #[doc = "3: Signal active on both edges"]
    VALUE4 = 3,
}
impl From<EV0EM_A> for u8 {
    #[inline(always)]
    fn from(variant: EV0EM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EV0EM`"]
pub type EV0EM_R = crate::R<u8, EV0EM_A>;
impl EV0EM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EV0EM_A {
        match self.bits {
            0 => EV0EM_A::VALUE1,
            1 => EV0EM_A::VALUE2,
            2 => EV0EM_A::VALUE3,
            3 => EV0EM_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EV0EM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EV0EM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == EV0EM_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == EV0EM_A::VALUE4
    }
}
#[doc = "Write proxy for field `EV0EM`"]
pub struct EV0EM_W<'a> {
    w: &'a mut W,
}
impl<'a> EV0EM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EV0EM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EV0EM_A::VALUE1)
    }
    #[doc = "Signal active on rising edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EV0EM_A::VALUE2)
    }
    #[doc = "Signal active on falling edge"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(EV0EM_A::VALUE3)
    }
    #[doc = "Signal active on both edges"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(EV0EM_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Event 1 Edge Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EV1EM_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Signal active on rising edge"]
    VALUE2 = 1,
    #[doc = "2: Signal active on falling edge"]
    VALUE3 = 2,
    #[doc = "3: Signal active on both edges"]
    VALUE4 = 3,
}
impl From<EV1EM_A> for u8 {
    #[inline(always)]
    fn from(variant: EV1EM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EV1EM`"]
pub type EV1EM_R = crate::R<u8, EV1EM_A>;
impl EV1EM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EV1EM_A {
        match self.bits {
            0 => EV1EM_A::VALUE1,
            1 => EV1EM_A::VALUE2,
            2 => EV1EM_A::VALUE3,
            3 => EV1EM_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EV1EM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EV1EM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == EV1EM_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == EV1EM_A::VALUE4
    }
}
#[doc = "Write proxy for field `EV1EM`"]
pub struct EV1EM_W<'a> {
    w: &'a mut W,
}
impl<'a> EV1EM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EV1EM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EV1EM_A::VALUE1)
    }
    #[doc = "Signal active on rising edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EV1EM_A::VALUE2)
    }
    #[doc = "Signal active on falling edge"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(EV1EM_A::VALUE3)
    }
    #[doc = "Signal active on both edges"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(EV1EM_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Event 2 Edge Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EV2EM_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Signal active on rising edge"]
    VALUE2 = 1,
    #[doc = "2: Signal active on falling edge"]
    VALUE3 = 2,
    #[doc = "3: Signal active on both edges"]
    VALUE4 = 3,
}
impl From<EV2EM_A> for u8 {
    #[inline(always)]
    fn from(variant: EV2EM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EV2EM`"]
pub type EV2EM_R = crate::R<u8, EV2EM_A>;
impl EV2EM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EV2EM_A {
        match self.bits {
            0 => EV2EM_A::VALUE1,
            1 => EV2EM_A::VALUE2,
            2 => EV2EM_A::VALUE3,
            3 => EV2EM_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EV2EM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EV2EM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == EV2EM_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == EV2EM_A::VALUE4
    }
}
#[doc = "Write proxy for field `EV2EM`"]
pub struct EV2EM_W<'a> {
    w: &'a mut W,
}
impl<'a> EV2EM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EV2EM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EV2EM_A::VALUE1)
    }
    #[doc = "Signal active on rising edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EV2EM_A::VALUE2)
    }
    #[doc = "Signal active on falling edge"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(EV2EM_A::VALUE3)
    }
    #[doc = "Signal active on both edges"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(EV2EM_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Event 0 Level Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EV0LM_A {
    #[doc = "0: Active on HIGH level"]
    VALUE1 = 0,
    #[doc = "1: Active on LOW level"]
    VALUE2 = 1,
}
impl From<EV0LM_A> for bool {
    #[inline(always)]
    fn from(variant: EV0LM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EV0LM`"]
pub type EV0LM_R = crate::R<bool, EV0LM_A>;
impl EV0LM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EV0LM_A {
        match self.bits {
            false => EV0LM_A::VALUE1,
            true => EV0LM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EV0LM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EV0LM_A::VALUE2
    }
}
#[doc = "Write proxy for field `EV0LM`"]
pub struct EV0LM_W<'a> {
    w: &'a mut W,
}
impl<'a> EV0LM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EV0LM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Active on HIGH level"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EV0LM_A::VALUE1)
    }
    #[doc = "Active on LOW level"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EV0LM_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Event 1 Level Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EV1LM_A {
    #[doc = "0: Active on HIGH level"]
    VALUE1 = 0,
    #[doc = "1: Active on LOW level"]
    VALUE2 = 1,
}
impl From<EV1LM_A> for bool {
    #[inline(always)]
    fn from(variant: EV1LM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EV1LM`"]
pub type EV1LM_R = crate::R<bool, EV1LM_A>;
impl EV1LM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EV1LM_A {
        match self.bits {
            false => EV1LM_A::VALUE1,
            true => EV1LM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EV1LM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EV1LM_A::VALUE2
    }
}
#[doc = "Write proxy for field `EV1LM`"]
pub struct EV1LM_W<'a> {
    w: &'a mut W,
}
impl<'a> EV1LM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EV1LM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Active on HIGH level"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EV1LM_A::VALUE1)
    }
    #[doc = "Active on LOW level"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EV1LM_A::VALUE2)
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
#[doc = "Event 2 Level Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EV2LM_A {
    #[doc = "0: Active on HIGH level"]
    VALUE1 = 0,
    #[doc = "1: Active on LOW level"]
    VALUE2 = 1,
}
impl From<EV2LM_A> for bool {
    #[inline(always)]
    fn from(variant: EV2LM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EV2LM`"]
pub type EV2LM_R = crate::R<bool, EV2LM_A>;
impl EV2LM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EV2LM_A {
        match self.bits {
            false => EV2LM_A::VALUE1,
            true => EV2LM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EV2LM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EV2LM_A::VALUE2
    }
}
#[doc = "Write proxy for field `EV2LM`"]
pub struct EV2LM_W<'a> {
    w: &'a mut W,
}
impl<'a> EV2LM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EV2LM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Active on HIGH level"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EV2LM_A::VALUE1)
    }
    #[doc = "Active on LOW level"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EV2LM_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Event 0 Low Pass Filter Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPF0M_A {
    #[doc = "0: LPF is disabled"]
    VALUE1 = 0,
    #[doc = "1: 3 clock cycles of fCCU4"]
    VALUE2 = 1,
    #[doc = "2: 5 clock cycles of fCCU4"]
    VALUE3 = 2,
    #[doc = "3: 7 clock cycles of fCCU4"]
    VALUE4 = 3,
}
impl From<LPF0M_A> for u8 {
    #[inline(always)]
    fn from(variant: LPF0M_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LPF0M`"]
pub type LPF0M_R = crate::R<u8, LPF0M_A>;
impl LPF0M_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPF0M_A {
        match self.bits {
            0 => LPF0M_A::VALUE1,
            1 => LPF0M_A::VALUE2,
            2 => LPF0M_A::VALUE3,
            3 => LPF0M_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LPF0M_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LPF0M_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == LPF0M_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == LPF0M_A::VALUE4
    }
}
#[doc = "Write proxy for field `LPF0M`"]
pub struct LPF0M_W<'a> {
    w: &'a mut W,
}
impl<'a> LPF0M_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPF0M_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "LPF is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LPF0M_A::VALUE1)
    }
    #[doc = "3 clock cycles of fCCU4"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LPF0M_A::VALUE2)
    }
    #[doc = "5 clock cycles of fCCU4"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(LPF0M_A::VALUE3)
    }
    #[doc = "7 clock cycles of fCCU4"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(LPF0M_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | (((value as u32) & 0x03) << 25);
        self.w
    }
}
#[doc = "Event 1 Low Pass Filter Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPF1M_A {
    #[doc = "0: LPF is disabled"]
    VALUE1 = 0,
    #[doc = "1: 3 clock cycles of fCCU4"]
    VALUE2 = 1,
    #[doc = "2: 5 clock cycles of fCCU4"]
    VALUE3 = 2,
    #[doc = "3: 7 clock cycles of fCCU4"]
    VALUE4 = 3,
}
impl From<LPF1M_A> for u8 {
    #[inline(always)]
    fn from(variant: LPF1M_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LPF1M`"]
pub type LPF1M_R = crate::R<u8, LPF1M_A>;
impl LPF1M_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPF1M_A {
        match self.bits {
            0 => LPF1M_A::VALUE1,
            1 => LPF1M_A::VALUE2,
            2 => LPF1M_A::VALUE3,
            3 => LPF1M_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LPF1M_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LPF1M_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == LPF1M_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == LPF1M_A::VALUE4
    }
}
#[doc = "Write proxy for field `LPF1M`"]
pub struct LPF1M_W<'a> {
    w: &'a mut W,
}
impl<'a> LPF1M_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPF1M_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "LPF is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LPF1M_A::VALUE1)
    }
    #[doc = "3 clock cycles of fCCU4"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LPF1M_A::VALUE2)
    }
    #[doc = "5 clock cycles of fCCU4"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(LPF1M_A::VALUE3)
    }
    #[doc = "7 clock cycles of fCCU4"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(LPF1M_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | (((value as u32) & 0x03) << 27);
        self.w
    }
}
#[doc = "Event 2 Low Pass Filter Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPF2M_A {
    #[doc = "0: LPF is disabled"]
    VALUE1 = 0,
    #[doc = "1: 3 clock cycles of fCCU4"]
    VALUE2 = 1,
    #[doc = "2: 5 clock cycles of fCCU4"]
    VALUE3 = 2,
    #[doc = "3: 7 clock cycles of fCCU4"]
    VALUE4 = 3,
}
impl From<LPF2M_A> for u8 {
    #[inline(always)]
    fn from(variant: LPF2M_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LPF2M`"]
pub type LPF2M_R = crate::R<u8, LPF2M_A>;
impl LPF2M_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPF2M_A {
        match self.bits {
            0 => LPF2M_A::VALUE1,
            1 => LPF2M_A::VALUE2,
            2 => LPF2M_A::VALUE3,
            3 => LPF2M_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LPF2M_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LPF2M_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == LPF2M_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == LPF2M_A::VALUE4
    }
}
#[doc = "Write proxy for field `LPF2M`"]
pub struct LPF2M_W<'a> {
    w: &'a mut W,
}
impl<'a> LPF2M_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPF2M_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "LPF is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LPF2M_A::VALUE1)
    }
    #[doc = "3 clock cycles of fCCU4"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LPF2M_A::VALUE2)
    }
    #[doc = "5 clock cycles of fCCU4"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(LPF2M_A::VALUE3)
    }
    #[doc = "7 clock cycles of fCCU4"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(LPF2M_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | (((value as u32) & 0x03) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Event 0 signal selection"]
    #[inline(always)]
    pub fn ev0is(&self) -> EV0IS_R {
        EV0IS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Event 1 signal selection"]
    #[inline(always)]
    pub fn ev1is(&self) -> EV1IS_R {
        EV1IS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Event 2 signal selection"]
    #[inline(always)]
    pub fn ev2is(&self) -> EV2IS_R {
        EV2IS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Event 0 Edge Selection"]
    #[inline(always)]
    pub fn ev0em(&self) -> EV0EM_R {
        EV0EM_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Event 1 Edge Selection"]
    #[inline(always)]
    pub fn ev1em(&self) -> EV1EM_R {
        EV1EM_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Event 2 Edge Selection"]
    #[inline(always)]
    pub fn ev2em(&self) -> EV2EM_R {
        EV2EM_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 22 - Event 0 Level Selection"]
    #[inline(always)]
    pub fn ev0lm(&self) -> EV0LM_R {
        EV0LM_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Event 1 Level Selection"]
    #[inline(always)]
    pub fn ev1lm(&self) -> EV1LM_R {
        EV1LM_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Event 2 Level Selection"]
    #[inline(always)]
    pub fn ev2lm(&self) -> EV2LM_R {
        EV2LM_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 25:26 - Event 0 Low Pass Filter Configuration"]
    #[inline(always)]
    pub fn lpf0m(&self) -> LPF0M_R {
        LPF0M_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bits 27:28 - Event 1 Low Pass Filter Configuration"]
    #[inline(always)]
    pub fn lpf1m(&self) -> LPF1M_R {
        LPF1M_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bits 29:30 - Event 2 Low Pass Filter Configuration"]
    #[inline(always)]
    pub fn lpf2m(&self) -> LPF2M_R {
        LPF2M_R::new(((self.bits >> 29) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Event 0 signal selection"]
    #[inline(always)]
    pub fn ev0is(&mut self) -> EV0IS_W {
        EV0IS_W { w: self }
    }
    #[doc = "Bits 4:7 - Event 1 signal selection"]
    #[inline(always)]
    pub fn ev1is(&mut self) -> EV1IS_W {
        EV1IS_W { w: self }
    }
    #[doc = "Bits 8:11 - Event 2 signal selection"]
    #[inline(always)]
    pub fn ev2is(&mut self) -> EV2IS_W {
        EV2IS_W { w: self }
    }
    #[doc = "Bits 16:17 - Event 0 Edge Selection"]
    #[inline(always)]
    pub fn ev0em(&mut self) -> EV0EM_W {
        EV0EM_W { w: self }
    }
    #[doc = "Bits 18:19 - Event 1 Edge Selection"]
    #[inline(always)]
    pub fn ev1em(&mut self) -> EV1EM_W {
        EV1EM_W { w: self }
    }
    #[doc = "Bits 20:21 - Event 2 Edge Selection"]
    #[inline(always)]
    pub fn ev2em(&mut self) -> EV2EM_W {
        EV2EM_W { w: self }
    }
    #[doc = "Bit 22 - Event 0 Level Selection"]
    #[inline(always)]
    pub fn ev0lm(&mut self) -> EV0LM_W {
        EV0LM_W { w: self }
    }
    #[doc = "Bit 23 - Event 1 Level Selection"]
    #[inline(always)]
    pub fn ev1lm(&mut self) -> EV1LM_W {
        EV1LM_W { w: self }
    }
    #[doc = "Bit 24 - Event 2 Level Selection"]
    #[inline(always)]
    pub fn ev2lm(&mut self) -> EV2LM_W {
        EV2LM_W { w: self }
    }
    #[doc = "Bits 25:26 - Event 0 Low Pass Filter Configuration"]
    #[inline(always)]
    pub fn lpf0m(&mut self) -> LPF0M_W {
        LPF0M_W { w: self }
    }
    #[doc = "Bits 27:28 - Event 1 Low Pass Filter Configuration"]
    #[inline(always)]
    pub fn lpf1m(&mut self) -> LPF1M_W {
        LPF1M_W { w: self }
    }
    #[doc = "Bits 29:30 - Event 2 Low Pass Filter Configuration"]
    #[inline(always)]
    pub fn lpf2m(&mut self) -> LPF2M_W {
        LPF2M_W { w: self }
    }
}
