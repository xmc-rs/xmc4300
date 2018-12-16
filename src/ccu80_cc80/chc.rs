#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHC {
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
#[doc = "Possible values of the field `ASE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASER {
    #[doc = "Asymmetric PWM is disabled"]
    VALUE1,
    #[doc = "Asymmetric PWM is enabled"]
    VALUE2,
}
impl ASER {
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
            ASER::VALUE1 => false,
            ASER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASER {
        match value {
            false => ASER::VALUE1,
            true => ASER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ASER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ASER::VALUE2
    }
}
#[doc = "Possible values of the field `OCS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCS1R {
    #[doc = "CC8yST1 signal path is connected to the CCU8x.OUTy0"]
    VALUE1,
    #[doc = "Inverted CC8yST1 signal path is connected to the CCU8x.OUTy0"]
    VALUE2,
}
impl OCS1R {
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
            OCS1R::VALUE1 => false,
            OCS1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OCS1R {
        match value {
            false => OCS1R::VALUE1,
            true => OCS1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == OCS1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == OCS1R::VALUE2
    }
}
#[doc = "Possible values of the field `OCS2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCS2R {
    #[doc = "Inverted CC8yST1 signal path is connected to the CCU8x.OUTy1"]
    VALUE1,
    #[doc = "CC8yST1 signal path is connected to the CCU8x.OUTy1"]
    VALUE2,
}
impl OCS2R {
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
            OCS2R::VALUE1 => false,
            OCS2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OCS2R {
        match value {
            false => OCS2R::VALUE1,
            true => OCS2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == OCS2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == OCS2R::VALUE2
    }
}
#[doc = "Possible values of the field `OCS3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCS3R {
    #[doc = "CC8yST2 signal path is connected to the CCU8x.OUTy2"]
    VALUE1,
    #[doc = "Inverted CCST2 signal path is connected to the CCU8x.OUTy2"]
    VALUE2,
}
impl OCS3R {
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
            OCS3R::VALUE1 => false,
            OCS3R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OCS3R {
        match value {
            false => OCS3R::VALUE1,
            true => OCS3R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == OCS3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == OCS3R::VALUE2
    }
}
#[doc = "Possible values of the field `OCS4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCS4R {
    #[doc = "Inverted CC8yST2 signal path is connected to the CCU8x.OUTy3"]
    VALUE1,
    #[doc = "CC8yST2 signal path is connected to the CCU8x.OUTy3"]
    VALUE2,
}
impl OCS4R {
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
            OCS4R::VALUE1 => false,
            OCS4R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OCS4R {
        match value {
            false => OCS4R::VALUE1,
            true => OCS4R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == OCS4R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == OCS4R::VALUE2
    }
}
#[doc = "Values that can be written to the field `ASE`"]
pub enum ASEW {
    #[doc = "Asymmetric PWM is disabled"]
    VALUE1,
    #[doc = "Asymmetric PWM is enabled"]
    VALUE2,
}
impl ASEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ASEW::VALUE1 => false,
            ASEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASEW<'a> {
    w: &'a mut W,
}
impl<'a> _ASEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Asymmetric PWM is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASEW::VALUE1)
    }
    #[doc = "Asymmetric PWM is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASEW::VALUE2)
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
#[doc = "Values that can be written to the field `OCS1`"]
pub enum OCS1W {
    #[doc = "CC8yST1 signal path is connected to the CCU8x.OUTy0"]
    VALUE1,
    #[doc = "Inverted CC8yST1 signal path is connected to the CCU8x.OUTy0"]
    VALUE2,
}
impl OCS1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OCS1W::VALUE1 => false,
            OCS1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OCS1W<'a> {
    w: &'a mut W,
}
impl<'a> _OCS1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OCS1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CC8yST1 signal path is connected to the CCU8x.OUTy0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(OCS1W::VALUE1)
    }
    #[doc = "Inverted CC8yST1 signal path is connected to the CCU8x.OUTy0"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(OCS1W::VALUE2)
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
#[doc = "Values that can be written to the field `OCS2`"]
pub enum OCS2W {
    #[doc = "Inverted CC8yST1 signal path is connected to the CCU8x.OUTy1"]
    VALUE1,
    #[doc = "CC8yST1 signal path is connected to the CCU8x.OUTy1"]
    VALUE2,
}
impl OCS2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OCS2W::VALUE1 => false,
            OCS2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OCS2W<'a> {
    w: &'a mut W,
}
impl<'a> _OCS2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OCS2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Inverted CC8yST1 signal path is connected to the CCU8x.OUTy1"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(OCS2W::VALUE1)
    }
    #[doc = "CC8yST1 signal path is connected to the CCU8x.OUTy1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(OCS2W::VALUE2)
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
#[doc = "Values that can be written to the field `OCS3`"]
pub enum OCS3W {
    #[doc = "CC8yST2 signal path is connected to the CCU8x.OUTy2"]
    VALUE1,
    #[doc = "Inverted CCST2 signal path is connected to the CCU8x.OUTy2"]
    VALUE2,
}
impl OCS3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OCS3W::VALUE1 => false,
            OCS3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OCS3W<'a> {
    w: &'a mut W,
}
impl<'a> _OCS3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OCS3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CC8yST2 signal path is connected to the CCU8x.OUTy2"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(OCS3W::VALUE1)
    }
    #[doc = "Inverted CCST2 signal path is connected to the CCU8x.OUTy2"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(OCS3W::VALUE2)
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
#[doc = "Values that can be written to the field `OCS4`"]
pub enum OCS4W {
    #[doc = "Inverted CC8yST2 signal path is connected to the CCU8x.OUTy3"]
    VALUE1,
    #[doc = "CC8yST2 signal path is connected to the CCU8x.OUTy3"]
    VALUE2,
}
impl OCS4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OCS4W::VALUE1 => false,
            OCS4W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OCS4W<'a> {
    w: &'a mut W,
}
impl<'a> _OCS4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OCS4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Inverted CC8yST2 signal path is connected to the CCU8x.OUTy3"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(OCS4W::VALUE1)
    }
    #[doc = "CC8yST2 signal path is connected to the CCU8x.OUTy3"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(OCS4W::VALUE2)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Asymmetric PWM mode Enable"]
    #[inline]
    pub fn ase(&self) -> ASER {
        ASER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Output selector for CCU8x.OUTy0"]
    #[inline]
    pub fn ocs1(&self) -> OCS1R {
        OCS1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Output selector for CCU8x.OUTy1"]
    #[inline]
    pub fn ocs2(&self) -> OCS2R {
        OCS2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Output selector for CCU8x.OUTy2"]
    #[inline]
    pub fn ocs3(&self) -> OCS3R {
        OCS3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Output selector for CCU8x.OUTy3"]
    #[inline]
    pub fn ocs4(&self) -> OCS4R {
        OCS4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
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
    #[doc = "Bit 0 - Asymmetric PWM mode Enable"]
    #[inline]
    pub fn ase(&mut self) -> _ASEW {
        _ASEW { w: self }
    }
    #[doc = "Bit 1 - Output selector for CCU8x.OUTy0"]
    #[inline]
    pub fn ocs1(&mut self) -> _OCS1W {
        _OCS1W { w: self }
    }
    #[doc = "Bit 2 - Output selector for CCU8x.OUTy1"]
    #[inline]
    pub fn ocs2(&mut self) -> _OCS2W {
        _OCS2W { w: self }
    }
    #[doc = "Bit 3 - Output selector for CCU8x.OUTy2"]
    #[inline]
    pub fn ocs3(&mut self) -> _OCS3W {
        _OCS3W { w: self }
    }
    #[doc = "Bit 4 - Output selector for CCU8x.OUTy3"]
    #[inline]
    pub fn ocs4(&mut self) -> _OCS4W {
        _OCS4W { w: self }
    }
}
