#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::WR_REG_PROTECT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `WR_REG_P`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WR_REG_PR {
    #[doc = "Protection disabled"]
    VALUE1,
    #[doc = "Protection enabled"]
    VALUE2,
}
impl WR_REG_PR {
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
            WR_REG_PR::VALUE1 => false,
            WR_REG_PR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WR_REG_PR {
        match value {
            false => WR_REG_PR::VALUE1,
            true => WR_REG_PR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WR_REG_PR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WR_REG_PR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Write register protection"]
    #[inline]
    pub fn wr_reg_p(&self) -> WR_REG_PR {
        WR_REG_PR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
}
