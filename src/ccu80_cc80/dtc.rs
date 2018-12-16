#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DTC {
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
#[doc = "Possible values of the field `DTE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTE1R {
    #[doc = "Dead Time for channel 1 is disabled"]
    VALUE1,
    #[doc = "Dead Time for channel 1 is enabled"]
    VALUE2,
}
impl DTE1R {
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
            DTE1R::VALUE1 => false,
            DTE1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DTE1R {
        match value {
            false => DTE1R::VALUE1,
            true => DTE1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DTE1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DTE1R::VALUE2
    }
}
#[doc = "Possible values of the field `DTE2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTE2R {
    #[doc = "Dead Time for channel 2 is disabled"]
    VALUE1,
    #[doc = "Dead Time for channel 2 is enabled"]
    VALUE2,
}
impl DTE2R {
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
            DTE2R::VALUE1 => false,
            DTE2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DTE2R {
        match value {
            false => DTE2R::VALUE1,
            true => DTE2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DTE2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DTE2R::VALUE2
    }
}
#[doc = "Possible values of the field `DCEN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCEN1R {
    #[doc = "Dead Time for CC8yST1 path is disabled"]
    VALUE1,
    #[doc = "Dead Time for CC8yST1 path is enabled"]
    VALUE2,
}
impl DCEN1R {
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
            DCEN1R::VALUE1 => false,
            DCEN1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCEN1R {
        match value {
            false => DCEN1R::VALUE1,
            true => DCEN1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DCEN1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DCEN1R::VALUE2
    }
}
#[doc = "Possible values of the field `DCEN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCEN2R {
    #[doc = "Dead Time for inverted CC8yST1 path is disabled"]
    VALUE1,
    #[doc = "Dead Time for inverted CC8yST1 path is enabled"]
    VALUE2,
}
impl DCEN2R {
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
            DCEN2R::VALUE1 => false,
            DCEN2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCEN2R {
        match value {
            false => DCEN2R::VALUE1,
            true => DCEN2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DCEN2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DCEN2R::VALUE2
    }
}
#[doc = "Possible values of the field `DCEN3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCEN3R {
    #[doc = "Dead Time for CC8yST2 path is disabled"]
    VALUE1,
    #[doc = "Dead Time for CC8yST2 path is enabled"]
    VALUE2,
}
impl DCEN3R {
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
            DCEN3R::VALUE1 => false,
            DCEN3R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCEN3R {
        match value {
            false => DCEN3R::VALUE1,
            true => DCEN3R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DCEN3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DCEN3R::VALUE2
    }
}
#[doc = "Possible values of the field `DCEN4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCEN4R {
    #[doc = "Dead Time for inverted CC8yST2 path is disabled"]
    VALUE1,
    #[doc = "Dead Time for inverted CC8yST2 path is enabled"]
    VALUE2,
}
impl DCEN4R {
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
            DCEN4R::VALUE1 => false,
            DCEN4R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCEN4R {
        match value {
            false => DCEN4R::VALUE1,
            true => DCEN4R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DCEN4R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DCEN4R::VALUE2
    }
}
#[doc = "Possible values of the field `DTCC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTCCR {
    #[doc = "ftclk"]
    VALUE1,
    #[doc = "ftclk/2"]
    VALUE2,
    #[doc = "ftclk/4"]
    VALUE3,
    #[doc = "ftclk/8"]
    VALUE4,
}
impl DTCCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DTCCR::VALUE1 => 0,
            DTCCR::VALUE2 => 1,
            DTCCR::VALUE3 => 2,
            DTCCR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DTCCR {
        match value {
            0 => DTCCR::VALUE1,
            1 => DTCCR::VALUE2,
            2 => DTCCR::VALUE3,
            3 => DTCCR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DTCCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DTCCR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == DTCCR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == DTCCR::VALUE4
    }
}
#[doc = "Values that can be written to the field `DTE1`"]
pub enum DTE1W {
    #[doc = "Dead Time for channel 1 is disabled"]
    VALUE1,
    #[doc = "Dead Time for channel 1 is enabled"]
    VALUE2,
}
impl DTE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DTE1W::VALUE1 => false,
            DTE1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTE1W<'a> {
    w: &'a mut W,
}
impl<'a> _DTE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTE1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Dead Time for channel 1 is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DTE1W::VALUE1)
    }
    #[doc = "Dead Time for channel 1 is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DTE1W::VALUE2)
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
#[doc = "Values that can be written to the field `DTE2`"]
pub enum DTE2W {
    #[doc = "Dead Time for channel 2 is disabled"]
    VALUE1,
    #[doc = "Dead Time for channel 2 is enabled"]
    VALUE2,
}
impl DTE2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DTE2W::VALUE1 => false,
            DTE2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTE2W<'a> {
    w: &'a mut W,
}
impl<'a> _DTE2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTE2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Dead Time for channel 2 is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DTE2W::VALUE1)
    }
    #[doc = "Dead Time for channel 2 is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DTE2W::VALUE2)
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
#[doc = "Values that can be written to the field `DCEN1`"]
pub enum DCEN1W {
    #[doc = "Dead Time for CC8yST1 path is disabled"]
    VALUE1,
    #[doc = "Dead Time for CC8yST1 path is enabled"]
    VALUE2,
}
impl DCEN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DCEN1W::VALUE1 => false,
            DCEN1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _DCEN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCEN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Dead Time for CC8yST1 path is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DCEN1W::VALUE1)
    }
    #[doc = "Dead Time for CC8yST1 path is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DCEN1W::VALUE2)
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
#[doc = "Values that can be written to the field `DCEN2`"]
pub enum DCEN2W {
    #[doc = "Dead Time for inverted CC8yST1 path is disabled"]
    VALUE1,
    #[doc = "Dead Time for inverted CC8yST1 path is enabled"]
    VALUE2,
}
impl DCEN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DCEN2W::VALUE1 => false,
            DCEN2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _DCEN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCEN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Dead Time for inverted CC8yST1 path is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DCEN2W::VALUE1)
    }
    #[doc = "Dead Time for inverted CC8yST1 path is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DCEN2W::VALUE2)
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
#[doc = "Values that can be written to the field `DCEN3`"]
pub enum DCEN3W {
    #[doc = "Dead Time for CC8yST2 path is disabled"]
    VALUE1,
    #[doc = "Dead Time for CC8yST2 path is enabled"]
    VALUE2,
}
impl DCEN3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DCEN3W::VALUE1 => false,
            DCEN3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCEN3W<'a> {
    w: &'a mut W,
}
impl<'a> _DCEN3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCEN3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Dead Time for CC8yST2 path is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DCEN3W::VALUE1)
    }
    #[doc = "Dead Time for CC8yST2 path is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DCEN3W::VALUE2)
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
#[doc = "Values that can be written to the field `DCEN4`"]
pub enum DCEN4W {
    #[doc = "Dead Time for inverted CC8yST2 path is disabled"]
    VALUE1,
    #[doc = "Dead Time for inverted CC8yST2 path is enabled"]
    VALUE2,
}
impl DCEN4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DCEN4W::VALUE1 => false,
            DCEN4W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCEN4W<'a> {
    w: &'a mut W,
}
impl<'a> _DCEN4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCEN4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Dead Time for inverted CC8yST2 path is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DCEN4W::VALUE1)
    }
    #[doc = "Dead Time for inverted CC8yST2 path is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DCEN4W::VALUE2)
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
#[doc = "Values that can be written to the field `DTCC`"]
pub enum DTCCW {
    #[doc = "ftclk"]
    VALUE1,
    #[doc = "ftclk/2"]
    VALUE2,
    #[doc = "ftclk/4"]
    VALUE3,
    #[doc = "ftclk/8"]
    VALUE4,
}
impl DTCCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DTCCW::VALUE1 => 0,
            DTCCW::VALUE2 => 1,
            DTCCW::VALUE3 => 2,
            DTCCW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTCCW<'a> {
    w: &'a mut W,
}
impl<'a> _DTCCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTCCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "ftclk"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DTCCW::VALUE1)
    }
    #[doc = "ftclk/2"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DTCCW::VALUE2)
    }
    #[doc = "ftclk/4"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(DTCCW::VALUE3)
    }
    #[doc = "ftclk/8"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(DTCCW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
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
    #[doc = "Bit 0 - Dead Time Enable for Channel 1"]
    #[inline]
    pub fn dte1(&self) -> DTE1R {
        DTE1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Dead Time Enable for Channel 2"]
    #[inline]
    pub fn dte2(&self) -> DTE2R {
        DTE2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Dead Time Enable for CC8yST1"]
    #[inline]
    pub fn dcen1(&self) -> DCEN1R {
        DCEN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Dead Time Enable for inverted CC8yST1"]
    #[inline]
    pub fn dcen2(&self) -> DCEN2R {
        DCEN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Dead Time Enable for CC8yST2"]
    #[inline]
    pub fn dcen3(&self) -> DCEN3R {
        DCEN3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Dead Time Enable for inverted CC8yST2"]
    #[inline]
    pub fn dcen4(&self) -> DCEN4R {
        DCEN4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 6:7 - Dead Time clock control"]
    #[inline]
    pub fn dtcc(&self) -> DTCCR {
        DTCCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
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
    #[doc = "Bit 0 - Dead Time Enable for Channel 1"]
    #[inline]
    pub fn dte1(&mut self) -> _DTE1W {
        _DTE1W { w: self }
    }
    #[doc = "Bit 1 - Dead Time Enable for Channel 2"]
    #[inline]
    pub fn dte2(&mut self) -> _DTE2W {
        _DTE2W { w: self }
    }
    #[doc = "Bit 2 - Dead Time Enable for CC8yST1"]
    #[inline]
    pub fn dcen1(&mut self) -> _DCEN1W {
        _DCEN1W { w: self }
    }
    #[doc = "Bit 3 - Dead Time Enable for inverted CC8yST1"]
    #[inline]
    pub fn dcen2(&mut self) -> _DCEN2W {
        _DCEN2W { w: self }
    }
    #[doc = "Bit 4 - Dead Time Enable for CC8yST2"]
    #[inline]
    pub fn dcen3(&mut self) -> _DCEN3W {
        _DCEN3W { w: self }
    }
    #[doc = "Bit 5 - Dead Time Enable for inverted CC8yST2"]
    #[inline]
    pub fn dcen4(&mut self) -> _DCEN4W {
        _DCEN4W { w: self }
    }
    #[doc = "Bits 6:7 - Dead Time clock control"]
    #[inline]
    pub fn dtcc(&mut self) -> _DTCCW {
        _DTCCW { w: self }
    }
}
