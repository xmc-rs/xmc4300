#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::DC_LATCH1_CONT {
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
#[doc = "Possible values of the field `L1_POS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L1_POSR {
    #[doc = "Continuous Latch active"]
    VALUE1,
    #[doc = "Single event (only first event active)"]
    VALUE2,
}
impl L1_POSR {
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
            L1_POSR::VALUE1 => false,
            L1_POSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> L1_POSR {
        match value {
            false => L1_POSR::VALUE1,
            true => L1_POSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == L1_POSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == L1_POSR::VALUE2
    }
}
#[doc = "Possible values of the field `L1_NEG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L1_NEGR {
    #[doc = "Continuous Latch active"]
    VALUE1,
    #[doc = "Single event (only first event active)"]
    VALUE2,
}
impl L1_NEGR {
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
            L1_NEGR::VALUE1 => false,
            L1_NEGR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> L1_NEGR {
        match value {
            false => L1_NEGR::VALUE1,
            true => L1_NEGR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == L1_NEGR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == L1_NEGR::VALUE2
    }
}
#[doc = "Values that can be written to the field `L1_POS`"]
pub enum L1_POSW {
    #[doc = "Continuous Latch active"]
    VALUE1,
    #[doc = "Single event (only first event active)"]
    VALUE2,
}
impl L1_POSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            L1_POSW::VALUE1 => false,
            L1_POSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _L1_POSW<'a> {
    w: &'a mut W,
}
impl<'a> _L1_POSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: L1_POSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Continuous Latch active"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(L1_POSW::VALUE1)
    }
    #[doc = "Single event (only first event active)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(L1_POSW::VALUE2)
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
#[doc = "Values that can be written to the field `L1_NEG`"]
pub enum L1_NEGW {
    #[doc = "Continuous Latch active"]
    VALUE1,
    #[doc = "Single event (only first event active)"]
    VALUE2,
}
impl L1_NEGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            L1_NEGW::VALUE1 => false,
            L1_NEGW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _L1_NEGW<'a> {
    w: &'a mut W,
}
impl<'a> _L1_NEGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: L1_NEGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Continuous Latch active"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(L1_NEGW::VALUE1)
    }
    #[doc = "Single event (only first event active)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(L1_NEGW::VALUE2)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Latch1 positive edge"]
    #[inline]
    pub fn l1_pos(&self) -> L1_POSR {
        L1_POSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Latch1 negative edge"]
    #[inline]
    pub fn l1_neg(&self) -> L1_NEGR {
        L1_NEGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
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
    #[doc = "Bit 0 - Latch1 positive edge"]
    #[inline]
    pub fn l1_pos(&mut self) -> _L1_POSW {
        _L1_POSW { w: self }
    }
    #[doc = "Bit 1 - Latch1 negative edge"]
    #[inline]
    pub fn l1_neg(&mut self) -> _L1_NEGW {
        _L1_NEGW { w: self }
    }
}
