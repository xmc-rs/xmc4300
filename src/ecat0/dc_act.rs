#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::DC_ACT {
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
#[doc = "Possible values of the field `SYNC_OUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC_OUTR {
    #[doc = "Deactivated"]
    VALUE1,
    #[doc = "Activated"]
    VALUE2,
}
impl SYNC_OUTR {
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
            SYNC_OUTR::VALUE1 => false,
            SYNC_OUTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYNC_OUTR {
        match value {
            false => SYNC_OUTR::VALUE1,
            true => SYNC_OUTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SYNC_OUTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SYNC_OUTR::VALUE2
    }
}
#[doc = "Possible values of the field `SYNC_0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC_0R {
    #[doc = "Deactivated"]
    VALUE1,
    #[doc = "SYNC0 pulse is generated"]
    VALUE2,
}
impl SYNC_0R {
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
            SYNC_0R::VALUE1 => false,
            SYNC_0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYNC_0R {
        match value {
            false => SYNC_0R::VALUE1,
            true => SYNC_0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SYNC_0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SYNC_0R::VALUE2
    }
}
#[doc = "Possible values of the field `SYNC_1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC_1R {
    #[doc = "Deactivated"]
    VALUE1,
    #[doc = "SYNC1 pulse is generated"]
    VALUE2,
}
impl SYNC_1R {
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
            SYNC_1R::VALUE1 => false,
            SYNC_1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYNC_1R {
        match value {
            false => SYNC_1R::VALUE1,
            true => SYNC_1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SYNC_1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SYNC_1R::VALUE2
    }
}
#[doc = "Values that can be written to the field `SYNC_OUT`"]
pub enum SYNC_OUTW {
    #[doc = "Deactivated"]
    VALUE1,
    #[doc = "Activated"]
    VALUE2,
}
impl SYNC_OUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SYNC_OUTW::VALUE1 => false,
            SYNC_OUTW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNC_OUTW<'a> {
    w: &'a mut W,
}
impl<'a> _SYNC_OUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNC_OUTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Deactivated"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SYNC_OUTW::VALUE1)
    }
    #[doc = "Activated"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SYNC_OUTW::VALUE2)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SYNC_0`"]
pub enum SYNC_0W {
    #[doc = "Deactivated"]
    VALUE1,
    #[doc = "SYNC0 pulse is generated"]
    VALUE2,
}
impl SYNC_0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SYNC_0W::VALUE1 => false,
            SYNC_0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNC_0W<'a> {
    w: &'a mut W,
}
impl<'a> _SYNC_0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNC_0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Deactivated"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SYNC_0W::VALUE1)
    }
    #[doc = "SYNC0 pulse is generated"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SYNC_0W::VALUE2)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SYNC_1`"]
pub enum SYNC_1W {
    #[doc = "Deactivated"]
    VALUE1,
    #[doc = "SYNC1 pulse is generated"]
    VALUE2,
}
impl SYNC_1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SYNC_1W::VALUE1 => false,
            SYNC_1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNC_1W<'a> {
    w: &'a mut W,
}
impl<'a> _SYNC_1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNC_1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Deactivated"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SYNC_1W::VALUE1)
    }
    #[doc = "SYNC1 pulse is generated"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SYNC_1W::VALUE2)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Sync Out Unit activation"]
    #[inline]
    pub fn sync_out(&self) -> SYNC_OUTR {
        SYNC_OUTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - SYNC0 generation"]
    #[inline]
    pub fn sync_0(&self) -> SYNC_0R {
        SYNC_0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - SYNC1 generation"]
    #[inline]
    pub fn sync_1(&self) -> SYNC_1R {
        SYNC_1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Sync Out Unit activation"]
    #[inline]
    pub fn sync_out(&mut self) -> _SYNC_OUTW {
        _SYNC_OUTW { w: self }
    }
    #[doc = "Bit 1 - SYNC0 generation"]
    #[inline]
    pub fn sync_0(&mut self) -> _SYNC_0W {
        _SYNC_0W { w: self }
    }
    #[doc = "Bit 2 - SYNC1 generation"]
    #[inline]
    pub fn sync_1(&mut self) -> _SYNC_1W {
        _SYNC_1W { w: self }
    }
}
