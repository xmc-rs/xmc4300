#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PCON {
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
#[doc = "Possible values of the field `IBYP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBYPR {
    #[doc = "Instruction prefetch buffer not bypassed."]
    CONST_0,
    #[doc = "Instruction prefetch buffer bypassed."]
    CONST_1,
}
impl IBYPR {
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
            IBYPR::CONST_0 => false,
            IBYPR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IBYPR {
        match value {
            false => IBYPR::CONST_0,
            true => IBYPR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == IBYPR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == IBYPR::CONST_1
    }
}
#[doc = "Possible values of the field `DBYP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBYPR {
    #[doc = "Prefetch Data buffer not bypassed."]
    VALUE1,
    #[doc = "Prefetch Data buffer bypassed."]
    VALUE2,
}
impl DBYPR {
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
            DBYPR::VALUE1 => false,
            DBYPR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DBYPR {
        match value {
            false => DBYPR::VALUE1,
            true => DBYPR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DBYPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DBYPR::VALUE2
    }
}
#[doc = "Values that can be written to the field `IBYP`"]
pub enum IBYPW {
    #[doc = "Instruction prefetch buffer not bypassed."]
    CONST_0,
    #[doc = "Instruction prefetch buffer bypassed."]
    CONST_1,
}
impl IBYPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IBYPW::CONST_0 => false,
            IBYPW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IBYPW<'a> {
    w: &'a mut W,
}
impl<'a> _IBYPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IBYPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Instruction prefetch buffer not bypassed."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(IBYPW::CONST_0)
    }
    #[doc = "Instruction prefetch buffer bypassed."]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(IBYPW::CONST_1)
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
#[doc = "Values that can be written to the field `IINV`"]
pub enum IINVW {
    #[doc = "No effect."]
    CONST_0,
    #[doc = "Initiate invalidation of entire instruction cache."]
    CONST_1,
}
impl IINVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IINVW::CONST_0 => false,
            IINVW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IINVW<'a> {
    w: &'a mut W,
}
impl<'a> _IINVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IINVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(IINVW::CONST_0)
    }
    #[doc = "Initiate invalidation of entire instruction cache."]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(IINVW::CONST_1)
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
#[doc = "Values that can be written to the field `DBYP`"]
pub enum DBYPW {
    #[doc = "Prefetch Data buffer not bypassed."]
    VALUE1,
    #[doc = "Prefetch Data buffer bypassed."]
    VALUE2,
}
impl DBYPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DBYPW::VALUE1 => false,
            DBYPW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DBYPW<'a> {
    w: &'a mut W,
}
impl<'a> _DBYPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DBYPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Prefetch Data buffer not bypassed."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DBYPW::VALUE1)
    }
    #[doc = "Prefetch Data buffer bypassed."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DBYPW::VALUE2)
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
    #[doc = "Bit 0 - Instruction Prefetch Buffer Bypass"]
    #[inline]
    pub fn ibyp(&self) -> IBYPR {
        IBYPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Data Buffer Bypass"]
    #[inline]
    pub fn dbyp(&self) -> DBYPR {
        DBYPR::_from({
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
    #[doc = "Bit 0 - Instruction Prefetch Buffer Bypass"]
    #[inline]
    pub fn ibyp(&mut self) -> _IBYPW {
        _IBYPW { w: self }
    }
    #[doc = "Bit 1 - Instruction Prefetch Buffer Invalidate"]
    #[inline]
    pub fn iinv(&mut self) -> _IINVW {
        _IINVW { w: self }
    }
    #[doc = "Bit 4 - Data Buffer Bypass"]
    #[inline]
    pub fn dbyp(&mut self) -> _DBYPW {
        _DBYPW { w: self }
    }
}
