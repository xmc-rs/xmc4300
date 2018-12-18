#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PPS {
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
#[doc = "Possible values of the field `PPS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS0R {
    #[doc = "Pin Power Save of Pn.x is disabled."]
    CONST_0,
    #[doc = "Pin Power Save of Pn.x is enabled."]
    CONST_1,
}
impl PPS0R {
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
            PPS0R::CONST_0 => false,
            PPS0R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PPS0R {
        match value {
            false => PPS0R::CONST_0,
            true => PPS0R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PPS0R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PPS0R::CONST_1
    }
}
#[doc = "Possible values of the field `PPS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS1R {
    #[doc = "Pin Power Save of Pn.x is disabled."]
    CONST_0,
    #[doc = "Pin Power Save of Pn.x is enabled."]
    CONST_1,
}
impl PPS1R {
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
            PPS1R::CONST_0 => false,
            PPS1R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PPS1R {
        match value {
            false => PPS1R::CONST_0,
            true => PPS1R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PPS1R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PPS1R::CONST_1
    }
}
#[doc = "Possible values of the field `PPS2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS2R {
    #[doc = "Pin Power Save of Pn.x is disabled."]
    CONST_0,
    #[doc = "Pin Power Save of Pn.x is enabled."]
    CONST_1,
}
impl PPS2R {
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
            PPS2R::CONST_0 => false,
            PPS2R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PPS2R {
        match value {
            false => PPS2R::CONST_0,
            true => PPS2R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PPS2R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PPS2R::CONST_1
    }
}
#[doc = "Possible values of the field `PPS3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS3R {
    #[doc = "Pin Power Save of Pn.x is disabled."]
    CONST_0,
    #[doc = "Pin Power Save of Pn.x is enabled."]
    CONST_1,
}
impl PPS3R {
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
            PPS3R::CONST_0 => false,
            PPS3R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PPS3R {
        match value {
            false => PPS3R::CONST_0,
            true => PPS3R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PPS3R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PPS3R::CONST_1
    }
}
#[doc = "Possible values of the field `PPS4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS4R {
    #[doc = "Pin Power Save of Pn.x is disabled."]
    CONST_0,
    #[doc = "Pin Power Save of Pn.x is enabled."]
    CONST_1,
}
impl PPS4R {
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
            PPS4R::CONST_0 => false,
            PPS4R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PPS4R {
        match value {
            false => PPS4R::CONST_0,
            true => PPS4R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PPS4R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PPS4R::CONST_1
    }
}
#[doc = "Possible values of the field `PPS5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS5R {
    #[doc = "Pin Power Save of Pn.x is disabled."]
    CONST_0,
    #[doc = "Pin Power Save of Pn.x is enabled."]
    CONST_1,
}
impl PPS5R {
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
            PPS5R::CONST_0 => false,
            PPS5R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PPS5R {
        match value {
            false => PPS5R::CONST_0,
            true => PPS5R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PPS5R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PPS5R::CONST_1
    }
}
#[doc = "Possible values of the field `PPS6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS6R {
    #[doc = "Pin Power Save of Pn.x is disabled."]
    CONST_0,
    #[doc = "Pin Power Save of Pn.x is enabled."]
    CONST_1,
}
impl PPS6R {
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
            PPS6R::CONST_0 => false,
            PPS6R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PPS6R {
        match value {
            false => PPS6R::CONST_0,
            true => PPS6R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PPS6R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PPS6R::CONST_1
    }
}
#[doc = "Possible values of the field `PPS7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS7R {
    #[doc = "Pin Power Save of Pn.x is disabled."]
    CONST_0,
    #[doc = "Pin Power Save of Pn.x is enabled."]
    CONST_1,
}
impl PPS7R {
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
            PPS7R::CONST_0 => false,
            PPS7R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PPS7R {
        match value {
            false => PPS7R::CONST_0,
            true => PPS7R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PPS7R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PPS7R::CONST_1
    }
}
#[doc = "Possible values of the field `PPS8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS8R {
    #[doc = "Pin Power Save of Pn.x is disabled."]
    CONST_0,
    #[doc = "Pin Power Save of Pn.x is enabled."]
    CONST_1,
}
impl PPS8R {
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
            PPS8R::CONST_0 => false,
            PPS8R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PPS8R {
        match value {
            false => PPS8R::CONST_0,
            true => PPS8R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PPS8R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PPS8R::CONST_1
    }
}
#[doc = "Possible values of the field `PPS9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS9R {
    #[doc = "Pin Power Save of Pn.x is disabled."]
    CONST_0,
    #[doc = "Pin Power Save of Pn.x is enabled."]
    CONST_1,
}
impl PPS9R {
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
            PPS9R::CONST_0 => false,
            PPS9R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PPS9R {
        match value {
            false => PPS9R::CONST_0,
            true => PPS9R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PPS9R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PPS9R::CONST_1
    }
}
#[doc = "Possible values of the field `PPS10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS10R {
    #[doc = "Pin Power Save of Pn.x is disabled."]
    CONST_0,
    #[doc = "Pin Power Save of Pn.x is enabled."]
    CONST_1,
}
impl PPS10R {
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
            PPS10R::CONST_0 => false,
            PPS10R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PPS10R {
        match value {
            false => PPS10R::CONST_0,
            true => PPS10R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PPS10R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PPS10R::CONST_1
    }
}
#[doc = "Possible values of the field `PPS11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS11R {
    #[doc = "Pin Power Save of Pn.x is disabled."]
    CONST_0,
    #[doc = "Pin Power Save of Pn.x is enabled."]
    CONST_1,
}
impl PPS11R {
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
            PPS11R::CONST_0 => false,
            PPS11R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PPS11R {
        match value {
            false => PPS11R::CONST_0,
            true => PPS11R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PPS11R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PPS11R::CONST_1
    }
}
#[doc = "Possible values of the field `PPS12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS12R {
    #[doc = "Pin Power Save of Pn.x is disabled."]
    CONST_0,
    #[doc = "Pin Power Save of Pn.x is enabled."]
    CONST_1,
}
impl PPS12R {
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
            PPS12R::CONST_0 => false,
            PPS12R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PPS12R {
        match value {
            false => PPS12R::CONST_0,
            true => PPS12R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PPS12R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PPS12R::CONST_1
    }
}
#[doc = "Possible values of the field `PPS13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS13R {
    #[doc = "Pin Power Save of Pn.x is disabled."]
    CONST_0,
    #[doc = "Pin Power Save of Pn.x is enabled."]
    CONST_1,
}
impl PPS13R {
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
            PPS13R::CONST_0 => false,
            PPS13R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PPS13R {
        match value {
            false => PPS13R::CONST_0,
            true => PPS13R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PPS13R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PPS13R::CONST_1
    }
}
#[doc = "Possible values of the field `PPS14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS14R {
    #[doc = "Pin Power Save of Pn.x is disabled."]
    CONST_0,
    #[doc = "Pin Power Save of Pn.x is enabled."]
    CONST_1,
}
impl PPS14R {
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
            PPS14R::CONST_0 => false,
            PPS14R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PPS14R {
        match value {
            false => PPS14R::CONST_0,
            true => PPS14R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PPS14R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PPS14R::CONST_1
    }
}
#[doc = "Possible values of the field `PPS15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS15R {
    #[doc = "Pin Power Save of Pn.x is disabled."]
    CONST_0,
    #[doc = "Pin Power Save of Pn.x is enabled."]
    CONST_1,
}
impl PPS15R {
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
            PPS15R::CONST_0 => false,
            PPS15R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PPS15R {
        match value {
            false => PPS15R::CONST_0,
            true => PPS15R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PPS15R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PPS15R::CONST_1
    }
}
#[doc = "Values that can be written to the field `PPS0`"]
pub enum PPS0W {
    #[doc = "Pin Power Save of Pn.x is disabled."]
    CONST_0,
    #[doc = "Pin Power Save of Pn.x is enabled."]
    CONST_1,
}
impl PPS0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PPS0W::CONST_0 => false,
            PPS0W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PPS0W<'a> {
    w: &'a mut W,
}
impl<'a> _PPS0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PPS0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PPS0W::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PPS0W::CONST_1)
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
#[doc = "Values that can be written to the field `PPS1`"]
pub enum PPS1W {
    #[doc = "Pin Power Save of Pn.x is disabled."]
    CONST_0,
    #[doc = "Pin Power Save of Pn.x is enabled."]
    CONST_1,
}
impl PPS1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PPS1W::CONST_0 => false,
            PPS1W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PPS1W<'a> {
    w: &'a mut W,
}
impl<'a> _PPS1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PPS1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PPS1W::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PPS1W::CONST_1)
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
#[doc = "Values that can be written to the field `PPS2`"]
pub enum PPS2W {
    #[doc = "Pin Power Save of Pn.x is disabled."]
    CONST_0,
    #[doc = "Pin Power Save of Pn.x is enabled."]
    CONST_1,
}
impl PPS2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PPS2W::CONST_0 => false,
            PPS2W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PPS2W<'a> {
    w: &'a mut W,
}
impl<'a> _PPS2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PPS2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PPS2W::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PPS2W::CONST_1)
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
#[doc = "Values that can be written to the field `PPS3`"]
pub enum PPS3W {
    #[doc = "Pin Power Save of Pn.x is disabled."]
    CONST_0,
    #[doc = "Pin Power Save of Pn.x is enabled."]
    CONST_1,
}
impl PPS3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PPS3W::CONST_0 => false,
            PPS3W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PPS3W<'a> {
    w: &'a mut W,
}
impl<'a> _PPS3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PPS3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PPS3W::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PPS3W::CONST_1)
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
#[doc = "Values that can be written to the field `PPS4`"]
pub enum PPS4W {
    #[doc = "Pin Power Save of Pn.x is disabled."]
    CONST_0,
    #[doc = "Pin Power Save of Pn.x is enabled."]
    CONST_1,
}
impl PPS4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PPS4W::CONST_0 => false,
            PPS4W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PPS4W<'a> {
    w: &'a mut W,
}
impl<'a> _PPS4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PPS4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PPS4W::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PPS4W::CONST_1)
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
#[doc = "Values that can be written to the field `PPS5`"]
pub enum PPS5W {
    #[doc = "Pin Power Save of Pn.x is disabled."]
    CONST_0,
    #[doc = "Pin Power Save of Pn.x is enabled."]
    CONST_1,
}
impl PPS5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PPS5W::CONST_0 => false,
            PPS5W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PPS5W<'a> {
    w: &'a mut W,
}
impl<'a> _PPS5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PPS5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PPS5W::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PPS5W::CONST_1)
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
#[doc = "Values that can be written to the field `PPS6`"]
pub enum PPS6W {
    #[doc = "Pin Power Save of Pn.x is disabled."]
    CONST_0,
    #[doc = "Pin Power Save of Pn.x is enabled."]
    CONST_1,
}
impl PPS6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PPS6W::CONST_0 => false,
            PPS6W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PPS6W<'a> {
    w: &'a mut W,
}
impl<'a> _PPS6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PPS6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PPS6W::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PPS6W::CONST_1)
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
#[doc = "Values that can be written to the field `PPS7`"]
pub enum PPS7W {
    #[doc = "Pin Power Save of Pn.x is disabled."]
    CONST_0,
    #[doc = "Pin Power Save of Pn.x is enabled."]
    CONST_1,
}
impl PPS7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PPS7W::CONST_0 => false,
            PPS7W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PPS7W<'a> {
    w: &'a mut W,
}
impl<'a> _PPS7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PPS7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PPS7W::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PPS7W::CONST_1)
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
#[doc = "Values that can be written to the field `PPS8`"]
pub enum PPS8W {
    #[doc = "Pin Power Save of Pn.x is disabled."]
    CONST_0,
    #[doc = "Pin Power Save of Pn.x is enabled."]
    CONST_1,
}
impl PPS8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PPS8W::CONST_0 => false,
            PPS8W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PPS8W<'a> {
    w: &'a mut W,
}
impl<'a> _PPS8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PPS8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PPS8W::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PPS8W::CONST_1)
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
#[doc = "Values that can be written to the field `PPS9`"]
pub enum PPS9W {
    #[doc = "Pin Power Save of Pn.x is disabled."]
    CONST_0,
    #[doc = "Pin Power Save of Pn.x is enabled."]
    CONST_1,
}
impl PPS9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PPS9W::CONST_0 => false,
            PPS9W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PPS9W<'a> {
    w: &'a mut W,
}
impl<'a> _PPS9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PPS9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PPS9W::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PPS9W::CONST_1)
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
#[doc = "Values that can be written to the field `PPS10`"]
pub enum PPS10W {
    #[doc = "Pin Power Save of Pn.x is disabled."]
    CONST_0,
    #[doc = "Pin Power Save of Pn.x is enabled."]
    CONST_1,
}
impl PPS10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PPS10W::CONST_0 => false,
            PPS10W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PPS10W<'a> {
    w: &'a mut W,
}
impl<'a> _PPS10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PPS10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PPS10W::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PPS10W::CONST_1)
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
#[doc = "Values that can be written to the field `PPS11`"]
pub enum PPS11W {
    #[doc = "Pin Power Save of Pn.x is disabled."]
    CONST_0,
    #[doc = "Pin Power Save of Pn.x is enabled."]
    CONST_1,
}
impl PPS11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PPS11W::CONST_0 => false,
            PPS11W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PPS11W<'a> {
    w: &'a mut W,
}
impl<'a> _PPS11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PPS11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PPS11W::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PPS11W::CONST_1)
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
#[doc = "Values that can be written to the field `PPS12`"]
pub enum PPS12W {
    #[doc = "Pin Power Save of Pn.x is disabled."]
    CONST_0,
    #[doc = "Pin Power Save of Pn.x is enabled."]
    CONST_1,
}
impl PPS12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PPS12W::CONST_0 => false,
            PPS12W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PPS12W<'a> {
    w: &'a mut W,
}
impl<'a> _PPS12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PPS12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PPS12W::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PPS12W::CONST_1)
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
#[doc = "Values that can be written to the field `PPS13`"]
pub enum PPS13W {
    #[doc = "Pin Power Save of Pn.x is disabled."]
    CONST_0,
    #[doc = "Pin Power Save of Pn.x is enabled."]
    CONST_1,
}
impl PPS13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PPS13W::CONST_0 => false,
            PPS13W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PPS13W<'a> {
    w: &'a mut W,
}
impl<'a> _PPS13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PPS13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PPS13W::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PPS13W::CONST_1)
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
#[doc = "Values that can be written to the field `PPS14`"]
pub enum PPS14W {
    #[doc = "Pin Power Save of Pn.x is disabled."]
    CONST_0,
    #[doc = "Pin Power Save of Pn.x is enabled."]
    CONST_1,
}
impl PPS14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PPS14W::CONST_0 => false,
            PPS14W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PPS14W<'a> {
    w: &'a mut W,
}
impl<'a> _PPS14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PPS14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PPS14W::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PPS14W::CONST_1)
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
#[doc = "Values that can be written to the field `PPS15`"]
pub enum PPS15W {
    #[doc = "Pin Power Save of Pn.x is disabled."]
    CONST_0,
    #[doc = "Pin Power Save of Pn.x is enabled."]
    CONST_1,
}
impl PPS15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PPS15W::CONST_0 => false,
            PPS15W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PPS15W<'a> {
    w: &'a mut W,
}
impl<'a> _PPS15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PPS15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PPS15W::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PPS15W::CONST_1)
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
    #[doc = "Bit 0 - Port n Pin Power Save Bit 0"]
    #[inline]
    pub fn pps0(&self) -> PPS0R {
        PPS0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Port n Pin Power Save Bit 1"]
    #[inline]
    pub fn pps1(&self) -> PPS1R {
        PPS1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Port n Pin Power Save Bit 2"]
    #[inline]
    pub fn pps2(&self) -> PPS2R {
        PPS2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Port n Pin Power Save Bit 3"]
    #[inline]
    pub fn pps3(&self) -> PPS3R {
        PPS3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Port n Pin Power Save Bit 4"]
    #[inline]
    pub fn pps4(&self) -> PPS4R {
        PPS4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Port n Pin Power Save Bit 5"]
    #[inline]
    pub fn pps5(&self) -> PPS5R {
        PPS5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Port n Pin Power Save Bit 6"]
    #[inline]
    pub fn pps6(&self) -> PPS6R {
        PPS6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Port n Pin Power Save Bit 7"]
    #[inline]
    pub fn pps7(&self) -> PPS7R {
        PPS7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Port n Pin Power Save Bit 8"]
    #[inline]
    pub fn pps8(&self) -> PPS8R {
        PPS8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Port n Pin Power Save Bit 9"]
    #[inline]
    pub fn pps9(&self) -> PPS9R {
        PPS9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Port n Pin Power Save Bit 10"]
    #[inline]
    pub fn pps10(&self) -> PPS10R {
        PPS10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Port n Pin Power Save Bit 11"]
    #[inline]
    pub fn pps11(&self) -> PPS11R {
        PPS11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Port n Pin Power Save Bit 12"]
    #[inline]
    pub fn pps12(&self) -> PPS12R {
        PPS12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Port n Pin Power Save Bit 13"]
    #[inline]
    pub fn pps13(&self) -> PPS13R {
        PPS13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Port n Pin Power Save Bit 14"]
    #[inline]
    pub fn pps14(&self) -> PPS14R {
        PPS14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Port n Pin Power Save Bit 15"]
    #[inline]
    pub fn pps15(&self) -> PPS15R {
        PPS15R::_from({
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
    #[doc = "Bit 0 - Port n Pin Power Save Bit 0"]
    #[inline]
    pub fn pps0(&mut self) -> _PPS0W {
        _PPS0W { w: self }
    }
    #[doc = "Bit 1 - Port n Pin Power Save Bit 1"]
    #[inline]
    pub fn pps1(&mut self) -> _PPS1W {
        _PPS1W { w: self }
    }
    #[doc = "Bit 2 - Port n Pin Power Save Bit 2"]
    #[inline]
    pub fn pps2(&mut self) -> _PPS2W {
        _PPS2W { w: self }
    }
    #[doc = "Bit 3 - Port n Pin Power Save Bit 3"]
    #[inline]
    pub fn pps3(&mut self) -> _PPS3W {
        _PPS3W { w: self }
    }
    #[doc = "Bit 4 - Port n Pin Power Save Bit 4"]
    #[inline]
    pub fn pps4(&mut self) -> _PPS4W {
        _PPS4W { w: self }
    }
    #[doc = "Bit 5 - Port n Pin Power Save Bit 5"]
    #[inline]
    pub fn pps5(&mut self) -> _PPS5W {
        _PPS5W { w: self }
    }
    #[doc = "Bit 6 - Port n Pin Power Save Bit 6"]
    #[inline]
    pub fn pps6(&mut self) -> _PPS6W {
        _PPS6W { w: self }
    }
    #[doc = "Bit 7 - Port n Pin Power Save Bit 7"]
    #[inline]
    pub fn pps7(&mut self) -> _PPS7W {
        _PPS7W { w: self }
    }
    #[doc = "Bit 8 - Port n Pin Power Save Bit 8"]
    #[inline]
    pub fn pps8(&mut self) -> _PPS8W {
        _PPS8W { w: self }
    }
    #[doc = "Bit 9 - Port n Pin Power Save Bit 9"]
    #[inline]
    pub fn pps9(&mut self) -> _PPS9W {
        _PPS9W { w: self }
    }
    #[doc = "Bit 10 - Port n Pin Power Save Bit 10"]
    #[inline]
    pub fn pps10(&mut self) -> _PPS10W {
        _PPS10W { w: self }
    }
    #[doc = "Bit 11 - Port n Pin Power Save Bit 11"]
    #[inline]
    pub fn pps11(&mut self) -> _PPS11W {
        _PPS11W { w: self }
    }
    #[doc = "Bit 12 - Port n Pin Power Save Bit 12"]
    #[inline]
    pub fn pps12(&mut self) -> _PPS12W {
        _PPS12W { w: self }
    }
    #[doc = "Bit 13 - Port n Pin Power Save Bit 13"]
    #[inline]
    pub fn pps13(&mut self) -> _PPS13W {
        _PPS13W { w: self }
    }
    #[doc = "Bit 14 - Port n Pin Power Save Bit 14"]
    #[inline]
    pub fn pps14(&mut self) -> _PPS14W {
        _PPS14W { w: self }
    }
    #[doc = "Bit 15 - Port n Pin Power Save Bit 15"]
    #[inline]
    pub fn pps15(&mut self) -> _PPS15W {
        _PPS15W { w: self }
    }
}
