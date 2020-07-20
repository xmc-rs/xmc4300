#[doc = "Reader of register WDTCLKCR"]
pub type R = crate::R<u32, super::WDTCLKCR>;
#[doc = "Writer for register WDTCLKCR"]
pub type W = crate::W<u32, super::WDTCLKCR>;
#[doc = "Register WDTCLKCR `reset()`'s with value 0"]
impl crate::ResetValue for super::WDTCLKCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
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
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - WDT Clock Divider Value"]
    #[inline(always)]
    pub fn wdtdiv(&self) -> WDTDIV_R {
        WDTDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - WDT Clock Selection Value"]
    #[inline(always)]
    pub fn wdtsel(&self) -> WDTSEL_R {
        WDTSEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - WDT Clock Divider Value"]
    #[inline(always)]
    pub fn wdtdiv(&mut self) -> WDTDIV_W {
        WDTDIV_W { w: self }
    }
    #[doc = "Bits 16:17 - WDT Clock Selection Value"]
    #[inline(always)]
    pub fn wdtsel(&mut self) -> WDTSEL_W {
        WDTSEL_W { w: self }
    }
}
