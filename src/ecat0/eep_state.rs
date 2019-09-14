#[doc = "Reader of register EEP_STATE"]
pub type R = crate::R<u8, super::EEP_STATE>;
#[doc = "Writer for register EEP_STATE"]
pub type W = crate::W<u8, super::EEP_STATE>;
#[doc = "Register EEP_STATE `reset()`'s with value 0"]
impl crate::ResetValue for super::EEP_STATE {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Access to EEPROM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACCESS_A {
    #[doc = "0: PDI releases EEPROM access"]
    VALUE1,
    #[doc = "1: PDI takes EEPROM access (PDI has EEPROM control)"]
    VALUE2,
}
impl From<ACCESS_A> for bool {
    #[inline(always)]
    fn from(variant: ACCESS_A) -> Self {
        match variant {
            ACCESS_A::VALUE1 => false,
            ACCESS_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `ACCESS`"]
pub type ACCESS_R = crate::R<bool, ACCESS_A>;
impl ACCESS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACCESS_A {
        match self.bits {
            false => ACCESS_A::VALUE1,
            true => ACCESS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ACCESS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ACCESS_A::VALUE2
    }
}
#[doc = "Write proxy for field `ACCESS`"]
pub struct ACCESS_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCESS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACCESS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PDI releases EEPROM access"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ACCESS_A::VALUE1)
    }
    #[doc = "PDI takes EEPROM access (PDI has EEPROM control)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ACCESS_A::VALUE2)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Access to EEPROM"]
    #[inline(always)]
    pub fn access(&self) -> ACCESS_R {
        ACCESS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Access to EEPROM"]
    #[inline(always)]
    pub fn access(&mut self) -> ACCESS_W {
        ACCESS_W { w: self }
    }
}
