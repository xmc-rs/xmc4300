#[doc = "Reader of register MLINKCLKCR"]
pub type R = crate::R<u32, super::MLINKCLKCR>;
#[doc = "Writer for register MLINKCLKCR"]
pub type W = crate::W<u32, super::MLINKCLKCR>;
#[doc = "Register MLINKCLKCR `reset()`'s with value 0"]
impl crate::ResetValue for super::MLINKCLKCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSDIV`"]
pub type SYSDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYSDIV`"]
pub struct SYSDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "System Clock Selection Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSSEL_A {
    #[doc = "0: fOFI clock"]
    CONST_0 = 0,
    #[doc = "1: fPLL clock"]
    CONST_1 = 1,
}
impl From<SYSSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SYSSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYSSEL`"]
pub type SYSSEL_R = crate::R<bool, SYSSEL_A>;
impl SYSSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSSEL_A {
        match self.bits {
            false => SYSSEL_A::CONST_0,
            true => SYSSEL_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == SYSSEL_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == SYSSEL_A::CONST_1
    }
}
#[doc = "Write proxy for field `SYSSEL`"]
pub struct SYSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "fOFI clock"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(SYSSEL_A::CONST_0)
    }
    #[doc = "fPLL clock"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(SYSSEL_A::CONST_1)
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
#[doc = "CPU Clock Divider Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPUDIV_A {
    #[doc = "0: fCPU = fSYS"]
    CONST_0 = 0,
    #[doc = "1: fCPU = fSYS / 2"]
    CONST_1 = 1,
}
impl From<CPUDIV_A> for bool {
    #[inline(always)]
    fn from(variant: CPUDIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPUDIV`"]
