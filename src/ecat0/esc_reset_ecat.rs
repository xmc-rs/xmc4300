#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::ESC_RESET_ECAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `RESET_CMD_STATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESET_CMD_STATER {
    #[doc = "after writing 0x52"]
    VALUE1,
    #[doc = "after writing 0x45 (if 0x52 was written before)"]
    VALUE2,
    #[doc = "default"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RESET_CMD_STATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RESET_CMD_STATER::VALUE1 => 1,
            RESET_CMD_STATER::VALUE2 => 2,
            RESET_CMD_STATER::VALUE3 => 0,
            RESET_CMD_STATER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RESET_CMD_STATER {
        match value {
            1 => RESET_CMD_STATER::VALUE1,
            2 => RESET_CMD_STATER::VALUE2,
            0 => RESET_CMD_STATER::VALUE3,
            i => RESET_CMD_STATER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RESET_CMD_STATER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RESET_CMD_STATER::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == RESET_CMD_STATER::VALUE3
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:1 - Progress of the reset procedure"]
    #[inline]
    pub fn reset_cmd_state(&self) -> RESET_CMD_STATER {
        RESET_CMD_STATER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
}
