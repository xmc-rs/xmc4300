#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GLOBEFLAG {
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
#[doc = "Possible values of the field `SEVGLB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEVGLBR {
    #[doc = "No source event"]
    VALUE1,
    #[doc = "A source event has occurred"]
    VALUE2,
}
impl SEVGLBR {
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
            SEVGLBR::VALUE1 => false,
            SEVGLBR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEVGLBR {
        match value {
            false => SEVGLBR::VALUE1,
            true => SEVGLBR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SEVGLBR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SEVGLBR::VALUE2
    }
}
#[doc = "Possible values of the field `REVGLB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REVGLBR {
    #[doc = "No result event"]
    VALUE1,
    #[doc = "New result was stored in register GLOBRES"]
    VALUE2,
}
impl REVGLBR {
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
            REVGLBR::VALUE1 => false,
            REVGLBR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REVGLBR {
        match value {
            false => REVGLBR::VALUE1,
            true => REVGLBR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REVGLBR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == REVGLBR::VALUE2
    }
}
#[doc = "Values that can be written to the field `SEVGLB`"]
pub enum SEVGLBW {
    #[doc = "No source event"]
    VALUE1,
    #[doc = "A source event has occurred"]
    VALUE2,
}
impl SEVGLBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEVGLBW::VALUE1 => false,
            SEVGLBW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEVGLBW<'a> {
    w: &'a mut W,
}
impl<'a> _SEVGLBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEVGLBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No source event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SEVGLBW::VALUE1)
    }
    #[doc = "A source event has occurred"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SEVGLBW::VALUE2)
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
#[doc = "Values that can be written to the field `REVGLB`"]
pub enum REVGLBW {
    #[doc = "No result event"]
    VALUE1,
    #[doc = "New result was stored in register GLOBRES"]
    VALUE2,
}
impl REVGLBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REVGLBW::VALUE1 => false,
            REVGLBW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REVGLBW<'a> {
    w: &'a mut W,
}
impl<'a> _REVGLBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REVGLBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No result event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(REVGLBW::VALUE1)
    }
    #[doc = "New result was stored in register GLOBRES"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(REVGLBW::VALUE2)
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
#[doc = "Values that can be written to the field `SEVGLBCLR`"]
pub enum SEVGLBCLRW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Clear the source event flag SEVGLB"]
    VALUE2,
}
impl SEVGLBCLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEVGLBCLRW::VALUE1 => false,
            SEVGLBCLRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEVGLBCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _SEVGLBCLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEVGLBCLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SEVGLBCLRW::VALUE1)
    }
    #[doc = "Clear the source event flag SEVGLB"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SEVGLBCLRW::VALUE2)
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
#[doc = "Values that can be written to the field `REVGLBCLR`"]
pub enum REVGLBCLRW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Clear the result event flag REVGLB"]
    VALUE2,
}
impl REVGLBCLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REVGLBCLRW::VALUE1 => false,
            REVGLBCLRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REVGLBCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _REVGLBCLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REVGLBCLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(REVGLBCLRW::VALUE1)
    }
    #[doc = "Clear the result event flag REVGLB"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(REVGLBCLRW::VALUE2)
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
        const OFFSET: u8 = 24;
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
    #[doc = "Bit 0 - Source Event (Background)"]
    #[inline]
    pub fn sevglb(&self) -> SEVGLBR {
        SEVGLBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Global Result Event"]
    #[inline]
    pub fn revglb(&self) -> REVGLBR {
        REVGLBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
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
    #[doc = "Bit 0 - Source Event (Background)"]
    #[inline]
    pub fn sevglb(&mut self) -> _SEVGLBW {
        _SEVGLBW { w: self }
    }
    #[doc = "Bit 8 - Global Result Event"]
    #[inline]
    pub fn revglb(&mut self) -> _REVGLBW {
        _REVGLBW { w: self }
    }
    #[doc = "Bit 16 - Clear Source Event (Background)"]
    #[inline]
    pub fn sevglbclr(&mut self) -> _SEVGLBCLRW {
        _SEVGLBCLRW { w: self }
    }
    #[doc = "Bit 24 - Clear Global Result Event"]
    #[inline]
    pub fn revglbclr(&mut self) -> _REVGLBCLRW {
        _REVGLBCLRW { w: self }
    }
}
