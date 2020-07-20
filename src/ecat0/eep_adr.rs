#[doc = "Reader of register EEP_ADR"]
pub type R = crate::R<u32, super::EEP_ADR>;
#[doc = "Writer for register EEP_ADR"]
pub type W = crate::W<u32, super::EEP_ADR>;
#[doc = "Register EEP_ADR `reset()`'s with value 0"]
impl crate::ResetValue for super::EEP_ADR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "EEPROM Address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum EEPROM_ADDR_A {
    #[doc = "0: First word (= 16 bits)"]
    VALUE1 = 0,
    #[doc = "1: Second word"]
    VALUE2 = 1,
}
impl From<EEPROM_ADDR_A> for u32 {
    #[inline(always)]
    fn from(variant: EEPROM_ADDR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EEPROM_ADDR`"]
pub type EEPROM_ADDR_R = crate::R<u32, EEPROM_ADDR_A>;
impl EEPROM_ADDR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, EEPROM_ADDR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EEPROM_ADDR_A::VALUE1),
            1 => Val(EEPROM_ADDR_A::VALUE2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EEPROM_ADDR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EEPROM_ADDR_A::VALUE2
    }
}
#[doc = "Write proxy for field `EEPROM_ADDR`"]
pub struct EEPROM_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> EEPROM_ADDR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEPROM_ADDR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "First word (= 16 bits)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EEPROM_ADDR_A::VALUE1)
    }
    #[doc = "Second word"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EEPROM_ADDR_A::VALUE2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - EEPROM Address"]
    #[inline(always)]
    pub fn eeprom_addr(&self) -> EEPROM_ADDR_R {
        EEPROM_ADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - EEPROM Address"]
    #[inline(always)]
    pub fn eeprom_addr(&mut self) -> EEPROM_ADDR_W {
        EEPROM_ADDR_W { w: self }
    }
}
