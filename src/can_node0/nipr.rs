#[doc = "Reader of register NIPR"]
pub type R = crate::R<u32, super::NIPR>;
#[doc = "Writer for register NIPR"]
pub type W = crate::W<u32, super::NIPR>;
#[doc = "Register NIPR `reset()`'s with value 0"]
impl crate::ResetValue for super::NIPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Alert Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALINP_A {
    #[doc = "0: Interrupt output line INT_O0 is selected."]
    VALUE1,
    #[doc = "1: Interrupt output line INT_O1 is selected."]
    VALUE2,
    #[doc = "14: Interrupt output line INT_O14 is selected."]
    VALUE3,
    #[doc = "15: Interrupt output line INT_O15 is selected."]
    VALUE4,
}
impl From<ALINP_A> for u8 {
    #[inline(always)]
    fn from(variant: ALINP_A) -> Self {
        match variant {
            ALINP_A::VALUE1 => 0,
            ALINP_A::VALUE2 => 1,
            ALINP_A::VALUE3 => 14,
            ALINP_A::VALUE4 => 15,
        }
    }
}
#[doc = "Reader of field `ALINP`"]
pub type ALINP_R = crate::R<u8, ALINP_A>;
impl ALINP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ALINP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ALINP_A::VALUE1),
            1 => Val(ALINP_A::VALUE2),
            14 => Val(ALINP_A::VALUE3),
            15 => Val(ALINP_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALINP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALINP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ALINP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == ALINP_A::VALUE4
    }
}
#[doc = "Write proxy for field `ALINP`"]
pub struct ALINP_W<'a> {
    w: &'a mut W,
}
impl<'a> ALINP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALINP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALINP_A::VALUE1)
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALINP_A::VALUE2)
    }
    #[doc = "Interrupt output line INT_O14 is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(ALINP_A::VALUE3)
    }
    #[doc = "Interrupt output line INT_O15 is selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(ALINP_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Last Error Code Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LECINP_A {
    #[doc = "0: Interrupt output line INT_O0 is selected."]
    VALUE1,
    #[doc = "1: Interrupt output line INT_O1 is selected."]
    VALUE2,
    #[doc = "14: Interrupt output line INT_O14 is selected."]
    VALUE3,
    #[doc = "15: Interrupt output line INT_O15 is selected."]
    VALUE4,
}
impl From<LECINP_A> for u8 {
    #[inline(always)]
    fn from(variant: LECINP_A) -> Self {
        match variant {
            LECINP_A::VALUE1 => 0,
            LECINP_A::VALUE2 => 1,
            LECINP_A::VALUE3 => 14,
            LECINP_A::VALUE4 => 15,
        }
    }
}
#[doc = "Reader of field `LECINP`"]
pub type LECINP_R = crate::R<u8, LECINP_A>;
impl LECINP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LECINP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LECINP_A::VALUE1),
            1 => Val(LECINP_A::VALUE2),
            14 => Val(LECINP_A::VALUE3),
            15 => Val(LECINP_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LECINP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LECINP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == LECINP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == LECINP_A::VALUE4
    }
}
#[doc = "Write proxy for field `LECINP`"]
pub struct LECINP_W<'a> {
    w: &'a mut W,
}
impl<'a> LECINP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LECINP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LECINP_A::VALUE1)
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LECINP_A::VALUE2)
    }
    #[doc = "Interrupt output line INT_O14 is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(LECINP_A::VALUE3)
    }
    #[doc = "Interrupt output line INT_O15 is selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(LECINP_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Transfer OK Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRINP_A {
    #[doc = "0: Interrupt output line INT_O0 is selected."]
    VALUE1,
    #[doc = "1: Interrupt output line INT_O1 is selected."]
    VALUE2,
    #[doc = "14: Interrupt output line INT_O14 is selected."]
    VALUE3,
    #[doc = "15: Interrupt output line INT_O15 is selected."]
    VALUE4,
}
impl From<TRINP_A> for u8 {
    #[inline(always)]
    fn from(variant: TRINP_A) -> Self {
        match variant {
            TRINP_A::VALUE1 => 0,
            TRINP_A::VALUE2 => 1,
            TRINP_A::VALUE3 => 14,
            TRINP_A::VALUE4 => 15,
        }
    }
}
#[doc = "Reader of field `TRINP`"]
pub type TRINP_R = crate::R<u8, TRINP_A>;
impl TRINP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TRINP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TRINP_A::VALUE1),
            1 => Val(TRINP_A::VALUE2),
            14 => Val(TRINP_A::VALUE3),
            15 => Val(TRINP_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TRINP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TRINP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TRINP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == TRINP_A::VALUE4
    }
}
#[doc = "Write proxy for field `TRINP`"]
pub struct TRINP_W<'a> {
    w: &'a mut W,
}
impl<'a> TRINP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRINP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TRINP_A::VALUE1)
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TRINP_A::VALUE2)
    }
    #[doc = "Interrupt output line INT_O14 is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(TRINP_A::VALUE3)
    }
    #[doc = "Interrupt output line INT_O15 is selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(TRINP_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Frame Counter Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFCINP_A {
    #[doc = "0: Interrupt output line INT_O0 is selected."]
    VALUE1,
    #[doc = "1: Interrupt output line INT_O1 is selected."]
    VALUE2,
    #[doc = "14: Interrupt output line INT_O14 is selected."]
    VALUE3,
    #[doc = "15: Interrupt output line INT_O15 is selected."]
    VALUE4,
}
impl From<CFCINP_A> for u8 {
    #[inline(always)]
    fn from(variant: CFCINP_A) -> Self {
        match variant {
            CFCINP_A::VALUE1 => 0,
            CFCINP_A::VALUE2 => 1,
            CFCINP_A::VALUE3 => 14,
            CFCINP_A::VALUE4 => 15,
        }
    }
}
#[doc = "Reader of field `CFCINP`"]
pub type CFCINP_R = crate::R<u8, CFCINP_A>;
impl CFCINP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CFCINP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CFCINP_A::VALUE1),
            1 => Val(CFCINP_A::VALUE2),
            14 => Val(CFCINP_A::VALUE3),
            15 => Val(CFCINP_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CFCINP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CFCINP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CFCINP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CFCINP_A::VALUE4
    }
}
#[doc = "Write proxy for field `CFCINP`"]
pub struct CFCINP_W<'a> {
    w: &'a mut W,
}
impl<'a> CFCINP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFCINP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CFCINP_A::VALUE1)
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CFCINP_A::VALUE2)
    }
    #[doc = "Interrupt output line INT_O14 is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CFCINP_A::VALUE3)
    }
    #[doc = "Interrupt output line INT_O15 is selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CFCINP_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Alert Interrupt Node Pointer"]
    #[inline(always)]
    pub fn alinp(&self) -> ALINP_R {
        ALINP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Last Error Code Interrupt Node Pointer"]
    #[inline(always)]
    pub fn lecinp(&self) -> LECINP_R {
        LECINP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Transfer OK Interrupt Node Pointer"]
    #[inline(always)]
    pub fn trinp(&self) -> TRINP_R {
        TRINP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Frame Counter Interrupt Node Pointer"]
    #[inline(always)]
    pub fn cfcinp(&self) -> CFCINP_R {
        CFCINP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Alert Interrupt Node Pointer"]
    #[inline(always)]
    pub fn alinp(&mut self) -> ALINP_W {
        ALINP_W { w: self }
    }
    #[doc = "Bits 4:7 - Last Error Code Interrupt Node Pointer"]
    #[inline(always)]
    pub fn lecinp(&mut self) -> LECINP_W {
        LECINP_W { w: self }
    }
    #[doc = "Bits 8:11 - Transfer OK Interrupt Node Pointer"]
    #[inline(always)]
    pub fn trinp(&mut self) -> TRINP_W {
        TRINP_W { w: self }
    }
    #[doc = "Bits 12:15 - Frame Counter Interrupt Node Pointer"]
    #[inline(always)]
    pub fn cfcinp(&mut self) -> CFCINP_W {
        CFCINP_W { w: self }
    }
}
