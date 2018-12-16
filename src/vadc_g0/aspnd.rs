#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ASPND {
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
#[doc = "Possible values of the field `CHPND0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHPND0R {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "Request conversion of this channel"]
    VALUE2,
}
impl CHPND0R {
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
            CHPND0R::VALUE1 => false,
            CHPND0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHPND0R {
        match value {
            false => CHPND0R::VALUE1,
            true => CHPND0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CHPND0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CHPND0R::VALUE2
    }
}
#[doc = "Possible values of the field `CHPND1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHPND1R {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "Request conversion of this channel"]
    VALUE2,
}
impl CHPND1R {
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
            CHPND1R::VALUE1 => false,
            CHPND1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHPND1R {
        match value {
            false => CHPND1R::VALUE1,
            true => CHPND1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CHPND1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CHPND1R::VALUE2
    }
}
#[doc = "Possible values of the field `CHPND2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHPND2R {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "Request conversion of this channel"]
    VALUE2,
}
impl CHPND2R {
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
            CHPND2R::VALUE1 => false,
            CHPND2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHPND2R {
        match value {
            false => CHPND2R::VALUE1,
            true => CHPND2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CHPND2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CHPND2R::VALUE2
    }
}
#[doc = "Possible values of the field `CHPND3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHPND3R {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "Request conversion of this channel"]
    VALUE2,
}
impl CHPND3R {
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
            CHPND3R::VALUE1 => false,
            CHPND3R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHPND3R {
        match value {
            false => CHPND3R::VALUE1,
            true => CHPND3R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CHPND3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CHPND3R::VALUE2
    }
}
#[doc = "Possible values of the field `CHPND4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHPND4R {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "Request conversion of this channel"]
    VALUE2,
}
impl CHPND4R {
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
            CHPND4R::VALUE1 => false,
            CHPND4R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHPND4R {
        match value {
            false => CHPND4R::VALUE1,
            true => CHPND4R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CHPND4R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CHPND4R::VALUE2
    }
}
#[doc = "Possible values of the field `CHPND5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHPND5R {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "Request conversion of this channel"]
    VALUE2,
}
impl CHPND5R {
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
            CHPND5R::VALUE1 => false,
            CHPND5R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHPND5R {
        match value {
            false => CHPND5R::VALUE1,
            true => CHPND5R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CHPND5R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CHPND5R::VALUE2
    }
}
#[doc = "Possible values of the field `CHPND6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHPND6R {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "Request conversion of this channel"]
    VALUE2,
}
impl CHPND6R {
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
            CHPND6R::VALUE1 => false,
            CHPND6R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHPND6R {
        match value {
            false => CHPND6R::VALUE1,
            true => CHPND6R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CHPND6R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CHPND6R::VALUE2
    }
}
#[doc = "Possible values of the field `CHPND7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHPND7R {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "Request conversion of this channel"]
    VALUE2,
}
impl CHPND7R {
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
            CHPND7R::VALUE1 => false,
            CHPND7R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHPND7R {
        match value {
            false => CHPND7R::VALUE1,
            true => CHPND7R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CHPND7R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CHPND7R::VALUE2
    }
}
#[doc = "Values that can be written to the field `CHPND0`"]
pub enum CHPND0W {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "Request conversion of this channel"]
    VALUE2,
}
impl CHPND0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHPND0W::VALUE1 => false,
            CHPND0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHPND0W<'a> {
    w: &'a mut W,
}
impl<'a> _CHPND0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHPND0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPND0W::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPND0W::VALUE2)
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
#[doc = "Values that can be written to the field `CHPND1`"]
pub enum CHPND1W {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "Request conversion of this channel"]
    VALUE2,
}
impl CHPND1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHPND1W::VALUE1 => false,
            CHPND1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHPND1W<'a> {
    w: &'a mut W,
}
impl<'a> _CHPND1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHPND1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPND1W::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPND1W::VALUE2)
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
#[doc = "Values that can be written to the field `CHPND2`"]
pub enum CHPND2W {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "Request conversion of this channel"]
    VALUE2,
}
impl CHPND2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHPND2W::VALUE1 => false,
            CHPND2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHPND2W<'a> {
    w: &'a mut W,
}
impl<'a> _CHPND2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHPND2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPND2W::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPND2W::VALUE2)
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
#[doc = "Values that can be written to the field `CHPND3`"]
pub enum CHPND3W {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "Request conversion of this channel"]
    VALUE2,
}
impl CHPND3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHPND3W::VALUE1 => false,
            CHPND3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHPND3W<'a> {
    w: &'a mut W,
}
impl<'a> _CHPND3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHPND3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPND3W::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPND3W::VALUE2)
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
#[doc = "Values that can be written to the field `CHPND4`"]
pub enum CHPND4W {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "Request conversion of this channel"]
    VALUE2,
}
impl CHPND4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHPND4W::VALUE1 => false,
            CHPND4W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHPND4W<'a> {
    w: &'a mut W,
}
impl<'a> _CHPND4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHPND4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPND4W::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPND4W::VALUE2)
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
#[doc = "Values that can be written to the field `CHPND5`"]
pub enum CHPND5W {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "Request conversion of this channel"]
    VALUE2,
}
impl CHPND5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHPND5W::VALUE1 => false,
            CHPND5W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHPND5W<'a> {
    w: &'a mut W,
}
impl<'a> _CHPND5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHPND5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPND5W::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPND5W::VALUE2)
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
#[doc = "Values that can be written to the field `CHPND6`"]
pub enum CHPND6W {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "Request conversion of this channel"]
    VALUE2,
}
impl CHPND6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHPND6W::VALUE1 => false,
            CHPND6W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHPND6W<'a> {
    w: &'a mut W,
}
impl<'a> _CHPND6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHPND6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPND6W::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPND6W::VALUE2)
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
#[doc = "Values that can be written to the field `CHPND7`"]
pub enum CHPND7W {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "Request conversion of this channel"]
    VALUE2,
}
impl CHPND7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHPND7W::VALUE1 => false,
            CHPND7W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHPND7W<'a> {
    w: &'a mut W,
}
impl<'a> _CHPND7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHPND7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPND7W::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPND7W::VALUE2)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Channels Pending"]
    #[inline]
    pub fn chpnd0(&self) -> CHPND0R {
        CHPND0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Channels Pending"]
    #[inline]
    pub fn chpnd1(&self) -> CHPND1R {
        CHPND1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Channels Pending"]
    #[inline]
    pub fn chpnd2(&self) -> CHPND2R {
        CHPND2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Channels Pending"]
    #[inline]
    pub fn chpnd3(&self) -> CHPND3R {
        CHPND3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Channels Pending"]
    #[inline]
    pub fn chpnd4(&self) -> CHPND4R {
        CHPND4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Channels Pending"]
    #[inline]
    pub fn chpnd5(&self) -> CHPND5R {
        CHPND5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Channels Pending"]
    #[inline]
    pub fn chpnd6(&self) -> CHPND6R {
        CHPND6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Channels Pending"]
    #[inline]
    pub fn chpnd7(&self) -> CHPND7R {
        CHPND7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
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
    #[doc = "Bit 0 - Channels Pending"]
    #[inline]
    pub fn chpnd0(&mut self) -> _CHPND0W {
        _CHPND0W { w: self }
    }
    #[doc = "Bit 1 - Channels Pending"]
    #[inline]
    pub fn chpnd1(&mut self) -> _CHPND1W {
        _CHPND1W { w: self }
    }
    #[doc = "Bit 2 - Channels Pending"]
    #[inline]
    pub fn chpnd2(&mut self) -> _CHPND2W {
        _CHPND2W { w: self }
    }
    #[doc = "Bit 3 - Channels Pending"]
    #[inline]
    pub fn chpnd3(&mut self) -> _CHPND3W {
        _CHPND3W { w: self }
    }
    #[doc = "Bit 4 - Channels Pending"]
    #[inline]
    pub fn chpnd4(&mut self) -> _CHPND4W {
        _CHPND4W { w: self }
    }
    #[doc = "Bit 5 - Channels Pending"]
    #[inline]
    pub fn chpnd5(&mut self) -> _CHPND5W {
        _CHPND5W { w: self }
    }
    #[doc = "Bit 6 - Channels Pending"]
    #[inline]
    pub fn chpnd6(&mut self) -> _CHPND6W {
        _CHPND6W { w: self }
    }
    #[doc = "Bit 7 - Channels Pending"]
    #[inline]
    pub fn chpnd7(&mut self) -> _CHPND7W {
        _CHPND7W { w: self }
    }
}
