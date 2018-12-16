#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BRSSEL {
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
#[doc = "Possible values of the field `CHSELG0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSELG0R {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "This channel is part of the scan sequence"]
    VALUE2,
}
impl CHSELG0R {
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
            CHSELG0R::VALUE1 => false,
            CHSELG0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHSELG0R {
        match value {
            false => CHSELG0R::VALUE1,
            true => CHSELG0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CHSELG0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CHSELG0R::VALUE2
    }
}
#[doc = "Possible values of the field `CHSELG1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSELG1R {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "This channel is part of the scan sequence"]
    VALUE2,
}
impl CHSELG1R {
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
            CHSELG1R::VALUE1 => false,
            CHSELG1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHSELG1R {
        match value {
            false => CHSELG1R::VALUE1,
            true => CHSELG1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CHSELG1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CHSELG1R::VALUE2
    }
}
#[doc = "Possible values of the field `CHSELG2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSELG2R {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "This channel is part of the scan sequence"]
    VALUE2,
}
impl CHSELG2R {
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
            CHSELG2R::VALUE1 => false,
            CHSELG2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHSELG2R {
        match value {
            false => CHSELG2R::VALUE1,
            true => CHSELG2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CHSELG2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CHSELG2R::VALUE2
    }
}
#[doc = "Possible values of the field `CHSELG3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSELG3R {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "This channel is part of the scan sequence"]
    VALUE2,
}
impl CHSELG3R {
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
            CHSELG3R::VALUE1 => false,
            CHSELG3R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHSELG3R {
        match value {
            false => CHSELG3R::VALUE1,
            true => CHSELG3R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CHSELG3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CHSELG3R::VALUE2
    }
}
#[doc = "Possible values of the field `CHSELG4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSELG4R {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "This channel is part of the scan sequence"]
    VALUE2,
}
impl CHSELG4R {
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
            CHSELG4R::VALUE1 => false,
            CHSELG4R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHSELG4R {
        match value {
            false => CHSELG4R::VALUE1,
            true => CHSELG4R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CHSELG4R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CHSELG4R::VALUE2
    }
}
#[doc = "Possible values of the field `CHSELG5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSELG5R {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "This channel is part of the scan sequence"]
    VALUE2,
}
impl CHSELG5R {
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
            CHSELG5R::VALUE1 => false,
            CHSELG5R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHSELG5R {
        match value {
            false => CHSELG5R::VALUE1,
            true => CHSELG5R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CHSELG5R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CHSELG5R::VALUE2
    }
}
#[doc = "Possible values of the field `CHSELG6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSELG6R {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "This channel is part of the scan sequence"]
    VALUE2,
}
impl CHSELG6R {
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
            CHSELG6R::VALUE1 => false,
            CHSELG6R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHSELG6R {
        match value {
            false => CHSELG6R::VALUE1,
            true => CHSELG6R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CHSELG6R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CHSELG6R::VALUE2
    }
}
#[doc = "Possible values of the field `CHSELG7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSELG7R {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "This channel is part of the scan sequence"]
    VALUE2,
}
impl CHSELG7R {
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
            CHSELG7R::VALUE1 => false,
            CHSELG7R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHSELG7R {
        match value {
            false => CHSELG7R::VALUE1,
            true => CHSELG7R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CHSELG7R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CHSELG7R::VALUE2
    }
}
#[doc = "Values that can be written to the field `CHSELG0`"]
pub enum CHSELG0W {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "This channel is part of the scan sequence"]
    VALUE2,
}
impl CHSELG0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHSELG0W::VALUE1 => false,
            CHSELG0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHSELG0W<'a> {
    w: &'a mut W,
}
impl<'a> _CHSELG0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHSELG0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSELG0W::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSELG0W::VALUE2)
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
#[doc = "Values that can be written to the field `CHSELG1`"]
pub enum CHSELG1W {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "This channel is part of the scan sequence"]
    VALUE2,
}
impl CHSELG1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHSELG1W::VALUE1 => false,
            CHSELG1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHSELG1W<'a> {
    w: &'a mut W,
}
impl<'a> _CHSELG1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHSELG1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSELG1W::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSELG1W::VALUE2)
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
#[doc = "Values that can be written to the field `CHSELG2`"]
pub enum CHSELG2W {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "This channel is part of the scan sequence"]
    VALUE2,
}
impl CHSELG2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHSELG2W::VALUE1 => false,
            CHSELG2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHSELG2W<'a> {
    w: &'a mut W,
}
impl<'a> _CHSELG2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHSELG2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSELG2W::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSELG2W::VALUE2)
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
#[doc = "Values that can be written to the field `CHSELG3`"]
pub enum CHSELG3W {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "This channel is part of the scan sequence"]
    VALUE2,
}
impl CHSELG3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHSELG3W::VALUE1 => false,
            CHSELG3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHSELG3W<'a> {
    w: &'a mut W,
}
impl<'a> _CHSELG3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHSELG3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSELG3W::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSELG3W::VALUE2)
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
#[doc = "Values that can be written to the field `CHSELG4`"]
pub enum CHSELG4W {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "This channel is part of the scan sequence"]
    VALUE2,
}
impl CHSELG4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHSELG4W::VALUE1 => false,
            CHSELG4W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHSELG4W<'a> {
    w: &'a mut W,
}
impl<'a> _CHSELG4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHSELG4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSELG4W::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSELG4W::VALUE2)
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
#[doc = "Values that can be written to the field `CHSELG5`"]
pub enum CHSELG5W {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "This channel is part of the scan sequence"]
    VALUE2,
}
impl CHSELG5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHSELG5W::VALUE1 => false,
            CHSELG5W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHSELG5W<'a> {
    w: &'a mut W,
}
impl<'a> _CHSELG5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHSELG5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSELG5W::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSELG5W::VALUE2)
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
#[doc = "Values that can be written to the field `CHSELG6`"]
pub enum CHSELG6W {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "This channel is part of the scan sequence"]
    VALUE2,
}
impl CHSELG6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHSELG6W::VALUE1 => false,
            CHSELG6W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHSELG6W<'a> {
    w: &'a mut W,
}
impl<'a> _CHSELG6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHSELG6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSELG6W::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSELG6W::VALUE2)
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
#[doc = "Values that can be written to the field `CHSELG7`"]
pub enum CHSELG7W {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "This channel is part of the scan sequence"]
    VALUE2,
}
impl CHSELG7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHSELG7W::VALUE1 => false,
            CHSELG7W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHSELG7W<'a> {
    w: &'a mut W,
}
impl<'a> _CHSELG7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHSELG7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSELG7W::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSELG7W::VALUE2)
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
    #[doc = "Bit 0 - Channel Selection Group x"]
    #[inline]
    pub fn chselg0(&self) -> CHSELG0R {
        CHSELG0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Channel Selection Group x"]
    #[inline]
    pub fn chselg1(&self) -> CHSELG1R {
        CHSELG1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Channel Selection Group x"]
    #[inline]
    pub fn chselg2(&self) -> CHSELG2R {
        CHSELG2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Channel Selection Group x"]
    #[inline]
    pub fn chselg3(&self) -> CHSELG3R {
        CHSELG3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Channel Selection Group x"]
    #[inline]
    pub fn chselg4(&self) -> CHSELG4R {
        CHSELG4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Channel Selection Group x"]
    #[inline]
    pub fn chselg5(&self) -> CHSELG5R {
        CHSELG5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Channel Selection Group x"]
    #[inline]
    pub fn chselg6(&self) -> CHSELG6R {
        CHSELG6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Channel Selection Group x"]
    #[inline]
    pub fn chselg7(&self) -> CHSELG7R {
        CHSELG7R::_from({
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
    #[doc = "Bit 0 - Channel Selection Group x"]
    #[inline]
    pub fn chselg0(&mut self) -> _CHSELG0W {
        _CHSELG0W { w: self }
    }
    #[doc = "Bit 1 - Channel Selection Group x"]
    #[inline]
    pub fn chselg1(&mut self) -> _CHSELG1W {
        _CHSELG1W { w: self }
    }
    #[doc = "Bit 2 - Channel Selection Group x"]
    #[inline]
    pub fn chselg2(&mut self) -> _CHSELG2W {
        _CHSELG2W { w: self }
    }
    #[doc = "Bit 3 - Channel Selection Group x"]
    #[inline]
    pub fn chselg3(&mut self) -> _CHSELG3W {
        _CHSELG3W { w: self }
    }
    #[doc = "Bit 4 - Channel Selection Group x"]
    #[inline]
    pub fn chselg4(&mut self) -> _CHSELG4W {
        _CHSELG4W { w: self }
    }
    #[doc = "Bit 5 - Channel Selection Group x"]
    #[inline]
    pub fn chselg5(&mut self) -> _CHSELG5W {
        _CHSELG5W { w: self }
    }
    #[doc = "Bit 6 - Channel Selection Group x"]
    #[inline]
    pub fn chselg6(&mut self) -> _CHSELG6W {
        _CHSELG6W { w: self }
    }
    #[doc = "Bit 7 - Channel Selection Group x"]
    #[inline]
    pub fn chselg7(&mut self) -> _CHSELG7W {
        _CHSELG7W { w: self }
    }
}
