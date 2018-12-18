#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GLOBRCR {
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
#[doc = "Possible values of the field `DRCTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRCTRR {
    #[doc = "Data reduction disabled"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DRCTRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DRCTRR::VALUE1 => 0,
            DRCTRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DRCTRR {
        match value {
            0 => DRCTRR::VALUE1,
            i => DRCTRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DRCTRR::VALUE1
    }
}
#[doc = "Possible values of the field `WFR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WFRR {
    #[doc = "Overwrite mode"]
    VALUE1,
    #[doc = "Wait-for-read mode enabled for this register"]
    VALUE2,
}
impl WFRR {
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
            WFRR::VALUE1 => false,
            WFRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WFRR {
        match value {
            false => WFRR::VALUE1,
            true => WFRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WFRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WFRR::VALUE2
    }
}
#[doc = "Possible values of the field `SRGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRGENR {
    #[doc = "No service request"]
    VALUE1,
    #[doc = "Service request after a result event"]
    VALUE2,
}
impl SRGENR {
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
            SRGENR::VALUE1 => false,
            SRGENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRGENR {
        match value {
            false => SRGENR::VALUE1,
            true => SRGENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SRGENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SRGENR::VALUE2
    }
}
#[doc = "Values that can be written to the field `DRCTR`"]
pub enum DRCTRW {
    #[doc = "Data reduction disabled"]
    VALUE1,
}
impl DRCTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DRCTRW::VALUE1 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DRCTRW<'a> {
    w: &'a mut W,
}
impl<'a> _DRCTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DRCTRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Data reduction disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DRCTRW::VALUE1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WFR`"]
pub enum WFRW {
    #[doc = "Overwrite mode"]
    VALUE1,
    #[doc = "Wait-for-read mode enabled for this register"]
    VALUE2,
}
impl WFRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WFRW::VALUE1 => false,
            WFRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WFRW<'a> {
    w: &'a mut W,
}
impl<'a> _WFRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WFRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Overwrite mode"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WFRW::VALUE1)
    }
    #[doc = "Wait-for-read mode enabled for this register"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WFRW::VALUE2)
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
#[doc = "Values that can be written to the field `SRGEN`"]
pub enum SRGENW {
    #[doc = "No service request"]
    VALUE1,
    #[doc = "Service request after a result event"]
    VALUE2,
}
impl SRGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRGENW::VALUE1 => false,
            SRGENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRGENW<'a> {
    w: &'a mut W,
}
impl<'a> _SRGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No service request"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRGENW::VALUE1)
    }
    #[doc = "Service request after a result event"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SRGENW::VALUE2)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 16:19 - Data Reduction Control"]
    #[inline]
    pub fn drctr(&self) -> DRCTRR {
        DRCTRR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 24 - Wait-for-Read Mode Enable"]
    #[inline]
    pub fn wfr(&self) -> WFRR {
        WFRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Service Request Generation Enable"]
    #[inline]
    pub fn srgen(&self) -> SRGENR {
        SRGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
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
    #[doc = "Bits 16:19 - Data Reduction Control"]
    #[inline]
    pub fn drctr(&mut self) -> _DRCTRW {
        _DRCTRW { w: self }
    }
    #[doc = "Bit 24 - Wait-for-Read Mode Enable"]
    #[inline]
    pub fn wfr(&mut self) -> _WFRW {
        _WFRW { w: self }
    }
    #[doc = "Bit 31 - Service Request Generation Enable"]
    #[inline]
    pub fn srgen(&mut self) -> _SRGENW {
        _SRGENW { w: self }
    }
}
