#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HWSEL {
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
#[doc = "Possible values of the field `HW0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW0R {
    #[doc = "Software control only."]
    CONST_00,
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HW0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HW0R::CONST_00 => 0,
            HW0R::CONST_01 => 1,
            HW0R::CONST_10 => 2,
            HW0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HW0R {
        match value {
            0 => HW0R::CONST_00,
            1 => HW0R::CONST_01,
            2 => HW0R::CONST_10,
            i => HW0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline]
    pub fn is_const_00(&self) -> bool {
        *self == HW0R::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline]
    pub fn is_const_01(&self) -> bool {
        *self == HW0R::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline]
    pub fn is_const_10(&self) -> bool {
        *self == HW0R::CONST_10
    }
}
#[doc = "Possible values of the field `HW1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW1R {
    #[doc = "Software control only."]
    CONST_00,
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HW1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HW1R::CONST_00 => 0,
            HW1R::CONST_01 => 1,
            HW1R::CONST_10 => 2,
            HW1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HW1R {
        match value {
            0 => HW1R::CONST_00,
            1 => HW1R::CONST_01,
            2 => HW1R::CONST_10,
            i => HW1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline]
    pub fn is_const_00(&self) -> bool {
        *self == HW1R::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline]
    pub fn is_const_01(&self) -> bool {
        *self == HW1R::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline]
    pub fn is_const_10(&self) -> bool {
        *self == HW1R::CONST_10
    }
}
#[doc = "Possible values of the field `HW2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW2R {
    #[doc = "Software control only."]
    CONST_00,
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HW2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HW2R::CONST_00 => 0,
            HW2R::CONST_01 => 1,
            HW2R::CONST_10 => 2,
            HW2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HW2R {
        match value {
            0 => HW2R::CONST_00,
            1 => HW2R::CONST_01,
            2 => HW2R::CONST_10,
            i => HW2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline]
    pub fn is_const_00(&self) -> bool {
        *self == HW2R::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline]
    pub fn is_const_01(&self) -> bool {
        *self == HW2R::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline]
    pub fn is_const_10(&self) -> bool {
        *self == HW2R::CONST_10
    }
}
#[doc = "Possible values of the field `HW3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW3R {
    #[doc = "Software control only."]
    CONST_00,
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HW3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HW3R::CONST_00 => 0,
            HW3R::CONST_01 => 1,
            HW3R::CONST_10 => 2,
            HW3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HW3R {
        match value {
            0 => HW3R::CONST_00,
            1 => HW3R::CONST_01,
            2 => HW3R::CONST_10,
            i => HW3R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline]
    pub fn is_const_00(&self) -> bool {
        *self == HW3R::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline]
    pub fn is_const_01(&self) -> bool {
        *self == HW3R::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline]
    pub fn is_const_10(&self) -> bool {
        *self == HW3R::CONST_10
    }
}
#[doc = "Possible values of the field `HW4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW4R {
    #[doc = "Software control only."]
    CONST_00,
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HW4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HW4R::CONST_00 => 0,
            HW4R::CONST_01 => 1,
            HW4R::CONST_10 => 2,
            HW4R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HW4R {
        match value {
            0 => HW4R::CONST_00,
            1 => HW4R::CONST_01,
            2 => HW4R::CONST_10,
            i => HW4R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline]
    pub fn is_const_00(&self) -> bool {
        *self == HW4R::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline]
    pub fn is_const_01(&self) -> bool {
        *self == HW4R::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline]
    pub fn is_const_10(&self) -> bool {
        *self == HW4R::CONST_10
    }
}
#[doc = "Possible values of the field `HW5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW5R {
    #[doc = "Software control only."]
    CONST_00,
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HW5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HW5R::CONST_00 => 0,
            HW5R::CONST_01 => 1,
            HW5R::CONST_10 => 2,
            HW5R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HW5R {
        match value {
            0 => HW5R::CONST_00,
            1 => HW5R::CONST_01,
            2 => HW5R::CONST_10,
            i => HW5R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline]
    pub fn is_const_00(&self) -> bool {
        *self == HW5R::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline]
    pub fn is_const_01(&self) -> bool {
        *self == HW5R::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline]
    pub fn is_const_10(&self) -> bool {
        *self == HW5R::CONST_10
    }
}
#[doc = "Possible values of the field `HW6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW6R {
    #[doc = "Software control only."]
    CONST_00,
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HW6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HW6R::CONST_00 => 0,
            HW6R::CONST_01 => 1,
            HW6R::CONST_10 => 2,
            HW6R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HW6R {
        match value {
            0 => HW6R::CONST_00,
            1 => HW6R::CONST_01,
            2 => HW6R::CONST_10,
            i => HW6R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline]
    pub fn is_const_00(&self) -> bool {
        *self == HW6R::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline]
    pub fn is_const_01(&self) -> bool {
        *self == HW6R::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline]
    pub fn is_const_10(&self) -> bool {
        *self == HW6R::CONST_10
    }
}
#[doc = "Possible values of the field `HW7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW7R {
    #[doc = "Software control only."]
    CONST_00,
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HW7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HW7R::CONST_00 => 0,
            HW7R::CONST_01 => 1,
            HW7R::CONST_10 => 2,
            HW7R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HW7R {
        match value {
            0 => HW7R::CONST_00,
            1 => HW7R::CONST_01,
            2 => HW7R::CONST_10,
            i => HW7R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline]
    pub fn is_const_00(&self) -> bool {
        *self == HW7R::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline]
    pub fn is_const_01(&self) -> bool {
        *self == HW7R::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline]
    pub fn is_const_10(&self) -> bool {
        *self == HW7R::CONST_10
    }
}
#[doc = "Possible values of the field `HW8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW8R {
    #[doc = "Software control only."]
    CONST_00,
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HW8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HW8R::CONST_00 => 0,
            HW8R::CONST_01 => 1,
            HW8R::CONST_10 => 2,
            HW8R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HW8R {
        match value {
            0 => HW8R::CONST_00,
            1 => HW8R::CONST_01,
            2 => HW8R::CONST_10,
            i => HW8R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline]
    pub fn is_const_00(&self) -> bool {
        *self == HW8R::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline]
    pub fn is_const_01(&self) -> bool {
        *self == HW8R::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline]
    pub fn is_const_10(&self) -> bool {
        *self == HW8R::CONST_10
    }
}
#[doc = "Possible values of the field `HW9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW9R {
    #[doc = "Software control only."]
    CONST_00,
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HW9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HW9R::CONST_00 => 0,
            HW9R::CONST_01 => 1,
            HW9R::CONST_10 => 2,
            HW9R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HW9R {
        match value {
            0 => HW9R::CONST_00,
            1 => HW9R::CONST_01,
            2 => HW9R::CONST_10,
            i => HW9R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline]
    pub fn is_const_00(&self) -> bool {
        *self == HW9R::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline]
    pub fn is_const_01(&self) -> bool {
        *self == HW9R::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline]
    pub fn is_const_10(&self) -> bool {
        *self == HW9R::CONST_10
    }
}
#[doc = "Possible values of the field `HW10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW10R {
    #[doc = "Software control only."]
    CONST_00,
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HW10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HW10R::CONST_00 => 0,
            HW10R::CONST_01 => 1,
            HW10R::CONST_10 => 2,
            HW10R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HW10R {
        match value {
            0 => HW10R::CONST_00,
            1 => HW10R::CONST_01,
            2 => HW10R::CONST_10,
            i => HW10R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline]
    pub fn is_const_00(&self) -> bool {
        *self == HW10R::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline]
    pub fn is_const_01(&self) -> bool {
        *self == HW10R::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline]
    pub fn is_const_10(&self) -> bool {
        *self == HW10R::CONST_10
    }
}
#[doc = "Possible values of the field `HW11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW11R {
    #[doc = "Software control only."]
    CONST_00,
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HW11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HW11R::CONST_00 => 0,
            HW11R::CONST_01 => 1,
            HW11R::CONST_10 => 2,
            HW11R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HW11R {
        match value {
            0 => HW11R::CONST_00,
            1 => HW11R::CONST_01,
            2 => HW11R::CONST_10,
            i => HW11R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline]
    pub fn is_const_00(&self) -> bool {
        *self == HW11R::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline]
    pub fn is_const_01(&self) -> bool {
        *self == HW11R::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline]
    pub fn is_const_10(&self) -> bool {
        *self == HW11R::CONST_10
    }
}
#[doc = "Possible values of the field `HW12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW12R {
    #[doc = "Software control only."]
    CONST_00,
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HW12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HW12R::CONST_00 => 0,
            HW12R::CONST_01 => 1,
            HW12R::CONST_10 => 2,
            HW12R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HW12R {
        match value {
            0 => HW12R::CONST_00,
            1 => HW12R::CONST_01,
            2 => HW12R::CONST_10,
            i => HW12R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline]
    pub fn is_const_00(&self) -> bool {
        *self == HW12R::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline]
    pub fn is_const_01(&self) -> bool {
        *self == HW12R::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline]
    pub fn is_const_10(&self) -> bool {
        *self == HW12R::CONST_10
    }
}
#[doc = "Possible values of the field `HW13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW13R {
    #[doc = "Software control only."]
    CONST_00,
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HW13R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HW13R::CONST_00 => 0,
            HW13R::CONST_01 => 1,
            HW13R::CONST_10 => 2,
            HW13R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HW13R {
        match value {
            0 => HW13R::CONST_00,
            1 => HW13R::CONST_01,
            2 => HW13R::CONST_10,
            i => HW13R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline]
    pub fn is_const_00(&self) -> bool {
        *self == HW13R::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline]
    pub fn is_const_01(&self) -> bool {
        *self == HW13R::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline]
    pub fn is_const_10(&self) -> bool {
        *self == HW13R::CONST_10
    }
}
#[doc = "Possible values of the field `HW14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW14R {
    #[doc = "Software control only."]
    CONST_00,
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HW14R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HW14R::CONST_00 => 0,
            HW14R::CONST_01 => 1,
            HW14R::CONST_10 => 2,
            HW14R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HW14R {
        match value {
            0 => HW14R::CONST_00,
            1 => HW14R::CONST_01,
            2 => HW14R::CONST_10,
            i => HW14R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline]
    pub fn is_const_00(&self) -> bool {
        *self == HW14R::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline]
    pub fn is_const_01(&self) -> bool {
        *self == HW14R::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline]
    pub fn is_const_10(&self) -> bool {
        *self == HW14R::CONST_10
    }
}
#[doc = "Possible values of the field `HW15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW15R {
    #[doc = "Software control only."]
    CONST_00,
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HW15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HW15R::CONST_00 => 0,
            HW15R::CONST_01 => 1,
            HW15R::CONST_10 => 2,
            HW15R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HW15R {
        match value {
            0 => HW15R::CONST_00,
            1 => HW15R::CONST_01,
            2 => HW15R::CONST_10,
            i => HW15R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline]
    pub fn is_const_00(&self) -> bool {
        *self == HW15R::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline]
    pub fn is_const_01(&self) -> bool {
        *self == HW15R::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline]
    pub fn is_const_10(&self) -> bool {
        *self == HW15R::CONST_10
    }
}
#[doc = "Values that can be written to the field `HW0`"]
pub enum HW0W {
    #[doc = "Software control only."]
    CONST_00,
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
}
impl HW0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HW0W::CONST_00 => 0,
            HW0W::CONST_01 => 1,
            HW0W::CONST_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HW0W<'a> {
    w: &'a mut W,
}
impl<'a> _HW0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HW0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Software control only."]
    #[inline]
    pub fn const_00(self) -> &'a mut W {
        self.variant(HW0W::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline]
    pub fn const_01(self) -> &'a mut W {
        self.variant(HW0W::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline]
    pub fn const_10(self) -> &'a mut W {
        self.variant(HW0W::CONST_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HW1`"]
pub enum HW1W {
    #[doc = "Software control only."]
    CONST_00,
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
}
impl HW1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HW1W::CONST_00 => 0,
            HW1W::CONST_01 => 1,
            HW1W::CONST_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HW1W<'a> {
    w: &'a mut W,
}
impl<'a> _HW1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HW1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Software control only."]
    #[inline]
    pub fn const_00(self) -> &'a mut W {
        self.variant(HW1W::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline]
    pub fn const_01(self) -> &'a mut W {
        self.variant(HW1W::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline]
    pub fn const_10(self) -> &'a mut W {
        self.variant(HW1W::CONST_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HW2`"]
pub enum HW2W {
    #[doc = "Software control only."]
    CONST_00,
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
}
impl HW2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HW2W::CONST_00 => 0,
            HW2W::CONST_01 => 1,
            HW2W::CONST_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HW2W<'a> {
    w: &'a mut W,
}
impl<'a> _HW2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HW2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Software control only."]
    #[inline]
    pub fn const_00(self) -> &'a mut W {
        self.variant(HW2W::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline]
    pub fn const_01(self) -> &'a mut W {
        self.variant(HW2W::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline]
    pub fn const_10(self) -> &'a mut W {
        self.variant(HW2W::CONST_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HW3`"]
pub enum HW3W {
    #[doc = "Software control only."]
    CONST_00,
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
}
impl HW3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HW3W::CONST_00 => 0,
            HW3W::CONST_01 => 1,
            HW3W::CONST_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HW3W<'a> {
    w: &'a mut W,
}
impl<'a> _HW3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HW3W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Software control only."]
    #[inline]
    pub fn const_00(self) -> &'a mut W {
        self.variant(HW3W::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline]
    pub fn const_01(self) -> &'a mut W {
        self.variant(HW3W::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline]
    pub fn const_10(self) -> &'a mut W {
        self.variant(HW3W::CONST_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HW4`"]
pub enum HW4W {
    #[doc = "Software control only."]
    CONST_00,
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
}
impl HW4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HW4W::CONST_00 => 0,
            HW4W::CONST_01 => 1,
            HW4W::CONST_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HW4W<'a> {
    w: &'a mut W,
}
impl<'a> _HW4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HW4W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Software control only."]
    #[inline]
    pub fn const_00(self) -> &'a mut W {
        self.variant(HW4W::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline]
    pub fn const_01(self) -> &'a mut W {
        self.variant(HW4W::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline]
    pub fn const_10(self) -> &'a mut W {
        self.variant(HW4W::CONST_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HW5`"]
pub enum HW5W {
    #[doc = "Software control only."]
    CONST_00,
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
}
impl HW5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HW5W::CONST_00 => 0,
            HW5W::CONST_01 => 1,
            HW5W::CONST_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HW5W<'a> {
    w: &'a mut W,
}
impl<'a> _HW5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HW5W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Software control only."]
    #[inline]
    pub fn const_00(self) -> &'a mut W {
        self.variant(HW5W::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline]
    pub fn const_01(self) -> &'a mut W {
        self.variant(HW5W::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline]
    pub fn const_10(self) -> &'a mut W {
        self.variant(HW5W::CONST_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HW6`"]
pub enum HW6W {
    #[doc = "Software control only."]
    CONST_00,
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
}
impl HW6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HW6W::CONST_00 => 0,
            HW6W::CONST_01 => 1,
            HW6W::CONST_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HW6W<'a> {
    w: &'a mut W,
}
impl<'a> _HW6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HW6W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Software control only."]
    #[inline]
    pub fn const_00(self) -> &'a mut W {
        self.variant(HW6W::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline]
    pub fn const_01(self) -> &'a mut W {
        self.variant(HW6W::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline]
    pub fn const_10(self) -> &'a mut W {
        self.variant(HW6W::CONST_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HW7`"]
pub enum HW7W {
    #[doc = "Software control only."]
    CONST_00,
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
}
impl HW7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HW7W::CONST_00 => 0,
            HW7W::CONST_01 => 1,
            HW7W::CONST_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HW7W<'a> {
    w: &'a mut W,
}
impl<'a> _HW7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HW7W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Software control only."]
    #[inline]
    pub fn const_00(self) -> &'a mut W {
        self.variant(HW7W::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline]
    pub fn const_01(self) -> &'a mut W {
        self.variant(HW7W::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline]
    pub fn const_10(self) -> &'a mut W {
        self.variant(HW7W::CONST_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HW8`"]
pub enum HW8W {
    #[doc = "Software control only."]
    CONST_00,
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
}
impl HW8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HW8W::CONST_00 => 0,
            HW8W::CONST_01 => 1,
            HW8W::CONST_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HW8W<'a> {
    w: &'a mut W,
}
impl<'a> _HW8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HW8W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Software control only."]
    #[inline]
    pub fn const_00(self) -> &'a mut W {
        self.variant(HW8W::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline]
    pub fn const_01(self) -> &'a mut W {
        self.variant(HW8W::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline]
    pub fn const_10(self) -> &'a mut W {
        self.variant(HW8W::CONST_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HW9`"]
pub enum HW9W {
    #[doc = "Software control only."]
    CONST_00,
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
}
impl HW9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HW9W::CONST_00 => 0,
            HW9W::CONST_01 => 1,
            HW9W::CONST_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HW9W<'a> {
    w: &'a mut W,
}
impl<'a> _HW9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HW9W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Software control only."]
    #[inline]
    pub fn const_00(self) -> &'a mut W {
        self.variant(HW9W::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline]
    pub fn const_01(self) -> &'a mut W {
        self.variant(HW9W::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline]
    pub fn const_10(self) -> &'a mut W {
        self.variant(HW9W::CONST_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HW10`"]
pub enum HW10W {
    #[doc = "Software control only."]
    CONST_00,
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
}
impl HW10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HW10W::CONST_00 => 0,
            HW10W::CONST_01 => 1,
            HW10W::CONST_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HW10W<'a> {
    w: &'a mut W,
}
impl<'a> _HW10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HW10W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Software control only."]
    #[inline]
    pub fn const_00(self) -> &'a mut W {
        self.variant(HW10W::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline]
    pub fn const_01(self) -> &'a mut W {
        self.variant(HW10W::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline]
    pub fn const_10(self) -> &'a mut W {
        self.variant(HW10W::CONST_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HW11`"]
pub enum HW11W {
    #[doc = "Software control only."]
    CONST_00,
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
}
impl HW11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HW11W::CONST_00 => 0,
            HW11W::CONST_01 => 1,
            HW11W::CONST_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HW11W<'a> {
    w: &'a mut W,
}
impl<'a> _HW11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HW11W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Software control only."]
    #[inline]
    pub fn const_00(self) -> &'a mut W {
        self.variant(HW11W::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline]
    pub fn const_01(self) -> &'a mut W {
        self.variant(HW11W::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline]
    pub fn const_10(self) -> &'a mut W {
        self.variant(HW11W::CONST_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HW12`"]
pub enum HW12W {
    #[doc = "Software control only."]
    CONST_00,
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
}
impl HW12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HW12W::CONST_00 => 0,
            HW12W::CONST_01 => 1,
            HW12W::CONST_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HW12W<'a> {
    w: &'a mut W,
}
impl<'a> _HW12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HW12W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Software control only."]
    #[inline]
    pub fn const_00(self) -> &'a mut W {
        self.variant(HW12W::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline]
    pub fn const_01(self) -> &'a mut W {
        self.variant(HW12W::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline]
    pub fn const_10(self) -> &'a mut W {
        self.variant(HW12W::CONST_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HW13`"]
pub enum HW13W {
    #[doc = "Software control only."]
    CONST_00,
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
}
impl HW13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HW13W::CONST_00 => 0,
            HW13W::CONST_01 => 1,
            HW13W::CONST_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HW13W<'a> {
    w: &'a mut W,
}
impl<'a> _HW13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HW13W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Software control only."]
    #[inline]
    pub fn const_00(self) -> &'a mut W {
        self.variant(HW13W::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline]
    pub fn const_01(self) -> &'a mut W {
        self.variant(HW13W::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline]
    pub fn const_10(self) -> &'a mut W {
        self.variant(HW13W::CONST_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HW14`"]
pub enum HW14W {
    #[doc = "Software control only."]
    CONST_00,
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
}
impl HW14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HW14W::CONST_00 => 0,
            HW14W::CONST_01 => 1,
            HW14W::CONST_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HW14W<'a> {
    w: &'a mut W,
}
impl<'a> _HW14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HW14W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Software control only."]
    #[inline]
    pub fn const_00(self) -> &'a mut W {
        self.variant(HW14W::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline]
    pub fn const_01(self) -> &'a mut W {
        self.variant(HW14W::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline]
    pub fn const_10(self) -> &'a mut W {
        self.variant(HW14W::CONST_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HW15`"]
pub enum HW15W {
    #[doc = "Software control only."]
    CONST_00,
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    CONST_01,
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    CONST_10,
}
impl HW15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HW15W::CONST_00 => 0,
            HW15W::CONST_01 => 1,
            HW15W::CONST_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HW15W<'a> {
    w: &'a mut W,
}
impl<'a> _HW15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HW15W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Software control only."]
    #[inline]
    pub fn const_00(self) -> &'a mut W {
        self.variant(HW15W::CONST_00)
    }
    #[doc = "HWI0/HWO0 control path can override the software configuration."]
    #[inline]
    pub fn const_01(self) -> &'a mut W {
        self.variant(HW15W::CONST_01)
    }
    #[doc = "HWI1/HWO1 control path can override the software configuration."]
    #[inline]
    pub fn const_10(self) -> &'a mut W {
        self.variant(HW15W::CONST_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:1 - Port n Pin Hardware Select Bit 0"]
    #[inline]
    pub fn hw0(&self) -> HW0R {
        HW0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Port n Pin Hardware Select Bit 1"]
    #[inline]
    pub fn hw1(&self) -> HW1R {
        HW1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Port n Pin Hardware Select Bit 2"]
    #[inline]
    pub fn hw2(&self) -> HW2R {
        HW2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Port n Pin Hardware Select Bit 3"]
    #[inline]
    pub fn hw3(&self) -> HW3R {
        HW3R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Port n Pin Hardware Select Bit 4"]
    #[inline]
    pub fn hw4(&self) -> HW4R {
        HW4R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Port n Pin Hardware Select Bit 5"]
    #[inline]
    pub fn hw5(&self) -> HW5R {
        HW5R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Port n Pin Hardware Select Bit 6"]
    #[inline]
    pub fn hw6(&self) -> HW6R {
        HW6R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Port n Pin Hardware Select Bit 7"]
    #[inline]
    pub fn hw7(&self) -> HW7R {
        HW7R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Port n Pin Hardware Select Bit 8"]
    #[inline]
    pub fn hw8(&self) -> HW8R {
        HW8R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - Port n Pin Hardware Select Bit 9"]
    #[inline]
    pub fn hw9(&self) -> HW9R {
        HW9R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Port n Pin Hardware Select Bit 10"]
    #[inline]
    pub fn hw10(&self) -> HW10R {
        HW10R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - Port n Pin Hardware Select Bit 11"]
    #[inline]
    pub fn hw11(&self) -> HW11R {
        HW11R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - Port n Pin Hardware Select Bit 12"]
    #[inline]
    pub fn hw12(&self) -> HW12R {
        HW12R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:27 - Port n Pin Hardware Select Bit 13"]
    #[inline]
    pub fn hw13(&self) -> HW13R {
        HW13R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:29 - Port n Pin Hardware Select Bit 14"]
    #[inline]
    pub fn hw14(&self) -> HW14R {
        HW14R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 30:31 - Port n Pin Hardware Select Bit 15"]
    #[inline]
    pub fn hw15(&self) -> HW15R {
        HW15R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 0:1 - Port n Pin Hardware Select Bit 0"]
    #[inline]
    pub fn hw0(&mut self) -> _HW0W {
        _HW0W { w: self }
    }
    #[doc = "Bits 2:3 - Port n Pin Hardware Select Bit 1"]
    #[inline]
    pub fn hw1(&mut self) -> _HW1W {
        _HW1W { w: self }
    }
    #[doc = "Bits 4:5 - Port n Pin Hardware Select Bit 2"]
    #[inline]
    pub fn hw2(&mut self) -> _HW2W {
        _HW2W { w: self }
    }
    #[doc = "Bits 6:7 - Port n Pin Hardware Select Bit 3"]
    #[inline]
    pub fn hw3(&mut self) -> _HW3W {
        _HW3W { w: self }
    }
    #[doc = "Bits 8:9 - Port n Pin Hardware Select Bit 4"]
    #[inline]
    pub fn hw4(&mut self) -> _HW4W {
        _HW4W { w: self }
    }
    #[doc = "Bits 10:11 - Port n Pin Hardware Select Bit 5"]
    #[inline]
    pub fn hw5(&mut self) -> _HW5W {
        _HW5W { w: self }
    }
    #[doc = "Bits 12:13 - Port n Pin Hardware Select Bit 6"]
    #[inline]
    pub fn hw6(&mut self) -> _HW6W {
        _HW6W { w: self }
    }
    #[doc = "Bits 14:15 - Port n Pin Hardware Select Bit 7"]
    #[inline]
    pub fn hw7(&mut self) -> _HW7W {
        _HW7W { w: self }
    }
    #[doc = "Bits 16:17 - Port n Pin Hardware Select Bit 8"]
    #[inline]
    pub fn hw8(&mut self) -> _HW8W {
        _HW8W { w: self }
    }
    #[doc = "Bits 18:19 - Port n Pin Hardware Select Bit 9"]
    #[inline]
    pub fn hw9(&mut self) -> _HW9W {
        _HW9W { w: self }
    }
    #[doc = "Bits 20:21 - Port n Pin Hardware Select Bit 10"]
    #[inline]
    pub fn hw10(&mut self) -> _HW10W {
        _HW10W { w: self }
    }
    #[doc = "Bits 22:23 - Port n Pin Hardware Select Bit 11"]
    #[inline]
    pub fn hw11(&mut self) -> _HW11W {
        _HW11W { w: self }
    }
    #[doc = "Bits 24:25 - Port n Pin Hardware Select Bit 12"]
    #[inline]
    pub fn hw12(&mut self) -> _HW12W {
        _HW12W { w: self }
    }
    #[doc = "Bits 26:27 - Port n Pin Hardware Select Bit 13"]
    #[inline]
    pub fn hw13(&mut self) -> _HW13W {
        _HW13W { w: self }
    }
    #[doc = "Bits 28:29 - Port n Pin Hardware Select Bit 14"]
    #[inline]
    pub fn hw14(&mut self) -> _HW14W {
        _HW14W { w: self }
    }
    #[doc = "Bits 30:31 - Port n Pin Hardware Select Bit 15"]
    #[inline]
    pub fn hw15(&mut self) -> _HW15W {
        _HW15W { w: self }
    }
}
