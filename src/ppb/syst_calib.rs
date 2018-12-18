#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SYST_CALIB {
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
pub struct TENMSR {
    bits: u32,
}
impl TENMSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Possible values of the field `SKEW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SKEWR {
    #[doc = "TENMS value is exact"]
    VALUE1,
    #[doc = "TENMS value is inexact, or not given."]
    VALUE2,
}
impl SKEWR {
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
            SKEWR::VALUE1 => false,
            SKEWR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SKEWR {
        match value {
            false => SKEWR::VALUE1,
            true => SKEWR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SKEWR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SKEWR::VALUE2
    }
}
#[doc = "Possible values of the field `NOREF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOREFR {
    #[doc = "reference clock provided"]
    VALUE1,
    #[doc = "no reference clock provided."]
    VALUE2,
}
impl NOREFR {
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
            NOREFR::VALUE1 => false,
            NOREFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NOREFR {
        match value {
            false => NOREFR::VALUE1,
            true => NOREFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == NOREFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == NOREFR::VALUE2
    }
}
#[doc = r" Proxy"]
pub struct _TENMSW<'a> {
    w: &'a mut W,
}
impl<'a> _TENMSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SKEW`"]
pub enum SKEWW {
    #[doc = "TENMS value is exact"]
    VALUE1,
    #[doc = "TENMS value is inexact, or not given."]
    VALUE2,
}
impl SKEWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SKEWW::VALUE1 => false,
            SKEWW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SKEWW<'a> {
    w: &'a mut W,
}
impl<'a> _SKEWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SKEWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TENMS value is exact"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SKEWW::VALUE1)
    }
    #[doc = "TENMS value is inexact, or not given."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SKEWW::VALUE2)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NOREF`"]
pub enum NOREFW {
    #[doc = "reference clock provided"]
    VALUE1,
    #[doc = "no reference clock provided."]
    VALUE2,
}
impl NOREFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NOREFW::VALUE1 => false,
            NOREFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NOREFW<'a> {
    w: &'a mut W,
}
impl<'a> _NOREFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NOREFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "reference clock provided"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(NOREFW::VALUE1)
    }
    #[doc = "no reference clock provided."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(NOREFW::VALUE2)
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
    #[doc = "Bits 0:23 - Ten Milliseconds Reload Value"]
    #[inline]
    pub fn tenms(&self) -> TENMSR {
        let bits = {
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        TENMSR { bits }
    }
    #[doc = "Bit 30 - Ten Milliseconds Skewed"]
    #[inline]
    pub fn skew(&self) -> SKEWR {
        SKEWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - No Reference Clock"]
    #[inline]
    pub fn noref(&self) -> NOREFR {
        NOREFR::_from({
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
        W { bits: 3221225472 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:23 - Ten Milliseconds Reload Value"]
    #[inline]
    pub fn tenms(&mut self) -> _TENMSW {
        _TENMSW { w: self }
    }
    #[doc = "Bit 30 - Ten Milliseconds Skewed"]
    #[inline]
    pub fn skew(&mut self) -> _SKEWW {
        _SKEWW { w: self }
    }
    #[doc = "Bit 31 - No Reference Clock"]
    #[inline]
    pub fn noref(&mut self) -> _NOREFW {
        _NOREFW { w: self }
    }
}
