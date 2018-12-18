#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::QSR0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `FILL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILLR {
    #[doc = "There is 1 ( if EMPTY = 0) or no (if EMPTY = 1) valid entry in the queue"]
    VALUE1,
    #[doc = "There are 2 valid entries in the queue"]
    VALUE2,
    #[doc = "There are 3 valid entries in the queue"]
    VALUE3,
    #[doc = "There are 8 valid entries in the queue"]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FILLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FILLR::VALUE1 => 0,
            FILLR::VALUE2 => 1,
            FILLR::VALUE3 => 2,
            FILLR::VALUE4 => 7,
            FILLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FILLR {
        match value {
            0 => FILLR::VALUE1,
            1 => FILLR::VALUE2,
            2 => FILLR::VALUE3,
            7 => FILLR::VALUE4,
            i => FILLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == FILLR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == FILLR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == FILLR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == FILLR::VALUE4
    }
}
#[doc = "Possible values of the field `EMPTY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMPTYR {
    #[doc = "There are valid entries in the queue (see FILL)"]
    VALUE1,
    #[doc = "No valid entries (queue is empty)"]
    VALUE2,
}
impl EMPTYR {
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
            EMPTYR::VALUE1 => false,
            EMPTYR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EMPTYR {
        match value {
            false => EMPTYR::VALUE1,
            true => EMPTYR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EMPTYR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EMPTYR::VALUE2
    }
}
#[doc = "Possible values of the field `REQGT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REQGTR {
    #[doc = "The gate input is low"]
    VALUE1,
    #[doc = "The gate input is high"]
    VALUE2,
}
impl REQGTR {
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
            REQGTR::VALUE1 => false,
            REQGTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REQGTR {
        match value {
            false => REQGTR::VALUE1,
            true => REQGTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REQGTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == REQGTR::VALUE2
    }
}
#[doc = "Possible values of the field `EV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVR {
    #[doc = "No trigger event"]
    VALUE1,
    #[doc = "A trigger event has been detected"]
    VALUE2,
}
impl EVR {
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
            EVR::VALUE1 => false,
            EVR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EVR {
        match value {
            false => EVR::VALUE1,
            true => EVR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EVR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EVR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Filling Level for Queue 2"]
    #[inline]
    pub fn fill(&self) -> FILLR {
        FILLR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 5 - Queue Empty"]
    #[inline]
    pub fn empty(&self) -> EMPTYR {
        EMPTYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Request Gate Level"]
    #[inline]
    pub fn reqgt(&self) -> REQGTR {
        REQGTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Event Detected"]
    #[inline]
    pub fn ev(&self) -> EVR {
        EVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
