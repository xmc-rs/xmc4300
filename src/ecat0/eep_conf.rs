#[doc = "Reader of register EEP_CONF"]
pub type R = crate::R<u8, super::EEP_CONF>;
#[doc = "EEPROM control is offered to PDI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TO_PDI_A {
    #[doc = "0: No"]
    VALUE1,
    #[doc = "1: Yes (PDI has EEPROM control)"]
    VALUE2,
}
impl From<TO_PDI_A> for bool {
    #[inline(always)]
    fn from(variant: TO_PDI_A) -> Self {
        match variant {
            TO_PDI_A::VALUE1 => false,
            TO_PDI_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `TO_PDI`"]
pub type TO_PDI_R = crate::R<bool, TO_PDI_A>;
impl TO_PDI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TO_PDI_A {
        match self.bits {
            false => TO_PDI_A::VALUE1,
            true => TO_PDI_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TO_PDI_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TO_PDI_A::VALUE2
    }
}
#[doc = "Force ECAT access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCE_A {
    #[doc = "0: Do not change Bit 501.0"]
    VALUE1,
    #[doc = "1: Reset Bit 501.0 to 0"]
    VALUE2,
}
impl From<FORCE_A> for bool {
    #[inline(always)]
    fn from(variant: FORCE_A) -> Self {
        match variant {
            FORCE_A::VALUE1 => false,
            FORCE_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `FORCE`"]
pub type FORCE_R = crate::R<bool, FORCE_A>;
impl FORCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCE_A {
        match self.bits {
            false => FORCE_A::VALUE1,
            true => FORCE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FORCE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FORCE_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - EEPROM control is offered to PDI"]
    #[inline(always)]
    pub fn to_pdi(&self) -> TO_PDI_R {
        TO_PDI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Force ECAT access"]
    #[inline(always)]
    pub fn force(&self) -> FORCE_R {
        FORCE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
