#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::IN {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `P0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0R {
    #[doc = "The input level of Pn.x is 0."]
    CONST_0,
    #[doc = "The input level of Pn.x is 1."]
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
    #[doc = "The input level of Pn.x is 0."]
    CONST_0,
    #[doc = "The input level of Pn.x is 1."]
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
    #[doc = "The input level of Pn.x is 0."]
    CONST_0,
    #[doc = "The input level of Pn.x is 1."]
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
    #[doc = "The input level of Pn.x is 0."]
    CONST_0,
    #[doc = "The input level of Pn.x is 1."]
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
    #[doc = "The input level of Pn.x is 0."]
    CONST_0,
    #[doc = "The input level of Pn.x is 1."]
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
    #[doc = "The input level of Pn.x is 0."]
    CONST_0,
    #[doc = "The input level of Pn.x is 1."]
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
    #[doc = "The input level of Pn.x is 0."]
    CONST_0,
    #[doc = "The input level of Pn.x is 1."]
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
    #[doc = "The input level of Pn.x is 0."]
    CONST_0,
    #[doc = "The input level of Pn.x is 1."]
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
    #[doc = "The input level of Pn.x is 0."]
    CONST_0,
    #[doc = "The input level of Pn.x is 1."]
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
    #[doc = "The input level of Pn.x is 0."]
    CONST_0,
    #[doc = "The input level of Pn.x is 1."]
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
    #[doc = "The input level of Pn.x is 0."]
    CONST_0,
    #[doc = "The input level of Pn.x is 1."]
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
    #[doc = "The input level of Pn.x is 0."]
    CONST_0,
    #[doc = "The input level of Pn.x is 1."]
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
    #[doc = "The input level of Pn.x is 0."]
    CONST_0,
    #[doc = "The input level of Pn.x is 1."]
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
    #[doc = "The input level of Pn.x is 0."]
    CONST_0,
    #[doc = "The input level of Pn.x is 1."]
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
    #[doc = "The input level of Pn.x is 0."]
    CONST_0,
    #[doc = "The input level of Pn.x is 1."]
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
    #[doc = "The input level of Pn.x is 0."]
    CONST_0,
    #[doc = "The input level of Pn.x is 1."]
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Port n Input Bit 0"]
    #[inline]
    pub fn p0(&self) -> P0R {
        P0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Port n Input Bit 1"]
    #[inline]
    pub fn p1(&self) -> P1R {
        P1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Port n Input Bit 2"]
    #[inline]
    pub fn p2(&self) -> P2R {
        P2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Port n Input Bit 3"]
    #[inline]
    pub fn p3(&self) -> P3R {
        P3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Port n Input Bit 4"]
    #[inline]
    pub fn p4(&self) -> P4R {
        P4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Port n Input Bit 5"]
    #[inline]
    pub fn p5(&self) -> P5R {
        P5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Port n Input Bit 6"]
    #[inline]
    pub fn p6(&self) -> P6R {
        P6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Port n Input Bit 7"]
    #[inline]
    pub fn p7(&self) -> P7R {
        P7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Port n Input Bit 8"]
    #[inline]
    pub fn p8(&self) -> P8R {
        P8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Port n Input Bit 9"]
    #[inline]
    pub fn p9(&self) -> P9R {
        P9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Port n Input Bit 10"]
    #[inline]
    pub fn p10(&self) -> P10R {
        P10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Port n Input Bit 11"]
    #[inline]
    pub fn p11(&self) -> P11R {
        P11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Port n Input Bit 12"]
    #[inline]
    pub fn p12(&self) -> P12R {
        P12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Port n Input Bit 13"]
    #[inline]
    pub fn p13(&self) -> P13R {
        P13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Port n Input Bit 14"]
    #[inline]
    pub fn p14(&self) -> P14R {
        P14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Port n Input Bit 15"]
    #[inline]
    pub fn p15(&self) -> P15R {
        P15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
