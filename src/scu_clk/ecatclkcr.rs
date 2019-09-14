#[doc = "Reader of register ECATCLKCR"]
pub type R = crate::R<u32, super::ECATCLKCR>;
#[doc = "Writer for register ECATCLKCR"]
pub type W = crate::W<u32, super::ECATCLKCR>;
#[doc = "Register ECATCLKCR `reset()`'s with value 0"]
impl crate::ResetValue for super::ECATCLKCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ECADIV`"]
pub type ECADIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ECADIV`"]
pub struct ECADIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ECADIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "EtherCAT Clock Selection Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECATSEL_A {
    #[doc = "0: fPLLUSB clock"]
    CONST_0,
    #[doc = "1: fPLL clock"]
    CONST_1,
}
impl From<ECATSEL_A> for bool {
    #[inline(always)]
    fn from(variant: ECATSEL_A) -> Self {
        match variant {
            ECATSEL_A::CONST_0 => false,
            ECATSEL_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `ECATSEL`"]
pub type ECATSEL_R = crate::R<bool, ECATSEL_A>;
impl ECATSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECATSEL_A {
        match self.bits {
            false => ECATSEL_A::CONST_0,
            true => ECATSEL_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == ECATSEL_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == ECATSEL_A::CONST_1
    }
}
#[doc = "Write proxy for field `ECATSEL`"]
pub struct ECATSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ECATSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECATSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "fPLLUSB clock"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ECATSEL_A::CONST_0)
    }
    #[doc = "fPLL clock"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ECATSEL_A::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - EtherCAT Clock Divider Value"]
    #[inline(always)]
    pub fn ecadiv(&self) -> ECADIV_R {
        ECADIV_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 16 - EtherCAT Clock Selection Value"]
    #[inline(always)]
    pub fn ecatsel(&self) -> ECATSEL_R {
        ECATSEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - EtherCAT Clock Divider Value"]
    #[inline(always)]
    pub fn ecadiv(&mut self) -> ECADIV_W {
        ECADIV_W { w: self }
    }
    #[doc = "Bit 16 - EtherCAT Clock Selection Value"]
    #[inline(always)]
    pub fn ecatsel(&mut self) -> ECATSEL_W {
        ECATSEL_W { w: self }
    }
}
