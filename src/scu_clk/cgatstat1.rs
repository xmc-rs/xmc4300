#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CGATSTAT1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `LEDTSCU0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEDTSCU0R {
    #[doc = "Gating de-asserted"]
    VALUE1,
    #[doc = "Gating asserted"]
    VALUE2,
}
impl LEDTSCU0R {
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
            LEDTSCU0R::VALUE1 => false,
            LEDTSCU0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LEDTSCU0R {
        match value {
            false => LEDTSCU0R::VALUE1,
            true => LEDTSCU0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LEDTSCU0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LEDTSCU0R::VALUE2
    }
}
#[doc = "Possible values of the field `MCAN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCAN0R {
    #[doc = "Gating de-asserted"]
    VALUE1,
    #[doc = "Gating asserted"]
    VALUE2,
}
impl MCAN0R {
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
            MCAN0R::VALUE1 => false,
            MCAN0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MCAN0R {
        match value {
            false => MCAN0R::VALUE1,
            true => MCAN0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MCAN0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MCAN0R::VALUE2
    }
}
#[doc = "Possible values of the field `DAC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACR {
    #[doc = "Gating de-asserted"]
    VALUE1,
    #[doc = "Gating asserted"]
    VALUE2,
}
impl DACR {
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
            DACR::VALUE1 => false,
            DACR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DACR {
        match value {
            false => DACR::VALUE1,
            true => DACR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DACR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DACR::VALUE2
    }
}
#[doc = "Possible values of the field `MMCI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMCIR {
    #[doc = "Gating de-asserted"]
    VALUE1,
    #[doc = "Gating asserted"]
    VALUE2,
}
impl MMCIR {
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
            MMCIR::VALUE1 => false,
            MMCIR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MMCIR {
        match value {
            false => MMCIR::VALUE1,
            true => MMCIR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MMCIR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MMCIR::VALUE2
    }
}
#[doc = "Possible values of the field `USIC1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USIC1R {
    #[doc = "Gating de-asserted"]
    VALUE1,
    #[doc = "Gating asserted"]
    VALUE2,
}
impl USIC1R {
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
            USIC1R::VALUE1 => false,
            USIC1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USIC1R {
        match value {
            false => USIC1R::VALUE1,
            true => USIC1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == USIC1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == USIC1R::VALUE2
    }
}
#[doc = "Possible values of the field `PPORTS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPORTSR {
    #[doc = "Gating de-asserted"]
    VALUE1,
    #[doc = "Gating asserted"]
    VALUE2,
}
impl PPORTSR {
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
            PPORTSR::VALUE1 => false,
            PPORTSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PPORTSR {
        match value {
            false => PPORTSR::VALUE1,
            true => PPORTSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PPORTSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PPORTSR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 3 - LEDTS Gating Status"]
    #[inline]
    pub fn ledtscu0(&self) -> LEDTSCU0R {
        LEDTSCU0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - MultiCAN Gating Status"]
    #[inline]
    pub fn mcan0(&self) -> MCAN0R {
        MCAN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - DAC Gating Status"]
    #[inline]
    pub fn dac(&self) -> DACR {
        DACR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - MMC Interface Gating Status"]
    #[inline]
    pub fn mmci(&self) -> MMCIR {
        MMCIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - USIC1 Gating Status"]
    #[inline]
    pub fn usic1(&self) -> USIC1R {
        USIC1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - PORTS Gating Status"]
    #[inline]
    pub fn pports(&self) -> PPORTSR {
        PPORTSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
