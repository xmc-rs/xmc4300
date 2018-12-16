#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::ESC_CONFIG {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `EMUL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMULR {
    #[doc = "AL status register has to be set by PDI"]
    VALUE1,
    #[doc = "AL status register will be set to value written to AL control register"]
    VALUE2,
}
impl EMULR {
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
            EMULR::VALUE1 => false,
            EMULR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EMULR {
        match value {
            false => EMULR::VALUE1,
            true => EMULR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EMULR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EMULR::VALUE2
    }
}
#[doc = "Possible values of the field `EHLD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EHLDR {
    #[doc = "disabled (if bits \\[7:4\\]=0)"]
    VALUE1,
    #[doc = "enabled at all ports (overrides bits \\[7:4\\])"]
    VALUE2,
}
impl EHLDR {
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
            EHLDR::VALUE1 => false,
            EHLDR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EHLDR {
        match value {
            false => EHLDR::VALUE1,
            true => EHLDR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EHLDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EHLDR::VALUE2
    }
}
#[doc = "Possible values of the field `CLKS_OUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKS_OUTR {
    #[doc = "disabled (power saving)"]
    VALUE1,
    #[doc = "enabled"]
    VALUE2,
}
impl CLKS_OUTR {
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
            CLKS_OUTR::VALUE1 => false,
            CLKS_OUTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLKS_OUTR {
        match value {
            false => CLKS_OUTR::VALUE1,
            true => CLKS_OUTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CLKS_OUTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CLKS_OUTR::VALUE2
    }
}
#[doc = "Possible values of the field `CLKS_IN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKS_INR {
    #[doc = "disabled (power saving)"]
    VALUE1,
    #[doc = "enabled"]
    VALUE2,
}
impl CLKS_INR {
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
            CLKS_INR::VALUE1 => false,
            CLKS_INR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLKS_INR {
        match value {
            false => CLKS_INR::VALUE1,
            true => CLKS_INR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CLKS_INR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CLKS_INR::VALUE2
    }
}
#[doc = "Possible values of the field `EHLD_P0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EHLD_P0R {
    #[doc = "disabled (if bit 1 = 0)"]
    VALUE1,
    #[doc = "enabled"]
    VALUE2,
}
impl EHLD_P0R {
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
            EHLD_P0R::VALUE1 => false,
            EHLD_P0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EHLD_P0R {
        match value {
            false => EHLD_P0R::VALUE1,
            true => EHLD_P0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EHLD_P0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EHLD_P0R::VALUE2
    }
}
#[doc = "Possible values of the field `EHLD_P1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EHLD_P1R {
    #[doc = "disabled (if bit 1 = 0)"]
    VALUE1,
    #[doc = "enabled"]
    VALUE2,
}
impl EHLD_P1R {
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
            EHLD_P1R::VALUE1 => false,
            EHLD_P1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EHLD_P1R {
        match value {
            false => EHLD_P1R::VALUE1,
            true => EHLD_P1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EHLD_P1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EHLD_P1R::VALUE2
    }
}
#[doc = "Possible values of the field `EHLD_P2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EHLD_P2R {
    #[doc = "disabled (if bit 1 = 0)"]
    VALUE1,
    #[doc = "enabled"]
    VALUE2,
}
impl EHLD_P2R {
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
            EHLD_P2R::VALUE1 => false,
            EHLD_P2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EHLD_P2R {
        match value {
            false => EHLD_P2R::VALUE1,
            true => EHLD_P2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EHLD_P2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EHLD_P2R::VALUE2
    }
}
#[doc = "Possible values of the field `EHLD_P3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EHLD_P3R {
    #[doc = "disabled (if bit 1 = 0)"]
    VALUE1,
    #[doc = "enabled"]
    VALUE2,
}
impl EHLD_P3R {
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
            EHLD_P3R::VALUE1 => false,
            EHLD_P3R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EHLD_P3R {
        match value {
            false => EHLD_P3R::VALUE1,
            true => EHLD_P3R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EHLD_P3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EHLD_P3R::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Device emulation (control of AL status)"]
    #[inline]
    pub fn emul(&self) -> EMULR {
        EMULR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Enhanced Link detection all ports"]
    #[inline]
    pub fn ehld(&self) -> EHLDR {
        EHLDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - Distributed Clocks SYNC Out Unit"]
    #[inline]
    pub fn clks_out(&self) -> CLKS_OUTR {
        CLKS_OUTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - Distributed Clocks Latch In Unit"]
    #[inline]
    pub fn clks_in(&self) -> CLKS_INR {
        CLKS_INR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 4 - Enhanced Link port 0"]
    #[inline]
    pub fn ehld_p0(&self) -> EHLD_P0R {
        EHLD_P0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - Enhanced Link port 1"]
    #[inline]
    pub fn ehld_p1(&self) -> EHLD_P1R {
        EHLD_P1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - Enhanced Link port 2"]
    #[inline]
    pub fn ehld_p2(&self) -> EHLD_P2R {
        EHLD_P2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - Enhanced Link port 3"]
    #[inline]
    pub fn ehld_p3(&self) -> EHLD_P3R {
        EHLD_P3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
}
