#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RSTSTAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `RSTSTAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTSTATR {
    #[doc = "PORST reset"]
    CONST_00000001,
    #[doc = "SWD reset"]
    CONST_00000010,
    #[doc = "PV reset"]
    CONST_00000100,
    #[doc = "CPU system reset"]
    CONST_00001000,
    #[doc = "CPU lockup reset"]
    CONST_00010000,
    #[doc = "WDT reset"]
    CONST_00100000,
    #[doc = "Parity Error reset"]
    CONST_10000000,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RSTSTATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RSTSTATR::CONST_00000001 => 1,
            RSTSTATR::CONST_00000010 => 2,
            RSTSTATR::CONST_00000100 => 4,
            RSTSTATR::CONST_00001000 => 8,
            RSTSTATR::CONST_00010000 => 16,
            RSTSTATR::CONST_00100000 => 32,
            RSTSTATR::CONST_10000000 => 128,
            RSTSTATR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RSTSTATR {
        match value {
            1 => RSTSTATR::CONST_00000001,
            2 => RSTSTATR::CONST_00000010,
            4 => RSTSTATR::CONST_00000100,
            8 => RSTSTATR::CONST_00001000,
            16 => RSTSTATR::CONST_00010000,
            32 => RSTSTATR::CONST_00100000,
            128 => RSTSTATR::CONST_10000000,
            i => RSTSTATR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00000001`"]
    #[inline]
    pub fn is_const_00000001(&self) -> bool {
        *self == RSTSTATR::CONST_00000001
    }
    #[doc = "Checks if the value of the field is `CONST_00000010`"]
    #[inline]
    pub fn is_const_00000010(&self) -> bool {
        *self == RSTSTATR::CONST_00000010
    }
    #[doc = "Checks if the value of the field is `CONST_00000100`"]
    #[inline]
    pub fn is_const_00000100(&self) -> bool {
        *self == RSTSTATR::CONST_00000100
    }
    #[doc = "Checks if the value of the field is `CONST_00001000`"]
    #[inline]
    pub fn is_const_00001000(&self) -> bool {
        *self == RSTSTATR::CONST_00001000
    }
    #[doc = "Checks if the value of the field is `CONST_00010000`"]
    #[inline]
    pub fn is_const_00010000(&self) -> bool {
        *self == RSTSTATR::CONST_00010000
    }
    #[doc = "Checks if the value of the field is `CONST_00100000`"]
    #[inline]
    pub fn is_const_00100000(&self) -> bool {
        *self == RSTSTATR::CONST_00100000
    }
    #[doc = "Checks if the value of the field is `CONST_10000000`"]
    #[inline]
    pub fn is_const_10000000(&self) -> bool {
        *self == RSTSTATR::CONST_10000000
    }
}
#[doc = "Possible values of the field `HIBWK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIBWKR {
    #[doc = "No Wake-up"]
    CONST_0,
    #[doc = "Wake-up event"]
    CONST_1,
}
impl HIBWKR {
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
            HIBWKR::CONST_0 => false,
            HIBWKR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HIBWKR {
        match value {
            false => HIBWKR::CONST_0,
            true => HIBWKR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == HIBWKR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == HIBWKR::CONST_1
    }
}
#[doc = "Possible values of the field `HIBRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIBRSR {
    #[doc = "Reset de-asserted"]
    CONST_0,
    #[doc = "Reset asserted"]
    CONST_1,
}
impl HIBRSR {
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
            HIBRSR::CONST_0 => false,
            HIBRSR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HIBRSR {
        match value {
            false => HIBRSR::CONST_0,
            true => HIBRSR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == HIBRSR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == HIBRSR::CONST_1
    }
}
#[doc = "Possible values of the field `LCKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCKENR {
    #[doc = "Reset by Lockup disabled"]
    CONST_0,
    #[doc = "Reset by Lockup enabled"]
    CONST_1,
}
impl LCKENR {
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
            LCKENR::CONST_0 => false,
            LCKENR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LCKENR {
        match value {
            false => LCKENR::CONST_0,
            true => LCKENR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == LCKENR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == LCKENR::CONST_1
    }
}
#[doc = "Possible values of the field `ECAT0RS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECAT0RSR {
    #[doc = "Reset did not occur"]
    CONST_0,
    #[doc = "Reset occurred"]
    CONST_1,
}
impl ECAT0RSR {
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
            ECAT0RSR::CONST_0 => false,
            ECAT0RSR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ECAT0RSR {
        match value {
            false => ECAT0RSR::CONST_0,
            true => ECAT0RSR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == ECAT0RSR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == ECAT0RSR::CONST_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Reset Status Information"]
    #[inline]
    pub fn rststat(&self) -> RSTSTATR {
        RSTSTATR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Hibernate Wake-up Status"]
    #[inline]
    pub fn hibwk(&self) -> HIBWKR {
        HIBWKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Hibernate Reset Status"]
    #[inline]
    pub fn hibrs(&self) -> HIBRSR {
        HIBRSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Enable Lockup Status"]
    #[inline]
    pub fn lcken(&self) -> LCKENR {
        LCKENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - ECAT0 Reset Status Information"]
    #[inline]
    pub fn ecat0rs(&self) -> ECAT0RSR {
        ECAT0RSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
