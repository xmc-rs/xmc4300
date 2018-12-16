#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PDISC {
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
#[doc = "Possible values of the field `PDIS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS0R {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1,
}
impl PDIS0R {
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
            PDIS0R::CONST_0 => false,
            PDIS0R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS0R {
        match value {
            false => PDIS0R::CONST_0,
            true => PDIS0R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS0R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS0R::CONST_1
    }
}
#[doc = "Possible values of the field `PDIS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS1R {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1,
}
impl PDIS1R {
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
            PDIS1R::CONST_0 => false,
            PDIS1R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS1R {
        match value {
            false => PDIS1R::CONST_0,
            true => PDIS1R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS1R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS1R::CONST_1
    }
}
#[doc = "Possible values of the field `PDIS2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS2R {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1,
}
impl PDIS2R {
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
            PDIS2R::CONST_0 => false,
            PDIS2R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS2R {
        match value {
            false => PDIS2R::CONST_0,
            true => PDIS2R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS2R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS2R::CONST_1
    }
}
#[doc = "Possible values of the field `PDIS3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS3R {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1,
}
impl PDIS3R {
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
            PDIS3R::CONST_0 => false,
            PDIS3R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS3R {
        match value {
            false => PDIS3R::CONST_0,
            true => PDIS3R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS3R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS3R::CONST_1
    }
}
#[doc = "Possible values of the field `PDIS4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS4R {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1,
}
impl PDIS4R {
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
            PDIS4R::CONST_0 => false,
            PDIS4R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS4R {
        match value {
            false => PDIS4R::CONST_0,
            true => PDIS4R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS4R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS4R::CONST_1
    }
}
#[doc = "Possible values of the field `PDIS5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS5R {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1,
}
impl PDIS5R {
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
            PDIS5R::CONST_0 => false,
            PDIS5R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS5R {
        match value {
            false => PDIS5R::CONST_0,
            true => PDIS5R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS5R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS5R::CONST_1
    }
}
#[doc = "Possible values of the field `PDIS6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS6R {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1,
}
impl PDIS6R {
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
            PDIS6R::CONST_0 => false,
            PDIS6R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS6R {
        match value {
            false => PDIS6R::CONST_0,
            true => PDIS6R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS6R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS6R::CONST_1
    }
}
#[doc = "Possible values of the field `PDIS7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS7R {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1,
}
impl PDIS7R {
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
            PDIS7R::CONST_0 => false,
            PDIS7R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS7R {
        match value {
            false => PDIS7R::CONST_0,
            true => PDIS7R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS7R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS7R::CONST_1
    }
}
#[doc = "Possible values of the field `PDIS8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS8R {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1,
}
impl PDIS8R {
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
            PDIS8R::CONST_0 => false,
            PDIS8R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS8R {
        match value {
            false => PDIS8R::CONST_0,
            true => PDIS8R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS8R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS8R::CONST_1
    }
}
#[doc = "Possible values of the field `PDIS9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS9R {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1,
}
impl PDIS9R {
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
            PDIS9R::CONST_0 => false,
            PDIS9R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS9R {
        match value {
            false => PDIS9R::CONST_0,
            true => PDIS9R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS9R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS9R::CONST_1
    }
}
#[doc = "Possible values of the field `PDIS10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS10R {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1,
}
impl PDIS10R {
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
            PDIS10R::CONST_0 => false,
            PDIS10R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS10R {
        match value {
            false => PDIS10R::CONST_0,
            true => PDIS10R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS10R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS10R::CONST_1
    }
}
#[doc = "Possible values of the field `PDIS11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS11R {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1,
}
impl PDIS11R {
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
            PDIS11R::CONST_0 => false,
            PDIS11R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS11R {
        match value {
            false => PDIS11R::CONST_0,
            true => PDIS11R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS11R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS11R::CONST_1
    }
}
#[doc = "Possible values of the field `PDIS12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS12R {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1,
}
impl PDIS12R {
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
            PDIS12R::CONST_0 => false,
            PDIS12R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS12R {
        match value {
            false => PDIS12R::CONST_0,
            true => PDIS12R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS12R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS12R::CONST_1
    }
}
#[doc = "Possible values of the field `PDIS13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS13R {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1,
}
impl PDIS13R {
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
            PDIS13R::CONST_0 => false,
            PDIS13R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS13R {
        match value {
            false => PDIS13R::CONST_0,
            true => PDIS13R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS13R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS13R::CONST_1
    }
}
#[doc = "Possible values of the field `PDIS14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS14R {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1,
}
impl PDIS14R {
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
            PDIS14R::CONST_0 => false,
            PDIS14R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS14R {
        match value {
            false => PDIS14R::CONST_0,
            true => PDIS14R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS14R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS14R::CONST_1
    }
}
#[doc = "Possible values of the field `PDIS15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS15R {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1,
}
impl PDIS15R {
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
            PDIS15R::CONST_0 => false,
            PDIS15R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS15R {
        match value {
            false => PDIS15R::CONST_0,
            true => PDIS15R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PDIS15R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PDIS15R::CONST_1
    }
}
#[doc = "Values that can be written to the field `PDIS0`"]
pub enum PDIS0W {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1,
}
impl PDIS0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS0W::CONST_0 => false,
            PDIS0W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS0W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PDIS0W::CONST_0)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PDIS0W::CONST_1)
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
#[doc = "Values that can be written to the field `PDIS1`"]
pub enum PDIS1W {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1,
}
impl PDIS1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS1W::CONST_0 => false,
            PDIS1W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS1W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PDIS1W::CONST_0)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PDIS1W::CONST_1)
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
#[doc = "Values that can be written to the field `PDIS2`"]
pub enum PDIS2W {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1,
}
impl PDIS2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS2W::CONST_0 => false,
            PDIS2W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS2W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PDIS2W::CONST_0)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PDIS2W::CONST_1)
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
#[doc = "Values that can be written to the field `PDIS3`"]
pub enum PDIS3W {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1,
}
impl PDIS3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS3W::CONST_0 => false,
            PDIS3W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS3W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PDIS3W::CONST_0)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PDIS3W::CONST_1)
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
#[doc = "Values that can be written to the field `PDIS4`"]
pub enum PDIS4W {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1,
}
impl PDIS4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS4W::CONST_0 => false,
            PDIS4W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS4W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PDIS4W::CONST_0)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PDIS4W::CONST_1)
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
#[doc = "Values that can be written to the field `PDIS5`"]
pub enum PDIS5W {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1,
}
impl PDIS5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS5W::CONST_0 => false,
            PDIS5W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS5W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PDIS5W::CONST_0)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PDIS5W::CONST_1)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PDIS6`"]
pub enum PDIS6W {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1,
}
impl PDIS6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS6W::CONST_0 => false,
            PDIS6W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS6W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PDIS6W::CONST_0)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PDIS6W::CONST_1)
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
#[doc = "Values that can be written to the field `PDIS7`"]
pub enum PDIS7W {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1,
}
impl PDIS7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS7W::CONST_0 => false,
            PDIS7W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS7W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PDIS7W::CONST_0)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PDIS7W::CONST_1)
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
#[doc = "Values that can be written to the field `PDIS8`"]
pub enum PDIS8W {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1,
}
impl PDIS8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS8W::CONST_0 => false,
            PDIS8W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS8W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PDIS8W::CONST_0)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PDIS8W::CONST_1)
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
#[doc = "Values that can be written to the field `PDIS9`"]
pub enum PDIS9W {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1,
}
impl PDIS9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS9W::CONST_0 => false,
            PDIS9W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS9W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PDIS9W::CONST_0)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PDIS9W::CONST_1)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PDIS10`"]
