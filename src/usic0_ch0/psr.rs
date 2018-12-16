#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PSR {
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
#[doc = r" Value of the field"]
pub struct ST0R {
    bits: bool,
}
impl ST0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct ST1R {
    bits: bool,
}
impl ST1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct ST2R {
    bits: bool,
}
impl ST2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct ST3R {
    bits: bool,
}
impl ST3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct ST4R {
    bits: bool,
}
impl ST4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct ST5R {
    bits: bool,
}
impl ST5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct ST6R {
    bits: bool,
}
impl ST6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct ST7R {
    bits: bool,
}
impl ST7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct ST8R {
    bits: bool,
}
impl ST8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct ST9R {
    bits: bool,
}
impl ST9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `RSIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSIFR {
    #[doc = "A receiver start event has not occurred."]
    VALUE1,
    #[doc = "A receiver start event has occurred."]
    VALUE2,
}
impl RSIFR {
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
            RSIFR::VALUE1 => false,
            RSIFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RSIFR {
        match value {
            false => RSIFR::VALUE1,
            true => RSIFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RSIFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RSIFR::VALUE2
    }
}
#[doc = "Possible values of the field `DLIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLIFR {
    #[doc = "A data lost event has not occurred."]
    VALUE1,
    #[doc = "A data lost event has occurred."]
    VALUE2,
}
impl DLIFR {
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
            DLIFR::VALUE1 => false,
            DLIFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DLIFR {
        match value {
            false => DLIFR::VALUE1,
            true => DLIFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DLIFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DLIFR::VALUE2
    }
}
#[doc = "Possible values of the field `TSIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSIFR {
    #[doc = "A transmit shift event has not occurred."]
    VALUE1,
    #[doc = "A transmit shift event has occurred."]
    VALUE2,
}
impl TSIFR {
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
            TSIFR::VALUE1 => false,
            TSIFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSIFR {
        match value {
            false => TSIFR::VALUE1,
            true => TSIFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TSIFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TSIFR::VALUE2
    }
}
#[doc = "Possible values of the field `TBIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBIFR {
    #[doc = "A transmit buffer event has not occurred."]
    VALUE1,
    #[doc = "A transmit buffer event has occurred."]
    VALUE2,
}
impl TBIFR {
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
            TBIFR::VALUE1 => false,
            TBIFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TBIFR {
        match value {
            false => TBIFR::VALUE1,
            true => TBIFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TBIFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TBIFR::VALUE2
    }
}
#[doc = "Possible values of the field `RIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RIFR {
    #[doc = "A receive event has not occurred."]
    VALUE1,
    #[doc = "A receive event has occurred."]
    VALUE2,
}
impl RIFR {
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
            RIFR::VALUE1 => false,
            RIFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RIFR {
        match value {
            false => RIFR::VALUE1,
            true => RIFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RIFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RIFR::VALUE2
    }
}
#[doc = "Possible values of the field `AIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AIFR {
    #[doc = "An alternative receive event has not occurred."]
    VALUE1,
    #[doc = "An alternative receive event has occurred."]
    VALUE2,
}
impl AIFR {
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
            AIFR::VALUE1 => false,
            AIFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AIFR {
        match value {
            false => AIFR::VALUE1,
            true => AIFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AIFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == AIFR::VALUE2
    }
}
#[doc = "Possible values of the field `BRGIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRGIFR {
    #[doc = "A baud rate generator event has not occurred."]
    VALUE1,
    #[doc = "A baud rate generator event has occurred."]
    VALUE2,
}
impl BRGIFR {
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
            BRGIFR::VALUE1 => false,
            BRGIFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BRGIFR {
        match value {
            false => BRGIFR::VALUE1,
            true => BRGIFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BRGIFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BRGIFR::VALUE2
    }
}
#[doc = r" Proxy"]
pub struct _ST0W<'a> {
    w: &'a mut W,
}
impl<'a> _ST0W<'a> {
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
#[doc = r" Proxy"]
pub struct _ST1W<'a> {
    w: &'a mut W,
}
impl<'a> _ST1W<'a> {
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
#[doc = r" Proxy"]
pub struct _ST2W<'a> {
    w: &'a mut W,
}
impl<'a> _ST2W<'a> {
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
#[doc = r" Proxy"]
pub struct _ST3W<'a> {
    w: &'a mut W,
}
impl<'a> _ST3W<'a> {
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
#[doc = r" Proxy"]
pub struct _ST4W<'a> {
    w: &'a mut W,
}
impl<'a> _ST4W<'a> {
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
#[doc = r" Proxy"]
pub struct _ST5W<'a> {
    w: &'a mut W,
}
impl<'a> _ST5W<'a> {
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
#[doc = r" Proxy"]
pub struct _ST6W<'a> {
    w: &'a mut W,
}
impl<'a> _ST6W<'a> {
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
#[doc = r" Proxy"]
pub struct _ST7W<'a> {
    w: &'a mut W,
}
impl<'a> _ST7W<'a> {
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
#[doc = r" Proxy"]
pub struct _ST8W<'a> {
    w: &'a mut W,
}
impl<'a> _ST8W<'a> {
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
#[doc = r" Proxy"]
pub struct _ST9W<'a> {
    w: &'a mut W,
}
impl<'a> _ST9W<'a> {
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
#[doc = "Values that can be written to the field `RSIF`"]
pub enum RSIFW {
    #[doc = "A receiver start event has not occurred."]
    VALUE1,
    #[doc = "A receiver start event has occurred."]
    VALUE2,
}
impl RSIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSIFW::VALUE1 => false,
            RSIFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSIFW<'a> {
    w: &'a mut W,
}
impl<'a> _RSIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A receiver start event has not occurred."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RSIFW::VALUE1)
    }
    #[doc = "A receiver start event has occurred."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RSIFW::VALUE2)
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
#[doc = "Values that can be written to the field `DLIF`"]
pub enum DLIFW {
    #[doc = "A data lost event has not occurred."]
    VALUE1,
    #[doc = "A data lost event has occurred."]
    VALUE2,
}
impl DLIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DLIFW::VALUE1 => false,
            DLIFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DLIFW<'a> {
    w: &'a mut W,
}
impl<'a> _DLIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DLIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A data lost event has not occurred."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DLIFW::VALUE1)
    }
    #[doc = "A data lost event has occurred."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DLIFW::VALUE2)
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
#[doc = "Values that can be written to the field `TSIF`"]
pub enum TSIFW {
    #[doc = "A transmit shift event has not occurred."]
    VALUE1,
    #[doc = "A transmit shift event has occurred."]
    VALUE2,
}
impl TSIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSIFW::VALUE1 => false,
            TSIFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSIFW<'a> {
    w: &'a mut W,
}
impl<'a> _TSIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A transmit shift event has not occurred."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TSIFW::VALUE1)
    }
    #[doc = "A transmit shift event has occurred."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TSIFW::VALUE2)
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
#[doc = "Values that can be written to the field `TBIF`"]
pub enum TBIFW {
    #[doc = "A transmit buffer event has not occurred."]
    VALUE1,
    #[doc = "A transmit buffer event has occurred."]
    VALUE2,
}
impl TBIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TBIFW::VALUE1 => false,
            TBIFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TBIFW<'a> {
    w: &'a mut W,
}
impl<'a> _TBIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TBIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A transmit buffer event has not occurred."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TBIFW::VALUE1)
    }
    #[doc = "A transmit buffer event has occurred."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TBIFW::VALUE2)
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
#[doc = "Values that can be written to the field `RIF`"]
pub enum RIFW {
    #[doc = "A receive event has not occurred."]
    VALUE1,
    #[doc = "A receive event has occurred."]
    VALUE2,
}
impl RIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RIFW::VALUE1 => false,
            RIFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RIFW<'a> {
    w: &'a mut W,
}
impl<'a> _RIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A receive event has not occurred."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RIFW::VALUE1)
    }
    #[doc = "A receive event has occurred."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RIFW::VALUE2)
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
#[doc = "Values that can be written to the field `AIF`"]
pub enum AIFW {
    #[doc = "An alternative receive event has not occurred."]
    VALUE1,
    #[doc = "An alternative receive event has occurred."]
    VALUE2,
}
impl AIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AIFW::VALUE1 => false,
            AIFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AIFW<'a> {
    w: &'a mut W,
}
impl<'a> _AIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An alternative receive event has not occurred."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(AIFW::VALUE1)
    }
    #[doc = "An alternative receive event has occurred."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(AIFW::VALUE2)
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
#[doc = "Values that can be written to the field `BRGIF`"]
pub enum BRGIFW {
    #[doc = "A baud rate generator event has not occurred."]
    VALUE1,
    #[doc = "A baud rate generator event has occurred."]
    VALUE2,
}
impl BRGIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BRGIFW::VALUE1 => false,
            BRGIFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BRGIFW<'a> {
    w: &'a mut W,
}
impl<'a> _BRGIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BRGIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A baud rate generator event has not occurred."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BRGIFW::VALUE1)
    }
    #[doc = "A baud rate generator event has occurred."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BRGIFW::VALUE2)
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
        const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - Protocol Status Flag 0"]
    #[inline]
    pub fn st0(&self) -> ST0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ST0R { bits }
    }
    #[doc = "Bit 1 - Protocol Status Flag 1"]
    #[inline]
    pub fn st1(&self) -> ST1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ST1R { bits }
    }
    #[doc = "Bit 2 - Protocol Status Flag 2"]
    #[inline]
    pub fn st2(&self) -> ST2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ST2R { bits }
    }
    #[doc = "Bit 3 - Protocol Status Flag 3"]
    #[inline]
    pub fn st3(&self) -> ST3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ST3R { bits }
    }
    #[doc = "Bit 4 - Protocol Status Flag 4"]
    #[inline]
    pub fn st4(&self) -> ST4R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ST4R { bits }
    }
    #[doc = "Bit 5 - Protocol Status Flag 5"]
    #[inline]
    pub fn st5(&self) -> ST5R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ST5R { bits }
    }
    #[doc = "Bit 6 - Protocol Status Flag 6"]
    #[inline]
    pub fn st6(&self) -> ST6R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ST6R { bits }
    }
    #[doc = "Bit 7 - Protocol Status Flag 7"]
    #[inline]
    pub fn st7(&self) -> ST7R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ST7R { bits }
    }
    #[doc = "Bit 8 - Protocol Status Flag 8"]
    #[inline]
    pub fn st8(&self) -> ST8R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ST8R { bits }
    }
    #[doc = "Bit 9 - Protocol Status Flag 9"]
    #[inline]
    pub fn st9(&self) -> ST9R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ST9R { bits }
    }
    #[doc = "Bit 10 - Receiver Start Indication Flag"]
    #[inline]
    pub fn rsif(&self) -> RSIFR {
        RSIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Data Lost Indication Flag"]
    #[inline]
    pub fn dlif(&self) -> DLIFR {
        DLIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Transmit Shift Indication Flag"]
    #[inline]
    pub fn tsif(&self) -> TSIFR {
        TSIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Transmit Buffer Indication Flag"]
    #[inline]
    pub fn tbif(&self) -> TBIFR {
        TBIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Receive Indication Flag"]
    #[inline]
    pub fn rif(&self) -> RIFR {
        RIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Alternative Receive Indication Flag"]
    #[inline]
    pub fn aif(&self) -> AIFR {
        AIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Baud Rate Generator Indication Flag"]
    #[inline]
    pub fn brgif(&self) -> BRGIFR {
        BRGIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - Protocol Status Flag 0"]
    #[inline]
    pub fn st0(&mut self) -> _ST0W {
        _ST0W { w: self }
    }
    #[doc = "Bit 1 - Protocol Status Flag 1"]
    #[inline]
    pub fn st1(&mut self) -> _ST1W {
        _ST1W { w: self }
    }
    #[doc = "Bit 2 - Protocol Status Flag 2"]
    #[inline]
    pub fn st2(&mut self) -> _ST2W {
        _ST2W { w: self }
    }
    #[doc = "Bit 3 - Protocol Status Flag 3"]
    #[inline]
    pub fn st3(&mut self) -> _ST3W {
        _ST3W { w: self }
    }
    #[doc = "Bit 4 - Protocol Status Flag 4"]
    #[inline]
    pub fn st4(&mut self) -> _ST4W {
        _ST4W { w: self }
    }
    #[doc = "Bit 5 - Protocol Status Flag 5"]
    #[inline]
    pub fn st5(&mut self) -> _ST5W {
        _ST5W { w: self }
    }
    #[doc = "Bit 6 - Protocol Status Flag 6"]
    #[inline]
    pub fn st6(&mut self) -> _ST6W {
        _ST6W { w: self }
    }
    #[doc = "Bit 7 - Protocol Status Flag 7"]
    #[inline]
    pub fn st7(&mut self) -> _ST7W {
        _ST7W { w: self }
    }
    #[doc = "Bit 8 - Protocol Status Flag 8"]
    #[inline]
    pub fn st8(&mut self) -> _ST8W {
        _ST8W { w: self }
    }
    #[doc = "Bit 9 - Protocol Status Flag 9"]
    #[inline]
    pub fn st9(&mut self) -> _ST9W {
        _ST9W { w: self }
    }
    #[doc = "Bit 10 - Receiver Start Indication Flag"]
    #[inline]
    pub fn rsif(&mut self) -> _RSIFW {
        _RSIFW { w: self }
    }
    #[doc = "Bit 11 - Data Lost Indication Flag"]
    #[inline]
    pub fn dlif(&mut self) -> _DLIFW {
        _DLIFW { w: self }
    }
    #[doc = "Bit 12 - Transmit Shift Indication Flag"]
    #[inline]
    pub fn tsif(&mut self) -> _TSIFW {
        _TSIFW { w: self }
    }
    #[doc = "Bit 13 - Transmit Buffer Indication Flag"]
    #[inline]
    pub fn tbif(&mut self) -> _TBIFW {
        _TBIFW { w: self }
    }
    #[doc = "Bit 14 - Receive Indication Flag"]
    #[inline]
    pub fn rif(&mut self) -> _RIFW {
        _RIFW { w: self }
    }
    #[doc = "Bit 15 - Alternative Receive Indication Flag"]
    #[inline]
    pub fn aif(&mut self) -> _AIFW {
        _AIFW { w: self }
    }
    #[doc = "Bit 16 - Baud Rate Generator Indication Flag"]
    #[inline]
    pub fn brgif(&mut self) -> _BRGIFW {
        _BRGIFW { w: self }
    }
}
