#[doc = "Reader of register MII_PDI_ACS_STATE"]
pub type R = crate::R<u8, super::MII_PDI_ACS_STATE>;
#[doc = "Writer for register MII_PDI_ACS_STATE"]
pub type W = crate::W<u8, super::MII_PDI_ACS_STATE>;
#[doc = "Register MII_PDI_ACS_STATE `reset()`'s with value 0"]
impl crate::ResetValue for super::MII_PDI_ACS_STATE {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Access to MII management\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACS_MII_BY_PDI_A {
    #[doc = "0: ECAT has access to MII managment"]
    VALUE1 = 0,
    #[doc = "1: PDI has access to MII managment"]
    VALUE2 = 1,
}
impl From<ACS_MII_BY_PDI_A> for bool {
    #[inline(always)]
    fn from(variant: ACS_MII_BY_PDI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACS_MII_BY_PDI`"]
pub type ACS_MII_BY_PDI_R = crate::R<bool, ACS_MII_BY_PDI_A>;
impl ACS_MII_BY_PDI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACS_MII_BY_PDI_A {
        match self.bits {
            false => ACS_MII_BY_PDI_A::VALUE1,
            true => ACS_MII_BY_PDI_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ACS_MII_BY_PDI_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ACS_MII_BY_PDI_A::VALUE2
    }
}
#[doc = "Write proxy for field `ACS_MII_BY_PDI`"]
pub struct ACS_MII_BY_PDI_W<'a> {
    w: &'a mut W,
}
impl<'a> ACS_MII_BY_PDI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACS_MII_BY_PDI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ECAT has access to MII managment"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ACS_MII_BY_PDI_A::VALUE1)
    }
    #[doc = "PDI has access to MII managment"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ACS_MII_BY_PDI_A::VALUE2)
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
#[doc = "Force PDI Access State by ECAT master\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCE_PDI_ACS_S_A {
    #[doc = "0: no change"]
    VALUE1 = 0,
    #[doc = "1: Reset Bit ACS_MII_BY_PDI"]
    VALUE2 = 1,
}
impl From<FORCE_PDI_ACS_S_A> for bool {
    #[inline(always)]
    fn from(variant: FORCE_PDI_ACS_S_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FORCE_PDI_ACS_S`"]
pub type FORCE_PDI_ACS_S_R = crate::R<bool, FORCE_PDI_ACS_S_A>;
impl FORCE_PDI_ACS_S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCE_PDI_ACS_S_A {
        match self.bits {
            false => FORCE_PDI_ACS_S_A::VALUE1,
            true => FORCE_PDI_ACS_S_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FORCE_PDI_ACS_S_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FORCE_PDI_ACS_S_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Access to MII management"]
    #[inline(always)]
    pub fn acs_mii_by_pdi(&self) -> ACS_MII_BY_PDI_R {
        ACS_MII_BY_PDI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Force PDI Access State by ECAT master"]
    #[inline(always)]
    pub fn force_pdi_acs_s(&self) -> FORCE_PDI_ACS_S_R {
        FORCE_PDI_ACS_S_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Access to MII management"]
    #[inline(always)]
    pub fn acs_mii_by_pdi(&mut self) -> ACS_MII_BY_PDI_W {
        ACS_MII_BY_PDI_W { w: self }
    }
}
