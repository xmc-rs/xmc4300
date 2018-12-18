#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::WAKEUP_CTRL {
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
#[doc = "Possible values of the field `WAKEUP_EVENT_EN_REM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUP_EVENT_EN_REMR {
    #[doc = "Disable"]
    VALUE1,
    #[doc = "Enable"]
    VALUE2,
}
impl WAKEUP_EVENT_EN_REMR {
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
            WAKEUP_EVENT_EN_REMR::VALUE1 => false,
            WAKEUP_EVENT_EN_REMR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKEUP_EVENT_EN_REMR {
        match value {
            false => WAKEUP_EVENT_EN_REMR::VALUE1,
            true => WAKEUP_EVENT_EN_REMR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WAKEUP_EVENT_EN_REMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WAKEUP_EVENT_EN_REMR::VALUE2
    }
}
#[doc = "Possible values of the field `WAKEUP_EVENT_EN_INS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUP_EVENT_EN_INSR {
    #[doc = "Disable"]
    VALUE1,
    #[doc = "Enable"]
    VALUE2,
}
impl WAKEUP_EVENT_EN_INSR {
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
            WAKEUP_EVENT_EN_INSR::VALUE1 => false,
            WAKEUP_EVENT_EN_INSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKEUP_EVENT_EN_INSR {
        match value {
            false => WAKEUP_EVENT_EN_INSR::VALUE1,
            true => WAKEUP_EVENT_EN_INSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WAKEUP_EVENT_EN_INSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WAKEUP_EVENT_EN_INSR::VALUE2
    }
}
#[doc = "Possible values of the field `WAKEUP_EVENT_EN_INT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUP_EVENT_EN_INTR {
    #[doc = "Disable"]
    VALUE1,
    #[doc = "Enable"]
    VALUE2,
}
impl WAKEUP_EVENT_EN_INTR {
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
            WAKEUP_EVENT_EN_INTR::VALUE1 => false,
            WAKEUP_EVENT_EN_INTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKEUP_EVENT_EN_INTR {
        match value {
            false => WAKEUP_EVENT_EN_INTR::VALUE1,
            true => WAKEUP_EVENT_EN_INTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WAKEUP_EVENT_EN_INTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WAKEUP_EVENT_EN_INTR::VALUE2
    }
}
#[doc = "Values that can be written to the field `WAKEUP_EVENT_EN_REM`"]
pub enum WAKEUP_EVENT_EN_REMW {
    #[doc = "Disable"]
    VALUE1,
    #[doc = "Enable"]
    VALUE2,
}
impl WAKEUP_EVENT_EN_REMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKEUP_EVENT_EN_REMW::VALUE1 => false,
            WAKEUP_EVENT_EN_REMW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKEUP_EVENT_EN_REMW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUP_EVENT_EN_REMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKEUP_EVENT_EN_REMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WAKEUP_EVENT_EN_REMW::VALUE1)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WAKEUP_EVENT_EN_REMW::VALUE2)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WAKEUP_EVENT_EN_INS`"]
pub enum WAKEUP_EVENT_EN_INSW {
    #[doc = "Disable"]
    VALUE1,
    #[doc = "Enable"]
    VALUE2,
}
impl WAKEUP_EVENT_EN_INSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKEUP_EVENT_EN_INSW::VALUE1 => false,
            WAKEUP_EVENT_EN_INSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKEUP_EVENT_EN_INSW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUP_EVENT_EN_INSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKEUP_EVENT_EN_INSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WAKEUP_EVENT_EN_INSW::VALUE1)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WAKEUP_EVENT_EN_INSW::VALUE2)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WAKEUP_EVENT_EN_INT`"]
pub enum WAKEUP_EVENT_EN_INTW {
    #[doc = "Disable"]
    VALUE1,
    #[doc = "Enable"]
    VALUE2,
}
impl WAKEUP_EVENT_EN_INTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKEUP_EVENT_EN_INTW::VALUE1 => false,
            WAKEUP_EVENT_EN_INTW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKEUP_EVENT_EN_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUP_EVENT_EN_INTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKEUP_EVENT_EN_INTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WAKEUP_EVENT_EN_INTW::VALUE1)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WAKEUP_EVENT_EN_INTW::VALUE2)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 2 - Wakeup Event Enable On SD Card Removal"]
    #[inline]
    pub fn wakeup_event_en_rem(&self) -> WAKEUP_EVENT_EN_REMR {
        WAKEUP_EVENT_EN_REMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Wakeup Event Enable On SD Card Insertion"]
    #[inline]
    pub fn wakeup_event_en_ins(&self) -> WAKEUP_EVENT_EN_INSR {
        WAKEUP_EVENT_EN_INSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 0 - Wakeup Event Enable On Card Interrupt"]
    #[inline]
    pub fn wakeup_event_en_int(&self) -> WAKEUP_EVENT_EN_INTR {
        WAKEUP_EVENT_EN_INTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 2 - Wakeup Event Enable On SD Card Removal"]
    #[inline]
    pub fn wakeup_event_en_rem(&mut self) -> _WAKEUP_EVENT_EN_REMW {
        _WAKEUP_EVENT_EN_REMW { w: self }
    }
    #[doc = "Bit 1 - Wakeup Event Enable On SD Card Insertion"]
    #[inline]
    pub fn wakeup_event_en_ins(&mut self) -> _WAKEUP_EVENT_EN_INSW {
        _WAKEUP_EVENT_EN_INSW { w: self }
    }
    #[doc = "Bit 0 - Wakeup Event Enable On Card Interrupt"]
    #[inline]
    pub fn wakeup_event_en_int(&mut self) -> _WAKEUP_EVENT_EN_INTW {
        _WAKEUP_EVENT_EN_INTW { w: self }
    }
}
