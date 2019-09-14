#[doc = "Reader of register STCON"]
pub type R = crate::R<u32, super::STCON>;
#[doc = "Writer for register STCON"]
pub type W = crate::W<u32, super::STCON>;
#[doc = "Register STCON `reset()`'s with value 0"]
impl crate::ResetValue for super::STCON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "HW Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HWCON_A {
    #[doc = "0: Normal mode, JTAG"]
    CONST_00,
    #[doc = "1: ASC BSL enabled"]
    CONST_01,
    #[doc = "2: BMI customized boot enabled"]
    CONST_10,
    #[doc = "3: CAN BSL enabled"]
    CONST_11,
}
impl From<HWCON_A> for u8 {
    #[inline(always)]
    fn from(variant: HWCON_A) -> Self {
        match variant {
            HWCON_A::CONST_00 => 0,
            HWCON_A::CONST_01 => 1,
            HWCON_A::CONST_10 => 2,
            HWCON_A::CONST_11 => 3,
        }
    }
}
#[doc = "Reader of field `HWCON`"]
pub type HWCON_R = crate::R<u8, HWCON_A>;
impl HWCON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HWCON_A {
        match self.bits {
            0 => HWCON_A::CONST_00,
            1 => HWCON_A::CONST_01,
            2 => HWCON_A::CONST_10,
            3 => HWCON_A::CONST_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == HWCON_A::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == HWCON_A::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == HWCON_A::CONST_10
    }
    #[doc = "Checks if the value of the field is `CONST_11`"]
    #[inline(always)]
    pub fn is_const_11(&self) -> bool {
        *self == HWCON_A::CONST_11
    }
}
#[doc = "SW Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWCON_A {
    #[doc = "0: Normal mode, boot from Boot ROM"]
    CONST_0000,
    #[doc = "1: ASC BSL enabled"]
    CONST_0001,
    #[doc = "2: BMI customized boot enabled"]
    CONST_0010,
    #[doc = "3: CAN BSL enabled"]
    CONST_0011,
    #[doc = "4: Boot from Code SRAM"]
    CONST_0100,
    #[doc = "8: Boot from alternate Flash Address 0"]
    CONST_1000,
    #[doc = "12: Boot from alternate Flash Address 1"]
    CONST_1100,
    #[doc = "14: Enable fallback Alternate Boot Mode (ABM)"]
    CONST_1110,
}
impl From<SWCON_A> for u8 {
    #[inline(always)]
    fn from(variant: SWCON_A) -> Self {
        match variant {
            SWCON_A::CONST_0000 => 0,
            SWCON_A::CONST_0001 => 1,
            SWCON_A::CONST_0010 => 2,
            SWCON_A::CONST_0011 => 3,
            SWCON_A::CONST_0100 => 4,
            SWCON_A::CONST_1000 => 8,
            SWCON_A::CONST_1100 => 12,
            SWCON_A::CONST_1110 => 14,
        }
    }
}
#[doc = "Reader of field `SWCON`"]
pub type SWCON_R = crate::R<u8, SWCON_A>;
impl SWCON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SWCON_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SWCON_A::CONST_0000),
            1 => Val(SWCON_A::CONST_0001),
            2 => Val(SWCON_A::CONST_0010),
            3 => Val(SWCON_A::CONST_0011),
            4 => Val(SWCON_A::CONST_0100),
            8 => Val(SWCON_A::CONST_1000),
            12 => Val(SWCON_A::CONST_1100),
            14 => Val(SWCON_A::CONST_1110),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0000`"]
    #[inline(always)]
    pub fn is_const_0000(&self) -> bool {
        *self == SWCON_A::CONST_0000
    }
    #[doc = "Checks if the value of the field is `CONST_0001`"]
    #[inline(always)]
    pub fn is_const_0001(&self) -> bool {
        *self == SWCON_A::CONST_0001
    }
    #[doc = "Checks if the value of the field is `CONST_0010`"]
    #[inline(always)]
    pub fn is_const_0010(&self) -> bool {
        *self == SWCON_A::CONST_0010
    }
    #[doc = "Checks if the value of the field is `CONST_0011`"]
    #[inline(always)]
    pub fn is_const_0011(&self) -> bool {
        *self == SWCON_A::CONST_0011
    }
    #[doc = "Checks if the value of the field is `CONST_0100`"]
    #[inline(always)]
    pub fn is_const_0100(&self) -> bool {
        *self == SWCON_A::CONST_0100
    }
    #[doc = "Checks if the value of the field is `CONST_1000`"]
    #[inline(always)]
    pub fn is_const_1000(&self) -> bool {
        *self == SWCON_A::CONST_1000
    }
    #[doc = "Checks if the value of the field is `CONST_1100`"]
    #[inline(always)]
    pub fn is_const_1100(&self) -> bool {
        *self == SWCON_A::CONST_1100
    }
    #[doc = "Checks if the value of the field is `CONST_1110`"]
    #[inline(always)]
    pub fn is_const_1110(&self) -> bool {
        *self == SWCON_A::CONST_1110
    }
}
#[doc = "Write proxy for field `SWCON`"]
pub struct SWCON_W<'a> {
    w: &'a mut W,
}
impl<'a> SWCON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWCON_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Normal mode, boot from Boot ROM"]
    #[inline(always)]
    pub fn const_0000(self) -> &'a mut W {
        self.variant(SWCON_A::CONST_0000)
    }
    #[doc = "ASC BSL enabled"]
    #[inline(always)]
    pub fn const_0001(self) -> &'a mut W {
        self.variant(SWCON_A::CONST_0001)
    }
    #[doc = "BMI customized boot enabled"]
    #[inline(always)]
    pub fn const_0010(self) -> &'a mut W {
        self.variant(SWCON_A::CONST_0010)
    }
    #[doc = "CAN BSL enabled"]
    #[inline(always)]
    pub fn const_0011(self) -> &'a mut W {
        self.variant(SWCON_A::CONST_0011)
    }
    #[doc = "Boot from Code SRAM"]
    #[inline(always)]
    pub fn const_0100(self) -> &'a mut W {
        self.variant(SWCON_A::CONST_0100)
    }
    #[doc = "Boot from alternate Flash Address 0"]
    #[inline(always)]
    pub fn const_1000(self) -> &'a mut W {
        self.variant(SWCON_A::CONST_1000)
    }
    #[doc = "Boot from alternate Flash Address 1"]
    #[inline(always)]
    pub fn const_1100(self) -> &'a mut W {
        self.variant(SWCON_A::CONST_1100)
    }
    #[doc = "Enable fallback Alternate Boot Mode (ABM)"]
    #[inline(always)]
    pub fn const_1110(self) -> &'a mut W {
        self.variant(SWCON_A::CONST_1110)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - HW Configuration"]
    #[inline(always)]
    pub fn hwcon(&self) -> HWCON_R {
        HWCON_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - SW Configuration"]
    #[inline(always)]
    pub fn swcon(&self) -> SWCON_R {
        SWCON_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11 - SW Configuration"]
    #[inline(always)]
    pub fn swcon(&mut self) -> SWCON_W {
        SWCON_W { w: self }
    }
}
