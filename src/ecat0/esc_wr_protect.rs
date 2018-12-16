#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::ESC_WR_PROTECT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `ESC_WR_PROT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESC_WR_PROTR {
    #[doc = "Protection disabled"]
    VALUE1,
    #[doc = "Protection enabled"]
    VALUE2,
}
impl ESC_WR_PROTR {
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
            ESC_WR_PROTR::VALUE1 => false,
            ESC_WR_PROTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ESC_WR_PROTR {
        match value {
            false => ESC_WR_PROTR::VALUE1,
            true => ESC_WR_PROTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ESC_WR_PROTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ESC_WR_PROTR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Write protect"]
    #[inline]
    pub fn esc_wr_prot(&self) -> ESC_WR_PROTR {
        ESC_WR_PROTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
}
