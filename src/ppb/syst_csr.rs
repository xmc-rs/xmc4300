#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SYST_CSR {
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
#[doc = "Possible values of the field `ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLER {
    #[doc = "counter disabled"]
    VALUE1,
    #[doc = "counter enabled."]
    VALUE2,
}
impl ENABLER {
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
            ENABLER::VALUE1 => false,
            ENABLER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENABLER {
        match value {
            false => ENABLER::VALUE1,
            true => ENABLER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ENABLER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ENABLER::VALUE2
    }
}
#[doc = "Possible values of the field `TICKINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TICKINTR {
    #[doc = "counting down to zero does not assert the SysTick exception request"]
    VALUE1,
    #[doc = "counting down to zero to asserts the SysTick exception request."]
    VALUE2,
}
impl TICKINTR {
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
            TICKINTR::VALUE1 => false,
            TICKINTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TICKINTR {
        match value {
            false => TICKINTR::VALUE1,
            true => TICKINTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TICKINTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TICKINTR::VALUE2
    }
}
#[doc = "Possible values of the field `CLKSOURCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSOURCER {
    #[doc = "external clock"]
    VALUE1,
    #[doc = "processor clock."]
    VALUE2,
}
impl CLKSOURCER {
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
            CLKSOURCER::VALUE1 => false,
            CLKSOURCER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLKSOURCER {
        match value {
            false => CLKSOURCER::VALUE1,
            true => CLKSOURCER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CLKSOURCER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CLKSOURCER::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct COUNTFLAGR {
    bits: bool,
}
impl COUNTFLAGR {
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
#[doc = "Values that can be written to the field `ENABLE`"]
pub enum ENABLEW {
    #[doc = "counter disabled"]
    VALUE1,
    #[doc = "counter enabled."]
    VALUE2,
}
impl ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENABLEW::VALUE1 => false,
            ENABLEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "counter disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENABLEW::VALUE1)
    }
    #[doc = "counter enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENABLEW::VALUE2)
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
#[doc = "Values that can be written to the field `TICKINT`"]
pub enum TICKINTW {
    #[doc = "counting down to zero does not assert the SysTick exception request"]
    VALUE1,
    #[doc = "counting down to zero to asserts the SysTick exception request."]
    VALUE2,
}
impl TICKINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TICKINTW::VALUE1 => false,
            TICKINTW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TICKINTW<'a> {
    w: &'a mut W,
}
impl<'a> _TICKINTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TICKINTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "counting down to zero does not assert the SysTick exception request"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TICKINTW::VALUE1)
    }
    #[doc = "counting down to zero to asserts the SysTick exception request."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TICKINTW::VALUE2)
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
#[doc = "Values that can be written to the field `CLKSOURCE`"]
pub enum CLKSOURCEW {
    #[doc = "external clock"]
    VALUE1,
    #[doc = "processor clock."]
    VALUE2,
}
impl CLKSOURCEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLKSOURCEW::VALUE1 => false,
            CLKSOURCEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKSOURCEW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKSOURCEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKSOURCEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "external clock"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CLKSOURCEW::VALUE1)
    }
    #[doc = "processor clock."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CLKSOURCEW::VALUE2)
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
#[doc = r" Proxy"]
pub struct _COUNTFLAGW<'a> {
    w: &'a mut W,
}
impl<'a> _COUNTFLAGW<'a> {
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
    #[doc = "Bit 0 - Enable"]
    #[inline]
    pub fn enable(&self) -> ENABLER {
        ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Tick Interrupt Enable"]
    #[inline]
    pub fn tickint(&self) -> TICKINTR {
        TICKINTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Indicates the clock source:"]
    #[inline]
    pub fn clksource(&self) -> CLKSOURCER {
        CLKSOURCER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Counter Flag"]
    #[inline]
    pub fn countflag(&self) -> COUNTFLAGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        COUNTFLAGR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Enable"]
    #[inline]
    pub fn enable(&mut self) -> _ENABLEW {
        _ENABLEW { w: self }
    }
    #[doc = "Bit 1 - Tick Interrupt Enable"]
    #[inline]
    pub fn tickint(&mut self) -> _TICKINTW {
        _TICKINTW { w: self }
    }
    #[doc = "Bit 2 - Indicates the clock source:"]
    #[inline]
    pub fn clksource(&mut self) -> _CLKSOURCEW {
        _CLKSOURCEW { w: self }
    }
    #[doc = "Bit 16 - Counter Flag"]
    #[inline]
    pub fn countflag(&mut self) -> _COUNTFLAGW {
        _COUNTFLAGW { w: self }
    }
}
