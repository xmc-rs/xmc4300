#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::QMR0 {
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
#[doc = "Possible values of the field `ENGT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENGTR {
    #[doc = "No conversion requests are issued"]
    VALUE1,
    #[doc = "Conversion requests are issued if a valid conversion request is pending in the queue 0 register or in the backup register"]
    VALUE2,
    #[doc = "Conversion requests are issued if a valid conversion request is pending in the queue 0 register or in the backup register and REQGTx = 1"]
    VALUE3,
    #[doc = "Conversion requests are issued if a valid conversion request is pending in the queue 0 register or in the backup register and REQGTx = 0"]
    VALUE4,
}
impl ENGTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ENGTR::VALUE1 => 0,
            ENGTR::VALUE2 => 1,
            ENGTR::VALUE3 => 2,
            ENGTR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ENGTR {
        match value {
            0 => ENGTR::VALUE1,
            1 => ENGTR::VALUE2,
            2 => ENGTR::VALUE3,
            3 => ENGTR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ENGTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ENGTR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == ENGTR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == ENGTR::VALUE4
    }
}
#[doc = "Possible values of the field `ENTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENTRR {
    #[doc = "External trigger disabled"]
    VALUE1,
    #[doc = "The selected edge at the selected trigger input signal REQTR generates the trigger event"]
    VALUE2,
}
impl ENTRR {
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
            ENTRR::VALUE1 => false,
            ENTRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENTRR {
        match value {
            false => ENTRR::VALUE1,
            true => ENTRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ENTRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ENTRR::VALUE2
    }
}
#[doc = "Possible values of the field `RPTDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RPTDISR {
    #[doc = "A cancelled conversion is repeated"]
    VALUE1,
    #[doc = "A cancelled conversion is discarded"]
    VALUE2,
}
impl RPTDISR {
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
            RPTDISR::VALUE1 => false,
            RPTDISR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RPTDISR {
        match value {
            false => RPTDISR::VALUE1,
            true => RPTDISR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RPTDISR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RPTDISR::VALUE2
    }
}
#[doc = "Values that can be written to the field `ENGT`"]
pub enum ENGTW {
    #[doc = "No conversion requests are issued"]
    VALUE1,
    #[doc = "Conversion requests are issued if a valid conversion request is pending in the queue 0 register or in the backup register"]
    VALUE2,
    #[doc = "Conversion requests are issued if a valid conversion request is pending in the queue 0 register or in the backup register and REQGTx = 1"]
    VALUE3,
    #[doc = "Conversion requests are issued if a valid conversion request is pending in the queue 0 register or in the backup register and REQGTx = 0"]
    VALUE4,
}
impl ENGTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ENGTW::VALUE1 => 0,
            ENGTW::VALUE2 => 1,
            ENGTW::VALUE3 => 2,
            ENGTW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENGTW<'a> {
    w: &'a mut W,
}
impl<'a> _ENGTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENGTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No conversion requests are issued"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENGTW::VALUE1)
    }
    #[doc = "Conversion requests are issued if a valid conversion request is pending in the queue 0 register or in the backup register"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENGTW::VALUE2)
    }
    #[doc = "Conversion requests are issued if a valid conversion request is pending in the queue 0 register or in the backup register and REQGTx = 1"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(ENGTW::VALUE3)
    }
    #[doc = "Conversion requests are issued if a valid conversion request is pending in the queue 0 register or in the backup register and REQGTx = 0"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(ENGTW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ENTR`"]
pub enum ENTRW {
    #[doc = "External trigger disabled"]
    VALUE1,
    #[doc = "The selected edge at the selected trigger input signal REQTR generates the trigger event"]
    VALUE2,
}
impl ENTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENTRW::VALUE1 => false,
            ENTRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENTRW<'a> {
    w: &'a mut W,
}
impl<'a> _ENTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENTRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "External trigger disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENTRW::VALUE1)
    }
    #[doc = "The selected edge at the selected trigger input signal REQTR generates the trigger event"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENTRW::VALUE2)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLRV`"]
