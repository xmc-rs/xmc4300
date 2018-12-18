#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SGLREQDSTREG {
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
pub struct CH0R {
    bits: bool,
}
impl CH0R {
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
pub struct CH1R {
    bits: bool,
}
impl CH1R {
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
pub struct CH2R {
    bits: bool,
}
impl CH2R {
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
pub struct CH3R {
    bits: bool,
}
impl CH3R {
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
pub struct CH4R {
    bits: bool,
}
impl CH4R {
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
pub struct CH5R {
    bits: bool,
}
impl CH5R {
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
pub struct CH6R {
    bits: bool,
}
impl CH6R {
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
pub struct CH7R {
    bits: bool,
}
impl CH7R {
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
#[doc = r" Proxy"]
pub struct _CH0W<'a> {
    w: &'a mut W,
}
impl<'a> _CH0W<'a> {
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
pub struct _CH1W<'a> {
    w: &'a mut W,
}
impl<'a> _CH1W<'a> {
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
pub struct _CH2W<'a> {
    w: &'a mut W,
}
impl<'a> _CH2W<'a> {
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
pub struct _CH3W<'a> {
    w: &'a mut W,
}
impl<'a> _CH3W<'a> {
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
pub struct _CH4W<'a> {
    w: &'a mut W,
}
impl<'a> _CH4W<'a> {
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
pub struct _CH5W<'a> {
    w: &'a mut W,
}
impl<'a> _CH5W<'a> {
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
pub struct _CH6W<'a> {
    w: &'a mut W,
}
impl<'a> _CH6W<'a> {
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
pub struct _CH7W<'a> {
    w: &'a mut W,
}
impl<'a> _CH7W<'a> {
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
    #[doc = "Bit 0 - Source request for channel 0"]
    #[inline]
    pub fn ch0(&self) -> CH0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH0R { bits }
    }
    #[doc = "Bit 1 - Source request for channel 1"]
    #[inline]
    pub fn ch1(&self) -> CH1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH1R { bits }
    }
    #[doc = "Bit 2 - Source request for channel 2"]
    #[inline]
    pub fn ch2(&self) -> CH2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH2R { bits }
    }
    #[doc = "Bit 3 - Source request for channel 3"]
    #[inline]
    pub fn ch3(&self) -> CH3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH3R { bits }
    }
    #[doc = "Bit 4 - Source request for channel 4"]
    #[inline]
    pub fn ch4(&self) -> CH4R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH4R { bits }
    }
    #[doc = "Bit 5 - Source request for channel 5"]
    #[inline]
    pub fn ch5(&self) -> CH5R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH5R { bits }
    }
    #[doc = "Bit 6 - Source request for channel 6"]
    #[inline]
    pub fn ch6(&self) -> CH6R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH6R { bits }
    }
    #[doc = "Bit 7 - Source request for channel 7"]
    #[inline]
    pub fn ch7(&self) -> CH7R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH7R { bits }
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
    #[doc = "Bit 8 - Source request write enable for channel 0"]
    #[inline]
    pub fn we_ch0(&mut self) -> _WE_CH0W {
        _WE_CH0W { w: self }
    }
    #[doc = "Bit 9 - Source request write enable for channel 1"]
    #[inline]
    pub fn we_ch1(&mut self) -> _WE_CH1W {
        _WE_CH1W { w: self }
    }
    #[doc = "Bit 10 - Source request write enable for channel 2"]
    #[inline]
    pub fn we_ch2(&mut self) -> _WE_CH2W {
        _WE_CH2W { w: self }
    }
    #[doc = "Bit 11 - Source request write enable for channel 3"]
    #[inline]
    pub fn we_ch3(&mut self) -> _WE_CH3W {
        _WE_CH3W { w: self }
    }
    #[doc = "Bit 12 - Source request write enable for channel 4"]
    #[inline]
    pub fn we_ch4(&mut self) -> _WE_CH4W {
        _WE_CH4W { w: self }
    }
    #[doc = "Bit 13 - Source request write enable for channel 5"]
    #[inline]
    pub fn we_ch5(&mut self) -> _WE_CH5W {
        _WE_CH5W { w: self }
    }
    #[doc = "Bit 14 - Source request write enable for channel 6"]
    #[inline]
    pub fn we_ch6(&mut self) -> _WE_CH6W {
        _WE_CH6W { w: self }
    }
    #[doc = "Bit 15 - Source request write enable for channel 7"]
    #[inline]
    pub fn we_ch7(&mut self) -> _WE_CH7W {
        _WE_CH7W { w: self }
    }
    #[doc = "Bit 0 - Source request for channel 0"]
    #[inline]
    pub fn ch0(&mut self) -> _CH0W {
        _CH0W { w: self }
    }
    #[doc = "Bit 1 - Source request for channel 1"]
    #[inline]
    pub fn ch1(&mut self) -> _CH1W {
        _CH1W { w: self }
    }
    #[doc = "Bit 2 - Source request for channel 2"]
    #[inline]
    pub fn ch2(&mut self) -> _CH2W {
        _CH2W { w: self }
    }
    #[doc = "Bit 3 - Source request for channel 3"]
    #[inline]
    pub fn ch3(&mut self) -> _CH3W {
        _CH3W { w: self }
    }
    #[doc = "Bit 4 - Source request for channel 4"]
    #[inline]
    pub fn ch4(&mut self) -> _CH4W {
        _CH4W { w: self }
    }
    #[doc = "Bit 5 - Source request for channel 5"]
    #[inline]
    pub fn ch5(&mut self) -> _CH5W {
        _CH5W { w: self }
    }
    #[doc = "Bit 6 - Source request for channel 6"]
    #[inline]
    pub fn ch6(&mut self) -> _CH6W {
        _CH6W { w: self }
    }
    #[doc = "Bit 7 - Source request for channel 7"]
    #[inline]
    pub fn ch7(&mut self) -> _CH7W {
        _CH7W { w: self }
    }
}
