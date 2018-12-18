#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ASCTRL {
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
#[doc = "Possible values of the field `SRCRESREG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRCRESREGR {
    #[doc = "Use GxCHCTRy.RESREG to select a group result register"]
    VALUE1,
    #[doc = "Store result in group result register GxRES1"]
    VALUE2,
    #[doc = "Store result in group result register GxRES15"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SRCRESREGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SRCRESREGR::VALUE1 => 0,
            SRCRESREGR::VALUE2 => 1,
            SRCRESREGR::VALUE3 => 15,
            SRCRESREGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SRCRESREGR {
        match value {
            0 => SRCRESREGR::VALUE1,
            1 => SRCRESREGR::VALUE2,
            15 => SRCRESREGR::VALUE3,
            i => SRCRESREGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SRCRESREGR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SRCRESREGR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == SRCRESREGR::VALUE3
    }
}
#[doc = r" Value of the field"]
pub struct XTSELR {
    bits: u8,
}
impl XTSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct XTLVLR {
    bits: bool,
}
impl XTLVLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `XTMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTMODER {
    #[doc = "No external trigger"]
    VALUE1,
    #[doc = "Trigger event upon a falling edge"]
    VALUE2,
    #[doc = "Trigger event upon a rising edge"]
    VALUE3,
    #[doc = "Trigger event upon any edge"]
    VALUE4,
}
impl XTMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            XTMODER::VALUE1 => 0,
            XTMODER::VALUE2 => 1,
            XTMODER::VALUE3 => 2,
            XTMODER::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> XTMODER {
        match value {
            0 => XTMODER::VALUE1,
            1 => XTMODER::VALUE2,
            2 => XTMODER::VALUE3,
            3 => XTMODER::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == XTMODER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == XTMODER::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == XTMODER::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == XTMODER::VALUE4
    }
}
#[doc = r" Value of the field"]
pub struct GTSELR {
    bits: u8,
}
impl GTSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GTLVLR {
    bits: bool,
}
impl GTLVLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `TMEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMENR {
    #[doc = "No timer mode: standard gating mechanism can be used"]
    VALUE1,
    #[doc = "Timer mode for equidistant sampling enabled: standard gating mechanism must be disabled"]
    VALUE2,
}
impl TMENR {
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
            TMENR::VALUE1 => false,
            TMENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMENR {
        match value {
            false => TMENR::VALUE1,
            true => TMENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TMENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TMENR::VALUE2
    }
}
#[doc = "Values that can be written to the field `SRCRESREG`"]
pub enum SRCRESREGW {
    #[doc = "Use GxCHCTRy.RESREG to select a group result register"]
    VALUE1,
    #[doc = "Store result in group result register GxRES1"]
    VALUE2,
    #[doc = "Store result in group result register GxRES15"]
    VALUE3,
}
impl SRCRESREGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRCRESREGW::VALUE1 => 0,
            SRCRESREGW::VALUE2 => 1,
            SRCRESREGW::VALUE3 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRCRESREGW<'a> {
    w: &'a mut W,
}
impl<'a> _SRCRESREGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRCRESREGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Use GxCHCTRy.RESREG to select a group result register"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRCRESREGW::VALUE1)
    }
    #[doc = "Store result in group result register GxRES1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SRCRESREGW::VALUE2)
    }
    #[doc = "Store result in group result register GxRES15"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(SRCRESREGW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _XTSELW<'a> {
    w: &'a mut W,
}
impl<'a> _XTSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `XTMODE`"]
pub enum XTMODEW {
    #[doc = "No external trigger"]
    VALUE1,
    #[doc = "Trigger event upon a falling edge"]
    VALUE2,
    #[doc = "Trigger event upon a rising edge"]
    VALUE3,
    #[doc = "Trigger event upon any edge"]
    VALUE4,
}
impl XTMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            XTMODEW::VALUE1 => 0,
            XTMODEW::VALUE2 => 1,
            XTMODEW::VALUE3 => 2,
            XTMODEW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _XTMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _XTMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XTMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No external trigger"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(XTMODEW::VALUE1)
    }
    #[doc = "Trigger event upon a falling edge"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(XTMODEW::VALUE2)
    }
    #[doc = "Trigger event upon a rising edge"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(XTMODEW::VALUE3)
    }
    #[doc = "Trigger event upon any edge"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(XTMODEW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `XTWC`"]
