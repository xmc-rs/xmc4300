#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HDCR {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `WKPEP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKPEPR {
    #[doc = "Wake-up event disabled"]
    CONST_0,
    #[doc = "Wake-up event enabled"]
    CONST_1,
}
impl WKPEPR {
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
            WKPEPR::CONST_0 => false,
            WKPEPR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKPEPR {
        match value {
            false => WKPEPR::CONST_0,
            true => WKPEPR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == WKPEPR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == WKPEPR::CONST_1
    }
}
#[doc = "Possible values of the field `WKPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKPENR {
    #[doc = "Wake-up event disabled"]
    CONST_0,
    #[doc = "Wake-up event enabled"]
    CONST_1,
}
impl WKPENR {
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
            WKPENR::CONST_0 => false,
            WKPENR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKPENR {
        match value {
            false => WKPENR::CONST_0,
            true => WKPENR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == WKPENR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == WKPENR::CONST_1
    }
}
#[doc = "Possible values of the field `RTCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCER {
    #[doc = "Wake-up event disabled"]
    CONST_0,
    #[doc = "Wake-up event enabled"]
    CONST_1,
}
impl RTCER {
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
            RTCER::CONST_0 => false,
            RTCER::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTCER {
        match value {
            false => RTCER::CONST_0,
            true => RTCER::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == RTCER::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == RTCER::CONST_1
    }
}
#[doc = "Possible values of the field `ULPWDGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ULPWDGENR {
    #[doc = "Wake-up event disabled"]
    CONST_0,
    #[doc = "Wake-up event enabled"]
    CONST_1,
}
impl ULPWDGENR {
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
            ULPWDGENR::CONST_0 => false,
            ULPWDGENR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ULPWDGENR {
        match value {
            false => ULPWDGENR::CONST_0,
            true => ULPWDGENR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == ULPWDGENR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == ULPWDGENR::CONST_1
    }
}
#[doc = "Possible values of the field `HIB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIBR {
    #[doc = "External hibernate request inactive"]
    CONST_0,
    #[doc = "External hibernate request active"]
    CONST_1,
}
impl HIBR {
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
            HIBR::CONST_0 => false,
            HIBR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HIBR {
        match value {
            false => HIBR::CONST_0,
            true => HIBR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == HIBR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == HIBR::CONST_1
    }
}
#[doc = "Possible values of the field `RCS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCSR {
    #[doc = "fOSI selected"]
    CONST_0,
    #[doc = "fULP selected"]
    CONST_1,
}
impl RCSR {
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
            RCSR::CONST_0 => false,
            RCSR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RCSR {
        match value {
            false => RCSR::CONST_0,
            true => RCSR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == RCSR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == RCSR::CONST_1
    }
}
#[doc = "Possible values of the field `STDBYSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STDBYSELR {
    #[doc = "fOSI selected"]
    CONST_0,
    #[doc = "fULP selected"]
    CONST_1,
}
impl STDBYSELR {
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
            STDBYSELR::CONST_0 => false,
            STDBYSELR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STDBYSELR {
        match value {
            false => STDBYSELR::CONST_0,
            true => STDBYSELR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == STDBYSELR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == STDBYSELR::CONST_1
    }
}
#[doc = "Possible values of the field `WKUPSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPSELR {
    #[doc = "HIB_IO_1 pin selected"]
    CONST_0,
    #[doc = "HIB_IO_0 pin selected"]
    CONST_1,
}
impl WKUPSELR {
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
            WKUPSELR::CONST_0 => false,
            WKUPSELR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPSELR {
        match value {
            false => WKUPSELR::CONST_0,
            true => WKUPSELR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == WKUPSELR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == WKUPSELR::CONST_1
    }
}
#[doc = "Possible values of the field `GPI0SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPI0SELR {
    #[doc = "#0"]
    CONST_0,
    #[doc = "HIB_IO_0 pin selected"]
    CONST_1,
}
impl GPI0SELR {
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
            GPI0SELR::CONST_0 => false,
            GPI0SELR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPI0SELR {
        match value {
            false => GPI0SELR::CONST_0,
            true => GPI0SELR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == GPI0SELR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == GPI0SELR::CONST_1
    }
}
#[doc = "Possible values of the field `HIBIO0POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIBIO0POLR {
    #[doc = "Direct value"]
    CONST_0,
    #[doc = "Inverted value"]
    CONST_1,
}
impl HIBIO0POLR {
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
            HIBIO0POLR::CONST_0 => false,
            HIBIO0POLR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HIBIO0POLR {
        match value {
            false => HIBIO0POLR::CONST_0,
            true => HIBIO0POLR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == HIBIO0POLR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == HIBIO0POLR::CONST_1
    }
}
#[doc = "Possible values of the field `HIBIO1POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIBIO1POLR {
    #[doc = "Direct value"]
    CONST_0,
    #[doc = "Inverted value"]
    CONST_1,
}
impl HIBIO1POLR {
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
            HIBIO1POLR::CONST_0 => false,
            HIBIO1POLR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HIBIO1POLR {
        match value {
            false => HIBIO1POLR::CONST_0,
            true => HIBIO1POLR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == HIBIO1POLR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == HIBIO1POLR::CONST_1
    }
}
#[doc = "Possible values of the field `HIBIO0SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIBIO0SELR {
    #[doc = "Direct input, No input pull device connected"]
    CONST_0000,
    #[doc = "Direct input, Input pull-down device connected"]
    CONST_0001,
    #[doc = "Direct input, Input pull-up device connected"]
    CONST_0010,
    #[doc = "Push-pull HIB Control output"]
    CONST_1000,
    #[doc = "Push-pull WDT service output"]
    CONST_1001,
    #[doc = "Push-pull GPIO output"]
    CONST_1010,
    #[doc = "Open-drain HIB Control output"]
    CONST_1100,
    #[doc = "Open-drain WDT service output"]
    CONST_1101,
    #[doc = "Open-drain GPIO output"]
    CONST_1110,
    #[doc = "#1111"]
    CONST_1111,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HIBIO0SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HIBIO0SELR::CONST_0000 => 0,
            HIBIO0SELR::CONST_0001 => 1,
            HIBIO0SELR::CONST_0010 => 2,
            HIBIO0SELR::CONST_1000 => 8,
            HIBIO0SELR::CONST_1001 => 9,
            HIBIO0SELR::CONST_1010 => 10,
            HIBIO0SELR::CONST_1100 => 12,
            HIBIO0SELR::CONST_1101 => 13,
            HIBIO0SELR::CONST_1110 => 14,
            HIBIO0SELR::CONST_1111 => 15,
            HIBIO0SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HIBIO0SELR {
        match value {
            0 => HIBIO0SELR::CONST_0000,
            1 => HIBIO0SELR::CONST_0001,
            2 => HIBIO0SELR::CONST_0010,
            8 => HIBIO0SELR::CONST_1000,
            9 => HIBIO0SELR::CONST_1001,
            10 => HIBIO0SELR::CONST_1010,
            12 => HIBIO0SELR::CONST_1100,
            13 => HIBIO0SELR::CONST_1101,
            14 => HIBIO0SELR::CONST_1110,
            15 => HIBIO0SELR::CONST_1111,
            i => HIBIO0SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0000`"]
    #[inline]
    pub fn is_const_0000(&self) -> bool {
        *self == HIBIO0SELR::CONST_0000
    }
    #[doc = "Checks if the value of the field is `CONST_0001`"]
    #[inline]
    pub fn is_const_0001(&self) -> bool {
        *self == HIBIO0SELR::CONST_0001
    }
    #[doc = "Checks if the value of the field is `CONST_0010`"]
    #[inline]
    pub fn is_const_0010(&self) -> bool {
        *self == HIBIO0SELR::CONST_0010
    }
    #[doc = "Checks if the value of the field is `CONST_1000`"]
    #[inline]
    pub fn is_const_1000(&self) -> bool {
        *self == HIBIO0SELR::CONST_1000
    }
    #[doc = "Checks if the value of the field is `CONST_1001`"]
    #[inline]
    pub fn is_const_1001(&self) -> bool {
        *self == HIBIO0SELR::CONST_1001
    }
    #[doc = "Checks if the value of the field is `CONST_1010`"]
    #[inline]
    pub fn is_const_1010(&self) -> bool {
        *self == HIBIO0SELR::CONST_1010
    }
    #[doc = "Checks if the value of the field is `CONST_1100`"]
    #[inline]
    pub fn is_const_1100(&self) -> bool {
        *self == HIBIO0SELR::CONST_1100
    }
    #[doc = "Checks if the value of the field is `CONST_1101`"]
    #[inline]
    pub fn is_const_1101(&self) -> bool {
        *self == HIBIO0SELR::CONST_1101
    }
    #[doc = "Checks if the value of the field is `CONST_1110`"]
    #[inline]
    pub fn is_const_1110(&self) -> bool {
        *self == HIBIO0SELR::CONST_1110
    }
    #[doc = "Checks if the value of the field is `CONST_1111`"]
    #[inline]
    pub fn is_const_1111(&self) -> bool {
        *self == HIBIO0SELR::CONST_1111
    }
}
#[doc = "Possible values of the field `HIBIO1SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIBIO1SELR {
    #[doc = "Direct input, No input pull device connected"]
    CONST_0000,
    #[doc = "Direct input, Input pull-down device connected"]
    CONST_0001,
    #[doc = "Direct input, Input pull-up device connected"]
    CONST_0010,
    #[doc = "Push-pull HIB Control output"]
    CONST_1000,
    #[doc = "Push-pull WDT service output"]
    CONST_1001,
    #[doc = "Push-pull GPIO output"]
    CONST_1010,
    #[doc = "Open-drain HIB Control output"]
    CONST_1100,
    #[doc = "Open-drain WDT service output"]
    CONST_1101,
    #[doc = "Open-drain GPIO output"]
    CONST_1110,
    #[doc = "#1111"]
    CONST_1111,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HIBIO1SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HIBIO1SELR::CONST_0000 => 0,
            HIBIO1SELR::CONST_0001 => 1,
            HIBIO1SELR::CONST_0010 => 2,
            HIBIO1SELR::CONST_1000 => 8,
            HIBIO1SELR::CONST_1001 => 9,
            HIBIO1SELR::CONST_1010 => 10,
            HIBIO1SELR::CONST_1100 => 12,
            HIBIO1SELR::CONST_1101 => 13,
            HIBIO1SELR::CONST_1110 => 14,
            HIBIO1SELR::CONST_1111 => 15,
            HIBIO1SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HIBIO1SELR {
        match value {
            0 => HIBIO1SELR::CONST_0000,
            1 => HIBIO1SELR::CONST_0001,
            2 => HIBIO1SELR::CONST_0010,
            8 => HIBIO1SELR::CONST_1000,
            9 => HIBIO1SELR::CONST_1001,
            10 => HIBIO1SELR::CONST_1010,
            12 => HIBIO1SELR::CONST_1100,
            13 => HIBIO1SELR::CONST_1101,
            14 => HIBIO1SELR::CONST_1110,
            15 => HIBIO1SELR::CONST_1111,
            i => HIBIO1SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0000`"]
    #[inline]
    pub fn is_const_0000(&self) -> bool {
        *self == HIBIO1SELR::CONST_0000
    }
    #[doc = "Checks if the value of the field is `CONST_0001`"]
    #[inline]
    pub fn is_const_0001(&self) -> bool {
        *self == HIBIO1SELR::CONST_0001
    }
    #[doc = "Checks if the value of the field is `CONST_0010`"]
    #[inline]
    pub fn is_const_0010(&self) -> bool {
        *self == HIBIO1SELR::CONST_0010
    }
    #[doc = "Checks if the value of the field is `CONST_1000`"]
    #[inline]
    pub fn is_const_1000(&self) -> bool {
        *self == HIBIO1SELR::CONST_1000
    }
    #[doc = "Checks if the value of the field is `CONST_1001`"]
    #[inline]
    pub fn is_const_1001(&self) -> bool {
        *self == HIBIO1SELR::CONST_1001
    }
    #[doc = "Checks if the value of the field is `CONST_1010`"]
    #[inline]
    pub fn is_const_1010(&self) -> bool {
        *self == HIBIO1SELR::CONST_1010
    }
    #[doc = "Checks if the value of the field is `CONST_1100`"]
    #[inline]
    pub fn is_const_1100(&self) -> bool {
        *self == HIBIO1SELR::CONST_1100
    }
    #[doc = "Checks if the value of the field is `CONST_1101`"]
    #[inline]
    pub fn is_const_1101(&self) -> bool {
        *self == HIBIO1SELR::CONST_1101
    }
    #[doc = "Checks if the value of the field is `CONST_1110`"]
    #[inline]
    pub fn is_const_1110(&self) -> bool {
        *self == HIBIO1SELR::CONST_1110
    }
    #[doc = "Checks if the value of the field is `CONST_1111`"]
    #[inline]
    pub fn is_const_1111(&self) -> bool {
        *self == HIBIO1SELR::CONST_1111
    }
}
#[doc = "Values that can be written to the field `WKPEP`"]
pub enum WKPEPW {
    #[doc = "Wake-up event disabled"]
    CONST_0,
    #[doc = "Wake-up event enabled"]
    CONST_1,
}
impl WKPEPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKPEPW::CONST_0 => false,
            WKPEPW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKPEPW<'a> {
    w: &'a mut W,
}
impl<'a> _WKPEPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKPEPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up event disabled"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(WKPEPW::CONST_0)
    }
    #[doc = "Wake-up event enabled"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(WKPEPW::CONST_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WKPEN`"]
pub enum WKPENW {
    #[doc = "Wake-up event disabled"]
    CONST_0,
    #[doc = "Wake-up event enabled"]
    CONST_1,
}
impl WKPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKPENW::CONST_0 => false,
            WKPENW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKPENW<'a> {
    w: &'a mut W,
}
impl<'a> _WKPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up event disabled"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(WKPENW::CONST_0)
    }
    #[doc = "Wake-up event enabled"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(WKPENW::CONST_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTCE`"]
pub enum RTCEW {
    #[doc = "Wake-up event disabled"]
    CONST_0,
    #[doc = "Wake-up event enabled"]
    CONST_1,
}
impl RTCEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTCEW::CONST_0 => false,
            RTCEW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTCEW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTCEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up event disabled"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(RTCEW::CONST_0)
    }
    #[doc = "Wake-up event enabled"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(RTCEW::CONST_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ULPWDGEN`"]
pub enum ULPWDGENW {
    #[doc = "Wake-up event disabled"]
    CONST_0,
    #[doc = "Wake-up event enabled"]
    CONST_1,
}
impl ULPWDGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ULPWDGENW::CONST_0 => false,
            ULPWDGENW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ULPWDGENW<'a> {
    w: &'a mut W,
}
impl<'a> _ULPWDGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ULPWDGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up event disabled"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ULPWDGENW::CONST_0)
    }
    #[doc = "Wake-up event enabled"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ULPWDGENW::CONST_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HIB`"]
pub enum HIBW {
    #[doc = "External hibernate request inactive"]
    CONST_0,
    #[doc = "External hibernate request active"]
    CONST_1,
}
impl HIBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HIBW::CONST_0 => false,
            HIBW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HIBW<'a> {
    w: &'a mut W,
}
impl<'a> _HIBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HIBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "External hibernate request inactive"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(HIBW::CONST_0)
    }
    #[doc = "External hibernate request active"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(HIBW::CONST_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RCS`"]
