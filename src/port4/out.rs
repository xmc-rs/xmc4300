#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OUT {
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
#[doc = "Possible values of the field `P0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0R {
    #[doc = "The output level of Pn.x is 0."]
    CONST_0,
    #[doc = "The output level of Pn.x is 1."]
    CONST_1,
}
impl P0R {
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
            P0R::CONST_0 => false,
            P0R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P0R {
        match value {
            false => P0R::CONST_0,
            true => P0R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == P0R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == P0R::CONST_1
    }
}
#[doc = "Possible values of the field `P1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1R {
    #[doc = "The output level of Pn.x is 0."]
    CONST_0,
    #[doc = "The output level of Pn.x is 1."]
    CONST_1,
}
impl P1R {
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
            P1R::CONST_0 => false,
            P1R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P1R {
        match value {
            false => P1R::CONST_0,
            true => P1R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == P1R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == P1R::CONST_1
    }
}
#[doc = "Possible values of the field `P2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2R {
    #[doc = "The output level of Pn.x is 0."]
    CONST_0,
    #[doc = "The output level of Pn.x is 1."]
    CONST_1,
}
impl P2R {
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
            P2R::CONST_0 => false,
            P2R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P2R {
        match value {
            false => P2R::CONST_0,
            true => P2R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == P2R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == P2R::CONST_1
    }
}
#[doc = "Possible values of the field `P3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P3R {
    #[doc = "The output level of Pn.x is 0."]
    CONST_0,
    #[doc = "The output level of Pn.x is 1."]
    CONST_1,
}
impl P3R {
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
            P3R::CONST_0 => false,
            P3R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P3R {
        match value {
            false => P3R::CONST_0,
            true => P3R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == P3R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == P3R::CONST_1
    }
}
#[doc = "Possible values of the field `P4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P4R {
    #[doc = "The output level of Pn.x is 0."]
    CONST_0,
    #[doc = "The output level of Pn.x is 1."]
    CONST_1,
}
impl P4R {
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
            P4R::CONST_0 => false,
            P4R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P4R {
        match value {
            false => P4R::CONST_0,
            true => P4R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == P4R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == P4R::CONST_1
    }
}
#[doc = "Possible values of the field `P5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P5R {
    #[doc = "The output level of Pn.x is 0."]
    CONST_0,
    #[doc = "The output level of Pn.x is 1."]
    CONST_1,
}
impl P5R {
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
            P5R::CONST_0 => false,
            P5R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P5R {
        match value {
            false => P5R::CONST_0,
            true => P5R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == P5R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == P5R::CONST_1
    }
}
#[doc = "Possible values of the field `P6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P6R {
    #[doc = "The output level of Pn.x is 0."]
    CONST_0,
    #[doc = "The output level of Pn.x is 1."]
    CONST_1,
}
impl P6R {
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
            P6R::CONST_0 => false,
            P6R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P6R {
        match value {
            false => P6R::CONST_0,
            true => P6R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == P6R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == P6R::CONST_1
    }
}
#[doc = "Possible values of the field `P7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P7R {
    #[doc = "The output level of Pn.x is 0."]
    CONST_0,
    #[doc = "The output level of Pn.x is 1."]
    CONST_1,
}
impl P7R {
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
            P7R::CONST_0 => false,
            P7R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P7R {
        match value {
            false => P7R::CONST_0,
            true => P7R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == P7R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == P7R::CONST_1
    }
}
#[doc = "Possible values of the field `P8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P8R {
    #[doc = "The output level of Pn.x is 0."]
    CONST_0,
    #[doc = "The output level of Pn.x is 1."]
    CONST_1,
}
impl P8R {
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
            P8R::CONST_0 => false,
            P8R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P8R {
        match value {
            false => P8R::CONST_0,
            true => P8R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == P8R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == P8R::CONST_1
    }
}
#[doc = "Possible values of the field `P9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P9R {
    #[doc = "The output level of Pn.x is 0."]
    CONST_0,
    #[doc = "The output level of Pn.x is 1."]
    CONST_1,
}
impl P9R {
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
            P9R::CONST_0 => false,
            P9R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P9R {
        match value {
            false => P9R::CONST_0,
            true => P9R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == P9R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == P9R::CONST_1
    }
}
#[doc = "Possible values of the field `P10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P10R {
    #[doc = "The output level of Pn.x is 0."]
    CONST_0,
    #[doc = "The output level of Pn.x is 1."]
    CONST_1,
}
impl P10R {
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
            P10R::CONST_0 => false,
            P10R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P10R {
        match value {
            false => P10R::CONST_0,
            true => P10R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == P10R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == P10R::CONST_1
    }
}
#[doc = "Possible values of the field `P11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P11R {
    #[doc = "The output level of Pn.x is 0."]
    CONST_0,
    #[doc = "The output level of Pn.x is 1."]
    CONST_1,
}
impl P11R {
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
            P11R::CONST_0 => false,
            P11R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P11R {
        match value {
            false => P11R::CONST_0,
            true => P11R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == P11R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == P11R::CONST_1
    }
}
#[doc = "Possible values of the field `P12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P12R {
    #[doc = "The output level of Pn.x is 0."]
    CONST_0,
    #[doc = "The output level of Pn.x is 1."]
    CONST_1,
}
impl P12R {
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
            P12R::CONST_0 => false,
            P12R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P12R {
        match value {
            false => P12R::CONST_0,
            true => P12R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == P12R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == P12R::CONST_1
    }
}
#[doc = "Possible values of the field `P13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P13R {
    #[doc = "The output level of Pn.x is 0."]
    CONST_0,
    #[doc = "The output level of Pn.x is 1."]
    CONST_1,
}
impl P13R {
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
            P13R::CONST_0 => false,
            P13R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P13R {
        match value {
            false => P13R::CONST_0,
            true => P13R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == P13R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == P13R::CONST_1
    }
}
#[doc = "Possible values of the field `P14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P14R {
    #[doc = "The output level of Pn.x is 0."]
    CONST_0,
    #[doc = "The output level of Pn.x is 1."]
    CONST_1,
}
impl P14R {
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
            P14R::CONST_0 => false,
            P14R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P14R {
        match value {
            false => P14R::CONST_0,
            true => P14R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == P14R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == P14R::CONST_1
    }
}
#[doc = "Possible values of the field `P15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P15R {
    #[doc = "The output level of Pn.x is 0."]
    CONST_0,
    #[doc = "The output level of Pn.x is 1."]
    CONST_1,
}
impl P15R {
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
            P15R::CONST_0 => false,
            P15R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P15R {
        match value {
            false => P15R::CONST_0,
            true => P15R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == P15R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == P15R::CONST_1
    }
}
#[doc = "Values that can be written to the field `P0`"]
pub enum P0W {
    #[doc = "The output level of Pn.x is 0."]
    CONST_0,
    #[doc = "The output level of Pn.x is 1."]
    CONST_1,
}
impl P0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P0W::CONST_0 => false,
            P0W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P0W<'a> {
    w: &'a mut W,
}
impl<'a> _P0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(P0W::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(P0W::CONST_1)
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
#[doc = "Values that can be written to the field `P1`"]
pub enum P1W {
    #[doc = "The output level of Pn.x is 0."]
    CONST_0,
    #[doc = "The output level of Pn.x is 1."]
    CONST_1,
}
impl P1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P1W::CONST_0 => false,
            P1W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1W<'a> {
    w: &'a mut W,
}
impl<'a> _P1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(P1W::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(P1W::CONST_1)
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
#[doc = "Values that can be written to the field `P2`"]
pub enum P2W {
    #[doc = "The output level of Pn.x is 0."]
    CONST_0,
    #[doc = "The output level of Pn.x is 1."]
    CONST_1,
}
impl P2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P2W::CONST_0 => false,
            P2W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2W<'a> {
    w: &'a mut W,
}
impl<'a> _P2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(P2W::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(P2W::CONST_1)
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
#[doc = "Values that can be written to the field `P3`"]
pub enum P3W {
    #[doc = "The output level of Pn.x is 0."]
    CONST_0,
    #[doc = "The output level of Pn.x is 1."]
    CONST_1,
}
impl P3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P3W::CONST_0 => false,
            P3W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P3W<'a> {
    w: &'a mut W,
}
impl<'a> _P3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(P3W::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(P3W::CONST_1)
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
#[doc = "Values that can be written to the field `P4`"]
pub enum P4W {
    #[doc = "The output level of Pn.x is 0."]
    CONST_0,
    #[doc = "The output level of Pn.x is 1."]
    CONST_1,
}
impl P4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P4W::CONST_0 => false,
            P4W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P4W<'a> {
    w: &'a mut W,
}
impl<'a> _P4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(P4W::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(P4W::CONST_1)
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
#[doc = "Values that can be written to the field `P5`"]
pub enum P5W {
    #[doc = "The output level of Pn.x is 0."]
    CONST_0,
    #[doc = "The output level of Pn.x is 1."]
    CONST_1,
}
impl P5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P5W::CONST_0 => false,
            P5W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P5W<'a> {
    w: &'a mut W,
}
impl<'a> _P5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(P5W::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(P5W::CONST_1)
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
#[doc = "Values that can be written to the field `P6`"]
pub enum P6W {
    #[doc = "The output level of Pn.x is 0."]
    CONST_0,
    #[doc = "The output level of Pn.x is 1."]
    CONST_1,
}
impl P6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P6W::CONST_0 => false,
            P6W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P6W<'a> {
    w: &'a mut W,
}
impl<'a> _P6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(P6W::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(P6W::CONST_1)
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
#[doc = "Values that can be written to the field `P7`"]
pub enum P7W {
    #[doc = "The output level of Pn.x is 0."]
    CONST_0,
    #[doc = "The output level of Pn.x is 1."]
    CONST_1,
}
impl P7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P7W::CONST_0 => false,
            P7W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P7W<'a> {
    w: &'a mut W,
}
impl<'a> _P7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(P7W::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(P7W::CONST_1)
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
#[doc = "Values that can be written to the field `P8`"]
pub enum P8W {
    #[doc = "The output level of Pn.x is 0."]
    CONST_0,
    #[doc = "The output level of Pn.x is 1."]
    CONST_1,
}
impl P8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P8W::CONST_0 => false,
            P8W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P8W<'a> {
    w: &'a mut W,
}
impl<'a> _P8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(P8W::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(P8W::CONST_1)
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
#[doc = "Values that can be written to the field `P9`"]
pub enum P9W {
    #[doc = "The output level of Pn.x is 0."]
    CONST_0,
    #[doc = "The output level of Pn.x is 1."]
    CONST_1,
}
impl P9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P9W::CONST_0 => false,
            P9W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P9W<'a> {
    w: &'a mut W,
}
impl<'a> _P9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(P9W::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(P9W::CONST_1)
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
#[doc = "Values that can be written to the field `P10`"]
pub enum P10W {
    #[doc = "The output level of Pn.x is 0."]
    CONST_0,
    #[doc = "The output level of Pn.x is 1."]
    CONST_1,
}
impl P10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P10W::CONST_0 => false,
            P10W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P10W<'a> {
    w: &'a mut W,
}
impl<'a> _P10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(P10W::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(P10W::CONST_1)
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
#[doc = "Values that can be written to the field `P11`"]
pub enum P11W {
    #[doc = "The output level of Pn.x is 0."]
    CONST_0,
    #[doc = "The output level of Pn.x is 1."]
    CONST_1,
}
impl P11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P11W::CONST_0 => false,
            P11W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P11W<'a> {
    w: &'a mut W,
}
impl<'a> _P11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(P11W::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(P11W::CONST_1)
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
#[doc = "Values that can be written to the field `P12`"]
pub enum P12W {
    #[doc = "The output level of Pn.x is 0."]
    CONST_0,
    #[doc = "The output level of Pn.x is 1."]
    CONST_1,
}
impl P12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P12W::CONST_0 => false,
            P12W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P12W<'a> {
    w: &'a mut W,
}
impl<'a> _P12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(P12W::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(P12W::CONST_1)
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
#[doc = "Values that can be written to the field `P13`"]
pub enum P13W {
    #[doc = "The output level of Pn.x is 0."]
    CONST_0,
    #[doc = "The output level of Pn.x is 1."]
    CONST_1,
}
impl P13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P13W::CONST_0 => false,
            P13W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P13W<'a> {
    w: &'a mut W,
}
impl<'a> _P13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(P13W::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(P13W::CONST_1)
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
#[doc = "Values that can be written to the field `P14`"]
pub enum P14W {
    #[doc = "The output level of Pn.x is 0."]
    CONST_0,
    #[doc = "The output level of Pn.x is 1."]
    CONST_1,
}
impl P14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P14W::CONST_0 => false,
            P14W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P14W<'a> {
    w: &'a mut W,
}
impl<'a> _P14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(P14W::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(P14W::CONST_1)
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
#[doc = "Values that can be written to the field `P15`"]
pub enum P15W {
    #[doc = "The output level of Pn.x is 0."]
    CONST_0,
    #[doc = "The output level of Pn.x is 1."]
    CONST_1,
}
impl P15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P15W::CONST_0 => false,
            P15W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P15W<'a> {
    w: &'a mut W,
}
impl<'a> _P15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(P15W::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(P15W::CONST_1)
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
    #[doc = "Bit 0 - Port n Output Bit 0"]
    #[inline]
    pub fn p0(&self) -> P0R {
        P0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Port n Output Bit 1"]
    #[inline]
    pub fn p1(&self) -> P1R {
        P1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Port n Output Bit 2"]
    #[inline]
    pub fn p2(&self) -> P2R {
        P2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Port n Output Bit 3"]
    #[inline]
    pub fn p3(&self) -> P3R {
        P3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Port n Output Bit 4"]
    #[inline]
    pub fn p4(&self) -> P4R {
        P4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Port n Output Bit 5"]
    #[inline]
    pub fn p5(&self) -> P5R {
        P5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Port n Output Bit 6"]
    #[inline]
    pub fn p6(&self) -> P6R {
        P6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Port n Output Bit 7"]
    #[inline]
    pub fn p7(&self) -> P7R {
        P7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Port n Output Bit 8"]
    #[inline]
    pub fn p8(&self) -> P8R {
        P8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Port n Output Bit 9"]
    #[inline]
    pub fn p9(&self) -> P9R {
        P9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Port n Output Bit 10"]
    #[inline]
    pub fn p10(&self) -> P10R {
        P10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Port n Output Bit 11"]
    #[inline]
    pub fn p11(&self) -> P11R {
        P11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Port n Output Bit 12"]
    #[inline]
    pub fn p12(&self) -> P12R {
        P12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Port n Output Bit 13"]
    #[inline]
    pub fn p13(&self) -> P13R {
        P13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Port n Output Bit 14"]
    #[inline]
    pub fn p14(&self) -> P14R {
        P14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Port n Output Bit 15"]
    #[inline]
    pub fn p15(&self) -> P15R {
        P15R::_from({
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
    #[doc = "Bit 0 - Port n Output Bit 0"]
    #[inline]
    pub fn p0(&mut self) -> _P0W {
        _P0W { w: self }
    }
    #[doc = "Bit 1 - Port n Output Bit 1"]
    #[inline]
    pub fn p1(&mut self) -> _P1W {
        _P1W { w: self }
    }
    #[doc = "Bit 2 - Port n Output Bit 2"]
    #[inline]
    pub fn p2(&mut self) -> _P2W {
        _P2W { w: self }
    }
    #[doc = "Bit 3 - Port n Output Bit 3"]
    #[inline]
    pub fn p3(&mut self) -> _P3W {
        _P3W { w: self }
    }
    #[doc = "Bit 4 - Port n Output Bit 4"]
    #[inline]
    pub fn p4(&mut self) -> _P4W {
        _P4W { w: self }
    }
    #[doc = "Bit 5 - Port n Output Bit 5"]
    #[inline]
    pub fn p5(&mut self) -> _P5W {
        _P5W { w: self }
    }
    #[doc = "Bit 6 - Port n Output Bit 6"]
    #[inline]
    pub fn p6(&mut self) -> _P6W {
        _P6W { w: self }
    }
    #[doc = "Bit 7 - Port n Output Bit 7"]
    #[inline]
    pub fn p7(&mut self) -> _P7W {
        _P7W { w: self }
    }
    #[doc = "Bit 8 - Port n Output Bit 8"]
    #[inline]
    pub fn p8(&mut self) -> _P8W {
        _P8W { w: self }
    }
    #[doc = "Bit 9 - Port n Output Bit 9"]
    #[inline]
    pub fn p9(&mut self) -> _P9W {
        _P9W { w: self }
    }
    #[doc = "Bit 10 - Port n Output Bit 10"]
    #[inline]
    pub fn p10(&mut self) -> _P10W {
        _P10W { w: self }
    }
    #[doc = "Bit 11 - Port n Output Bit 11"]
    #[inline]
    pub fn p11(&mut self) -> _P11W {
        _P11W { w: self }
    }
    #[doc = "Bit 12 - Port n Output Bit 12"]
    #[inline]
    pub fn p12(&mut self) -> _P12W {
        _P12W { w: self }
    }
    #[doc = "Bit 13 - Port n Output Bit 13"]
    #[inline]
    pub fn p13(&mut self) -> _P13W {
        _P13W { w: self }
    }
    #[doc = "Bit 14 - Port n Output Bit 14"]
    #[inline]
    pub fn p14(&mut self) -> _P14W {
        _P14W { w: self }
    }
    #[doc = "Bit 15 - Port n Output Bit 15"]
    #[inline]
    pub fn p15(&mut self) -> _P15W {
        _P15W { w: self }
    }
}
