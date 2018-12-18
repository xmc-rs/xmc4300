#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RCR {
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
pub struct DRCTRR {
    bits: u8,
}
impl DRCTRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `DMM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMMR {
    #[doc = "Standard data reduction (accumulation)"]
    VALUE1,
    #[doc = "Result filtering mode"]
    VALUE2,
    #[doc = "Difference mode"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DMMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DMMR::VALUE1 => 0,
            DMMR::VALUE2 => 1,
            DMMR::VALUE3 => 2,
            DMMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DMMR {
        match value {
            0 => DMMR::VALUE1,
            1 => DMMR::VALUE2,
            2 => DMMR::VALUE3,
            i => DMMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DMMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DMMR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == DMMR::VALUE3
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
#[doc = "Possible values of the field `FEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FENR {
    #[doc = "Separate result register"]
    VALUE1,
    #[doc = "Part of a FIFO structure: copy each new valid result"]
    VALUE2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FENR::VALUE1 => 0,
            FENR::VALUE2 => 1,
            FENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FENR {
        match value {
            0 => FENR::VALUE1,
            1 => FENR::VALUE2,
            i => FENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == FENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == FENR::VALUE2
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
#[doc = r" Proxy"]
pub struct _DRCTRW<'a> {
    w: &'a mut W,
}
impl<'a> _DRCTRW<'a> {
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
#[doc = "Values that can be written to the field `DMM`"]
pub enum DMMW {
    #[doc = "Standard data reduction (accumulation)"]
    VALUE1,
    #[doc = "Result filtering mode"]
    VALUE2,
    #[doc = "Difference mode"]
    VALUE3,
}
impl DMMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DMMW::VALUE1 => 0,
            DMMW::VALUE2 => 1,
            DMMW::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMMW<'a> {
    w: &'a mut W,
}
impl<'a> _DMMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Standard data reduction (accumulation)"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DMMW::VALUE1)
    }
    #[doc = "Result filtering mode"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DMMW::VALUE2)
    }
    #[doc = "Difference mode"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(DMMW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
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
#[doc = "Values that can be written to the field `FEN`"]
pub enum FENW {
    #[doc = "Separate result register"]
    VALUE1,
    #[doc = "Part of a FIFO structure: copy each new valid result"]
    VALUE2,
}
impl FENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FENW::VALUE1 => 0,
            FENW::VALUE2 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FENW<'a> {
    w: &'a mut W,
}
impl<'a> _FENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Separate result register"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FENW::VALUE1)
    }
    #[doc = "Part of a FIFO structure: copy each new valid result"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FENW::VALUE2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 25;
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
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DRCTRR { bits }
    }
    #[doc = "Bits 20:21 - Data Modification Mode"]
    #[inline]
    pub fn dmm(&self) -> DMMR {
        DMMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
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
    #[doc = "Bits 25:26 - FIFO Mode Enable"]
    #[inline]
    pub fn fen(&self) -> FENR {
        FENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 20:21 - Data Modification Mode"]
    #[inline]
    pub fn dmm(&mut self) -> _DMMW {
        _DMMW { w: self }
    }
    #[doc = "Bit 24 - Wait-for-Read Mode Enable"]
    #[inline]
    pub fn wfr(&mut self) -> _WFRW {
        _WFRW { w: self }
    }
    #[doc = "Bits 25:26 - FIFO Mode Enable"]
    #[inline]
    pub fn fen(&mut self) -> _FENW {
        _FENW { w: self }
    }
    #[doc = "Bit 31 - Service Request Generation Enable"]
    #[inline]
    pub fn srgen(&mut self) -> _SRGENW {
        _SRGENW { w: self }
    }
}
