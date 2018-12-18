#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
impl super::DC_PULSE_LEN {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `PULS_LENGTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PULS_LENGTHR {
    #[doc = "Acknowledge mode: SyncSignal will be cleared by reading SYNC\\[1:0\\] Status register"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl PULS_LENGTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            PULS_LENGTHR::VALUE1 => 0,
            PULS_LENGTHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> PULS_LENGTHR {
        match value {
            0 => PULS_LENGTHR::VALUE1,
            i => PULS_LENGTHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PULS_LENGTHR::VALUE1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:15 - Pulse length of SyncSignals"]
    #[inline]
    pub fn puls_length(&self) -> PULS_LENGTHR {
        PULS_LENGTHR::_from({
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u16
        })
    }
}
