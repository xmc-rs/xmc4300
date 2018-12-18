#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::DC_CYC_CONT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `SYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCR {
    #[doc = "ECAT controlled"]
    VALUE1,
    #[doc = "PDI controlled"]
    VALUE2,
}
impl SYNCR {
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
            SYNCR::VALUE1 => false,
            SYNCR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYNCR {
        match value {
            false => SYNCR::VALUE1,
            true => SYNCR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SYNCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SYNCR::VALUE2
    }
}
#[doc = "Possible values of the field `LATCH_U0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LATCH_U0R {
    #[doc = "ECAT controlled"]
    VALUE1,
    #[doc = "PDI controlled"]
    VALUE2,
}
impl LATCH_U0R {
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
            LATCH_U0R::VALUE1 => false,
            LATCH_U0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LATCH_U0R {
        match value {
            false => LATCH_U0R::VALUE1,
            true => LATCH_U0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LATCH_U0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LATCH_U0R::VALUE2
    }
}
#[doc = "Possible values of the field `LATCH_U1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LATCH_U1R {
    #[doc = "ECAT controlled"]
    VALUE1,
    #[doc = "PDI controlled"]
    VALUE2,
}
impl LATCH_U1R {
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
            LATCH_U1R::VALUE1 => false,
            LATCH_U1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LATCH_U1R {
        match value {
            false => LATCH_U1R::VALUE1,
            true => LATCH_U1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LATCH_U1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LATCH_U1R::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - SYNC out unit control"]
    #[inline]
    pub fn sync(&self) -> SYNCR {
        SYNCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 4 - Latch In unit 0"]
    #[inline]
    pub fn latch_u0(&self) -> LATCH_U0R {
        LATCH_U0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - Latch In unit 1"]
    #[inline]
    pub fn latch_u1(&self) -> LATCH_U1R {
        LATCH_U1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
}
