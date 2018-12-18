#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OSCHPCTRL {
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
#[doc = "Possible values of the field `X1DEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum X1DENR {
    #[doc = "Bit X1D is not updated"]
    CONST_0,
    #[doc = "Bit X1D can be updated"]
    CONST_1,
}
impl X1DENR {
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
            X1DENR::CONST_0 => false,
            X1DENR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> X1DENR {
        match value {
            false => X1DENR::CONST_0,
            true => X1DENR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == X1DENR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == X1DENR::CONST_1
    }
}
#[doc = "Possible values of the field `SHBY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHBYR {
    #[doc = "The shaper is not bypassed"]
    CONST_0,
    #[doc = "The shaper is bypassed"]
    CONST_1,
}
impl SHBYR {
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
            SHBYR::CONST_0 => false,
            SHBYR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SHBYR {
        match value {
            false => SHBYR::CONST_0,
            true => SHBYR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == SHBYR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == SHBYR::CONST_1
    }
}
#[doc = "Possible values of the field `GAINSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GAINSELR {
    #[doc = "The gain control is configured for frequencies from 4 MHz to 8 MHz"]
    CONST_00,
    #[doc = "The gain control is configured for frequencies from 4 MHz to 16 MHz"]
    CONST_01,
    #[doc = "The gain control is configured for frequencies from 4 MHz to 20 MHz"]
    CONST_10,
    #[doc = "The gain control is configured for frequencies from 4 MHz to 25 MHz"]
    CONST_11,
}
impl GAINSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GAINSELR::CONST_00 => 0,
            GAINSELR::CONST_01 => 1,
            GAINSELR::CONST_10 => 2,
            GAINSELR::CONST_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GAINSELR {
        match value {
            0 => GAINSELR::CONST_00,
            1 => GAINSELR::CONST_01,
            2 => GAINSELR::CONST_10,
            3 => GAINSELR::CONST_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline]
    pub fn is_const_00(&self) -> bool {
        *self == GAINSELR::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline]
    pub fn is_const_01(&self) -> bool {
        *self == GAINSELR::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline]
    pub fn is_const_10(&self) -> bool {
        *self == GAINSELR::CONST_10
    }
    #[doc = "Checks if the value of the field is `CONST_11`"]
    #[inline]
    pub fn is_const_11(&self) -> bool {
        *self == GAINSELR::CONST_11
    }
}
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "External Crystal Mode and External Input Clock Mode. The oscillator Power-Saving Mode is not entered."]
    CONST_00,
    #[doc = "OSC is disabled. The oscillator Power-Saving Mode is not entered."]
    CONST_01,
    #[doc = "External Input Clock Mode and the oscillator Power-Saving Mode is entered"]
    CONST_10,
    #[doc = "OSC is disabled. The oscillator Power-Saving Mode is entered."]
    CONST_11,
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::CONST_00 => 0,
            MODER::CONST_01 => 1,
            MODER::CONST_10 => 2,
            MODER::CONST_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::CONST_00,
            1 => MODER::CONST_01,
            2 => MODER::CONST_10,
            3 => MODER::CONST_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline]
    pub fn is_const_00(&self) -> bool {
        *self == MODER::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline]
    pub fn is_const_01(&self) -> bool {
        *self == MODER::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline]
    pub fn is_const_10(&self) -> bool {
        *self == MODER::CONST_10
    }
    #[doc = "Checks if the value of the field is `CONST_11`"]
    #[inline]
    pub fn is_const_11(&self) -> bool {
        *self == MODER::CONST_11
    }
}
#[doc = r" Value of the field"]
pub struct OSCVALR {
    bits: u8,
}
impl OSCVALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `X1DEN`"]
pub enum X1DENW {
    #[doc = "Bit X1D is not updated"]
    CONST_0,
    #[doc = "Bit X1D can be updated"]
    CONST_1,
}
impl X1DENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            X1DENW::CONST_0 => false,
            X1DENW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _X1DENW<'a> {
    w: &'a mut W,
}
impl<'a> _X1DENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: X1DENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bit X1D is not updated"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(X1DENW::CONST_0)
    }
    #[doc = "Bit X1D can be updated"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(X1DENW::CONST_1)
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
#[doc = "Values that can be written to the field `SHBY`"]
pub enum SHBYW {
    #[doc = "The shaper is not bypassed"]
    CONST_0,
    #[doc = "The shaper is bypassed"]
    CONST_1,
}
impl SHBYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SHBYW::CONST_0 => false,
            SHBYW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SHBYW<'a> {
    w: &'a mut W,
}
impl<'a> _SHBYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SHBYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The shaper is not bypassed"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(SHBYW::CONST_0)
    }
    #[doc = "The shaper is bypassed"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(SHBYW::CONST_1)
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
#[doc = "Values that can be written to the field `GAINSEL`"]
pub enum GAINSELW {
    #[doc = "The gain control is configured for frequencies from 4 MHz to 8 MHz"]
    CONST_00,
    #[doc = "The gain control is configured for frequencies from 4 MHz to 16 MHz"]
    CONST_01,
    #[doc = "The gain control is configured for frequencies from 4 MHz to 20 MHz"]
    CONST_10,
    #[doc = "The gain control is configured for frequencies from 4 MHz to 25 MHz"]
    CONST_11,
}
impl GAINSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GAINSELW::CONST_00 => 0,
            GAINSELW::CONST_01 => 1,
            GAINSELW::CONST_10 => 2,
            GAINSELW::CONST_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GAINSELW<'a> {
    w: &'a mut W,
}
impl<'a> _GAINSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GAINSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The gain control is configured for frequencies from 4 MHz to 8 MHz"]
    #[inline]
    pub fn const_00(self) -> &'a mut W {
        self.variant(GAINSELW::CONST_00)
    }
    #[doc = "The gain control is configured for frequencies from 4 MHz to 16 MHz"]
    #[inline]
    pub fn const_01(self) -> &'a mut W {
        self.variant(GAINSELW::CONST_01)
    }
    #[doc = "The gain control is configured for frequencies from 4 MHz to 20 MHz"]
    #[inline]
    pub fn const_10(self) -> &'a mut W {
        self.variant(GAINSELW::CONST_10)
    }
    #[doc = "The gain control is configured for frequencies from 4 MHz to 25 MHz"]
    #[inline]
    pub fn const_11(self) -> &'a mut W {
        self.variant(GAINSELW::CONST_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "External Crystal Mode and External Input Clock Mode. The oscillator Power-Saving Mode is not entered."]
    CONST_00,
    #[doc = "OSC is disabled. The oscillator Power-Saving Mode is not entered."]
    CONST_01,
    #[doc = "External Input Clock Mode and the oscillator Power-Saving Mode is entered"]
    CONST_10,
    #[doc = "OSC is disabled. The oscillator Power-Saving Mode is entered."]
    CONST_11,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::CONST_00 => 0,
            MODEW::CONST_01 => 1,
            MODEW::CONST_10 => 2,
            MODEW::CONST_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "External Crystal Mode and External Input Clock Mode. The oscillator Power-Saving Mode is not entered."]
    #[inline]
    pub fn const_00(self) -> &'a mut W {
        self.variant(MODEW::CONST_00)
    }
    #[doc = "OSC is disabled. The oscillator Power-Saving Mode is not entered."]
    #[inline]
    pub fn const_01(self) -> &'a mut W {
        self.variant(MODEW::CONST_01)
    }
    #[doc = "External Input Clock Mode and the oscillator Power-Saving Mode is entered"]
    #[inline]
    pub fn const_10(self) -> &'a mut W {
        self.variant(MODEW::CONST_10)
    }
    #[doc = "OSC is disabled. The oscillator Power-Saving Mode is entered."]
    #[inline]
    pub fn const_11(self) -> &'a mut W {
        self.variant(MODEW::CONST_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OSCVALW<'a> {
    w: &'a mut W,
}
impl<'a> _OSCVALW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - XTAL1 Data Enable"]
    #[inline]
    pub fn x1den(&self) -> X1DENR {
        X1DENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Shaper Bypass"]
    #[inline]
    pub fn shby(&self) -> SHBYR {
        SHBYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 2:3 - Oscillator Gain Selection"]
    #[inline]
    pub fn gainsel(&self) -> GAINSELR {
        GAINSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Oscillator Mode"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - OSC Frequency Value"]
    #[inline]
    pub fn oscval(&self) -> OSCVALR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OSCVALR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 60 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - XTAL1 Data Enable"]
    #[inline]
    pub fn x1den(&mut self) -> _X1DENW {
        _X1DENW { w: self }
    }
    #[doc = "Bit 1 - Shaper Bypass"]
    #[inline]
    pub fn shby(&mut self) -> _SHBYW {
        _SHBYW { w: self }
    }
    #[doc = "Bits 2:3 - Oscillator Gain Selection"]
    #[inline]
    pub fn gainsel(&mut self) -> _GAINSELW {
        _GAINSELW { w: self }
    }
    #[doc = "Bits 4:5 - Oscillator Mode"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bits 16:19 - OSC Frequency Value"]
    #[inline]
    pub fn oscval(&mut self) -> _OSCVALW {
        _OSCVALW { w: self }
    }
}
