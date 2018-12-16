#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SYSCLKCR {
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
pub struct SYSDIVR {
    bits: u8,
}
impl SYSDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `SYSSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSSELR {
    #[doc = "fOFI clock"]
    CONST_0,
    #[doc = "fPLL clock"]
    CONST_1,
}
impl SYSSELR {
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
            SYSSELR::CONST_0 => false,
            SYSSELR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYSSELR {
        match value {
            false => SYSSELR::CONST_0,
            true => SYSSELR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == SYSSELR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == SYSSELR::CONST_1
    }
}
#[doc = r" Proxy"]
pub struct _SYSDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSDIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SYSSEL`"]
pub enum SYSSELW {
    #[doc = "fOFI clock"]
    CONST_0,
    #[doc = "fPLL clock"]
    CONST_1,
}
impl SYSSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SYSSELW::CONST_0 => false,
            SYSSELW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYSSELW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYSSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "fOFI clock"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(SYSSELW::CONST_0)
    }
    #[doc = "fPLL clock"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(SYSSELW::CONST_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - System Clock Division Value"]
    #[inline]
    pub fn sysdiv(&self) -> SYSDIVR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYSDIVR { bits }
    }
    #[doc = "Bit 16 - System Clock Selection Value"]
    #[inline]
    pub fn syssel(&self) -> SYSSELR {
        SYSSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:7 - System Clock Division Value"]
    #[inline]
    pub fn sysdiv(&mut self) -> _SYSDIVW {
        _SYSDIVW { w: self }
    }
    #[doc = "Bit 16 - System Clock Selection Value"]
    #[inline]
    pub fn syssel(&mut self) -> _SYSSELW {
        _SYSSELW { w: self }
    }
}
