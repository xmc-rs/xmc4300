#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CLKMXSTAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `SYSCLKMUX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCLKMUXR {
    #[doc = "fOFI clock active"]
    CONST_X1,
    #[doc = "fPLL clock active"]
    CONST_1X,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SYSCLKMUXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SYSCLKMUXR::CONST_X1 => 1,
            SYSCLKMUXR::CONST_1X => 2,
            SYSCLKMUXR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SYSCLKMUXR {
        match value {
            1 => SYSCLKMUXR::CONST_X1,
            2 => SYSCLKMUXR::CONST_1X,
            i => SYSCLKMUXR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_X1`"]
    #[inline]
    pub fn is_const_x1(&self) -> bool {
        *self == SYSCLKMUXR::CONST_X1
    }
    #[doc = "Checks if the value of the field is `CONST_1X`"]
    #[inline]
    pub fn is_const_1x(&self) -> bool {
        *self == SYSCLKMUXR::CONST_1X
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Status of System Clock Multiplexing Upon Source Switching"]
    #[inline]
    pub fn sysclkmux(&self) -> SYSCLKMUXR {
        SYSCLKMUXR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
