#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ICSR {
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
#[doc = "Possible values of the field `VECTACTIVE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VECTACTIVER {
    #[doc = "Thread mode"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl VECTACTIVER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            VECTACTIVER::VALUE1 => 0,
            VECTACTIVER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> VECTACTIVER {
        match value {
            0 => VECTACTIVER::VALUE1,
            i => VECTACTIVER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VECTACTIVER::VALUE1
    }
}
#[doc = "Possible values of the field `RETTOBASE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RETTOBASER {
    #[doc = "there are preempted active exceptions to execute"]
    VALUE1,
    #[doc = "there are no active exceptions, or the currently-executing exception is the only active exception."]
    VALUE2,
}
impl RETTOBASER {
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
            RETTOBASER::VALUE1 => false,
            RETTOBASER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RETTOBASER {
        match value {
            false => RETTOBASER::VALUE1,
            true => RETTOBASER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RETTOBASER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RETTOBASER::VALUE2
    }
}
#[doc = "Possible values of the field `VECTPENDING`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VECTPENDINGR {
    #[doc = "no pending exceptions"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl VECTPENDINGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VECTPENDINGR::VALUE1 => 0,
            VECTPENDINGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VECTPENDINGR {
        match value {
            0 => VECTPENDINGR::VALUE1,
            i => VECTPENDINGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VECTPENDINGR::VALUE1
    }
}
#[doc = "Possible values of the field `ISRPENDING`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISRPENDINGR {
    #[doc = "interrupt not pending"]
    VALUE1,
    #[doc = "interrupt pending."]
    VALUE2,
}
impl ISRPENDINGR {
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
            ISRPENDINGR::VALUE1 => false,
            ISRPENDINGR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ISRPENDINGR {
        match value {
            false => ISRPENDINGR::VALUE1,
            true => ISRPENDINGR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ISRPENDINGR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ISRPENDINGR::VALUE2
    }
}
#[doc = "Possible values of the field `PENDSTSET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSTSETR {
    #[doc = "no effect"]
    VALUE1,
    #[doc = "changes SysTick exception state to pending."]
    VALUE2,
}
impl PENDSTSETR {
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
            PENDSTSETR::VALUE1 => false,
            PENDSTSETR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PENDSTSETR {
        match value {
            false => PENDSTSETR::VALUE1,
            true => PENDSTSETR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PENDSTSETR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PENDSTSETR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct PENDSVSETR {
    bits: bool,
}
impl PENDSVSETR {
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
#[doc = r" Value of the field"]
pub struct NMIPENDSETR {
    bits: bool,
}
impl NMIPENDSETR {
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
#[doc = "Values that can be written to the field `PENDSTCLR`"]
pub enum PENDSTCLRW {
    #[doc = "no effect"]
    VALUE1,
    #[doc = "removes the pending state from the SysTick exception."]
    VALUE2,
}
impl PENDSTCLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PENDSTCLRW::VALUE1 => false,
            PENDSTCLRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PENDSTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _PENDSTCLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PENDSTCLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PENDSTCLRW::VALUE1)
    }
    #[doc = "removes the pending state from the SysTick exception."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PENDSTCLRW::VALUE2)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PENDSTSET`"]
pub enum PENDSTSETW {
    #[doc = "no effect"]
    VALUE1,
    #[doc = "changes SysTick exception state to pending."]
    VALUE2,
}
impl PENDSTSETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PENDSTSETW::VALUE1 => false,
            PENDSTSETW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PENDSTSETW<'a> {
    w: &'a mut W,
}
impl<'a> _PENDSTSETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PENDSTSETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PENDSTSETW::VALUE1)
    }
    #[doc = "changes SysTick exception state to pending."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PENDSTSETW::VALUE2)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PENDSVCLR`"]
pub enum PENDSVCLRW {
    #[doc = "no effect"]
    VALUE1,
    #[doc = "removes the pending state from the PendSV exception."]
    VALUE2,
}
impl PENDSVCLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PENDSVCLRW::VALUE1 => false,
            PENDSVCLRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PENDSVCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _PENDSVCLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PENDSVCLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PENDSVCLRW::VALUE1)
    }
    #[doc = "removes the pending state from the PendSV exception."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PENDSVCLRW::VALUE2)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PENDSVSETW<'a> {
    w: &'a mut W,
}
impl<'a> _PENDSVSETW<'a> {
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
#[doc = r" Proxy"]
pub struct _NMIPENDSETW<'a> {
    w: &'a mut W,
}
impl<'a> _NMIPENDSETW<'a> {
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
    #[doc = "Bits 0:8 - Active exception number"]
    #[inline]
    pub fn vectactive(&self) -> VECTACTIVER {
        VECTACTIVER::_from({
            const MASK: u16 = 511;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bit 11 - Return to Base"]
    #[inline]
    pub fn rettobase(&self) -> RETTOBASER {
        RETTOBASER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 12:17 - Vector Pending"]
    #[inline]
    pub fn vectpending(&self) -> VECTPENDINGR {
        VECTPENDINGR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 22 - Interrupt pending flag"]
    #[inline]
    pub fn isrpending(&self) -> ISRPENDINGR {
        ISRPENDINGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - SysTick exception set-pending bit"]
    #[inline]
    pub fn pendstset(&self) -> PENDSTSETR {
        PENDSTSETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - PendSV set-pending bit: 0b0=no effect, 0b1=changes PendSV exception state to pending., 0b0=PendSV exception is not pending, 0b1=PendSV exception is pending.,"]
    #[inline]
    pub fn pendsvset(&self) -> PENDSVSETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PENDSVSETR { bits }
    }
    #[doc = "Bit 31 - NMI set-pending bit: 0b0=no effect, 0b1=changes NMI exception state to pending., 0b0=NMI exception is not pending, 0b1=NMI exception is pending.,"]
    #[inline]
    pub fn nmipendset(&self) -> NMIPENDSETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NMIPENDSETR { bits }
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
    #[doc = "Bit 25 - SysTick exception clear-pending bit"]
    #[inline]
    pub fn pendstclr(&mut self) -> _PENDSTCLRW {
        _PENDSTCLRW { w: self }
    }
    #[doc = "Bit 26 - SysTick exception set-pending bit"]
    #[inline]
    pub fn pendstset(&mut self) -> _PENDSTSETW {
        _PENDSTSETW { w: self }
    }
    #[doc = "Bit 27 - PendSV clear-pending bit"]
    #[inline]
    pub fn pendsvclr(&mut self) -> _PENDSVCLRW {
        _PENDSVCLRW { w: self }
    }
    #[doc = "Bit 28 - PendSV set-pending bit: 0b0=no effect, 0b1=changes PendSV exception state to pending., 0b0=PendSV exception is not pending, 0b1=PendSV exception is pending.,"]
    #[inline]
    pub fn pendsvset(&mut self) -> _PENDSVSETW {
        _PENDSVSETW { w: self }
    }
    #[doc = "Bit 31 - NMI set-pending bit: 0b0=no effect, 0b1=changes NMI exception state to pending., 0b0=NMI exception is not pending, 0b1=NMI exception is pending.,"]
    #[inline]
    pub fn nmipendset(&mut self) -> _NMIPENDSETW {
        _NMIPENDSETW { w: self }
    }
}
