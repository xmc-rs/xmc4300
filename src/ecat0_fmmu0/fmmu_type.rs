#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::FMMU_TYPE {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `R_ACC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum R_ACCR {
    #[doc = "Ignore mapping for read accesses"]
    VALUE1,
    #[doc = "Use mapping for read accesses"]
    VALUE2,
}
impl R_ACCR {
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
            R_ACCR::VALUE1 => false,
            R_ACCR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> R_ACCR {
        match value {
            false => R_ACCR::VALUE1,
            true => R_ACCR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == R_ACCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == R_ACCR::VALUE2
    }
}
#[doc = "Possible values of the field `W_ACC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum W_ACCR {
    #[doc = "Ignore mapping for write accesses"]
    VALUE1,
    #[doc = "Use mapping for write accesses"]
    VALUE2,
}
impl W_ACCR {
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
            W_ACCR::VALUE1 => false,
            W_ACCR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> W_ACCR {
        match value {
            false => W_ACCR::VALUE1,
            true => W_ACCR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == W_ACCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == W_ACCR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Read Access"]
    #[inline]
    pub fn r_acc(&self) -> R_ACCR {
        R_ACCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Write Access"]
    #[inline]
    pub fn w_acc(&self) -> W_ACCR {
        W_ACCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
}