pub enum RCSW {
    #[doc = "fOSI selected"]
    CONST_0,
    #[doc = "fULP selected"]
    CONST_1,
}
impl RCSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RCSW::CONST_0 => false,
            RCSW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RCSW<'a> {
    w: &'a mut W,
}
impl<'a> _RCSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RCSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "fOSI selected"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(RCSW::CONST_0)
    }
    #[doc = "fULP selected"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(RCSW::CONST_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STDBYSEL`"]
pub enum STDBYSELW {
    #[doc = "fOSI selected"]
    CONST_0,
    #[doc = "fULP selected"]
    CONST_1,
}
impl STDBYSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STDBYSELW::CONST_0 => false,
            STDBYSELW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STDBYSELW<'a> {
    w: &'a mut W,
}
impl<'a> _STDBYSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STDBYSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "fOSI selected"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(STDBYSELW::CONST_0)
    }
    #[doc = "fULP selected"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(STDBYSELW::CONST_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WKUPSEL`"]
pub enum WKUPSELW {
    #[doc = "HIB_IO_1 pin selected"]
    CONST_0,
    #[doc = "HIB_IO_0 pin selected"]
    CONST_1,
}
impl WKUPSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPSELW::CONST_0 => false,
            WKUPSELW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPSELW<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HIB_IO_1 pin selected"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(WKUPSELW::CONST_0)
    }
    #[doc = "HIB_IO_0 pin selected"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(WKUPSELW::CONST_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPI0SEL`"]
