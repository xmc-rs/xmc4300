#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SDMMC_CON {
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
#[doc = "Possible values of the field `WPSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPSELR {
    #[doc = "P1.1 input pin selected"]
    VALUE1,
    #[doc = "Software bit WPVAL is selected"]
    VALUE2,
}
impl WPSELR {
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
            WPSELR::VALUE1 => false,
            WPSELR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WPSELR {
        match value {
            false => WPSELR::VALUE1,
            true => WPSELR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WPSELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WPSELR::VALUE2
    }
}
#[doc = "Possible values of the field `WPSVAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPSVALR {
    #[doc = "No write protection"]
    VALUE1,
    #[doc = "Write protection active"]
    VALUE2,
}
impl WPSVALR {
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
            WPSVALR::VALUE1 => false,
            WPSVALR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WPSVALR {
        match value {
            false => WPSVALR::VALUE1,
            true => WPSVALR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WPSVALR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WPSVALR::VALUE2
    }
}
#[doc = "Possible values of the field `CDSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDSELR {
    #[doc = "P1.10 input pin selected"]
    VALUE1,
    #[doc = "Software bit CDSVAL is selected"]
    VALUE2,
}
impl CDSELR {
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
            CDSELR::VALUE1 => false,
            CDSELR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CDSELR {
        match value {
            false => CDSELR::VALUE1,
            true => CDSELR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CDSELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CDSELR::VALUE2
    }
}
#[doc = "Possible values of the field `CDSVAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDSVALR {
    #[doc = "No card detected"]
    VALUE1,
    #[doc = "Card detected"]
    VALUE2,
}
impl CDSVALR {
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
            CDSVALR::VALUE1 => false,
            CDSVALR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CDSVALR {
        match value {
            false => CDSVALR::VALUE1,
            true => CDSVALR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CDSVALR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CDSVALR::VALUE2
    }
}
#[doc = "Values that can be written to the field `WPSEL`"]
pub enum WPSELW {
    #[doc = "P1.1 input pin selected"]
    VALUE1,
    #[doc = "Software bit WPVAL is selected"]
    VALUE2,
}
impl WPSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WPSELW::VALUE1 => false,
            WPSELW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WPSELW<'a> {
    w: &'a mut W,
}
impl<'a> _WPSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WPSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "P1.1 input pin selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WPSELW::VALUE1)
    }
    #[doc = "Software bit WPVAL is selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WPSELW::VALUE2)
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
#[doc = "Values that can be written to the field `WPSVAL`"]
pub enum WPSVALW {
    #[doc = "No write protection"]
    VALUE1,
    #[doc = "Write protection active"]
    VALUE2,
}
impl WPSVALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WPSVALW::VALUE1 => false,
            WPSVALW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WPSVALW<'a> {
    w: &'a mut W,
}
impl<'a> _WPSVALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WPSVALW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No write protection"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WPSVALW::VALUE1)
    }
    #[doc = "Write protection active"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WPSVALW::VALUE2)
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
#[doc = "Values that can be written to the field `CDSEL`"]
pub enum CDSELW {
    #[doc = "P1.10 input pin selected"]
    VALUE1,
    #[doc = "Software bit CDSVAL is selected"]
    VALUE2,
}
impl CDSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CDSELW::VALUE1 => false,
            CDSELW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CDSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CDSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CDSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "P1.10 input pin selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CDSELW::VALUE1)
    }
    #[doc = "Software bit CDSVAL is selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CDSELW::VALUE2)
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
#[doc = "Values that can be written to the field `CDSVAL`"]
pub enum CDSVALW {
    #[doc = "No card detected"]
    VALUE1,
    #[doc = "Card detected"]
    VALUE2,
}
impl CDSVALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CDSVALW::VALUE1 => false,
            CDSVALW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CDSVALW<'a> {
    w: &'a mut W,
}
impl<'a> _CDSVALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CDSVALW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No card detected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CDSVALW::VALUE1)
    }
    #[doc = "Card detected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CDSVALW::VALUE2)
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
        const OFFSET: u8 = 20;
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
    #[doc = "Bit 0 - SDMMC Write Protection Input Multiplexer Control"]
    #[inline]
    pub fn wpsel(&self) -> WPSELR {
        WPSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - SDMMC Write Protect Software Control"]
    #[inline]
    pub fn wpsval(&self) -> WPSVALR {
        WPSVALR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - SDMMC Card Detection Control"]
    #[inline]
    pub fn cdsel(&self) -> CDSELR {
        CDSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - SDMMC Write Protect Software Control"]
    #[inline]
    pub fn cdsval(&self) -> CDSVALR {
        CDSVALR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
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
    #[doc = "Bit 0 - SDMMC Write Protection Input Multiplexer Control"]
    #[inline]
    pub fn wpsel(&mut self) -> _WPSELW {
        _WPSELW { w: self }
    }
    #[doc = "Bit 4 - SDMMC Write Protect Software Control"]
    #[inline]
    pub fn wpsval(&mut self) -> _WPSVALW {
        _WPSVALW { w: self }
    }
    #[doc = "Bit 16 - SDMMC Card Detection Control"]
    #[inline]
    pub fn cdsel(&mut self) -> _CDSELW {
        _CDSELW { w: self }
    }
    #[doc = "Bit 20 - SDMMC Write Protect Software Control"]
    #[inline]
    pub fn cdsval(&mut self) -> _CDSVALW {
        _CDSVALW { w: self }
    }
}
