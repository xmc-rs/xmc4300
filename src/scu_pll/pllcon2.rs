#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PLLCON2 {
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
#[doc = "Possible values of the field `PINSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINSELR {
    #[doc = "PLL external oscillator selected"]
    CONST_0,
    #[doc = "Backup clock fofi selected"]
    CONST_1,
}
impl PINSELR {
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
            PINSELR::CONST_0 => false,
            PINSELR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PINSELR {
        match value {
            false => PINSELR::CONST_0,
            true => PINSELR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PINSELR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PINSELR::CONST_1
    }
}
#[doc = "Possible values of the field `K1INSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum K1INSELR {
    #[doc = "PLL external oscillator selected"]
    CONST_0,
    #[doc = "Backup clock fofi selected"]
    CONST_1,
}
impl K1INSELR {
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
            K1INSELR::CONST_0 => false,
            K1INSELR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> K1INSELR {
        match value {
            false => K1INSELR::CONST_0,
            true => K1INSELR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == K1INSELR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == K1INSELR::CONST_1
    }
}
#[doc = "Values that can be written to the field `PINSEL`"]
pub enum PINSELW {
    #[doc = "PLL external oscillator selected"]
    CONST_0,
    #[doc = "Backup clock fofi selected"]
    CONST_1,
}
impl PINSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PINSELW::CONST_0 => false,
            PINSELW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PINSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PINSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PINSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PLL external oscillator selected"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PINSELW::CONST_0)
    }
    #[doc = "Backup clock fofi selected"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PINSELW::CONST_1)
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
#[doc = "Values that can be written to the field `K1INSEL`"]
pub enum K1INSELW {
    #[doc = "PLL external oscillator selected"]
    CONST_0,
    #[doc = "Backup clock fofi selected"]
    CONST_1,
}
impl K1INSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            K1INSELW::CONST_0 => false,
            K1INSELW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _K1INSELW<'a> {
    w: &'a mut W,
}
impl<'a> _K1INSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: K1INSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PLL external oscillator selected"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(K1INSELW::CONST_0)
    }
    #[doc = "Backup clock fofi selected"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(K1INSELW::CONST_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - P-Divider Input Selection"]
    #[inline]
    pub fn pinsel(&self) -> PINSELR {
        PINSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - K1-Divider Input Selection"]
    #[inline]
    pub fn k1insel(&self) -> K1INSELR {
        K1INSELR::_from({
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
        W { bits: 1 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - P-Divider Input Selection"]
    #[inline]
    pub fn pinsel(&mut self) -> _PINSELW {
        _PINSELW { w: self }
    }
    #[doc = "Bit 8 - K1-Divider Input Selection"]
    #[inline]
    pub fn k1insel(&mut self) -> _K1INSELW {
        _K1INSELW { w: self }
    }
}
