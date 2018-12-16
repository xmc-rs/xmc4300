#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CLKSTAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `USBCST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBCSTR {
    #[doc = "Clock disabled"]
    CONST_0,
    #[doc = "Clock enabled"]
    CONST_1,
}
impl USBCSTR {
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
            USBCSTR::CONST_0 => false,
            USBCSTR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USBCSTR {
        match value {
            false => USBCSTR::CONST_0,
            true => USBCSTR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == USBCSTR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == USBCSTR::CONST_1
    }
}
#[doc = "Possible values of the field `MMCCST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMCCSTR {
    #[doc = "Clock disabled"]
    CONST_0,
    #[doc = "Clock enabled"]
    CONST_1,
}
impl MMCCSTR {
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
            MMCCSTR::CONST_0 => false,
            MMCCSTR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MMCCSTR {
        match value {
            false => MMCCSTR::CONST_0,
            true => MMCCSTR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == MMCCSTR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == MMCCSTR::CONST_1
    }
}
#[doc = "Possible values of the field `ETH0CST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETH0CSTR {
    #[doc = "Clock disabled"]
    CONST_0,
    #[doc = "Clock enabled"]
    CONST_1,
}
impl ETH0CSTR {
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
            ETH0CSTR::CONST_0 => false,
            ETH0CSTR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ETH0CSTR {
        match value {
            false => ETH0CSTR::CONST_0,
            true => ETH0CSTR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == ETH0CSTR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == ETH0CSTR::CONST_1
    }
}
#[doc = "Possible values of the field `CCUCST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCUCSTR {
    #[doc = "Clock disabled"]
    CONST_0,
    #[doc = "Clock enabled"]
    CONST_1,
}
impl CCUCSTR {
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
            CCUCSTR::CONST_0 => false,
            CCUCSTR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCUCSTR {
        match value {
            false => CCUCSTR::CONST_0,
            true => CCUCSTR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == CCUCSTR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == CCUCSTR::CONST_1
    }
}
#[doc = "Possible values of the field `WDTCST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTCSTR {
    #[doc = "Clock disabled"]
    CONST_0,
    #[doc = "Clock enabled"]
    CONST_1,
}
impl WDTCSTR {
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
            WDTCSTR::CONST_0 => false,
            WDTCSTR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WDTCSTR {
        match value {
            false => WDTCSTR::CONST_0,
            true => WDTCSTR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == WDTCSTR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == WDTCSTR::CONST_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - USB Clock Status"]
    #[inline]
    pub fn usbcst(&self) -> USBCSTR {
        USBCSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - MMC Clock Status"]
    #[inline]
    pub fn mmccst(&self) -> MMCCSTR {
        MMCCSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Ethernet Clock Status"]
    #[inline]
    pub fn eth0cst(&self) -> ETH0CSTR {
        ETH0CSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - CCU Clock Status"]
    #[inline]
    pub fn ccucst(&self) -> CCUCSTR {
        CCUCSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - WDT Clock Status"]
    #[inline]
    pub fn wdtcst(&self) -> WDTCSTR {
        WDTCSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
