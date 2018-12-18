#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
impl super::AL_CONTROL {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `IST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISTR {
    #[doc = "Request Init State"]
    VALUE1,
    #[doc = "Request Pre-Operational State"]
    VALUE2,
    #[doc = "Request Bootstrap State"]
    VALUE3,
    #[doc = "Request Safe-Operational State"]
    VALUE4,
    #[doc = "Request Operational State"]
    VALUE5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ISTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ISTR::VALUE1 => 1,
            ISTR::VALUE2 => 2,
            ISTR::VALUE3 => 3,
            ISTR::VALUE4 => 4,
            ISTR::VALUE5 => 8,
            ISTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ISTR {
        match value {
            1 => ISTR::VALUE1,
            2 => ISTR::VALUE2,
            3 => ISTR::VALUE3,
            4 => ISTR::VALUE4,
            8 => ISTR::VALUE5,
            i => ISTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ISTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ISTR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == ISTR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == ISTR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == ISTR::VALUE5
    }
}
#[doc = "Possible values of the field `EIA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EIAR {
    #[doc = "No Ack of Error Ind in AL status register"]
    VALUE1,
    #[doc = "Ack of Error Ind in AL status register"]
    VALUE2,
}
impl EIAR {
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
            EIAR::VALUE1 => false,
            EIAR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EIAR {
        match value {
            false => EIAR::VALUE1,
            true => EIAR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EIAR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EIAR::VALUE2
    }
}
#[doc = "Possible values of the field `DID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIDR {
    #[doc = "No request"]
    VALUE1,
    #[doc = "Device Identification request"]
    VALUE2,
}
impl DIDR {
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
            DIDR::VALUE1 => false,
            DIDR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIDR {
        match value {
            false => DIDR::VALUE1,
            true => DIDR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DIDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DIDR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:3 - Initiate State Transition of the Device StateMachine"]
    #[inline]
    pub fn ist(&self) -> ISTR {
        ISTR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 4 - Error Ind Ack"]
    #[inline]
    pub fn eia(&self) -> EIAR {
        EIAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 5 - Device Identification"]
    #[inline]
    pub fn did(&self) -> DIDR {
        DIDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
}
