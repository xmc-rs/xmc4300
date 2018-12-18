#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PROCON1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `S0L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0LR {
    #[doc = "No write protection is configured for sector n."]
    CONST_0,
    #[doc = "Write protection is configured for sector n."]
    CONST_1,
}
impl S0LR {
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
            S0LR::CONST_0 => false,
            S0LR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S0LR {
        match value {
            false => S0LR::CONST_0,
            true => S0LR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == S0LR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == S0LR::CONST_1
    }
}
#[doc = "Possible values of the field `S1L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1LR {
    #[doc = "No write protection is configured for sector n."]
    CONST_0,
    #[doc = "Write protection is configured for sector n."]
    CONST_1,
}
impl S1LR {
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
            S1LR::CONST_0 => false,
            S1LR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S1LR {
        match value {
            false => S1LR::CONST_0,
            true => S1LR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == S1LR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == S1LR::CONST_1
    }
}
#[doc = "Possible values of the field `S2L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S2LR {
    #[doc = "No write protection is configured for sector n."]
    CONST_0,
    #[doc = "Write protection is configured for sector n."]
    CONST_1,
}
impl S2LR {
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
            S2LR::CONST_0 => false,
            S2LR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S2LR {
        match value {
            false => S2LR::CONST_0,
            true => S2LR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == S2LR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == S2LR::CONST_1
    }
}
#[doc = "Possible values of the field `S3L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S3LR {
    #[doc = "No write protection is configured for sector n."]
    CONST_0,
    #[doc = "Write protection is configured for sector n."]
    CONST_1,
}
impl S3LR {
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
            S3LR::CONST_0 => false,
            S3LR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S3LR {
        match value {
            false => S3LR::CONST_0,
            true => S3LR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == S3LR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == S3LR::CONST_1
    }
}
#[doc = "Possible values of the field `S4L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S4LR {
    #[doc = "No write protection is configured for sector n."]
    CONST_0,
    #[doc = "Write protection is configured for sector n."]
    CONST_1,
}
impl S4LR {
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
            S4LR::CONST_0 => false,
            S4LR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S4LR {
        match value {
            false => S4LR::CONST_0,
            true => S4LR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == S4LR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == S4LR::CONST_1
    }
}
#[doc = "Possible values of the field `S5L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S5LR {
    #[doc = "No write protection is configured for sector n."]
    CONST_0,
    #[doc = "Write protection is configured for sector n."]
    CONST_1,
}
impl S5LR {
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
            S5LR::CONST_0 => false,
            S5LR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S5LR {
        match value {
            false => S5LR::CONST_0,
            true => S5LR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == S5LR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == S5LR::CONST_1
    }
}
#[doc = "Possible values of the field `S6L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S6LR {
    #[doc = "No write protection is configured for sector n."]
    CONST_0,
    #[doc = "Write protection is configured for sector n."]
    CONST_1,
}
impl S6LR {
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
            S6LR::CONST_0 => false,
            S6LR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S6LR {
        match value {
            false => S6LR::CONST_0,
            true => S6LR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == S6LR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == S6LR::CONST_1
    }
}
#[doc = "Possible values of the field `S7L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S7LR {
    #[doc = "No write protection is configured for sector n."]
    CONST_0,
    #[doc = "Write protection is configured for sector n."]
    CONST_1,
}
impl S7LR {
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
            S7LR::CONST_0 => false,
            S7LR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S7LR {
        match value {
            false => S7LR::CONST_0,
            true => S7LR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == S7LR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == S7LR::CONST_1
    }
}
#[doc = "Possible values of the field `S8L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S8LR {
    #[doc = "No write protection is configured for sector n."]
    CONST_0,
    #[doc = "Write protection is configured for sector n."]
    CONST_1,
}
impl S8LR {
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
            S8LR::CONST_0 => false,
            S8LR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S8LR {
        match value {
            false => S8LR::CONST_0,
            true => S8LR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == S8LR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == S8LR::CONST_1
    }
}
#[doc = "Possible values of the field `PSR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSRR {
    #[doc = "Physical Sector Repair command disabled; Erase Physical Sector command sequence available."]
    CONST_0,
    #[doc = "Physical Sector Repair command sequence enabled; Erase Physical Sector command sequence disabled."]
    CONST_1,
}
impl PSRR {
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
            PSRR::CONST_0 => false,
            PSRR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PSRR {
        match value {
            false => PSRR::CONST_0,
            true => PSRR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PSRR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PSRR::CONST_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Sector 0 Locked for Write Protection by User 1"]
    #[inline]
    pub fn s0l(&self) -> S0LR {
        S0LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Sector 1 Locked for Write Protection by User 1"]
    #[inline]
    pub fn s1l(&self) -> S1LR {
        S1LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Sector 2 Locked for Write Protection by User 1"]
    #[inline]
    pub fn s2l(&self) -> S2LR {
        S2LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Sector 3 Locked for Write Protection by User 1"]
    #[inline]
    pub fn s3l(&self) -> S3LR {
        S3LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Sector 4 Locked for Write Protection by User 1"]
    #[inline]
    pub fn s4l(&self) -> S4LR {
        S4LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Sector 5 Locked for Write Protection by User 1"]
    #[inline]
    pub fn s5l(&self) -> S5LR {
        S5LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Sector 6 Locked for Write Protection by User 1"]
    #[inline]
    pub fn s6l(&self) -> S6LR {
        S6LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Sector 7 Locked for Write Protection by User 1"]
    #[inline]
    pub fn s7l(&self) -> S7LR {
        S7LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Sector 8 Locked for Write Protection by User 1"]
    #[inline]
    pub fn s8l(&self) -> S8LR {
        S8LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Physical Sector Repair"]
    #[inline]
    pub fn psr(&self) -> PSRR {
        PSRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
