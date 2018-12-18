#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CCFG {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `SSC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSCR {
    #[doc = "The SSC protocol is not available."]
    VALUE1,
    #[doc = "The SSC protocol is available."]
    VALUE2,
}
impl SSCR {
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
            SSCR::VALUE1 => false,
            SSCR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SSCR {
        match value {
            false => SSCR::VALUE1,
            true => SSCR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SSCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SSCR::VALUE2
    }
}
#[doc = "Possible values of the field `ASC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASCR {
    #[doc = "The ASC protocol is not available."]
    VALUE1,
    #[doc = "The ASC protocol is available."]
    VALUE2,
}
impl ASCR {
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
            ASCR::VALUE1 => false,
            ASCR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASCR {
        match value {
            false => ASCR::VALUE1,
            true => ASCR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ASCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ASCR::VALUE2
    }
}
#[doc = "Possible values of the field `IIC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IICR {
    #[doc = "The IIC protocol is not available."]
    VALUE1,
    #[doc = "The IIC protocol is available."]
    VALUE2,
}
impl IICR {
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
            IICR::VALUE1 => false,
            IICR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IICR {
        match value {
            false => IICR::VALUE1,
            true => IICR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == IICR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == IICR::VALUE2
    }
}
#[doc = "Possible values of the field `IIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IISR {
    #[doc = "The IIS protocol is not available."]
    VALUE1,
    #[doc = "The IIS protocol is available."]
    VALUE2,
}
impl IISR {
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
            IISR::VALUE1 => false,
            IISR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IISR {
        match value {
            false => IISR::VALUE1,
            true => IISR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == IISR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == IISR::VALUE2
    }
}
#[doc = "Possible values of the field `RB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBR {
    #[doc = "A receive FIFO buffer is not available."]
    VALUE1,
    #[doc = "A receive FIFO buffer is available."]
    VALUE2,
}
impl RBR {
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
            RBR::VALUE1 => false,
            RBR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RBR {
        match value {
            false => RBR::VALUE1,
            true => RBR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RBR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RBR::VALUE2
    }
}
#[doc = "Possible values of the field `TB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBR {
    #[doc = "A transmit FIFO buffer is not available."]
    VALUE1,
    #[doc = "A transmit FIFO buffer is available."]
    VALUE2,
}
impl TBR {
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
            TBR::VALUE1 => false,
            TBR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TBR {
        match value {
            false => TBR::VALUE1,
            true => TBR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TBR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TBR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - SSC Protocol Available"]
    #[inline]
    pub fn ssc(&self) -> SSCR {
        SSCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - ASC Protocol Available"]
    #[inline]
    pub fn asc(&self) -> ASCR {
        ASCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - IIC Protocol Available"]
    #[inline]
    pub fn iic(&self) -> IICR {
        IICR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - IIS Protocol Available"]
    #[inline]
    pub fn iis(&self) -> IISR {
        IISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Receive FIFO Buffer Available"]
    #[inline]
    pub fn rb(&self) -> RBR {
        RBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Transmit FIFO Buffer Available"]
    #[inline]
    pub fn tb(&self) -> TBR {
        TBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
