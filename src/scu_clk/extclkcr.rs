#[doc = "Reader of register EXTCLKCR"]
pub type R = crate::R<u32, super::EXTCLKCR>;
#[doc = "Writer for register EXTCLKCR"]
pub type W = crate::W<u32, super::EXTCLKCR>;
#[doc = "Register EXTCLKCR `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTCLKCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "External Clock Selection Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECKSEL_A {
    #[doc = "0: fSYS clock"]
    CONST_00,
    #[doc = "2: fUSB clock divided according to ECKDIV bit field configuration"]
    CONST_10,
    #[doc = "3: fPLL clock divided according to ECKDIV bit field configuration"]
    CONST_11,
}
impl From<ECKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ECKSEL_A) -> Self {
        match variant {
            ECKSEL_A::CONST_00 => 0,
            ECKSEL_A::CONST_10 => 2,
            ECKSEL_A::CONST_11 => 3,
        }
    }
}
#[doc = "Reader of field `ECKSEL`"]
pub type ECKSEL_R = crate::R<u8, ECKSEL_A>;
impl ECKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ECKSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ECKSEL_A::CONST_00),
            2 => Val(ECKSEL_A::CONST_10),
            3 => Val(ECKSEL_A::CONST_11),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == ECKSEL_A::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == ECKSEL_A::CONST_10
    }
    #[doc = "Checks if the value of the field is `CONST_11`"]
    #[inline(always)]
    pub fn is_const_11(&self) -> bool {
        *self == ECKSEL_A::CONST_11
    }
}
#[doc = "Write proxy for field `ECKSEL`"]
pub struct ECKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ECKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECKSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "fSYS clock"]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut W {
        self.variant(ECKSEL_A::CONST_00)
    }
    #[doc = "fUSB clock divided according to ECKDIV bit field configuration"]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut W {
        self.variant(ECKSEL_A::CONST_10)
    }
    #[doc = "fPLL clock divided according to ECKDIV bit field configuration"]
    #[inline(always)]
    pub fn const_11(self) -> &'a mut W {
        self.variant(ECKSEL_A::CONST_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `ECKDIV`"]
pub type ECKDIV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ECKDIV`"]
pub struct ECKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ECKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | (((value as u32) & 0x01ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - External Clock Selection Value"]
    #[inline(always)]
    pub fn ecksel(&self) -> ECKSEL_R {
        ECKSEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 16:24 - External Clock Divider Value"]
    #[inline(always)]
    pub fn eckdiv(&self) -> ECKDIV_R {
        ECKDIV_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - External Clock Selection Value"]
    #[inline(always)]
    pub fn ecksel(&mut self) -> ECKSEL_W {
        ECKSEL_W { w: self }
    }
    #[doc = "Bits 16:24 - External Clock Divider Value"]
    #[inline(always)]
    pub fn eckdiv(&mut self) -> ECKDIV_W {
        ECKDIV_W { w: self }
    }
}
