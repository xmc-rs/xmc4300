#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::DC_LATCH0_STAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `EV_L0_POS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EV_L0_POSR {
    #[doc = "Positive edge not detected or continuous mode"]
    VALUE1,
    #[doc = "Positive edge detected in single event mode only"]
    VALUE2,
}
impl EV_L0_POSR {
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
            EV_L0_POSR::VALUE1 => false,
            EV_L0_POSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EV_L0_POSR {
        match value {
            false => EV_L0_POSR::VALUE1,
            true => EV_L0_POSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EV_L0_POSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EV_L0_POSR::VALUE2
    }
}
#[doc = "Possible values of the field `EV_L0_NEG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EV_L0_NEGR {
    #[doc = "Negative edge not detected or continuous mode"]
    VALUE1,
    #[doc = "Negative edge detected in single event mode only"]
    VALUE2,
}
impl EV_L0_NEGR {
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
            EV_L0_NEGR::VALUE1 => false,
            EV_L0_NEGR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EV_L0_NEGR {
        match value {
            false => EV_L0_NEGR::VALUE1,
            true => EV_L0_NEGR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EV_L0_NEGR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EV_L0_NEGR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct L0_PINR {
    bits: bool,
}
impl L0_PINR {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Event Latch0 positive edge"]
    #[inline]
    pub fn ev_l0_pos(&self) -> EV_L0_POSR {
        EV_L0_POSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Event Latch0 negative edge"]
    #[inline]
    pub fn ev_l0_neg(&self) -> EV_L0_NEGR {
        EV_L0_NEGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - Latch0 pin state"]
    #[inline]
    pub fn l0_pin(&self) -> L0_PINR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        L0_PINR { bits }
    }
}
