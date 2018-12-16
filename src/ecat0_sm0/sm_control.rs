#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::SM_CONTROL {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `OP_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OP_MODER {
    #[doc = "Buffered (3 buffer mode)"]
    VALUE1,
    #[doc = "Mailbox (Single buffer mode)"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OP_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OP_MODER::VALUE1 => 0,
            OP_MODER::VALUE3 => 2,
            OP_MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OP_MODER {
        match value {
            0 => OP_MODER::VALUE1,
            2 => OP_MODER::VALUE3,
            i => OP_MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == OP_MODER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == OP_MODER::VALUE3
    }
}
#[doc = "Possible values of the field `DIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRR {
    #[doc = "Read: ECAT read access, PDI write access"]
    VALUE1,
    #[doc = "Write: ECAT write access, PDI read access"]
    VALUE2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DIRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DIRR::VALUE1 => 0,
            DIRR::VALUE2 => 1,
            DIRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DIRR {
        match value {
            0 => DIRR::VALUE1,
            1 => DIRR::VALUE2,
            i => DIRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DIRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DIRR::VALUE2
    }
}
#[doc = "Possible values of the field `INT_ECAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT_ECATR {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl INT_ECATR {
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
            INT_ECATR::VALUE1 => false,
            INT_ECATR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT_ECATR {
        match value {
            false => INT_ECATR::VALUE1,
            true => INT_ECATR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == INT_ECATR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == INT_ECATR::VALUE2
    }
}
#[doc = "Possible values of the field `INT_PDI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT_PDIR {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl INT_PDIR {
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
            INT_PDIR::VALUE1 => false,
            INT_PDIR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT_PDIR {
        match value {
            false => INT_PDIR::VALUE1,
            true => INT_PDIR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == INT_PDIR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == INT_PDIR::VALUE2
    }
}
#[doc = "Possible values of the field `WD_TRG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WD_TRGR {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl WD_TRGR {
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
            WD_TRGR::VALUE1 => false,
            WD_TRGR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WD_TRGR {
        match value {
            false => WD_TRGR::VALUE1,
            true => WD_TRGR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WD_TRGR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WD_TRGR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:1 - Operation Mode"]
    #[inline]
    pub fn op_mode(&self) -> OP_MODER {
        OP_MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 2:3 - Direction"]
    #[inline]
    pub fn dir(&self) -> DIRR {
        DIRR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 4 - Interrupt in ECAT Event Request Register"]
    #[inline]
    pub fn int_ecat(&self) -> INT_ECATR {
        INT_ECATR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - Interrupt in PDI Event Request Register"]
    #[inline]
    pub fn int_pdi(&self) -> INT_PDIR {
        INT_PDIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - Watchdog Trigger Enable"]
    #[inline]
    pub fn wd_trg(&self) -> WD_TRGR {
        WD_TRGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
}