pub enum XTWCW {
    #[doc = "No write access to trigger configuration"]
    VALUE1,
    #[doc = "Bitfields XTMODE and XTSEL can be written"]
    VALUE2,
}
impl XTWCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            XTWCW::VALUE1 => false,
            XTWCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _XTWCW<'a> {
    w: &'a mut W,
}
impl<'a> _XTWCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XTWCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No write access to trigger configuration"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(XTWCW::VALUE1)
    }
    #[doc = "Bitfields XTMODE and XTSEL can be written"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(XTWCW::VALUE2)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GTSELW<'a> {
    w: &'a mut W,
}
impl<'a> _GTSELW<'a> {
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
#[doc = "Values that can be written to the field `GTWC`"]
pub enum GTWCW {
    #[doc = "No write access to gate configuration"]
    VALUE1,
    #[doc = "Bitfield GTSEL can be written"]
    VALUE2,
}
impl GTWCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GTWCW::VALUE1 => false,
            GTWCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GTWCW<'a> {
    w: &'a mut W,
}
impl<'a> _GTWCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GTWCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No write access to gate configuration"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(GTWCW::VALUE1)
    }
    #[doc = "Bitfield GTSEL can be written"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(GTWCW::VALUE2)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TMEN`"]
pub enum TMENW {
    #[doc = "No timer mode: standard gating mechanism can be used"]
    VALUE1,
    #[doc = "Timer mode for equidistant sampling enabled: standard gating mechanism must be disabled"]
    VALUE2,
}
impl TMENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMENW::VALUE1 => false,
            TMENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMENW<'a> {
    w: &'a mut W,
}
impl<'a> _TMENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No timer mode: standard gating mechanism can be used"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TMENW::VALUE1)
    }
    #[doc = "Timer mode for equidistant sampling enabled: standard gating mechanism must be disabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TMENW::VALUE2)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TMWC`"]
pub enum TMWCW {
    #[doc = "No write access to timer mode"]
    VALUE1,
    #[doc = "Bitfield TMEN can be written"]
    VALUE2,
}
impl TMWCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMWCW::VALUE1 => false,
            TMWCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMWCW<'a> {
    w: &'a mut W,
}
impl<'a> _TMWCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMWCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No write access to timer mode"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TMWCW::VALUE1)
    }
    #[doc = "Bitfield TMEN can be written"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TMWCW::VALUE2)
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
    #[doc = "Bits 0:3 - Source-specific Result Register"]
    #[inline]
    pub fn srcresreg(&self) -> SRCRESREGR {
        SRCRESREGR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - External Trigger Input Selection"]
    #[inline]
    pub fn xtsel(&self) -> XTSELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        XTSELR { bits }
    }
    #[doc = "Bit 12 - External Trigger Level"]
    #[inline]
    pub fn xtlvl(&self) -> XTLVLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        XTLVLR { bits }
    }
    #[doc = "Bits 13:14 - Trigger Operating Mode"]
    #[inline]
    pub fn xtmode(&self) -> XTMODER {
        XTMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Gate Input Selection"]
    #[inline]
    pub fn gtsel(&self) -> GTSELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GTSELR { bits }
    }
    #[doc = "Bit 20 - Gate Input Level"]
    #[inline]
    pub fn gtlvl(&self) -> GTLVLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GTLVLR { bits }
    }
    #[doc = "Bit 28 - Timer Mode Enable"]
    #[inline]
    pub fn tmen(&self) -> TMENR {
        TMENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:3 - Source-specific Result Register"]
    #[inline]
    pub fn srcresreg(&mut self) -> _SRCRESREGW {
        _SRCRESREGW { w: self }
    }
    #[doc = "Bits 8:11 - External Trigger Input Selection"]
    #[inline]
    pub fn xtsel(&mut self) -> _XTSELW {
        _XTSELW { w: self }
    }
    #[doc = "Bits 13:14 - Trigger Operating Mode"]
    #[inline]
    pub fn xtmode(&mut self) -> _XTMODEW {
        _XTMODEW { w: self }
    }
    #[doc = "Bit 15 - Write Control for Trigger Configuration"]
    #[inline]
    pub fn xtwc(&mut self) -> _XTWCW {
        _XTWCW { w: self }
    }
    #[doc = "Bits 16:19 - Gate Input Selection"]
    #[inline]
    pub fn gtsel(&mut self) -> _GTSELW {
        _GTSELW { w: self }
    }
    #[doc = "Bit 23 - Write Control for Gate Configuration"]
    #[inline]
    pub fn gtwc(&mut self) -> _GTWCW {
        _GTWCW { w: self }
    }
    #[doc = "Bit 28 - Timer Mode Enable"]
    #[inline]
    pub fn tmen(&mut self) -> _TMENW {
        _TMENW { w: self }
    }
    #[doc = "Bit 31 - Write Control for Timer Mode"]
    #[inline]
    pub fn tmwc(&mut self) -> _TMWCW {
        _TMWCW { w: self }
    }
}
