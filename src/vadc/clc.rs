#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLC {
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
#[doc = "Possible values of the field `DISR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISRR {
    #[doc = "On request: enable the module clock"]
    VALUE1,
    #[doc = "Off request: stop the module clock"]
    VALUE2,
}
impl DISRR {
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
            DISRR::VALUE1 => false,
            DISRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DISRR {
        match value {
            false => DISRR::VALUE1,
            true => DISRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DISRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DISRR::VALUE2
    }
}
#[doc = "Possible values of the field `DISS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISSR {
    #[doc = "Module clock is enabled"]
    VALUE1,
    #[doc = "Off: module is not clocked"]
    VALUE2,
}
impl DISSR {
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
            DISSR::VALUE1 => false,
            DISSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DISSR {
        match value {
            false => DISSR::VALUE1,
            true => DISSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DISSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DISSR::VALUE2
    }
}
#[doc = "Possible values of the field `EDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDISR {
    #[doc = "Sleep mode request is enabled and functional"]
    VALUE1,
    #[doc = "Module disregards the sleep mode control signal"]
    VALUE2,
}
impl EDISR {
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
            EDISR::VALUE1 => false,
            EDISR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDISR {
        match value {
            false => EDISR::VALUE1,
            true => EDISR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EDISR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EDISR::VALUE2
    }
}
#[doc = "Values that can be written to the field `DISR`"]
pub enum DISRW {
    #[doc = "On request: enable the module clock"]
    VALUE1,
    #[doc = "Off request: stop the module clock"]
    VALUE2,
}
impl DISRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DISRW::VALUE1 => false,
            DISRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DISRW<'a> {
    w: &'a mut W,
}
impl<'a> _DISRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DISRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "On request: enable the module clock"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DISRW::VALUE1)
    }
    #[doc = "Off request: stop the module clock"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DISRW::VALUE2)
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
#[doc = "Values that can be written to the field `EDIS`"]
pub enum EDISW {
    #[doc = "Sleep mode request is enabled and functional"]
    VALUE1,
    #[doc = "Module disregards the sleep mode control signal"]
    VALUE2,
}
impl EDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDISW::VALUE1 => false,
            EDISW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDISW<'a> {
    w: &'a mut W,
}
impl<'a> _EDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Sleep mode request is enabled and functional"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EDISW::VALUE1)
    }
    #[doc = "Module disregards the sleep mode control signal"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EDISW::VALUE2)
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
        const OFFSET: u8 = 3;
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
    #[doc = "Bit 0 - Module Disable Request Bit"]
    #[inline]
    pub fn disr(&self) -> DISRR {
        DISRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Module Disable Status Bit"]
    #[inline]
    pub fn diss(&self) -> DISSR {
        DISSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Sleep Mode Enable Control"]
    #[inline]
    pub fn edis(&self) -> EDISR {
        EDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 3 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Module Disable Request Bit"]
    #[inline]
    pub fn disr(&mut self) -> _DISRW {
        _DISRW { w: self }
    }
    #[doc = "Bit 3 - Sleep Mode Enable Control"]
    #[inline]
    pub fn edis(&mut self) -> _EDISW {
        _EDISW { w: self }
    }
}
