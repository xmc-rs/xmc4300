#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ASSEL {
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
#[doc = "Possible values of the field `CHSEL0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSEL0R {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "This channel is part of the scan sequence"]
    VALUE2,
}
impl CHSEL0R {
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
            CHSEL0R::VALUE1 => false,
            CHSEL0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHSEL0R {
        match value {
            false => CHSEL0R::VALUE1,
            true => CHSEL0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CHSEL0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CHSEL0R::VALUE2
    }
}
#[doc = "Possible values of the field `CHSEL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSEL1R {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "This channel is part of the scan sequence"]
    VALUE2,
}
impl CHSEL1R {
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
            CHSEL1R::VALUE1 => false,
            CHSEL1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHSEL1R {
        match value {
            false => CHSEL1R::VALUE1,
            true => CHSEL1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CHSEL1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CHSEL1R::VALUE2
    }
}
#[doc = "Possible values of the field `CHSEL2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSEL2R {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "This channel is part of the scan sequence"]
    VALUE2,
}
impl CHSEL2R {
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
            CHSEL2R::VALUE1 => false,
            CHSEL2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHSEL2R {
        match value {
            false => CHSEL2R::VALUE1,
            true => CHSEL2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CHSEL2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CHSEL2R::VALUE2
    }
}
#[doc = "Possible values of the field `CHSEL3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSEL3R {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "This channel is part of the scan sequence"]
    VALUE2,
}
impl CHSEL3R {
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
            CHSEL3R::VALUE1 => false,
            CHSEL3R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHSEL3R {
        match value {
            false => CHSEL3R::VALUE1,
            true => CHSEL3R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CHSEL3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CHSEL3R::VALUE2
    }
}
#[doc = "Possible values of the field `CHSEL4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSEL4R {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "This channel is part of the scan sequence"]
    VALUE2,
}
impl CHSEL4R {
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
            CHSEL4R::VALUE1 => false,
            CHSEL4R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHSEL4R {
        match value {
            false => CHSEL4R::VALUE1,
            true => CHSEL4R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CHSEL4R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CHSEL4R::VALUE2
    }
}
#[doc = "Possible values of the field `CHSEL5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSEL5R {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "This channel is part of the scan sequence"]
    VALUE2,
}
impl CHSEL5R {
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
            CHSEL5R::VALUE1 => false,
            CHSEL5R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHSEL5R {
        match value {
            false => CHSEL5R::VALUE1,
            true => CHSEL5R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CHSEL5R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CHSEL5R::VALUE2
    }
}
#[doc = "Possible values of the field `CHSEL6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSEL6R {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "This channel is part of the scan sequence"]
    VALUE2,
}
impl CHSEL6R {
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
            CHSEL6R::VALUE1 => false,
            CHSEL6R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHSEL6R {
        match value {
            false => CHSEL6R::VALUE1,
            true => CHSEL6R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CHSEL6R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CHSEL6R::VALUE2
    }
}
#[doc = "Possible values of the field `CHSEL7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSEL7R {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "This channel is part of the scan sequence"]
    VALUE2,
}
impl CHSEL7R {
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
            CHSEL7R::VALUE1 => false,
            CHSEL7R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHSEL7R {
        match value {
            false => CHSEL7R::VALUE1,
            true => CHSEL7R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CHSEL7R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CHSEL7R::VALUE2
    }
}
#[doc = "Values that can be written to the field `CHSEL0`"]
pub enum CHSEL0W {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "This channel is part of the scan sequence"]
    VALUE2,
}
impl CHSEL0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHSEL0W::VALUE1 => false,
            CHSEL0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHSEL0W<'a> {
    w: &'a mut W,
}
impl<'a> _CHSEL0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHSEL0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSEL0W::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSEL0W::VALUE2)
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
#[doc = "Values that can be written to the field `CHSEL1`"]
pub enum CHSEL1W {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "This channel is part of the scan sequence"]
    VALUE2,
}
impl CHSEL1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHSEL1W::VALUE1 => false,
            CHSEL1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHSEL1W<'a> {
    w: &'a mut W,
}
impl<'a> _CHSEL1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHSEL1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSEL1W::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSEL1W::VALUE2)
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
#[doc = "Values that can be written to the field `CHSEL2`"]
pub enum CHSEL2W {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "This channel is part of the scan sequence"]
    VALUE2,
}
impl CHSEL2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHSEL2W::VALUE1 => false,
            CHSEL2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHSEL2W<'a> {
    w: &'a mut W,
}
impl<'a> _CHSEL2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHSEL2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSEL2W::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSEL2W::VALUE2)
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
#[doc = "Values that can be written to the field `CHSEL3`"]
pub enum CHSEL3W {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "This channel is part of the scan sequence"]
    VALUE2,
}
impl CHSEL3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHSEL3W::VALUE1 => false,
            CHSEL3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHSEL3W<'a> {
    w: &'a mut W,
}
impl<'a> _CHSEL3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHSEL3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSEL3W::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSEL3W::VALUE2)
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
#[doc = "Values that can be written to the field `CHSEL4`"]
pub enum CHSEL4W {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "This channel is part of the scan sequence"]
    VALUE2,
}
impl CHSEL4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHSEL4W::VALUE1 => false,
            CHSEL4W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHSEL4W<'a> {
    w: &'a mut W,
}
impl<'a> _CHSEL4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHSEL4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSEL4W::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSEL4W::VALUE2)
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
#[doc = "Values that can be written to the field `CHSEL5`"]
pub enum CHSEL5W {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "This channel is part of the scan sequence"]
    VALUE2,
}
impl CHSEL5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHSEL5W::VALUE1 => false,
            CHSEL5W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHSEL5W<'a> {
    w: &'a mut W,
}
impl<'a> _CHSEL5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHSEL5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSEL5W::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSEL5W::VALUE2)
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
#[doc = "Values that can be written to the field `CHSEL6`"]
pub enum CHSEL6W {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "This channel is part of the scan sequence"]
    VALUE2,
}
impl CHSEL6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHSEL6W::VALUE1 => false,
            CHSEL6W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHSEL6W<'a> {
    w: &'a mut W,
}
impl<'a> _CHSEL6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHSEL6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSEL6W::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSEL6W::VALUE2)
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
#[doc = "Values that can be written to the field `CHSEL7`"]
pub enum CHSEL7W {
    #[doc = "Ignore this channel"]
    VALUE1,
    #[doc = "This channel is part of the scan sequence"]
    VALUE2,
}
impl CHSEL7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHSEL7W::VALUE1 => false,
            CHSEL7W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHSEL7W<'a> {
    w: &'a mut W,
}
impl<'a> _CHSEL7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHSEL7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Ignore this channel"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSEL7W::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSEL7W::VALUE2)
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
    #[doc = "Bit 0 - Channel Selection"]
    #[inline]
    pub fn chsel0(&self) -> CHSEL0R {
        CHSEL0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Channel Selection"]
    #[inline]
    pub fn chsel1(&self) -> CHSEL1R {
        CHSEL1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Channel Selection"]
    #[inline]
    pub fn chsel2(&self) -> CHSEL2R {
        CHSEL2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Channel Selection"]
    #[inline]
    pub fn chsel3(&self) -> CHSEL3R {
        CHSEL3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Channel Selection"]
    #[inline]
    pub fn chsel4(&self) -> CHSEL4R {
        CHSEL4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Channel Selection"]
    #[inline]
    pub fn chsel5(&self) -> CHSEL5R {
        CHSEL5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Channel Selection"]
    #[inline]
    pub fn chsel6(&self) -> CHSEL6R {
        CHSEL6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Channel Selection"]
    #[inline]
    pub fn chsel7(&self) -> CHSEL7R {
        CHSEL7R::_from({
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
    #[doc = "Bit 0 - Channel Selection"]
    #[inline]
    pub fn chsel0(&mut self) -> _CHSEL0W {
        _CHSEL0W { w: self }
    }
    #[doc = "Bit 1 - Channel Selection"]
    #[inline]
    pub fn chsel1(&mut self) -> _CHSEL1W {
        _CHSEL1W { w: self }
    }
    #[doc = "Bit 2 - Channel Selection"]
    #[inline]
    pub fn chsel2(&mut self) -> _CHSEL2W {
        _CHSEL2W { w: self }
    }
    #[doc = "Bit 3 - Channel Selection"]
    #[inline]
    pub fn chsel3(&mut self) -> _CHSEL3W {
        _CHSEL3W { w: self }
    }
    #[doc = "Bit 4 - Channel Selection"]
    #[inline]
    pub fn chsel4(&mut self) -> _CHSEL4W {
        _CHSEL4W { w: self }
    }
    #[doc = "Bit 5 - Channel Selection"]
    #[inline]
    pub fn chsel5(&mut self) -> _CHSEL5W {
        _CHSEL5W { w: self }
    }
    #[doc = "Bit 6 - Channel Selection"]
    #[inline]
    pub fn chsel6(&mut self) -> _CHSEL6W {
        _CHSEL6W { w: self }
    }
    #[doc = "Bit 7 - Channel Selection"]
    #[inline]
    pub fn chsel7(&mut self) -> _CHSEL7W {
        _CHSEL7W { w: self }
    }
}