pub enum CLRVW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "The next pending valid queue entry in the sequence and the event flag EV are cleared. If there is a valid entry in the queue backup register (QBUR.V = 1), this entry is cleared, otherwise the entry in queue register 0 is cleared."]
    VALUE2,
}
impl CLRVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLRVW::VALUE1 => false,
            CLRVW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLRVW<'a> {
    w: &'a mut W,
}
impl<'a> _CLRVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLRVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CLRVW::VALUE1)
    }
    #[doc = "The next pending valid queue entry in the sequence and the event flag EV are cleared. If there is a valid entry in the queue backup register (QBUR.V = 1), this entry is cleared, otherwise the entry in queue register 0 is cleared."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CLRVW::VALUE2)
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
#[doc = "Values that can be written to the field `TREV`"]
pub enum TREVW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Generate a trigger event by software"]
    VALUE2,
}
impl TREVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TREVW::VALUE1 => false,
            TREVW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TREVW<'a> {
    w: &'a mut W,
}
impl<'a> _TREVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TREVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TREVW::VALUE1)
    }
    #[doc = "Generate a trigger event by software"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TREVW::VALUE2)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FLUSH`"]
pub enum FLUSHW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Clear all queue entries (including backup stage) and the event flag EV. The queue contains no more valid entry."]
    VALUE2,
}
impl FLUSHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLUSHW::VALUE1 => false,
            FLUSHW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLUSHW<'a> {
    w: &'a mut W,
}
impl<'a> _FLUSHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLUSHW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FLUSHW::VALUE1)
    }
    #[doc = "Clear all queue entries (including backup stage) and the event flag EV. The queue contains no more valid entry."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FLUSHW::VALUE2)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CEV`"]
pub enum CEVW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Clear bit EV"]
    VALUE2,
}
impl CEVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CEVW::VALUE1 => false,
            CEVW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CEVW<'a> {
    w: &'a mut W,
}
impl<'a> _CEVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CEVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEVW::VALUE1)
    }
    #[doc = "Clear bit EV"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEVW::VALUE2)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RPTDIS`"]
pub enum RPTDISW {
    #[doc = "A cancelled conversion is repeated"]
    VALUE1,
    #[doc = "A cancelled conversion is discarded"]
    VALUE2,
}
impl RPTDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RPTDISW::VALUE1 => false,
            RPTDISW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RPTDISW<'a> {
    w: &'a mut W,
}
impl<'a> _RPTDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RPTDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A cancelled conversion is repeated"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RPTDISW::VALUE1)
    }
    #[doc = "A cancelled conversion is discarded"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RPTDISW::VALUE2)
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
    #[doc = "Bits 0:1 - Enable Gate"]
    #[inline]
    pub fn engt(&self) -> ENGTR {
        ENGTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Enable External Trigger"]
    #[inline]
    pub fn entr(&self) -> ENTRR {
        ENTRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Repeat Disable"]
    #[inline]
    pub fn rptdis(&self) -> RPTDISR {
        RPTDISR::_from({
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
    #[doc = "Bits 0:1 - Enable Gate"]
    #[inline]
    pub fn engt(&mut self) -> _ENGTW {
        _ENGTW { w: self }
    }
    #[doc = "Bit 2 - Enable External Trigger"]
    #[inline]
    pub fn entr(&mut self) -> _ENTRW {
        _ENTRW { w: self }
    }
    #[doc = "Bit 8 - Clear Valid Bit"]
    #[inline]
    pub fn clrv(&mut self) -> _CLRVW {
        _CLRVW { w: self }
    }
    #[doc = "Bit 9 - Trigger Event"]
    #[inline]
    pub fn trev(&mut self) -> _TREVW {
        _TREVW { w: self }
    }
    #[doc = "Bit 10 - Flush Queue"]
    #[inline]
    pub fn flush(&mut self) -> _FLUSHW {
        _FLUSHW { w: self }
    }
    #[doc = "Bit 11 - Clear Event Flag"]
    #[inline]
    pub fn cev(&mut self) -> _CEVW {
        _CEVW { w: self }
    }
    #[doc = "Bit 16 - Repeat Disable"]
    #[inline]
    pub fn rptdis(&mut self) -> _RPTDISW {
        _RPTDISW { w: self }
    }
}
