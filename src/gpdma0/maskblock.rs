#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MASKBLOCK {
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
#[doc = "Possible values of the field `CH0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0R {
    #[doc = "masked"]
    VALUE1,
    #[doc = "unmasked"]
    VALUE2,
}
impl CH0R {
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
            CH0R::VALUE1 => false,
            CH0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH0R {
        match value {
            false => CH0R::VALUE1,
            true => CH0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CH0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CH0R::VALUE2
    }
}
#[doc = "Possible values of the field `CH1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1R {
    #[doc = "masked"]
    VALUE1,
    #[doc = "unmasked"]
    VALUE2,
}
impl CH1R {
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
            CH1R::VALUE1 => false,
            CH1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH1R {
        match value {
            false => CH1R::VALUE1,
            true => CH1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CH1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CH1R::VALUE2
    }
}
#[doc = "Possible values of the field `CH2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2R {
    #[doc = "masked"]
    VALUE1,
    #[doc = "unmasked"]
    VALUE2,
}
impl CH2R {
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
            CH2R::VALUE1 => false,
            CH2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH2R {
        match value {
            false => CH2R::VALUE1,
            true => CH2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CH2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CH2R::VALUE2
    }
}
#[doc = "Possible values of the field `CH3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3R {
    #[doc = "masked"]
    VALUE1,
    #[doc = "unmasked"]
    VALUE2,
}
impl CH3R {
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
            CH3R::VALUE1 => false,
            CH3R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH3R {
        match value {
            false => CH3R::VALUE1,
            true => CH3R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CH3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CH3R::VALUE2
    }
}
#[doc = "Possible values of the field `CH4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4R {
    #[doc = "masked"]
    VALUE1,
    #[doc = "unmasked"]
    VALUE2,
}
impl CH4R {
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
            CH4R::VALUE1 => false,
            CH4R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH4R {
        match value {
            false => CH4R::VALUE1,
            true => CH4R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CH4R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CH4R::VALUE2
    }
}
#[doc = "Possible values of the field `CH5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5R {
    #[doc = "masked"]
    VALUE1,
    #[doc = "unmasked"]
    VALUE2,
}
impl CH5R {
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
            CH5R::VALUE1 => false,
            CH5R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH5R {
        match value {
            false => CH5R::VALUE1,
            true => CH5R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CH5R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CH5R::VALUE2
    }
}
#[doc = "Possible values of the field `CH6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6R {
    #[doc = "masked"]
    VALUE1,
    #[doc = "unmasked"]
    VALUE2,
}
impl CH6R {
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
            CH6R::VALUE1 => false,
            CH6R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH6R {
        match value {
            false => CH6R::VALUE1,
            true => CH6R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CH6R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CH6R::VALUE2
    }
}
#[doc = "Possible values of the field `CH7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7R {
    #[doc = "masked"]
    VALUE1,
    #[doc = "unmasked"]
    VALUE2,
}
impl CH7R {
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
            CH7R::VALUE1 => false,
            CH7R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH7R {
        match value {
            false => CH7R::VALUE1,
            true => CH7R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CH7R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CH7R::VALUE2
    }
}
#[doc = "Values that can be written to the field `WE_CH0`"]
pub enum WE_CH0W {
    #[doc = "write disabled"]
    VALUE1,
    #[doc = "write enabled"]
    VALUE2,
}
impl WE_CH0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WE_CH0W::VALUE1 => false,
            WE_CH0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WE_CH0W<'a> {
    w: &'a mut W,
}
impl<'a> _WE_CH0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WE_CH0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "write disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WE_CH0W::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WE_CH0W::VALUE2)
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
#[doc = "Values that can be written to the field `WE_CH1`"]
pub enum WE_CH1W {
    #[doc = "write disabled"]
    VALUE1,
    #[doc = "write enabled"]
    VALUE2,
}
impl WE_CH1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WE_CH1W::VALUE1 => false,
            WE_CH1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WE_CH1W<'a> {
    w: &'a mut W,
}
impl<'a> _WE_CH1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WE_CH1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "write disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WE_CH1W::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WE_CH1W::VALUE2)
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
#[doc = "Values that can be written to the field `WE_CH2`"]
pub enum WE_CH2W {
    #[doc = "write disabled"]
    VALUE1,
    #[doc = "write enabled"]
    VALUE2,
}
impl WE_CH2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WE_CH2W::VALUE1 => false,
            WE_CH2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WE_CH2W<'a> {
    w: &'a mut W,
}
impl<'a> _WE_CH2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WE_CH2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "write disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WE_CH2W::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WE_CH2W::VALUE2)
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
#[doc = "Values that can be written to the field `WE_CH3`"]
pub enum WE_CH3W {
    #[doc = "write disabled"]
    VALUE1,
    #[doc = "write enabled"]
    VALUE2,
}
impl WE_CH3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WE_CH3W::VALUE1 => false,
            WE_CH3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WE_CH3W<'a> {
    w: &'a mut W,
}
impl<'a> _WE_CH3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WE_CH3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "write disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WE_CH3W::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WE_CH3W::VALUE2)
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
#[doc = "Values that can be written to the field `WE_CH4`"]
pub enum WE_CH4W {
    #[doc = "write disabled"]
    VALUE1,
    #[doc = "write enabled"]
    VALUE2,
}
impl WE_CH4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WE_CH4W::VALUE1 => false,
            WE_CH4W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WE_CH4W<'a> {
    w: &'a mut W,
}
impl<'a> _WE_CH4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WE_CH4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "write disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WE_CH4W::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WE_CH4W::VALUE2)
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
#[doc = "Values that can be written to the field `WE_CH5`"]
pub enum WE_CH5W {
    #[doc = "write disabled"]
    VALUE1,
    #[doc = "write enabled"]
    VALUE2,
}
impl WE_CH5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WE_CH5W::VALUE1 => false,
            WE_CH5W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WE_CH5W<'a> {
    w: &'a mut W,
}
impl<'a> _WE_CH5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WE_CH5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "write disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WE_CH5W::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WE_CH5W::VALUE2)
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
#[doc = "Values that can be written to the field `WE_CH6`"]
pub enum WE_CH6W {
    #[doc = "write disabled"]
    VALUE1,
    #[doc = "write enabled"]
    VALUE2,
}
impl WE_CH6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WE_CH6W::VALUE1 => false,
            WE_CH6W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WE_CH6W<'a> {
    w: &'a mut W,
}
impl<'a> _WE_CH6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WE_CH6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "write disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WE_CH6W::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WE_CH6W::VALUE2)
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
#[doc = "Values that can be written to the field `WE_CH7`"]
pub enum WE_CH7W {
    #[doc = "write disabled"]
    VALUE1,
    #[doc = "write enabled"]
    VALUE2,
}
impl WE_CH7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WE_CH7W::VALUE1 => false,
            WE_CH7W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WE_CH7W<'a> {
    w: &'a mut W,
}
impl<'a> _WE_CH7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WE_CH7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "write disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WE_CH7W::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WE_CH7W::VALUE2)
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
#[doc = "Values that can be written to the field `CH0`"]
pub enum CH0W {
    #[doc = "masked"]
    VALUE1,
    #[doc = "unmasked"]
    VALUE2,
}
impl CH0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH0W::VALUE1 => false,
            CH0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH0W<'a> {
    w: &'a mut W,
}
impl<'a> _CH0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "masked"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH0W::VALUE1)
    }
    #[doc = "unmasked"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH0W::VALUE2)
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
#[doc = "Values that can be written to the field `CH1`"]
pub enum CH1W {
    #[doc = "masked"]
    VALUE1,
    #[doc = "unmasked"]
    VALUE2,
}
impl CH1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH1W::VALUE1 => false,
            CH1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH1W<'a> {
    w: &'a mut W,
}
impl<'a> _CH1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "masked"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH1W::VALUE1)
    }
    #[doc = "unmasked"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH1W::VALUE2)
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
#[doc = "Values that can be written to the field `CH2`"]
pub enum CH2W {
    #[doc = "masked"]
    VALUE1,
    #[doc = "unmasked"]
    VALUE2,
}
impl CH2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH2W::VALUE1 => false,
            CH2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH2W<'a> {
    w: &'a mut W,
}
impl<'a> _CH2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "masked"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH2W::VALUE1)
    }
    #[doc = "unmasked"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH2W::VALUE2)
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
#[doc = "Values that can be written to the field `CH3`"]
pub enum CH3W {
    #[doc = "masked"]
    VALUE1,
    #[doc = "unmasked"]
    VALUE2,
}
impl CH3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH3W::VALUE1 => false,
            CH3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH3W<'a> {
    w: &'a mut W,
}
impl<'a> _CH3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "masked"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH3W::VALUE1)
    }
    #[doc = "unmasked"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH3W::VALUE2)
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
#[doc = "Values that can be written to the field `CH4`"]
pub enum CH4W {
    #[doc = "masked"]
    VALUE1,
    #[doc = "unmasked"]
    VALUE2,
}
impl CH4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH4W::VALUE1 => false,
            CH4W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH4W<'a> {
    w: &'a mut W,
}
impl<'a> _CH4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "masked"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH4W::VALUE1)
    }
    #[doc = "unmasked"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH4W::VALUE2)
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
#[doc = "Values that can be written to the field `CH5`"]
pub enum CH5W {
    #[doc = "masked"]
    VALUE1,
    #[doc = "unmasked"]
    VALUE2,
}
impl CH5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH5W::VALUE1 => false,
            CH5W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH5W<'a> {
    w: &'a mut W,
}
impl<'a> _CH5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "masked"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH5W::VALUE1)
    }
    #[doc = "unmasked"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH5W::VALUE2)
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
#[doc = "Values that can be written to the field `CH6`"]
pub enum CH6W {
    #[doc = "masked"]
    VALUE1,
    #[doc = "unmasked"]
    VALUE2,
}
impl CH6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH6W::VALUE1 => false,
            CH6W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH6W<'a> {
    w: &'a mut W,
}
impl<'a> _CH6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "masked"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH6W::VALUE1)
    }
    #[doc = "unmasked"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH6W::VALUE2)
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
#[doc = "Values that can be written to the field `CH7`"]
pub enum CH7W {
    #[doc = "masked"]
    VALUE1,
    #[doc = "unmasked"]
    VALUE2,
}
impl CH7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH7W::VALUE1 => false,
            CH7W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH7W<'a> {
    w: &'a mut W,
}
impl<'a> _CH7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "masked"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH7W::VALUE1)
    }
    #[doc = "unmasked"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH7W::VALUE2)
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
    #[doc = "Bit 0 - Mask bit for channel 0"]
    #[inline]
    pub fn ch0(&self) -> CH0R {
        CH0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Mask bit for channel 1"]
    #[inline]
    pub fn ch1(&self) -> CH1R {
        CH1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Mask bit for channel 2"]
    #[inline]
    pub fn ch2(&self) -> CH2R {
        CH2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Mask bit for channel 3"]
    #[inline]
    pub fn ch3(&self) -> CH3R {
        CH3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Mask bit for channel 4"]
    #[inline]
    pub fn ch4(&self) -> CH4R {
        CH4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Mask bit for channel 5"]
    #[inline]
    pub fn ch5(&self) -> CH5R {
        CH5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Mask bit for channel 6"]
    #[inline]
    pub fn ch6(&self) -> CH6R {
        CH6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Mask bit for channel 7"]
    #[inline]
    pub fn ch7(&self) -> CH7R {
        CH7R::_from({
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
    #[doc = "Bit 8 - Write enable for mask bit of channel 0"]
    #[inline]
    pub fn we_ch0(&mut self) -> _WE_CH0W {
        _WE_CH0W { w: self }
    }
    #[doc = "Bit 9 - Write enable for mask bit of channel 1"]
    #[inline]
    pub fn we_ch1(&mut self) -> _WE_CH1W {
        _WE_CH1W { w: self }
    }
    #[doc = "Bit 10 - Write enable for mask bit of channel 2"]
    #[inline]
    pub fn we_ch2(&mut self) -> _WE_CH2W {
        _WE_CH2W { w: self }
    }
    #[doc = "Bit 11 - Write enable for mask bit of channel 3"]
    #[inline]
    pub fn we_ch3(&mut self) -> _WE_CH3W {
        _WE_CH3W { w: self }
    }
    #[doc = "Bit 12 - Write enable for mask bit of channel 4"]
    #[inline]
    pub fn we_ch4(&mut self) -> _WE_CH4W {
        _WE_CH4W { w: self }
    }
    #[doc = "Bit 13 - Write enable for mask bit of channel 5"]
    #[inline]
    pub fn we_ch5(&mut self) -> _WE_CH5W {
        _WE_CH5W { w: self }
    }
    #[doc = "Bit 14 - Write enable for mask bit of channel 6"]
    #[inline]
    pub fn we_ch6(&mut self) -> _WE_CH6W {
        _WE_CH6W { w: self }
    }
    #[doc = "Bit 15 - Write enable for mask bit of channel 7"]
    #[inline]
    pub fn we_ch7(&mut self) -> _WE_CH7W {
        _WE_CH7W { w: self }
    }
    #[doc = "Bit 0 - Mask bit for channel 0"]
    #[inline]
    pub fn ch0(&mut self) -> _CH0W {
        _CH0W { w: self }
    }
    #[doc = "Bit 1 - Mask bit for channel 1"]
    #[inline]
    pub fn ch1(&mut self) -> _CH1W {
        _CH1W { w: self }
    }
    #[doc = "Bit 2 - Mask bit for channel 2"]
    #[inline]
    pub fn ch2(&mut self) -> _CH2W {
        _CH2W { w: self }
    }
    #[doc = "Bit 3 - Mask bit for channel 3"]
    #[inline]
    pub fn ch3(&mut self) -> _CH3W {
        _CH3W { w: self }
    }
    #[doc = "Bit 4 - Mask bit for channel 4"]
    #[inline]
    pub fn ch4(&mut self) -> _CH4W {
        _CH4W { w: self }
    }
    #[doc = "Bit 5 - Mask bit for channel 5"]
    #[inline]
    pub fn ch5(&mut self) -> _CH5W {
        _CH5W { w: self }
    }
    #[doc = "Bit 6 - Mask bit for channel 6"]
    #[inline]
    pub fn ch6(&mut self) -> _CH6W {
        _CH6W { w: self }
    }
    #[doc = "Bit 7 - Mask bit for channel 7"]
    #[inline]
    pub fn ch7(&mut self) -> _CH7W {
        _CH7W { w: self }
    }
}
