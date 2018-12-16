#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::CLOCK_CTRL {
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
#[doc = "Possible values of the field `SDCLK_FREQ_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDCLK_FREQ_SELR {
    #[doc = "base clock(10MHz-63MHz)"]
    VALUE1,
    #[doc = "base clock divided by 2"]
    VALUE2,
    #[doc = "base clock divided by 32"]
    VALUE3,
    #[doc = "base clock divided by 4"]
    VALUE4,
    #[doc = "base clock divided by 8"]
    VALUE5,
    #[doc = "base clock divided by 16"]
    VALUE6,
    #[doc = "base clock divided by 64"]
    VALUE7,
    #[doc = "base clock divided by 128"]
    VALUE8,
    #[doc = "base clock divided by 256"]
    VALUE9,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SDCLK_FREQ_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SDCLK_FREQ_SELR::VALUE1 => 0,
            SDCLK_FREQ_SELR::VALUE2 => 1,
            SDCLK_FREQ_SELR::VALUE3 => 16,
            SDCLK_FREQ_SELR::VALUE4 => 2,
            SDCLK_FREQ_SELR::VALUE5 => 4,
            SDCLK_FREQ_SELR::VALUE6 => 8,
            SDCLK_FREQ_SELR::VALUE7 => 32,
            SDCLK_FREQ_SELR::VALUE8 => 64,
            SDCLK_FREQ_SELR::VALUE9 => 128,
            SDCLK_FREQ_SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SDCLK_FREQ_SELR {
        match value {
            0 => SDCLK_FREQ_SELR::VALUE1,
            1 => SDCLK_FREQ_SELR::VALUE2,
            16 => SDCLK_FREQ_SELR::VALUE3,
            2 => SDCLK_FREQ_SELR::VALUE4,
            4 => SDCLK_FREQ_SELR::VALUE5,
            8 => SDCLK_FREQ_SELR::VALUE6,
            32 => SDCLK_FREQ_SELR::VALUE7,
            64 => SDCLK_FREQ_SELR::VALUE8,
            128 => SDCLK_FREQ_SELR::VALUE9,
            i => SDCLK_FREQ_SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SDCLK_FREQ_SELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SDCLK_FREQ_SELR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == SDCLK_FREQ_SELR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == SDCLK_FREQ_SELR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == SDCLK_FREQ_SELR::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == SDCLK_FREQ_SELR::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == SDCLK_FREQ_SELR::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == SDCLK_FREQ_SELR::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline]
    pub fn is_value9(&self) -> bool {
        *self == SDCLK_FREQ_SELR::VALUE9
    }
}
#[doc = "Possible values of the field `SDCLOCK_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDCLOCK_ENR {
    #[doc = "Disable"]
    VALUE1,
    #[doc = "Enable"]
    VALUE2,
}
impl SDCLOCK_ENR {
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
            SDCLOCK_ENR::VALUE1 => false,
            SDCLOCK_ENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDCLOCK_ENR {
        match value {
            false => SDCLOCK_ENR::VALUE1,
            true => SDCLOCK_ENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SDCLOCK_ENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SDCLOCK_ENR::VALUE2
    }
}
#[doc = "Possible values of the field `INTERNAL_CLOCK_STABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTERNAL_CLOCK_STABLER {
    #[doc = "Not Ready"]
    VALUE1,
    #[doc = "Ready"]
    VALUE2,
}
impl INTERNAL_CLOCK_STABLER {
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
            INTERNAL_CLOCK_STABLER::VALUE1 => false,
            INTERNAL_CLOCK_STABLER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INTERNAL_CLOCK_STABLER {
        match value {
            false => INTERNAL_CLOCK_STABLER::VALUE1,
            true => INTERNAL_CLOCK_STABLER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == INTERNAL_CLOCK_STABLER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == INTERNAL_CLOCK_STABLER::VALUE2
    }
}
#[doc = "Possible values of the field `INTERNAL_CLOCK_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTERNAL_CLOCK_ENR {
    #[doc = "Stop"]
    VALUE1,
    #[doc = "Oscillate"]
    VALUE2,
}
impl INTERNAL_CLOCK_ENR {
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
            INTERNAL_CLOCK_ENR::VALUE1 => false,
            INTERNAL_CLOCK_ENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INTERNAL_CLOCK_ENR {
        match value {
            false => INTERNAL_CLOCK_ENR::VALUE1,
            true => INTERNAL_CLOCK_ENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == INTERNAL_CLOCK_ENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == INTERNAL_CLOCK_ENR::VALUE2
    }
}
#[doc = "Values that can be written to the field `SDCLK_FREQ_SEL`"]
pub enum SDCLK_FREQ_SELW {
    #[doc = "base clock(10MHz-63MHz)"]
    VALUE1,
    #[doc = "base clock divided by 2"]
    VALUE2,
    #[doc = "base clock divided by 32"]
    VALUE3,
    #[doc = "base clock divided by 4"]
    VALUE4,
    #[doc = "base clock divided by 8"]
    VALUE5,
    #[doc = "base clock divided by 16"]
    VALUE6,
    #[doc = "base clock divided by 64"]
    VALUE7,
    #[doc = "base clock divided by 128"]
    VALUE8,
    #[doc = "base clock divided by 256"]
    VALUE9,
}
impl SDCLK_FREQ_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SDCLK_FREQ_SELW::VALUE1 => 0,
            SDCLK_FREQ_SELW::VALUE2 => 1,
            SDCLK_FREQ_SELW::VALUE3 => 16,
            SDCLK_FREQ_SELW::VALUE4 => 2,
            SDCLK_FREQ_SELW::VALUE5 => 4,
            SDCLK_FREQ_SELW::VALUE6 => 8,
            SDCLK_FREQ_SELW::VALUE7 => 32,
            SDCLK_FREQ_SELW::VALUE8 => 64,
            SDCLK_FREQ_SELW::VALUE9 => 128,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDCLK_FREQ_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SDCLK_FREQ_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDCLK_FREQ_SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "base clock(10MHz-63MHz)"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SDCLK_FREQ_SELW::VALUE1)
    }
    #[doc = "base clock divided by 2"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SDCLK_FREQ_SELW::VALUE2)
    }
    #[doc = "base clock divided by 32"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(SDCLK_FREQ_SELW::VALUE3)
    }
    #[doc = "base clock divided by 4"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(SDCLK_FREQ_SELW::VALUE4)
    }
    #[doc = "base clock divided by 8"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(SDCLK_FREQ_SELW::VALUE5)
    }
    #[doc = "base clock divided by 16"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(SDCLK_FREQ_SELW::VALUE6)
    }
    #[doc = "base clock divided by 64"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(SDCLK_FREQ_SELW::VALUE7)
    }
    #[doc = "base clock divided by 128"]
    #[inline]
    pub fn value8(self) -> &'a mut W {
        self.variant(SDCLK_FREQ_SELW::VALUE8)
    }
    #[doc = "base clock divided by 256"]
    #[inline]
    pub fn value9(self) -> &'a mut W {
        self.variant(SDCLK_FREQ_SELW::VALUE9)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SDCLOCK_EN`"]
pub enum SDCLOCK_ENW {
    #[doc = "Disable"]
    VALUE1,
    #[doc = "Enable"]
    VALUE2,
}
impl SDCLOCK_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SDCLOCK_ENW::VALUE1 => false,
            SDCLOCK_ENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDCLOCK_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SDCLOCK_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDCLOCK_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SDCLOCK_ENW::VALUE1)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SDCLOCK_ENW::VALUE2)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INTERNAL_CLOCK_EN`"]