pub type CPUDIV_R = crate::R<bool, CPUDIV_A>;
impl CPUDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPUDIV_A {
        match self.bits {
            false => CPUDIV_A::CONST_0,
            true => CPUDIV_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == CPUDIV_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == CPUDIV_A::CONST_1
    }
}
#[doc = "Write proxy for field `CPUDIV`"]
pub struct CPUDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CPUDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPUDIV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "fCPU = fSYS"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(CPUDIV_A::CONST_0)
    }
    #[doc = "fCPU = fSYS / 2"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(CPUDIV_A::CONST_1)
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
#[doc = "PB Clock Divider Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PBDIV_A {
    #[doc = "0: fPERIPH = fCPU"]
    CONST_0 = 0,
    #[doc = "1: fPERIPH = fCPU / 2"]
    CONST_1 = 1,
}
impl From<PBDIV_A> for bool {
    #[inline(always)]
    fn from(variant: PBDIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PBDIV`"]
pub type PBDIV_R = crate::R<bool, PBDIV_A>;
impl PBDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PBDIV_A {
        match self.bits {
            false => PBDIV_A::CONST_0,
            true => PBDIV_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PBDIV_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PBDIV_A::CONST_1
    }
}
#[doc = "Write proxy for field `PBDIV`"]
pub struct PBDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PBDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PBDIV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "fPERIPH = fCPU"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PBDIV_A::CONST_0)
    }
    #[doc = "fPERIPH = fCPU / 2"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PBDIV_A::CONST_1)
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
#[doc = "CCU Clock Divider Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCUDIV_A {
    #[doc = "0: fCCU = fSYS"]
    CONST_0 = 0,
    #[doc = "1: fCCU = fSYS / 2"]
    CONST_1 = 1,
}
impl From<CCUDIV_A> for bool {
    #[inline(always)]
    fn from(variant: CCUDIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CCUDIV`"]
pub type CCUDIV_R = crate::R<bool, CCUDIV_A>;
impl CCUDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCUDIV_A {
        match self.bits {
            false => CCUDIV_A::CONST_0,
            true => CCUDIV_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == CCUDIV_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == CCUDIV_A::CONST_1
    }
}
#[doc = "Write proxy for field `CCUDIV`"]
pub struct CCUDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CCUDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCUDIV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "fCCU = fSYS"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(CCUDIV_A::CONST_0)
    }
    #[doc = "fCCU = fSYS / 2"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(CCUDIV_A::CONST_1)
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
#[doc = "Reader of field `WDTDIV`"]
pub type WDTDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WDTDIV`"]
pub struct WDTDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "WDT Clock Selection Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WDTSEL_A {
    #[doc = "0: fOFI clock"]
    CONST_00 = 0,
    #[doc = "1: fSTDBY clock"]
    CONST_01 = 1,
    #[doc = "2: fPLL clock"]
    CONST_10 = 2,
}
impl From<WDTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: WDTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WDTSEL`"]
pub type WDTSEL_R = crate::R<u8, WDTSEL_A>;
impl WDTSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WDTSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WDTSEL_A::CONST_00),
            1 => Val(WDTSEL_A::CONST_01),
            2 => Val(WDTSEL_A::CONST_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == WDTSEL_A::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == WDTSEL_A::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == WDTSEL_A::CONST_10
    }
}
#[doc = "Write proxy for field `WDTSEL`"]
pub struct WDTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDTSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "fOFI clock"]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut W {
        self.variant(WDTSEL_A::CONST_00)
    }
    #[doc = "fSTDBY clock"]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut W {
        self.variant(WDTSEL_A::CONST_01)
    }
    #[doc = "fPLL clock"]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut W {
        self.variant(WDTSEL_A::CONST_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - System Clock Division Value"]
    #[inline(always)]
    pub fn sysdiv(&self) -> SYSDIV_R {
        SYSDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - System Clock Selection Value"]
    #[inline(always)]
    pub fn syssel(&self) -> SYSSEL_R {
        SYSSEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CPU Clock Divider Enable"]
    #[inline(always)]
    pub fn cpudiv(&self) -> CPUDIV_R {
        CPUDIV_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PB Clock Divider Enable"]
    #[inline(always)]
    pub fn pbdiv(&self) -> PBDIV_R {
        PBDIV_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - CCU Clock Divider Enable"]
    #[inline(always)]
    pub fn ccudiv(&self) -> CCUDIV_R {
        CCUDIV_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - WDT Clock Divider Value"]
    #[inline(always)]
    pub fn wdtdiv(&self) -> WDTDIV_R {
        WDTDIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - WDT Clock Selection Value"]
    #[inline(always)]
    pub fn wdtsel(&self) -> WDTSEL_R {
        WDTSEL_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - System Clock Division Value"]
    #[inline(always)]
    pub fn sysdiv(&mut self) -> SYSDIV_W {
        SYSDIV_W { w: self }
    }
    #[doc = "Bit 8 - System Clock Selection Value"]
    #[inline(always)]
    pub fn syssel(&mut self) -> SYSSEL_W {
        SYSSEL_W { w: self }
    }
    #[doc = "Bit 10 - CPU Clock Divider Enable"]
    #[inline(always)]
    pub fn cpudiv(&mut self) -> CPUDIV_W {
        CPUDIV_W { w: self }
    }
    #[doc = "Bit 12 - PB Clock Divider Enable"]
    #[inline(always)]
    pub fn pbdiv(&mut self) -> PBDIV_W {
        PBDIV_W { w: self }
    }
    #[doc = "Bit 14 - CCU Clock Divider Enable"]
    #[inline(always)]
    pub fn ccudiv(&mut self) -> CCUDIV_W {
        CCUDIV_W { w: self }
    }
    #[doc = "Bits 16:23 - WDT Clock Divider Value"]
    #[inline(always)]
    pub fn wdtdiv(&mut self) -> WDTDIV_W {
        WDTDIV_W { w: self }
    }
    #[doc = "Bits 24:25 - WDT Clock Selection Value"]
    #[inline(always)]
    pub fn wdtsel(&mut self) -> WDTSEL_W {
        WDTSEL_W { w: self }
    }
}
