#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::REFLAG {
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
#[doc = "Possible values of the field `REV0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV0R {
    #[doc = "No result event"]
    VALUE1,
    #[doc = "New result was stored in register GxRESy"]
    VALUE2,
}
impl REV0R {
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
            REV0R::VALUE1 => false,
            REV0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REV0R {
        match value {
            false => REV0R::VALUE1,
            true => REV0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REV0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == REV0R::VALUE2
    }
}
#[doc = "Possible values of the field `REV1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV1R {
    #[doc = "No result event"]
    VALUE1,
    #[doc = "New result was stored in register GxRESy"]
    VALUE2,
}
impl REV1R {
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
            REV1R::VALUE1 => false,
            REV1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REV1R {
        match value {
            false => REV1R::VALUE1,
            true => REV1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REV1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == REV1R::VALUE2
    }
}
#[doc = "Possible values of the field `REV2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV2R {
    #[doc = "No result event"]
    VALUE1,
    #[doc = "New result was stored in register GxRESy"]
    VALUE2,
}
impl REV2R {
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
            REV2R::VALUE1 => false,
            REV2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REV2R {
        match value {
            false => REV2R::VALUE1,
            true => REV2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REV2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == REV2R::VALUE2
    }
}
#[doc = "Possible values of the field `REV3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV3R {
    #[doc = "No result event"]
    VALUE1,
    #[doc = "New result was stored in register GxRESy"]
    VALUE2,
}
impl REV3R {
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
            REV3R::VALUE1 => false,
            REV3R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REV3R {
        match value {
            false => REV3R::VALUE1,
            true => REV3R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REV3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == REV3R::VALUE2
    }
}
#[doc = "Possible values of the field `REV4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV4R {
    #[doc = "No result event"]
    VALUE1,
    #[doc = "New result was stored in register GxRESy"]
    VALUE2,
}
impl REV4R {
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
            REV4R::VALUE1 => false,
            REV4R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REV4R {
        match value {
            false => REV4R::VALUE1,
            true => REV4R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REV4R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == REV4R::VALUE2
    }
}
#[doc = "Possible values of the field `REV5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV5R {
    #[doc = "No result event"]
    VALUE1,
    #[doc = "New result was stored in register GxRESy"]
    VALUE2,
}
impl REV5R {
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
            REV5R::VALUE1 => false,
            REV5R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REV5R {
        match value {
            false => REV5R::VALUE1,
            true => REV5R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REV5R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == REV5R::VALUE2
    }
}
#[doc = "Possible values of the field `REV6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV6R {
    #[doc = "No result event"]
    VALUE1,
    #[doc = "New result was stored in register GxRESy"]
    VALUE2,
}
impl REV6R {
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
            REV6R::VALUE1 => false,
            REV6R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REV6R {
        match value {
            false => REV6R::VALUE1,
            true => REV6R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REV6R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == REV6R::VALUE2
    }
}
#[doc = "Possible values of the field `REV7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV7R {
    #[doc = "No result event"]
    VALUE1,
    #[doc = "New result was stored in register GxRESy"]
    VALUE2,
}
impl REV7R {
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
            REV7R::VALUE1 => false,
            REV7R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REV7R {
        match value {
            false => REV7R::VALUE1,
            true => REV7R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REV7R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == REV7R::VALUE2
    }
}
#[doc = "Possible values of the field `REV8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV8R {
    #[doc = "No result event"]
    VALUE1,
    #[doc = "New result was stored in register GxRESy"]
    VALUE2,
}
impl REV8R {
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
            REV8R::VALUE1 => false,
            REV8R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REV8R {
        match value {
            false => REV8R::VALUE1,
            true => REV8R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REV8R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == REV8R::VALUE2
    }
}
#[doc = "Possible values of the field `REV9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV9R {
    #[doc = "No result event"]
    VALUE1,
    #[doc = "New result was stored in register GxRESy"]
    VALUE2,
}
impl REV9R {
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
            REV9R::VALUE1 => false,
            REV9R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REV9R {
        match value {
            false => REV9R::VALUE1,
            true => REV9R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REV9R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == REV9R::VALUE2
    }
}
#[doc = "Possible values of the field `REV10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV10R {
    #[doc = "No result event"]
    VALUE1,
    #[doc = "New result was stored in register GxRESy"]
    VALUE2,
}
impl REV10R {
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
            REV10R::VALUE1 => false,
            REV10R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REV10R {
        match value {
            false => REV10R::VALUE1,
            true => REV10R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REV10R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == REV10R::VALUE2
    }
}
#[doc = "Possible values of the field `REV11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV11R {
    #[doc = "No result event"]
    VALUE1,
    #[doc = "New result was stored in register GxRESy"]
    VALUE2,
}
impl REV11R {
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
            REV11R::VALUE1 => false,
            REV11R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REV11R {
        match value {
            false => REV11R::VALUE1,
            true => REV11R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REV11R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == REV11R::VALUE2
    }
}
#[doc = "Possible values of the field `REV12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV12R {
    #[doc = "No result event"]
    VALUE1,
    #[doc = "New result was stored in register GxRESy"]
    VALUE2,
}
impl REV12R {
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
            REV12R::VALUE1 => false,
            REV12R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REV12R {
        match value {
            false => REV12R::VALUE1,
            true => REV12R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REV12R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == REV12R::VALUE2
    }
}
#[doc = "Possible values of the field `REV13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV13R {
    #[doc = "No result event"]
    VALUE1,
    #[doc = "New result was stored in register GxRESy"]
    VALUE2,
}
impl REV13R {
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
            REV13R::VALUE1 => false,
            REV13R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REV13R {
        match value {
            false => REV13R::VALUE1,
            true => REV13R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REV13R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == REV13R::VALUE2
    }
}
#[doc = "Possible values of the field `REV14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV14R {
    #[doc = "No result event"]
    VALUE1,
    #[doc = "New result was stored in register GxRESy"]
    VALUE2,
}
impl REV14R {
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
            REV14R::VALUE1 => false,
            REV14R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REV14R {
        match value {
            false => REV14R::VALUE1,
            true => REV14R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REV14R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == REV14R::VALUE2
    }
}
#[doc = "Possible values of the field `REV15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV15R {
    #[doc = "No result event"]
    VALUE1,
    #[doc = "New result was stored in register GxRESy"]
    VALUE2,
}
impl REV15R {
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
            REV15R::VALUE1 => false,
            REV15R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REV15R {
        match value {
            false => REV15R::VALUE1,
            true => REV15R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REV15R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == REV15R::VALUE2
    }
}
#[doc = "Values that can be written to the field `REV0`"]
pub enum REV0W {
    #[doc = "No result event"]
    VALUE1,
    #[doc = "New result was stored in register GxRESy"]
    VALUE2,
}
impl REV0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REV0W::VALUE1 => false,
            REV0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REV0W<'a> {
    w: &'a mut W,
}
impl<'a> _REV0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REV0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No result event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV0W::VALUE1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV0W::VALUE2)
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
#[doc = "Values that can be written to the field `REV1`"]
pub enum REV1W {
    #[doc = "No result event"]
    VALUE1,
    #[doc = "New result was stored in register GxRESy"]
    VALUE2,
}
impl REV1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REV1W::VALUE1 => false,
            REV1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REV1W<'a> {
    w: &'a mut W,
}
impl<'a> _REV1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REV1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No result event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV1W::VALUE1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV1W::VALUE2)
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
#[doc = "Values that can be written to the field `REV2`"]
pub enum REV2W {
    #[doc = "No result event"]
    VALUE1,
    #[doc = "New result was stored in register GxRESy"]
    VALUE2,
}
impl REV2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REV2W::VALUE1 => false,
            REV2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REV2W<'a> {
    w: &'a mut W,
}
impl<'a> _REV2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REV2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No result event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV2W::VALUE1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV2W::VALUE2)
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
#[doc = "Values that can be written to the field `REV3`"]
pub enum REV3W {
    #[doc = "No result event"]
    VALUE1,
    #[doc = "New result was stored in register GxRESy"]
    VALUE2,
}
impl REV3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REV3W::VALUE1 => false,
            REV3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REV3W<'a> {
    w: &'a mut W,
}
impl<'a> _REV3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REV3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No result event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV3W::VALUE1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV3W::VALUE2)
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
#[doc = "Values that can be written to the field `REV4`"]
pub enum REV4W {
    #[doc = "No result event"]
    VALUE1,
    #[doc = "New result was stored in register GxRESy"]
    VALUE2,
}
impl REV4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REV4W::VALUE1 => false,
            REV4W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REV4W<'a> {
    w: &'a mut W,
}
impl<'a> _REV4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REV4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No result event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV4W::VALUE1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV4W::VALUE2)
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
#[doc = "Values that can be written to the field `REV5`"]
pub enum REV5W {
    #[doc = "No result event"]
    VALUE1,
    #[doc = "New result was stored in register GxRESy"]
    VALUE2,
}
impl REV5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REV5W::VALUE1 => false,
            REV5W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REV5W<'a> {
    w: &'a mut W,
}
impl<'a> _REV5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REV5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No result event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV5W::VALUE1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV5W::VALUE2)
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
#[doc = "Values that can be written to the field `REV6`"]
pub enum REV6W {
    #[doc = "No result event"]
    VALUE1,
    #[doc = "New result was stored in register GxRESy"]
    VALUE2,
}
impl REV6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REV6W::VALUE1 => false,
            REV6W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REV6W<'a> {
    w: &'a mut W,
}
impl<'a> _REV6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REV6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No result event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV6W::VALUE1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV6W::VALUE2)
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
#[doc = "Values that can be written to the field `REV7`"]
pub enum REV7W {
    #[doc = "No result event"]
    VALUE1,
    #[doc = "New result was stored in register GxRESy"]
    VALUE2,
}
impl REV7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REV7W::VALUE1 => false,
            REV7W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REV7W<'a> {
    w: &'a mut W,
}
impl<'a> _REV7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REV7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No result event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV7W::VALUE1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV7W::VALUE2)
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
#[doc = "Values that can be written to the field `REV8`"]
pub enum REV8W {
    #[doc = "No result event"]
    VALUE1,
    #[doc = "New result was stored in register GxRESy"]
    VALUE2,
}
impl REV8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REV8W::VALUE1 => false,
            REV8W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REV8W<'a> {
    w: &'a mut W,
}
impl<'a> _REV8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REV8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No result event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV8W::VALUE1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV8W::VALUE2)
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
#[doc = "Values that can be written to the field `REV9`"]
pub enum REV9W {
    #[doc = "No result event"]
    VALUE1,
    #[doc = "New result was stored in register GxRESy"]
    VALUE2,
}
impl REV9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REV9W::VALUE1 => false,
            REV9W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REV9W<'a> {
    w: &'a mut W,
}
impl<'a> _REV9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REV9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No result event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV9W::VALUE1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV9W::VALUE2)
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
#[doc = "Values that can be written to the field `REV10`"]
pub enum REV10W {
    #[doc = "No result event"]
    VALUE1,
    #[doc = "New result was stored in register GxRESy"]
    VALUE2,
}
impl REV10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REV10W::VALUE1 => false,
            REV10W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REV10W<'a> {
    w: &'a mut W,
}
impl<'a> _REV10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REV10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No result event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV10W::VALUE1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV10W::VALUE2)
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
#[doc = "Values that can be written to the field `REV11`"]
pub enum REV11W {
    #[doc = "No result event"]
    VALUE1,
    #[doc = "New result was stored in register GxRESy"]
    VALUE2,
}
impl REV11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REV11W::VALUE1 => false,
            REV11W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REV11W<'a> {
    w: &'a mut W,
}
impl<'a> _REV11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REV11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No result event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV11W::VALUE1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV11W::VALUE2)
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
#[doc = "Values that can be written to the field `REV12`"]
pub enum REV12W {
    #[doc = "No result event"]
    VALUE1,
    #[doc = "New result was stored in register GxRESy"]
    VALUE2,
}
impl REV12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REV12W::VALUE1 => false,
            REV12W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REV12W<'a> {
    w: &'a mut W,
}
impl<'a> _REV12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REV12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No result event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV12W::VALUE1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV12W::VALUE2)
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
#[doc = "Values that can be written to the field `REV13`"]
pub enum REV13W {
    #[doc = "No result event"]
    VALUE1,
    #[doc = "New result was stored in register GxRESy"]
    VALUE2,
}
impl REV13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REV13W::VALUE1 => false,
            REV13W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REV13W<'a> {
    w: &'a mut W,
}
impl<'a> _REV13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REV13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No result event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV13W::VALUE1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV13W::VALUE2)
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
#[doc = "Values that can be written to the field `REV14`"]
pub enum REV14W {
    #[doc = "No result event"]
    VALUE1,
    #[doc = "New result was stored in register GxRESy"]
    VALUE2,
}
impl REV14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REV14W::VALUE1 => false,
            REV14W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REV14W<'a> {
    w: &'a mut W,
}
impl<'a> _REV14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REV14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No result event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV14W::VALUE1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV14W::VALUE2)
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
#[doc = "Values that can be written to the field `REV15`"]
pub enum REV15W {
    #[doc = "No result event"]
    VALUE1,
    #[doc = "New result was stored in register GxRESy"]
    VALUE2,
}
impl REV15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REV15W::VALUE1 => false,
            REV15W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REV15W<'a> {
    w: &'a mut W,
}
impl<'a> _REV15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REV15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No result event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV15W::VALUE1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV15W::VALUE2)
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
    #[doc = "Bit 0 - Result Event for Result Register 0"]
    #[inline]
    pub fn rev0(&self) -> REV0R {
        REV0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Result Event for Result Register 1"]
    #[inline]
    pub fn rev1(&self) -> REV1R {
        REV1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Result Event for Result Register 2"]
    #[inline]
    pub fn rev2(&self) -> REV2R {
        REV2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Result Event for Result Register 3"]
    #[inline]
    pub fn rev3(&self) -> REV3R {
        REV3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Result Event for Result Register 4"]
    #[inline]
    pub fn rev4(&self) -> REV4R {
        REV4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Result Event for Result Register 5"]
    #[inline]
    pub fn rev5(&self) -> REV5R {
        REV5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Result Event for Result Register 6"]
    #[inline]
    pub fn rev6(&self) -> REV6R {
        REV6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Result Event for Result Register 7"]
    #[inline]
    pub fn rev7(&self) -> REV7R {
        REV7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Result Event for Result Register 8"]
    #[inline]
    pub fn rev8(&self) -> REV8R {
        REV8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Result Event for Result Register 9"]
    #[inline]
    pub fn rev9(&self) -> REV9R {
        REV9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Result Event for Result Register 10"]
    #[inline]
    pub fn rev10(&self) -> REV10R {
        REV10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Result Event for Result Register 11"]
    #[inline]
    pub fn rev11(&self) -> REV11R {
        REV11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Result Event for Result Register 12"]
    #[inline]
    pub fn rev12(&self) -> REV12R {
        REV12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Result Event for Result Register 13"]
    #[inline]
    pub fn rev13(&self) -> REV13R {
        REV13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Result Event for Result Register 14"]
    #[inline]
    pub fn rev14(&self) -> REV14R {
        REV14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Result Event for Result Register 15"]
    #[inline]
    pub fn rev15(&self) -> REV15R {
        REV15R::_from({
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
    #[doc = "Bit 0 - Result Event for Result Register 0"]
    #[inline]
    pub fn rev0(&mut self) -> _REV0W {
        _REV0W { w: self }
    }
    #[doc = "Bit 1 - Result Event for Result Register 1"]
    #[inline]
    pub fn rev1(&mut self) -> _REV1W {
        _REV1W { w: self }
    }
    #[doc = "Bit 2 - Result Event for Result Register 2"]
    #[inline]
    pub fn rev2(&mut self) -> _REV2W {
        _REV2W { w: self }
    }
    #[doc = "Bit 3 - Result Event for Result Register 3"]
    #[inline]
    pub fn rev3(&mut self) -> _REV3W {
        _REV3W { w: self }
    }
    #[doc = "Bit 4 - Result Event for Result Register 4"]
    #[inline]
    pub fn rev4(&mut self) -> _REV4W {
        _REV4W { w: self }
    }
    #[doc = "Bit 5 - Result Event for Result Register 5"]
    #[inline]
    pub fn rev5(&mut self) -> _REV5W {
        _REV5W { w: self }
    }
    #[doc = "Bit 6 - Result Event for Result Register 6"]
    #[inline]
    pub fn rev6(&mut self) -> _REV6W {
        _REV6W { w: self }
    }
    #[doc = "Bit 7 - Result Event for Result Register 7"]
    #[inline]
    pub fn rev7(&mut self) -> _REV7W {
        _REV7W { w: self }
    }
    #[doc = "Bit 8 - Result Event for Result Register 8"]
    #[inline]
    pub fn rev8(&mut self) -> _REV8W {
        _REV8W { w: self }
    }
    #[doc = "Bit 9 - Result Event for Result Register 9"]
    #[inline]
    pub fn rev9(&mut self) -> _REV9W {
        _REV9W { w: self }
    }
    #[doc = "Bit 10 - Result Event for Result Register 10"]
    #[inline]
    pub fn rev10(&mut self) -> _REV10W {
        _REV10W { w: self }
    }
    #[doc = "Bit 11 - Result Event for Result Register 11"]
    #[inline]
    pub fn rev11(&mut self) -> _REV11W {
        _REV11W { w: self }
    }
    #[doc = "Bit 12 - Result Event for Result Register 12"]
    #[inline]
    pub fn rev12(&mut self) -> _REV12W {
        _REV12W { w: self }
    }
    #[doc = "Bit 13 - Result Event for Result Register 13"]
    #[inline]
    pub fn rev13(&mut self) -> _REV13W {
        _REV13W { w: self }
    }
    #[doc = "Bit 14 - Result Event for Result Register 14"]
    #[inline]
    pub fn rev14(&mut self) -> _REV14W {
        _REV14W { w: self }
    }
    #[doc = "Bit 15 - Result Event for Result Register 15"]
    #[inline]
    pub fn rev15(&mut self) -> _REV15W {
        _REV15W { w: self }
    }
}
