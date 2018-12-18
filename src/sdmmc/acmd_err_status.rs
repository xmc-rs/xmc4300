#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
impl super::ACMD_ERR_STATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `CMD_NOT_ISSUED_BY_ACMD12_ERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_NOT_ISSUED_BY_ACMD12_ERRR {
    #[doc = "No Error"]
    VALUE1,
    #[doc = "Not Issued"]
    VALUE2,
}
impl CMD_NOT_ISSUED_BY_ACMD12_ERRR {
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
            CMD_NOT_ISSUED_BY_ACMD12_ERRR::VALUE1 => false,
            CMD_NOT_ISSUED_BY_ACMD12_ERRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMD_NOT_ISSUED_BY_ACMD12_ERRR {
        match value {
            false => CMD_NOT_ISSUED_BY_ACMD12_ERRR::VALUE1,
            true => CMD_NOT_ISSUED_BY_ACMD12_ERRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CMD_NOT_ISSUED_BY_ACMD12_ERRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CMD_NOT_ISSUED_BY_ACMD12_ERRR::VALUE2
    }
}
#[doc = "Possible values of the field `ACMD_IND_ERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMD_IND_ERRR {
    #[doc = "No Error"]
    VALUE1,
    #[doc = "Error"]
    VALUE2,
}
impl ACMD_IND_ERRR {
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
            ACMD_IND_ERRR::VALUE1 => false,
            ACMD_IND_ERRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMD_IND_ERRR {
        match value {
            false => ACMD_IND_ERRR::VALUE1,
            true => ACMD_IND_ERRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ACMD_IND_ERRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ACMD_IND_ERRR::VALUE2
    }
}
#[doc = "Possible values of the field `ACMD_END_BIT_ERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMD_END_BIT_ERRR {
    #[doc = "No Error"]
    VALUE1,
    #[doc = "End Bit Error Generated"]
    VALUE2,
}
impl ACMD_END_BIT_ERRR {
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
            ACMD_END_BIT_ERRR::VALUE1 => false,
            ACMD_END_BIT_ERRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMD_END_BIT_ERRR {
        match value {
            false => ACMD_END_BIT_ERRR::VALUE1,
            true => ACMD_END_BIT_ERRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ACMD_END_BIT_ERRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ACMD_END_BIT_ERRR::VALUE2
    }
}
#[doc = "Possible values of the field `ACMD_CRC_ERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMD_CRC_ERRR {
    #[doc = "No Error"]
    VALUE1,
    #[doc = "CRC Error Generated"]
    VALUE2,
}
impl ACMD_CRC_ERRR {
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
            ACMD_CRC_ERRR::VALUE1 => false,
            ACMD_CRC_ERRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMD_CRC_ERRR {
        match value {
            false => ACMD_CRC_ERRR::VALUE1,
            true => ACMD_CRC_ERRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ACMD_CRC_ERRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ACMD_CRC_ERRR::VALUE2
    }
}
#[doc = "Possible values of the field `ACMD_TIMEOUT_ERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMD_TIMEOUT_ERRR {
    #[doc = "No Error"]
    VALUE1,
    #[doc = "Timeout"]
    VALUE2,
}
impl ACMD_TIMEOUT_ERRR {
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
            ACMD_TIMEOUT_ERRR::VALUE1 => false,
            ACMD_TIMEOUT_ERRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMD_TIMEOUT_ERRR {
        match value {
            false => ACMD_TIMEOUT_ERRR::VALUE1,
            true => ACMD_TIMEOUT_ERRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ACMD_TIMEOUT_ERRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ACMD_TIMEOUT_ERRR::VALUE2
    }
}
#[doc = "Possible values of the field `ACMD12_NOT_EXEC_ERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMD12_NOT_EXEC_ERRR {
    #[doc = "Executed"]
    VALUE1,
    #[doc = "Not Executed"]
    VALUE2,
}
impl ACMD12_NOT_EXEC_ERRR {
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
            ACMD12_NOT_EXEC_ERRR::VALUE1 => false,
            ACMD12_NOT_EXEC_ERRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMD12_NOT_EXEC_ERRR {
        match value {
            false => ACMD12_NOT_EXEC_ERRR::VALUE1,
            true => ACMD12_NOT_EXEC_ERRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ACMD12_NOT_EXEC_ERRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ACMD12_NOT_EXEC_ERRR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 7 - Command Not Issued By Auto CMD12 Error"]
    #[inline]
    pub fn cmd_not_issued_by_acmd12_err(&self) -> CMD_NOT_ISSUED_BY_ACMD12_ERRR {
        CMD_NOT_ISSUED_BY_ACMD12_ERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - Auto CMD Index Error"]
    #[inline]
    pub fn acmd_ind_err(&self) -> ACMD_IND_ERRR {
        ACMD_IND_ERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 3 - Auto CMD End Bit Error"]
    #[inline]
    pub fn acmd_end_bit_err(&self) -> ACMD_END_BIT_ERRR {
        ACMD_END_BIT_ERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 2 - Auto CMD CRC Error"]
    #[inline]
    pub fn acmd_crc_err(&self) -> ACMD_CRC_ERRR {
        ACMD_CRC_ERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - Auto CMD Timeout Error"]
    #[inline]
    pub fn acmd_timeout_err(&self) -> ACMD_TIMEOUT_ERRR {
        ACMD_TIMEOUT_ERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 0 - Auto CMD12 Not Executed"]
    #[inline]
    pub fn acmd12_not_exec_err(&self) -> ACMD12_NOT_EXEC_ERRR {
        ACMD12_NOT_EXEC_ERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
}
