#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::PDI_CONTROL {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `PDI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIR {
    #[doc = "Interface deactivated (no PDI)"]
    VALUE1,
    #[doc = "On-chip Bus"]
    VALUE2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PDIR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PDIR::VALUE1 => 0,
            PDIR::VALUE2 => 128,
            PDIR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PDIR {
        match value {
            0 => PDIR::VALUE1,
            128 => PDIR::VALUE2,
            i => PDIR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PDIR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PDIR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:7 - On-chip bus clock"]
    #[inline]
    pub fn pdi(&self) -> PDIR {
        PDIR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
}
