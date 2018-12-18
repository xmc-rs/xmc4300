#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::SM_STATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `INT_W`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT_WR {
    #[doc = "Interrupt cleared after first byte of buffer was read"]
    VALUE1,
    #[doc = "Interrupt after buffer was completely and successfully written"]
    VALUE2,
}
impl INT_WR {
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
            INT_WR::VALUE1 => false,
            INT_WR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT_WR {
        match value {
            false => INT_WR::VALUE1,
            true => INT_WR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == INT_WR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == INT_WR::VALUE2
    }
}
#[doc = "Possible values of the field `INT_R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT_RR {
    #[doc = "Interrupt cleared after first byte of buffer was written"]
    VALUE1,
    #[doc = "Interrupt after buffer was completely and successful read"]
    VALUE2,
}
impl INT_RR {
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
            INT_RR::VALUE1 => false,
            INT_RR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT_RR {
        match value {
            false => INT_RR::VALUE1,
            true => INT_RR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == INT_RR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == INT_RR::VALUE2
    }
}
#[doc = "Possible values of the field `MB_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MB_STATUSR {
    #[doc = "Mailbox empty"]
    VALUE1,
    #[doc = "Mailbox full"]
    VALUE2,
}
impl MB_STATUSR {
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
            MB_STATUSR::VALUE1 => false,
            MB_STATUSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MB_STATUSR {
        match value {
            false => MB_STATUSR::VALUE1,
            true => MB_STATUSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MB_STATUSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MB_STATUSR::VALUE2
    }
}
#[doc = "Possible values of the field `BUF_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF_STATUSR {
    #[doc = "1. buffer"]
    VALUE1,
    #[doc = "2. buffer"]
    VALUE2,
    #[doc = "3. buffer"]
    VALUE3,
    #[doc = "(no buffer written)"]
    VALUE4,
}
impl BUF_STATUSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BUF_STATUSR::VALUE1 => 0,
            BUF_STATUSR::VALUE2 => 1,
            BUF_STATUSR::VALUE3 => 2,
            BUF_STATUSR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BUF_STATUSR {
        match value {
            0 => BUF_STATUSR::VALUE1,
            1 => BUF_STATUSR::VALUE2,
            2 => BUF_STATUSR::VALUE3,
            3 => BUF_STATUSR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BUF_STATUSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BUF_STATUSR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == BUF_STATUSR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == BUF_STATUSR::VALUE4
    }
}
#[doc = "Possible values of the field `R_BUF_IU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum R_BUF_IUR {
    #[doc = "buffer not in use"]
    VALUE1,
    #[doc = "buffer in use"]
    VALUE2,
}
impl R_BUF_IUR {
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
            R_BUF_IUR::VALUE1 => false,
            R_BUF_IUR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> R_BUF_IUR {
        match value {
            false => R_BUF_IUR::VALUE1,
            true => R_BUF_IUR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == R_BUF_IUR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == R_BUF_IUR::VALUE2
    }
}
#[doc = "Possible values of the field `W_BUF_IU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum W_BUF_IUR {
    #[doc = "buffer not in use"]
    VALUE1,
    #[doc = "buffer in use"]
    VALUE2,
}
impl W_BUF_IUR {
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
            W_BUF_IUR::VALUE1 => false,
            W_BUF_IUR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> W_BUF_IUR {
        match value {
            false => W_BUF_IUR::VALUE1,
            true => W_BUF_IUR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == W_BUF_IUR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == W_BUF_IUR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Interrupt Write"]
    #[inline]
    pub fn int_w(&self) -> INT_WR {
        INT_WR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Interrupt Read"]
    #[inline]
    pub fn int_r(&self) -> INT_RR {
        INT_RR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - Mailbox mode: mailbox status"]
    #[inline]
    pub fn mb_status(&self) -> MB_STATUSR {
        MB_STATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bits 4:5 - Buffered mode: buffer status (last written buffer)"]
    #[inline]
    pub fn buf_status(&self) -> BUF_STATUSR {
        BUF_STATUSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 6 - Read buffer in use (opened)"]
    #[inline]
    pub fn r_buf_iu(&self) -> R_BUF_IUR {
        R_BUF_IUR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - Write buffer in use (opened)"]
    #[inline]
    pub fn w_buf_iu(&self) -> W_BUF_IUR {
        W_BUF_IUR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
}