pub enum INTERNAL_CLOCK_ENW {
    #[doc = "Stop"]
    VALUE1,
    #[doc = "Oscillate"]
    VALUE2,
}
impl INTERNAL_CLOCK_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INTERNAL_CLOCK_ENW::VALUE1 => false,
            INTERNAL_CLOCK_ENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INTERNAL_CLOCK_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _INTERNAL_CLOCK_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INTERNAL_CLOCK_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Stop"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(INTERNAL_CLOCK_ENW::VALUE1)
    }
    #[doc = "Oscillate"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(INTERNAL_CLOCK_ENW::VALUE2)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 8:15 - SDCLK Frequency Select"]
    #[inline]
    pub fn sdclk_freq_sel(&self) -> SDCLK_FREQ_SELR {
        SDCLK_FREQ_SELR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 2 - SD Clock Enable"]
    #[inline]
    pub fn sdclock_en(&self) -> SDCLOCK_ENR {
        SDCLOCK_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - Internal Clock Stable"]
    #[inline]
    pub fn internal_clock_stable(&self) -> INTERNAL_CLOCK_STABLER {
        INTERNAL_CLOCK_STABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 0 - Internal Clock Enable"]
    #[inline]
    pub fn internal_clock_en(&self) -> INTERNAL_CLOCK_ENR {
        INTERNAL_CLOCK_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
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
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 8:15 - SDCLK Frequency Select"]
    #[inline]
    pub fn sdclk_freq_sel(&mut self) -> _SDCLK_FREQ_SELW {
        _SDCLK_FREQ_SELW { w: self }
    }
    #[doc = "Bit 2 - SD Clock Enable"]
    #[inline]
    pub fn sdclock_en(&mut self) -> _SDCLOCK_ENW {
        _SDCLOCK_ENW { w: self }
    }
    #[doc = "Bit 0 - Internal Clock Enable"]
    #[inline]
    pub fn internal_clock_en(&mut self) -> _INTERNAL_CLOCK_ENW {
        _INTERNAL_CLOCK_ENW { w: self }
    }
}