pub enum GPI0SELW {
    #[doc = "#0"]
    CONST_0,
    #[doc = "HIB_IO_0 pin selected"]
    CONST_1,
}
impl GPI0SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPI0SELW::CONST_0 => false,
            GPI0SELW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPI0SELW<'a> {
    w: &'a mut W,
}
impl<'a> _GPI0SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPI0SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "#0"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(GPI0SELW::CONST_0)
    }
    #[doc = "HIB_IO_0 pin selected"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(GPI0SELW::CONST_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HIBIO0POL`"]
pub enum HIBIO0POLW {
    #[doc = "Direct value"]
    CONST_0,
    #[doc = "Inverted value"]
    CONST_1,
}
impl HIBIO0POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HIBIO0POLW::CONST_0 => false,
            HIBIO0POLW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HIBIO0POLW<'a> {
    w: &'a mut W,
}
impl<'a> _HIBIO0POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HIBIO0POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Direct value"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(HIBIO0POLW::CONST_0)
    }
    #[doc = "Inverted value"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(HIBIO0POLW::CONST_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HIBIO1POL`"]
pub enum HIBIO1POLW {
    #[doc = "Direct value"]
    CONST_0,
    #[doc = "Inverted value"]
    CONST_1,
}
impl HIBIO1POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HIBIO1POLW::CONST_0 => false,
            HIBIO1POLW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HIBIO1POLW<'a> {
    w: &'a mut W,
}
impl<'a> _HIBIO1POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HIBIO1POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Direct value"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(HIBIO1POLW::CONST_0)
    }
    #[doc = "Inverted value"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(HIBIO1POLW::CONST_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HIBIO0SEL`"]
