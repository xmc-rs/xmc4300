#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::EEP_CONF {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `TO_PDI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TO_PDIR {
    #[doc = "No"]
    VALUE1,
    #[doc = "Yes (PDI has EEPROM control)"]
    VALUE2,
}
impl TO_PDIR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TO_PDIR::VALUE1 => false,
            TO_PDIR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TO_PDIR {
        match value {
            false => TO_PDIR::VALUE1,
            true => TO_PDIR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TO_PDIR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TO_PDIR::VALUE2
    }
}
#[doc = "Possible values of the field `FORCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCER {
    #[doc = "Do not change Bit 501.0"]
    VALUE1,
    #[doc = "Reset Bit 501.0 to 0"]
    VALUE2,
}
impl FORCER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            FORCER::VALUE1 => false,
            FORCER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FORCER {
        match value {
            false => FORCER::VALUE1,
            true => FORCER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == FORCER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == FORCER::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - EEPROM control is offered to PDI"]
    #[inline]
    pub fn to_pdi(&self) -> TO_PDIR {
        TO_PDIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Force ECAT access"]
    #[inline]
    pub fn force(&self) -> FORCER {
        FORCER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
}