pub enum PDIS10W {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1,
}
impl PDIS10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS10W::CONST_0 => false,
            PDIS10W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS10W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PDIS10W::CONST_0)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PDIS10W::CONST_1)
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
#[doc = "Values that can be written to the field `PDIS11`"]
pub enum PDIS11W {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1,
}
impl PDIS11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS11W::CONST_0 => false,
            PDIS11W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS11W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PDIS11W::CONST_0)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PDIS11W::CONST_1)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PDIS12`"]
pub enum PDIS12W {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1,
}
impl PDIS12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS12W::CONST_0 => false,
            PDIS12W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS12W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PDIS12W::CONST_0)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PDIS12W::CONST_1)
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
#[doc = "Values that can be written to the field `PDIS13`"]
pub enum PDIS13W {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1,
}
impl PDIS13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS13W::CONST_0 => false,
            PDIS13W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS13W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PDIS13W::CONST_0)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PDIS13W::CONST_1)
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
#[doc = "Values that can be written to the field `PDIS14`"]
pub enum PDIS14W {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1,
}
impl PDIS14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS14W::CONST_0 => false,
            PDIS14W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS14W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PDIS14W::CONST_0)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PDIS14W::CONST_1)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PDIS15`"]
pub enum PDIS15W {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    CONST_0,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    CONST_1,
}
impl PDIS15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS15W::CONST_0 => false,
            PDIS15W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS15W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PDIS15W::CONST_0)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PDIS15W::CONST_1)
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
        const OFFSET: u8 = 15;
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
    #[doc = "Bit 0 - Pad Disable for Port n Pin 0"]
    #[inline]
    pub fn pdis0(&self) -> PDIS0R {
        PDIS0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Pad Disable for Port n Pin 1"]
    #[inline]
    pub fn pdis1(&self) -> PDIS1R {
        PDIS1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Pad Disable for Port n Pin 2"]
    #[inline]
    pub fn pdis2(&self) -> PDIS2R {
        PDIS2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Pad Disable for Port n Pin 3"]
    #[inline]
    pub fn pdis3(&self) -> PDIS3R {
        PDIS3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Pad Disable for Port n Pin 4"]
    #[inline]
    pub fn pdis4(&self) -> PDIS4R {
        PDIS4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Pad Disable for Port n Pin 5"]
    #[inline]
    pub fn pdis5(&self) -> PDIS5R {
        PDIS5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Pad Disable for Port n Pin 6"]
    #[inline]
    pub fn pdis6(&self) -> PDIS6R {
        PDIS6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Pad Disable for Port n Pin 7"]
    #[inline]
    pub fn pdis7(&self) -> PDIS7R {
        PDIS7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Pad Disable for Port n Pin 8"]
    #[inline]
    pub fn pdis8(&self) -> PDIS8R {
        PDIS8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Pad Disable for Port n Pin 9"]
    #[inline]
    pub fn pdis9(&self) -> PDIS9R {
        PDIS9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Pad Disable for Port n Pin 10"]
    #[inline]
    pub fn pdis10(&self) -> PDIS10R {
        PDIS10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Pad Disable for Port n Pin 11"]
    #[inline]
    pub fn pdis11(&self) -> PDIS11R {
        PDIS11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Pad Disable for Port n Pin 12"]
    #[inline]
    pub fn pdis12(&self) -> PDIS12R {
        PDIS12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Pad Disable for Port n Pin 13"]
    #[inline]
    pub fn pdis13(&self) -> PDIS13R {
        PDIS13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Pad Disable for Port n Pin 14"]
    #[inline]
    pub fn pdis14(&self) -> PDIS14R {
        PDIS14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Pad Disable for Port n Pin 15"]
    #[inline]
    pub fn pdis15(&self) -> PDIS15R {
        PDIS15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Pad Disable for Port n Pin 0"]
    #[inline]
    pub fn pdis0(&mut self) -> _PDIS0W {
        _PDIS0W { w: self }
    }
    #[doc = "Bit 1 - Pad Disable for Port n Pin 1"]
    #[inline]
    pub fn pdis1(&mut self) -> _PDIS1W {
        _PDIS1W { w: self }
    }
    #[doc = "Bit 2 - Pad Disable for Port n Pin 2"]
    #[inline]
    pub fn pdis2(&mut self) -> _PDIS2W {
        _PDIS2W { w: self }
    }
    #[doc = "Bit 3 - Pad Disable for Port n Pin 3"]
    #[inline]
    pub fn pdis3(&mut self) -> _PDIS3W {
        _PDIS3W { w: self }
    }
    #[doc = "Bit 4 - Pad Disable for Port n Pin 4"]
    #[inline]
    pub fn pdis4(&mut self) -> _PDIS4W {
        _PDIS4W { w: self }
    }
    #[doc = "Bit 5 - Pad Disable for Port n Pin 5"]
    #[inline]
    pub fn pdis5(&mut self) -> _PDIS5W {
        _PDIS5W { w: self }
    }
    #[doc = "Bit 6 - Pad Disable for Port n Pin 6"]
    #[inline]
    pub fn pdis6(&mut self) -> _PDIS6W {
        _PDIS6W { w: self }
    }
    #[doc = "Bit 7 - Pad Disable for Port n Pin 7"]
    #[inline]
    pub fn pdis7(&mut self) -> _PDIS7W {
        _PDIS7W { w: self }
    }
    #[doc = "Bit 8 - Pad Disable for Port n Pin 8"]
    #[inline]
    pub fn pdis8(&mut self) -> _PDIS8W {
        _PDIS8W { w: self }
    }
    #[doc = "Bit 9 - Pad Disable for Port n Pin 9"]
    #[inline]
    pub fn pdis9(&mut self) -> _PDIS9W {
        _PDIS9W { w: self }
    }
    #[doc = "Bit 10 - Pad Disable for Port n Pin 10"]
    #[inline]
    pub fn pdis10(&mut self) -> _PDIS10W {
        _PDIS10W { w: self }
    }
    #[doc = "Bit 11 - Pad Disable for Port n Pin 11"]
    #[inline]
    pub fn pdis11(&mut self) -> _PDIS11W {
        _PDIS11W { w: self }
    }
    #[doc = "Bit 12 - Pad Disable for Port n Pin 12"]
    #[inline]
    pub fn pdis12(&mut self) -> _PDIS12W {
        _PDIS12W { w: self }
    }
    #[doc = "Bit 13 - Pad Disable for Port n Pin 13"]
    #[inline]
    pub fn pdis13(&mut self) -> _PDIS13W {
        _PDIS13W { w: self }
    }
    #[doc = "Bit 14 - Pad Disable for Port n Pin 14"]
    #[inline]
    pub fn pdis14(&mut self) -> _PDIS14W {
        _PDIS14W { w: self }
    }
    #[doc = "Bit 15 - Pad Disable for Port n Pin 15"]
    #[inline]
    pub fn pdis15(&mut self) -> _PDIS15W {
        _PDIS15W { w: self }
    }
}