pub enum HIBIO0SELW {
    #[doc = "Direct input, No input pull device connected"]
    CONST_0000,
    #[doc = "Direct input, Input pull-down device connected"]
    CONST_0001,
    #[doc = "Direct input, Input pull-up device connected"]
    CONST_0010,
    #[doc = "Push-pull HIB Control output"]
    CONST_1000,
    #[doc = "Push-pull WDT service output"]
    CONST_1001,
    #[doc = "Push-pull GPIO output"]
    CONST_1010,
    #[doc = "Open-drain HIB Control output"]
    CONST_1100,
    #[doc = "Open-drain WDT service output"]
    CONST_1101,
    #[doc = "Open-drain GPIO output"]
    CONST_1110,
    #[doc = "#1111"]
    CONST_1111,
}
impl HIBIO0SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HIBIO0SELW::CONST_0000 => 0,
            HIBIO0SELW::CONST_0001 => 1,
            HIBIO0SELW::CONST_0010 => 2,
            HIBIO0SELW::CONST_1000 => 8,
            HIBIO0SELW::CONST_1001 => 9,
            HIBIO0SELW::CONST_1010 => 10,
            HIBIO0SELW::CONST_1100 => 12,
            HIBIO0SELW::CONST_1101 => 13,
            HIBIO0SELW::CONST_1110 => 14,
            HIBIO0SELW::CONST_1111 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HIBIO0SELW<'a> {
    w: &'a mut W,
}
impl<'a> _HIBIO0SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HIBIO0SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Direct input, No input pull device connected"]
    #[inline]
    pub fn const_0000(self) -> &'a mut W {
        self.variant(HIBIO0SELW::CONST_0000)
    }
    #[doc = "Direct input, Input pull-down device connected"]
    #[inline]
    pub fn const_0001(self) -> &'a mut W {
        self.variant(HIBIO0SELW::CONST_0001)
    }
    #[doc = "Direct input, Input pull-up device connected"]
    #[inline]
    pub fn const_0010(self) -> &'a mut W {
        self.variant(HIBIO0SELW::CONST_0010)
    }
    #[doc = "Push-pull HIB Control output"]
    #[inline]
    pub fn const_1000(self) -> &'a mut W {
        self.variant(HIBIO0SELW::CONST_1000)
    }
    #[doc = "Push-pull WDT service output"]
    #[inline]
    pub fn const_1001(self) -> &'a mut W {
        self.variant(HIBIO0SELW::CONST_1001)
    }
    #[doc = "Push-pull GPIO output"]
    #[inline]
    pub fn const_1010(self) -> &'a mut W {
        self.variant(HIBIO0SELW::CONST_1010)
    }
    #[doc = "Open-drain HIB Control output"]
    #[inline]
    pub fn const_1100(self) -> &'a mut W {
        self.variant(HIBIO0SELW::CONST_1100)
    }
    #[doc = "Open-drain WDT service output"]
    #[inline]
    pub fn const_1101(self) -> &'a mut W {
        self.variant(HIBIO0SELW::CONST_1101)
    }
    #[doc = "Open-drain GPIO output"]
    #[inline]
    pub fn const_1110(self) -> &'a mut W {
        self.variant(HIBIO0SELW::CONST_1110)
    }
    #[doc = "#1111"]
    #[inline]
    pub fn const_1111(self) -> &'a mut W {
        self.variant(HIBIO0SELW::CONST_1111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HIBIO1SEL`"]
pub enum HIBIO1SELW {
    #[doc = "Direct input, No input pull device connected"]
    CONST_0000,
    #[doc = "Direct input, Input pull-down device connected"]
    CONST_0001,
    #[doc = "Direct input, Input pull-up device connected"]
    CONST_0010,
    #[doc = "Push-pull HIB Control output"]
    CONST_1000,
    #[doc = "Push-pull WDT service output"]
    CONST_1001,
    #[doc = "Push-pull GPIO output"]
    CONST_1010,
    #[doc = "Open-drain HIB Control output"]
    CONST_1100,
    #[doc = "Open-drain WDT service output"]
    CONST_1101,
    #[doc = "Open-drain GPIO output"]
    CONST_1110,
    #[doc = "#1111"]
    CONST_1111,
}
impl HIBIO1SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HIBIO1SELW::CONST_0000 => 0,
            HIBIO1SELW::CONST_0001 => 1,
            HIBIO1SELW::CONST_0010 => 2,
            HIBIO1SELW::CONST_1000 => 8,
            HIBIO1SELW::CONST_1001 => 9,
            HIBIO1SELW::CONST_1010 => 10,
            HIBIO1SELW::CONST_1100 => 12,
            HIBIO1SELW::CONST_1101 => 13,
            HIBIO1SELW::CONST_1110 => 14,
            HIBIO1SELW::CONST_1111 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HIBIO1SELW<'a> {
    w: &'a mut W,
}
impl<'a> _HIBIO1SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HIBIO1SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Direct input, No input pull device connected"]
    #[inline]
    pub fn const_0000(self) -> &'a mut W {
        self.variant(HIBIO1SELW::CONST_0000)
    }
    #[doc = "Direct input, Input pull-down device connected"]
    #[inline]
    pub fn const_0001(self) -> &'a mut W {
        self.variant(HIBIO1SELW::CONST_0001)
    }
    #[doc = "Direct input, Input pull-up device connected"]
    #[inline]
    pub fn const_0010(self) -> &'a mut W {
        self.variant(HIBIO1SELW::CONST_0010)
    }
    #[doc = "Push-pull HIB Control output"]
    #[inline]
    pub fn const_1000(self) -> &'a mut W {
        self.variant(HIBIO1SELW::CONST_1000)
    }
    #[doc = "Push-pull WDT service output"]
    #[inline]
    pub fn const_1001(self) -> &'a mut W {
        self.variant(HIBIO1SELW::CONST_1001)
    }
    #[doc = "Push-pull GPIO output"]
    #[inline]
    pub fn const_1010(self) -> &'a mut W {
        self.variant(HIBIO1SELW::CONST_1010)
    }
    #[doc = "Open-drain HIB Control output"]
    #[inline]
    pub fn const_1100(self) -> &'a mut W {
        self.variant(HIBIO1SELW::CONST_1100)
    }
    #[doc = "Open-drain WDT service output"]
    #[inline]
    pub fn const_1101(self) -> &'a mut W {
        self.variant(HIBIO1SELW::CONST_1101)
    }
    #[doc = "Open-drain GPIO output"]
    #[inline]
    pub fn const_1110(self) -> &'a mut W {
        self.variant(HIBIO1SELW::CONST_1110)
    }
    #[doc = "#1111"]
    #[inline]
    pub fn const_1111(self) -> &'a mut W {
        self.variant(HIBIO1SELW::CONST_1111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Wake-Up on Pin Event Positive Edge Enable"]
    #[inline]
    pub fn wkpep(&self) -> WKPEPR {
        WKPEPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Wake-up on Pin Event Negative Edge Enable"]
    #[inline]
    pub fn wkpen(&self) -> WKPENR {
        WKPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Wake-up on RTC Event Enable"]
    #[inline]
    pub fn rtce(&self) -> RTCER {
        RTCER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - ULP WDG Alarm Enable"]
    #[inline]
    pub fn ulpwdgen(&self) -> ULPWDGENR {
        ULPWDGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Hibernate Request Value Set"]
    #[inline]
    pub fn hib(&self) -> HIBR {
        HIBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - fRTC Clock Selection"]
    #[inline]
    pub fn rcs(&self) -> RCSR {
        RCSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - fSTDBY Clock Selection"]
    #[inline]
    pub fn stdbysel(&self) -> STDBYSELR {
        STDBYSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Wake-Up from Hibernate Trigger Input Selection"]
    #[inline]
    pub fn wkupsel(&self) -> WKUPSELR {
        WKUPSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - General Purpose Input 0 Selection"]
    #[inline]
    pub fn gpi0sel(&self) -> GPI0SELR {
        GPI0SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - HIBIO0 Polarity Set"]
    #[inline]
    pub fn hibio0pol(&self) -> HIBIO0POLR {
        HIBIO0POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - HIBIO1 Polarity Set"]
    #[inline]
    pub fn hibio1pol(&self) -> HIBIO1POLR {
        HIBIO1POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:19 - HIB_IO_0 Pin I/O Control (default HIBOUT)"]
    #[inline]
    pub fn hibio0sel(&self) -> HIBIO0SELR {
        HIBIO0SELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - HIB_IO_1 Pin I/O Control (Default WKUP)"]
    #[inline]
    pub fn hibio1sel(&self) -> HIBIO1SELR {
        HIBIO1SELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 794624 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Wake-Up on Pin Event Positive Edge Enable"]
    #[inline]
    pub fn wkpep(&mut self) -> _WKPEPW {
        _WKPEPW { w: self }
    }
    #[doc = "Bit 1 - Wake-up on Pin Event Negative Edge Enable"]
    #[inline]
    pub fn wkpen(&mut self) -> _WKPENW {
        _WKPENW { w: self }
    }
    #[doc = "Bit 2 - Wake-up on RTC Event Enable"]
    #[inline]
    pub fn rtce(&mut self) -> _RTCEW {
        _RTCEW { w: self }
    }
    #[doc = "Bit 3 - ULP WDG Alarm Enable"]
    #[inline]
    pub fn ulpwdgen(&mut self) -> _ULPWDGENW {
        _ULPWDGENW { w: self }
    }
    #[doc = "Bit 4 - Hibernate Request Value Set"]
    #[inline]
    pub fn hib(&mut self) -> _HIBW {
        _HIBW { w: self }
    }
    #[doc = "Bit 6 - fRTC Clock Selection"]
    #[inline]
    pub fn rcs(&mut self) -> _RCSW {
        _RCSW { w: self }
    }
    #[doc = "Bit 7 - fSTDBY Clock Selection"]
    #[inline]
    pub fn stdbysel(&mut self) -> _STDBYSELW {
        _STDBYSELW { w: self }
    }
    #[doc = "Bit 8 - Wake-Up from Hibernate Trigger Input Selection"]
    #[inline]
    pub fn wkupsel(&mut self) -> _WKUPSELW {
        _WKUPSELW { w: self }
    }
    #[doc = "Bit 10 - General Purpose Input 0 Selection"]
    #[inline]
    pub fn gpi0sel(&mut self) -> _GPI0SELW {
        _GPI0SELW { w: self }
    }
    #[doc = "Bit 12 - HIBIO0 Polarity Set"]
    #[inline]
    pub fn hibio0pol(&mut self) -> _HIBIO0POLW {
        _HIBIO0POLW { w: self }
    }
    #[doc = "Bit 13 - HIBIO1 Polarity Set"]
    #[inline]
    pub fn hibio1pol(&mut self) -> _HIBIO1POLW {
        _HIBIO1POLW { w: self }
    }
    #[doc = "Bits 16:19 - HIB_IO_0 Pin I/O Control (default HIBOUT)"]
    #[inline]
    pub fn hibio0sel(&mut self) -> _HIBIO0SELW {
        _HIBIO0SELW { w: self }
    }
    #[doc = "Bits 20:23 - HIB_IO_1 Pin I/O Control (Default WKUP)"]
    #[inline]
    pub fn hibio1sel(&mut self) -> _HIBIO1SELW {
        _HIBIO1SELW { w: self }
    }
}
