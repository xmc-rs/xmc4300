#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::VFR {
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
#[doc = "Possible values of the field `VF0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VF0R {
    #[doc = "Read access: No new valid data available Write access: No effect"]
    VALUE1,
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2,
}
impl VF0R {
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
            VF0R::VALUE1 => false,
            VF0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VF0R {
        match value {
            false => VF0R::VALUE1,
            true => VF0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VF0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VF0R::VALUE2
    }
}
#[doc = "Possible values of the field `VF1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VF1R {
    #[doc = "Read access: No new valid data available Write access: No effect"]
    VALUE1,
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2,
}
impl VF1R {
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
            VF1R::VALUE1 => false,
            VF1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VF1R {
        match value {
            false => VF1R::VALUE1,
            true => VF1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VF1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VF1R::VALUE2
    }
}
#[doc = "Possible values of the field `VF2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VF2R {
    #[doc = "Read access: No new valid data available Write access: No effect"]
    VALUE1,
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2,
}
impl VF2R {
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
            VF2R::VALUE1 => false,
            VF2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VF2R {
        match value {
            false => VF2R::VALUE1,
            true => VF2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VF2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VF2R::VALUE2
    }
}
#[doc = "Possible values of the field `VF3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VF3R {
    #[doc = "Read access: No new valid data available Write access: No effect"]
    VALUE1,
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2,
}
impl VF3R {
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
            VF3R::VALUE1 => false,
            VF3R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VF3R {
        match value {
            false => VF3R::VALUE1,
            true => VF3R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VF3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VF3R::VALUE2
    }
}
#[doc = "Possible values of the field `VF4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VF4R {
    #[doc = "Read access: No new valid data available Write access: No effect"]
    VALUE1,
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2,
}
impl VF4R {
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
            VF4R::VALUE1 => false,
            VF4R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VF4R {
        match value {
            false => VF4R::VALUE1,
            true => VF4R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VF4R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VF4R::VALUE2
    }
}
#[doc = "Possible values of the field `VF5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VF5R {
    #[doc = "Read access: No new valid data available Write access: No effect"]
    VALUE1,
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2,
}
impl VF5R {
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
            VF5R::VALUE1 => false,
            VF5R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VF5R {
        match value {
            false => VF5R::VALUE1,
            true => VF5R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VF5R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VF5R::VALUE2
    }
}
#[doc = "Possible values of the field `VF6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VF6R {
    #[doc = "Read access: No new valid data available Write access: No effect"]
    VALUE1,
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2,
}
impl VF6R {
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
            VF6R::VALUE1 => false,
            VF6R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VF6R {
        match value {
            false => VF6R::VALUE1,
            true => VF6R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VF6R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VF6R::VALUE2
    }
}
#[doc = "Possible values of the field `VF7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VF7R {
    #[doc = "Read access: No new valid data available Write access: No effect"]
    VALUE1,
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2,
}
impl VF7R {
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
            VF7R::VALUE1 => false,
            VF7R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VF7R {
        match value {
            false => VF7R::VALUE1,
            true => VF7R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VF7R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VF7R::VALUE2
    }
}
#[doc = "Possible values of the field `VF8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VF8R {
    #[doc = "Read access: No new valid data available Write access: No effect"]
    VALUE1,
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2,
}
impl VF8R {
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
            VF8R::VALUE1 => false,
            VF8R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VF8R {
        match value {
            false => VF8R::VALUE1,
            true => VF8R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VF8R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VF8R::VALUE2
    }
}
#[doc = "Possible values of the field `VF9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VF9R {
    #[doc = "Read access: No new valid data available Write access: No effect"]
    VALUE1,
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2,
}
impl VF9R {
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
            VF9R::VALUE1 => false,
            VF9R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VF9R {
        match value {
            false => VF9R::VALUE1,
            true => VF9R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VF9R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VF9R::VALUE2
    }
}
#[doc = "Possible values of the field `VF10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VF10R {
    #[doc = "Read access: No new valid data available Write access: No effect"]
    VALUE1,
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2,
}
impl VF10R {
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
            VF10R::VALUE1 => false,
            VF10R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VF10R {
        match value {
            false => VF10R::VALUE1,
            true => VF10R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VF10R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VF10R::VALUE2
    }
}
#[doc = "Possible values of the field `VF11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VF11R {
    #[doc = "Read access: No new valid data available Write access: No effect"]
    VALUE1,
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2,
}
impl VF11R {
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
            VF11R::VALUE1 => false,
            VF11R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VF11R {
        match value {
            false => VF11R::VALUE1,
            true => VF11R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VF11R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VF11R::VALUE2
    }
}
#[doc = "Possible values of the field `VF12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VF12R {
    #[doc = "Read access: No new valid data available Write access: No effect"]
    VALUE1,
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2,
}
impl VF12R {
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
            VF12R::VALUE1 => false,
            VF12R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VF12R {
        match value {
            false => VF12R::VALUE1,
            true => VF12R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VF12R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VF12R::VALUE2
    }
}
#[doc = "Possible values of the field `VF13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VF13R {
    #[doc = "Read access: No new valid data available Write access: No effect"]
    VALUE1,
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2,
}
impl VF13R {
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
            VF13R::VALUE1 => false,
            VF13R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VF13R {
        match value {
            false => VF13R::VALUE1,
            true => VF13R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VF13R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VF13R::VALUE2
    }
}
#[doc = "Possible values of the field `VF14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VF14R {
    #[doc = "Read access: No new valid data available Write access: No effect"]
    VALUE1,
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2,
}
impl VF14R {
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
            VF14R::VALUE1 => false,
            VF14R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VF14R {
        match value {
            false => VF14R::VALUE1,
            true => VF14R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VF14R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VF14R::VALUE2
    }
}
#[doc = "Possible values of the field `VF15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VF15R {
    #[doc = "Read access: No new valid data available Write access: No effect"]
    VALUE1,
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2,
}
impl VF15R {
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
            VF15R::VALUE1 => false,
            VF15R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VF15R {
        match value {
            false => VF15R::VALUE1,
            true => VF15R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VF15R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VF15R::VALUE2
    }
}
#[doc = "Values that can be written to the field `VF0`"]
pub enum VF0W {
    #[doc = "Read access: No new valid data available Write access: No effect"]
    VALUE1,
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2,
}
impl VF0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VF0W::VALUE1 => false,
            VF0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VF0W<'a> {
    w: &'a mut W,
}
impl<'a> _VF0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VF0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(VF0W::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(VF0W::VALUE2)
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
#[doc = "Values that can be written to the field `VF1`"]
pub enum VF1W {
    #[doc = "Read access: No new valid data available Write access: No effect"]
    VALUE1,
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2,
}
impl VF1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VF1W::VALUE1 => false,
            VF1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VF1W<'a> {
    w: &'a mut W,
}
impl<'a> _VF1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VF1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(VF1W::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(VF1W::VALUE2)
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
#[doc = "Values that can be written to the field `VF2`"]
pub enum VF2W {
    #[doc = "Read access: No new valid data available Write access: No effect"]
    VALUE1,
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2,
}
impl VF2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VF2W::VALUE1 => false,
            VF2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VF2W<'a> {
    w: &'a mut W,
}
impl<'a> _VF2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VF2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(VF2W::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(VF2W::VALUE2)
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
#[doc = "Values that can be written to the field `VF3`"]
pub enum VF3W {
    #[doc = "Read access: No new valid data available Write access: No effect"]
    VALUE1,
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2,
}
impl VF3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VF3W::VALUE1 => false,
            VF3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VF3W<'a> {
    w: &'a mut W,
}
impl<'a> _VF3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VF3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(VF3W::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(VF3W::VALUE2)
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
#[doc = "Values that can be written to the field `VF4`"]
pub enum VF4W {
    #[doc = "Read access: No new valid data available Write access: No effect"]
    VALUE1,
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2,
}
impl VF4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VF4W::VALUE1 => false,
            VF4W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VF4W<'a> {
    w: &'a mut W,
}
impl<'a> _VF4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VF4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(VF4W::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(VF4W::VALUE2)
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
#[doc = "Values that can be written to the field `VF5`"]
pub enum VF5W {
    #[doc = "Read access: No new valid data available Write access: No effect"]
    VALUE1,
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2,
}
impl VF5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VF5W::VALUE1 => false,
            VF5W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VF5W<'a> {
    w: &'a mut W,
}
impl<'a> _VF5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VF5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(VF5W::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(VF5W::VALUE2)
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
#[doc = "Values that can be written to the field `VF6`"]
pub enum VF6W {
    #[doc = "Read access: No new valid data available Write access: No effect"]
    VALUE1,
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2,
}
impl VF6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VF6W::VALUE1 => false,
            VF6W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VF6W<'a> {
    w: &'a mut W,
}
impl<'a> _VF6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VF6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(VF6W::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(VF6W::VALUE2)
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
#[doc = "Values that can be written to the field `VF7`"]
pub enum VF7W {
    #[doc = "Read access: No new valid data available Write access: No effect"]
    VALUE1,
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2,
}
impl VF7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VF7W::VALUE1 => false,
            VF7W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VF7W<'a> {
    w: &'a mut W,
}
impl<'a> _VF7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VF7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(VF7W::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(VF7W::VALUE2)
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
#[doc = "Values that can be written to the field `VF8`"]
pub enum VF8W {
    #[doc = "Read access: No new valid data available Write access: No effect"]
    VALUE1,
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2,
}
impl VF8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VF8W::VALUE1 => false,
            VF8W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VF8W<'a> {
    w: &'a mut W,
}
impl<'a> _VF8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VF8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(VF8W::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(VF8W::VALUE2)
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
#[doc = "Values that can be written to the field `VF9`"]
pub enum VF9W {
    #[doc = "Read access: No new valid data available Write access: No effect"]
    VALUE1,
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2,
}
impl VF9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VF9W::VALUE1 => false,
            VF9W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VF9W<'a> {
    w: &'a mut W,
}
impl<'a> _VF9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VF9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(VF9W::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(VF9W::VALUE2)
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
#[doc = "Values that can be written to the field `VF10`"]
pub enum VF10W {
    #[doc = "Read access: No new valid data available Write access: No effect"]
    VALUE1,
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2,
}
impl VF10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VF10W::VALUE1 => false,
            VF10W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VF10W<'a> {
    w: &'a mut W,
}
impl<'a> _VF10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VF10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(VF10W::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(VF10W::VALUE2)
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
#[doc = "Values that can be written to the field `VF11`"]
pub enum VF11W {
    #[doc = "Read access: No new valid data available Write access: No effect"]
    VALUE1,
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2,
}
impl VF11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VF11W::VALUE1 => false,
            VF11W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VF11W<'a> {
    w: &'a mut W,
}
impl<'a> _VF11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VF11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(VF11W::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(VF11W::VALUE2)
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
#[doc = "Values that can be written to the field `VF12`"]
pub enum VF12W {
    #[doc = "Read access: No new valid data available Write access: No effect"]
    VALUE1,
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2,
}
impl VF12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VF12W::VALUE1 => false,
            VF12W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VF12W<'a> {
    w: &'a mut W,
}
impl<'a> _VF12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VF12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(VF12W::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(VF12W::VALUE2)
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
#[doc = "Values that can be written to the field `VF13`"]
pub enum VF13W {
    #[doc = "Read access: No new valid data available Write access: No effect"]
    VALUE1,
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2,
}
impl VF13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VF13W::VALUE1 => false,
            VF13W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VF13W<'a> {
    w: &'a mut W,
}
impl<'a> _VF13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VF13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(VF13W::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(VF13W::VALUE2)
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
#[doc = "Values that can be written to the field `VF14`"]
pub enum VF14W {
    #[doc = "Read access: No new valid data available Write access: No effect"]
    VALUE1,
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2,
}
impl VF14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VF14W::VALUE1 => false,
            VF14W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VF14W<'a> {
    w: &'a mut W,
}
impl<'a> _VF14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VF14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(VF14W::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(VF14W::VALUE2)
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
#[doc = "Values that can be written to the field `VF15`"]
pub enum VF15W {
    #[doc = "Read access: No new valid data available Write access: No effect"]
    VALUE1,
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2,
}
impl VF15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VF15W::VALUE1 => false,
            VF15W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VF15W<'a> {
    w: &'a mut W,
}
impl<'a> _VF15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VF15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(VF15W::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(VF15W::VALUE2)
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
    #[doc = "Bit 0 - Valid Flag of Result Register x"]
    #[inline]
    pub fn vf0(&self) -> VF0R {
        VF0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Valid Flag of Result Register x"]
    #[inline]
    pub fn vf1(&self) -> VF1R {
        VF1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Valid Flag of Result Register x"]
    #[inline]
    pub fn vf2(&self) -> VF2R {
        VF2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Valid Flag of Result Register x"]
    #[inline]
    pub fn vf3(&self) -> VF3R {
        VF3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Valid Flag of Result Register x"]
    #[inline]
    pub fn vf4(&self) -> VF4R {
        VF4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Valid Flag of Result Register x"]
    #[inline]
    pub fn vf5(&self) -> VF5R {
        VF5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Valid Flag of Result Register x"]
    #[inline]
    pub fn vf6(&self) -> VF6R {
        VF6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Valid Flag of Result Register x"]
    #[inline]
    pub fn vf7(&self) -> VF7R {
        VF7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Valid Flag of Result Register x"]
    #[inline]
    pub fn vf8(&self) -> VF8R {
        VF8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Valid Flag of Result Register x"]
    #[inline]
    pub fn vf9(&self) -> VF9R {
        VF9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Valid Flag of Result Register x"]
    #[inline]
    pub fn vf10(&self) -> VF10R {
        VF10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Valid Flag of Result Register x"]
    #[inline]
    pub fn vf11(&self) -> VF11R {
        VF11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Valid Flag of Result Register x"]
    #[inline]
    pub fn vf12(&self) -> VF12R {
        VF12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Valid Flag of Result Register x"]
    #[inline]
    pub fn vf13(&self) -> VF13R {
        VF13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Valid Flag of Result Register x"]
    #[inline]
    pub fn vf14(&self) -> VF14R {
        VF14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Valid Flag of Result Register x"]
    #[inline]
    pub fn vf15(&self) -> VF15R {
        VF15R::_from({
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
    #[doc = "Bit 0 - Valid Flag of Result Register x"]
    #[inline]
    pub fn vf0(&mut self) -> _VF0W {
        _VF0W { w: self }
    }
    #[doc = "Bit 1 - Valid Flag of Result Register x"]
    #[inline]
    pub fn vf1(&mut self) -> _VF1W {
        _VF1W { w: self }
    }
    #[doc = "Bit 2 - Valid Flag of Result Register x"]
    #[inline]
    pub fn vf2(&mut self) -> _VF2W {
        _VF2W { w: self }
    }
    #[doc = "Bit 3 - Valid Flag of Result Register x"]
    #[inline]
    pub fn vf3(&mut self) -> _VF3W {
        _VF3W { w: self }
    }
    #[doc = "Bit 4 - Valid Flag of Result Register x"]
    #[inline]
    pub fn vf4(&mut self) -> _VF4W {
        _VF4W { w: self }
    }
    #[doc = "Bit 5 - Valid Flag of Result Register x"]
    #[inline]
    pub fn vf5(&mut self) -> _VF5W {
        _VF5W { w: self }
    }
    #[doc = "Bit 6 - Valid Flag of Result Register x"]
    #[inline]
    pub fn vf6(&mut self) -> _VF6W {
        _VF6W { w: self }
    }
    #[doc = "Bit 7 - Valid Flag of Result Register x"]
    #[inline]
    pub fn vf7(&mut self) -> _VF7W {
        _VF7W { w: self }
    }
    #[doc = "Bit 8 - Valid Flag of Result Register x"]
    #[inline]
    pub fn vf8(&mut self) -> _VF8W {
        _VF8W { w: self }
    }
    #[doc = "Bit 9 - Valid Flag of Result Register x"]
    #[inline]
    pub fn vf9(&mut self) -> _VF9W {
        _VF9W { w: self }
    }
    #[doc = "Bit 10 - Valid Flag of Result Register x"]
    #[inline]
    pub fn vf10(&mut self) -> _VF10W {
        _VF10W { w: self }
    }
    #[doc = "Bit 11 - Valid Flag of Result Register x"]
    #[inline]
    pub fn vf11(&mut self) -> _VF11W {
        _VF11W { w: self }
    }
    #[doc = "Bit 12 - Valid Flag of Result Register x"]
    #[inline]
    pub fn vf12(&mut self) -> _VF12W {
        _VF12W { w: self }
    }
    #[doc = "Bit 13 - Valid Flag of Result Register x"]
    #[inline]
    pub fn vf13(&mut self) -> _VF13W {
        _VF13W { w: self }
    }
    #[doc = "Bit 14 - Valid Flag of Result Register x"]
    #[inline]
    pub fn vf14(&mut self) -> _VF14W {
        _VF14W { w: self }
    }
    #[doc = "Bit 15 - Valid Flag of Result Register x"]
    #[inline]
    pub fn vf15(&mut self) -> _VF15W {
        _VF15W { w: self }
    }
}
