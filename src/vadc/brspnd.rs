#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BRSPND {
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
#[doc = "Possible values of the field `CHPNDG0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHPNDG0R {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "Request conversion of this channel"]
    VALUE2,
}
impl CHPNDG0R {
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
            CHPNDG0R::VALUE1 => false,
            CHPNDG0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHPNDG0R {
        match value {
            false => CHPNDG0R::VALUE1,
            true => CHPNDG0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CHPNDG0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CHPNDG0R::VALUE2
    }
}
#[doc = "Possible values of the field `CHPNDG1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHPNDG1R {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "Request conversion of this channel"]
    VALUE2,
}
impl CHPNDG1R {
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
            CHPNDG1R::VALUE1 => false,
            CHPNDG1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHPNDG1R {
        match value {
            false => CHPNDG1R::VALUE1,
            true => CHPNDG1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CHPNDG1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CHPNDG1R::VALUE2
    }
}
#[doc = "Possible values of the field `CHPNDG2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHPNDG2R {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "Request conversion of this channel"]
    VALUE2,
}
impl CHPNDG2R {
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
            CHPNDG2R::VALUE1 => false,
            CHPNDG2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHPNDG2R {
        match value {
            false => CHPNDG2R::VALUE1,
            true => CHPNDG2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CHPNDG2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CHPNDG2R::VALUE2
    }
}
#[doc = "Possible values of the field `CHPNDG3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHPNDG3R {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "Request conversion of this channel"]
    VALUE2,
}
impl CHPNDG3R {
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
            CHPNDG3R::VALUE1 => false,
            CHPNDG3R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHPNDG3R {
        match value {
            false => CHPNDG3R::VALUE1,
            true => CHPNDG3R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CHPNDG3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CHPNDG3R::VALUE2
    }
}
#[doc = "Possible values of the field `CHPNDG4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHPNDG4R {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "Request conversion of this channel"]
    VALUE2,
}
impl CHPNDG4R {
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
            CHPNDG4R::VALUE1 => false,
            CHPNDG4R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHPNDG4R {
        match value {
            false => CHPNDG4R::VALUE1,
            true => CHPNDG4R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CHPNDG4R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CHPNDG4R::VALUE2
    }
}
#[doc = "Possible values of the field `CHPNDG5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHPNDG5R {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "Request conversion of this channel"]
    VALUE2,
}
impl CHPNDG5R {
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
            CHPNDG5R::VALUE1 => false,
            CHPNDG5R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHPNDG5R {
        match value {
            false => CHPNDG5R::VALUE1,
            true => CHPNDG5R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CHPNDG5R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CHPNDG5R::VALUE2
    }
}
#[doc = "Possible values of the field `CHPNDG6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHPNDG6R {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "Request conversion of this channel"]
    VALUE2,
}
impl CHPNDG6R {
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
            CHPNDG6R::VALUE1 => false,
            CHPNDG6R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHPNDG6R {
        match value {
            false => CHPNDG6R::VALUE1,
            true => CHPNDG6R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CHPNDG6R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CHPNDG6R::VALUE2
    }
}
#[doc = "Possible values of the field `CHPNDG7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHPNDG7R {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "Request conversion of this channel"]
    VALUE2,
}
impl CHPNDG7R {
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
            CHPNDG7R::VALUE1 => false,
            CHPNDG7R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHPNDG7R {
        match value {
            false => CHPNDG7R::VALUE1,
            true => CHPNDG7R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CHPNDG7R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CHPNDG7R::VALUE2
    }
}
#[doc = "Values that can be written to the field `CHPNDG0`"]
pub enum CHPNDG0W {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "Request conversion of this channel"]
    VALUE2,
}
impl CHPNDG0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHPNDG0W::VALUE1 => false,
            CHPNDG0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHPNDG0W<'a> {
    w: &'a mut W,
}
impl<'a> _CHPNDG0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHPNDG0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPNDG0W::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPNDG0W::VALUE2)
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
#[doc = "Values that can be written to the field `CHPNDG1`"]
pub enum CHPNDG1W {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "Request conversion of this channel"]
    VALUE2,
}
impl CHPNDG1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHPNDG1W::VALUE1 => false,
            CHPNDG1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHPNDG1W<'a> {
    w: &'a mut W,
}
impl<'a> _CHPNDG1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHPNDG1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPNDG1W::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPNDG1W::VALUE2)
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
#[doc = "Values that can be written to the field `CHPNDG2`"]
pub enum CHPNDG2W {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "Request conversion of this channel"]
    VALUE2,
}
impl CHPNDG2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHPNDG2W::VALUE1 => false,
            CHPNDG2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHPNDG2W<'a> {
    w: &'a mut W,
}
impl<'a> _CHPNDG2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHPNDG2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPNDG2W::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPNDG2W::VALUE2)
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
#[doc = "Values that can be written to the field `CHPNDG3`"]
pub enum CHPNDG3W {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "Request conversion of this channel"]
    VALUE2,
}
impl CHPNDG3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHPNDG3W::VALUE1 => false,
            CHPNDG3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHPNDG3W<'a> {
    w: &'a mut W,
}
impl<'a> _CHPNDG3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHPNDG3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPNDG3W::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPNDG3W::VALUE2)
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
#[doc = "Values that can be written to the field `CHPNDG4`"]
pub enum CHPNDG4W {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "Request conversion of this channel"]
    VALUE2,
}
impl CHPNDG4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHPNDG4W::VALUE1 => false,
            CHPNDG4W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHPNDG4W<'a> {
    w: &'a mut W,
}
impl<'a> _CHPNDG4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHPNDG4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPNDG4W::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPNDG4W::VALUE2)
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
#[doc = "Values that can be written to the field `CHPNDG5`"]
pub enum CHPNDG5W {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "Request conversion of this channel"]
    VALUE2,
}
impl CHPNDG5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHPNDG5W::VALUE1 => false,
            CHPNDG5W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHPNDG5W<'a> {
    w: &'a mut W,
}
impl<'a> _CHPNDG5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHPNDG5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPNDG5W::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPNDG5W::VALUE2)
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
#[doc = "Values that can be written to the field `CHPNDG6`"]
pub enum CHPNDG6W {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "Request conversion of this channel"]
    VALUE2,
}
impl CHPNDG6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHPNDG6W::VALUE1 => false,
            CHPNDG6W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHPNDG6W<'a> {
    w: &'a mut W,
}
impl<'a> _CHPNDG6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHPNDG6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPNDG6W::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPNDG6W::VALUE2)
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
#[doc = "Values that can be written to the field `CHPNDG7`"]
pub enum CHPNDG7W {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "Request conversion of this channel"]
    VALUE2,
}
impl CHPNDG7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHPNDG7W::VALUE1 => false,
            CHPNDG7W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHPNDG7W<'a> {
    w: &'a mut W,
}
impl<'a> _CHPNDG7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHPNDG7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPNDG7W::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPNDG7W::VALUE2)
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
    #[doc = "Bit 0 - Channels Pending Group x"]
    #[inline]
    pub fn chpndg0(&self) -> CHPNDG0R {
        CHPNDG0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Channels Pending Group x"]
    #[inline]
    pub fn chpndg1(&self) -> CHPNDG1R {
        CHPNDG1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Channels Pending Group x"]
    #[inline]
    pub fn chpndg2(&self) -> CHPNDG2R {
        CHPNDG2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Channels Pending Group x"]
    #[inline]
    pub fn chpndg3(&self) -> CHPNDG3R {
        CHPNDG3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Channels Pending Group x"]
    #[inline]
    pub fn chpndg4(&self) -> CHPNDG4R {
        CHPNDG4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Channels Pending Group x"]
    #[inline]
    pub fn chpndg5(&self) -> CHPNDG5R {
        CHPNDG5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Channels Pending Group x"]
    #[inline]
    pub fn chpndg6(&self) -> CHPNDG6R {
        CHPNDG6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Channels Pending Group x"]
    #[inline]
    pub fn chpndg7(&self) -> CHPNDG7R {
        CHPNDG7R::_from({
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
    #[doc = "Bit 0 - Channels Pending Group x"]
    #[inline]
    pub fn chpndg0(&mut self) -> _CHPNDG0W {
        _CHPNDG0W { w: self }
    }
    #[doc = "Bit 1 - Channels Pending Group x"]
    #[inline]
    pub fn chpndg1(&mut self) -> _CHPNDG1W {
        _CHPNDG1W { w: self }
    }
    #[doc = "Bit 2 - Channels Pending Group x"]
    #[inline]
    pub fn chpndg2(&mut self) -> _CHPNDG2W {
        _CHPNDG2W { w: self }
    }
    #[doc = "Bit 3 - Channels Pending Group x"]
    #[inline]
    pub fn chpndg3(&mut self) -> _CHPNDG3W {
        _CHPNDG3W { w: self }
    }
    #[doc = "Bit 4 - Channels Pending Group x"]
    #[inline]
    pub fn chpndg4(&mut self) -> _CHPNDG4W {
        _CHPNDG4W { w: self }
    }
    #[doc = "Bit 5 - Channels Pending Group x"]
    #[inline]
    pub fn chpndg5(&mut self) -> _CHPNDG5W {
        _CHPNDG5W { w: self }
    }
    #[doc = "Bit 6 - Channels Pending Group x"]
    #[inline]
    pub fn chpndg6(&mut self) -> _CHPNDG6W {
        _CHPNDG6W { w: self }
    }
    #[doc = "Bit 7 - Channels Pending Group x"]
    #[inline]
    pub fn chpndg7(&mut self) -> _CHPNDG7W {
        _CHPNDG7W { w: self }
    }
}
