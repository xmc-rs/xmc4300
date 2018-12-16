#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PRSTAT1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `LEDTSCU0RS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEDTSCU0RSR {
    #[doc = "Reset de-asserted"]
    CONST_0,
    #[doc = "Reset asserted"]
    CONST_1,
}
impl LEDTSCU0RSR {
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
            LEDTSCU0RSR::CONST_0 => false,
            LEDTSCU0RSR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LEDTSCU0RSR {
        match value {
            false => LEDTSCU0RSR::CONST_0,
            true => LEDTSCU0RSR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == LEDTSCU0RSR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == LEDTSCU0RSR::CONST_1
    }
}
#[doc = "Possible values of the field `MCAN0RS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCAN0RSR {
    #[doc = "Reset de-asserted"]
    CONST_0,
    #[doc = "Reset asserted"]
    CONST_1,
}
impl MCAN0RSR {
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
            MCAN0RSR::CONST_0 => false,
            MCAN0RSR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MCAN0RSR {
        match value {
            false => MCAN0RSR::CONST_0,
            true => MCAN0RSR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == MCAN0RSR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == MCAN0RSR::CONST_1
    }
}
#[doc = "Possible values of the field `DACRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACRSR {
    #[doc = "Reset de-asserted"]
    CONST_0,
    #[doc = "Reset asserted"]
    CONST_1,
}
impl DACRSR {
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
            DACRSR::CONST_0 => false,
            DACRSR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DACRSR {
        match value {
            false => DACRSR::CONST_0,
            true => DACRSR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == DACRSR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == DACRSR::CONST_1
    }
}
#[doc = "Possible values of the field `MMCIRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMCIRSR {
    #[doc = "Reset de-asserted"]
    CONST_0,
    #[doc = "Reset asserted"]
    CONST_1,
}
impl MMCIRSR {
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
            MMCIRSR::CONST_0 => false,
            MMCIRSR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MMCIRSR {
        match value {
            false => MMCIRSR::CONST_0,
            true => MMCIRSR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == MMCIRSR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == MMCIRSR::CONST_1
    }
}
#[doc = "Possible values of the field `USIC1RS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USIC1RSR {
    #[doc = "Reset de-asserted"]
    CONST_0,
    #[doc = "Reset asserted"]
    CONST_1,
}
impl USIC1RSR {
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
            USIC1RSR::CONST_0 => false,
            USIC1RSR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USIC1RSR {
        match value {
            false => USIC1RSR::CONST_0,
            true => USIC1RSR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == USIC1RSR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == USIC1RSR::CONST_1
    }
}
#[doc = "Possible values of the field `PPORTSRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPORTSRSR {
    #[doc = "Reset de-asserted"]
    CONST_0,
    #[doc = "Reset asserted"]
    CONST_1,
}
impl PPORTSRSR {
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
            PPORTSRSR::CONST_0 => false,
            PPORTSRSR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PPORTSRSR {
        match value {
            false => PPORTSRSR::CONST_0,
            true => PPORTSRSR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PPORTSRSR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PPORTSRSR::CONST_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 3 - LEDTS Reset Status"]
    #[inline]
    pub fn ledtscu0rs(&self) -> LEDTSCU0RSR {
        LEDTSCU0RSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - MultiCAN Reset Status"]
    #[inline]
    pub fn mcan0rs(&self) -> MCAN0RSR {
        MCAN0RSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - DAC Reset Status"]
    #[inline]
    pub fn dacrs(&self) -> DACRSR {
        DACRSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - MMC Interface Reset Status"]
    #[inline]
    pub fn mmcirs(&self) -> MMCIRSR {
        MMCIRSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - USIC1 Reset Status"]
    #[inline]
    pub fn usic1rs(&self) -> USIC1RSR {
        USIC1RSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - PORTS Reset Status"]
    #[inline]
    pub fn pportsrs(&self) -> PPORTSRSR {
        PPORTSRSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
