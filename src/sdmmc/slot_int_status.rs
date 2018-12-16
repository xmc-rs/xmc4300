#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
impl super::SLOT_INT_STATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `SLOT_INT_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLOT_INT_STATUSR {
    #[doc = "Slot 1"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SLOT_INT_STATUSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SLOT_INT_STATUSR::VALUE1 => 0,
            SLOT_INT_STATUSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SLOT_INT_STATUSR {
        match value {
            0 => SLOT_INT_STATUSR::VALUE1,
            i => SLOT_INT_STATUSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SLOT_INT_STATUSR::VALUE1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:7 - Interrupt Signal for Card Slot"]
    #[inline]
    pub fn slot_int_status(&self) -> SLOT_INT_STATUSR {
        SLOT_INT_STATUSR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
}
