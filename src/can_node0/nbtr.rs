#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::NBTR {
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
pub struct BRPR {
    bits: u8,
}
impl BRPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SJWR {
    bits: u8,
}
impl SJWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TSEG1R {
    bits: u8,
}
impl TSEG1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TSEG2R {
    bits: u8,
}
impl TSEG2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `DIV8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIV8R {
    #[doc = "A time quantum lasts (BRP+1) clock cycles."]
    VALUE1,
    #[doc = "A time quantum lasts 8 (BRP+1) clock cycles."]
    VALUE2,
}
impl DIV8R {
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
            DIV8R::VALUE1 => false,
            DIV8R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIV8R {
        match value {
            false => DIV8R::VALUE1,
            true => DIV8R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DIV8R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DIV8R::VALUE2
    }
}
#[doc = r" Proxy"]
pub struct _BRPW<'a> {
    w: &'a mut W,
}
impl<'a> _BRPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SJWW<'a> {
    w: &'a mut W,
}
impl<'a> _SJWW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TSEG1W<'a> {
    w: &'a mut W,
}
impl<'a> _TSEG1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TSEG2W<'a> {
    w: &'a mut W,
}
impl<'a> _TSEG2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DIV8`"]
pub enum DIV8W {
    #[doc = "A time quantum lasts (BRP+1) clock cycles."]
    VALUE1,
    #[doc = "A time quantum lasts 8 (BRP+1) clock cycles."]
    VALUE2,
}
impl DIV8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIV8W::VALUE1 => false,
            DIV8W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIV8W<'a> {
    w: &'a mut W,
}
impl<'a> _DIV8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIV8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A time quantum lasts (BRP+1) clock cycles."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DIV8W::VALUE1)
    }
    #[doc = "A time quantum lasts 8 (BRP+1) clock cycles."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DIV8W::VALUE2)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:5 - Baud Rate Prescaler"]
    #[inline]
    pub fn brp(&self) -> BRPR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BRPR { bits }
    }
    #[doc = "Bits 6:7 - (Re) Synchronization Jump Width"]
    #[inline]
    pub fn sjw(&self) -> SJWR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SJWR { bits }
    }
    #[doc = "Bits 8:11 - Time Segment Before Sample Point"]
    #[inline]
    pub fn tseg1(&self) -> TSEG1R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TSEG1R { bits }
    }
    #[doc = "Bits 12:14 - Time Segment After Sample Point"]
    #[inline]
    pub fn tseg2(&self) -> TSEG2R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TSEG2R { bits }
    }
    #[doc = "Bit 15 - Divide Prescaler Clock by 8"]
    #[inline]
    pub fn div8(&self) -> DIV8R {
        DIV8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
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
    #[doc = "Bits 0:5 - Baud Rate Prescaler"]
    #[inline]
    pub fn brp(&mut self) -> _BRPW {
        _BRPW { w: self }
    }
    #[doc = "Bits 6:7 - (Re) Synchronization Jump Width"]
    #[inline]
    pub fn sjw(&mut self) -> _SJWW {
        _SJWW { w: self }
    }
    #[doc = "Bits 8:11 - Time Segment Before Sample Point"]
    #[inline]
    pub fn tseg1(&mut self) -> _TSEG1W {
        _TSEG1W { w: self }
    }
    #[doc = "Bits 12:14 - Time Segment After Sample Point"]
    #[inline]
    pub fn tseg2(&mut self) -> _TSEG2W {
        _TSEG2W { w: self }
    }
    #[doc = "Bit 15 - Divide Prescaler Clock by 8"]
    #[inline]
    pub fn div8(&mut self) -> _DIV8W {
        _DIV8W { w: self }
    }
}
