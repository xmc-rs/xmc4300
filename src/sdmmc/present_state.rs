#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PRESENT_STATE {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct DAT_7_4_PIN_LEVELR {
    bits: u8,
}
impl DAT_7_4_PIN_LEVELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CMD_LINE_LEVELR {
    bits: bool,
}
impl CMD_LINE_LEVELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct DAT_3_0_PIN_LEVELR {
    bits: u8,
}
impl DAT_3_0_PIN_LEVELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `WRITE_PROTECT_PIN_LEVEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRITE_PROTECT_PIN_LEVELR {
    #[doc = "Write protected (SDWP = 1)"]
    VALUE1,
    #[doc = "Write enabled (SDWP = 0)"]
    VALUE2,
}
impl WRITE_PROTECT_PIN_LEVELR {
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
            WRITE_PROTECT_PIN_LEVELR::VALUE1 => false,
            WRITE_PROTECT_PIN_LEVELR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WRITE_PROTECT_PIN_LEVELR {
        match value {
            false => WRITE_PROTECT_PIN_LEVELR::VALUE1,
            true => WRITE_PROTECT_PIN_LEVELR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WRITE_PROTECT_PIN_LEVELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WRITE_PROTECT_PIN_LEVELR::VALUE2
    }
}
#[doc = "Possible values of the field `CARD_DETECT_PIN_LEVEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARD_DETECT_PIN_LEVELR {
    #[doc = "No Card present (SDCD = 1)"]
    VALUE1,
    #[doc = "Card present (SDCD = 0)"]
    VALUE2,
}
impl CARD_DETECT_PIN_LEVELR {
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
            CARD_DETECT_PIN_LEVELR::VALUE1 => false,
            CARD_DETECT_PIN_LEVELR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CARD_DETECT_PIN_LEVELR {
        match value {
            false => CARD_DETECT_PIN_LEVELR::VALUE1,
            true => CARD_DETECT_PIN_LEVELR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CARD_DETECT_PIN_LEVELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CARD_DETECT_PIN_LEVELR::VALUE2
    }
}
#[doc = "Possible values of the field `CARD_STATE_STABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARD_STATE_STABLER {
    #[doc = "Reset of Debouncing"]
    VALUE1,
    #[doc = "No Card or Inserted"]
    VALUE2,
}
impl CARD_STATE_STABLER {
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
            CARD_STATE_STABLER::VALUE1 => false,
            CARD_STATE_STABLER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CARD_STATE_STABLER {
        match value {
            false => CARD_STATE_STABLER::VALUE1,
            true => CARD_STATE_STABLER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CARD_STATE_STABLER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CARD_STATE_STABLER::VALUE2
    }
}
#[doc = "Possible values of the field `CARD_INSERTED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARD_INSERTEDR {
    #[doc = "Reset or Debouncing or No Card"]
    VALUE1,
    #[doc = "Card Inserted"]
    VALUE2,
}
impl CARD_INSERTEDR {
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
            CARD_INSERTEDR::VALUE1 => false,
            CARD_INSERTEDR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CARD_INSERTEDR {
        match value {
            false => CARD_INSERTEDR::VALUE1,
            true => CARD_INSERTEDR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CARD_INSERTEDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CARD_INSERTEDR::VALUE2
    }
}
#[doc = "Possible values of the field `BUFFER_READ_ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFFER_READ_ENABLER {
    #[doc = "Read Disable"]
    VALUE1,
    #[doc = "Read Enable."]
    VALUE2,
}
impl BUFFER_READ_ENABLER {
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
            BUFFER_READ_ENABLER::VALUE1 => false,
            BUFFER_READ_ENABLER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUFFER_READ_ENABLER {
        match value {
            false => BUFFER_READ_ENABLER::VALUE1,
            true => BUFFER_READ_ENABLER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BUFFER_READ_ENABLER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BUFFER_READ_ENABLER::VALUE2
    }
}
#[doc = "Possible values of the field `BUFFER_WRITE_ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFFER_WRITE_ENABLER {
    #[doc = "Write Disable"]
    VALUE1,
    #[doc = "Write Enable."]
    VALUE2,
}
impl BUFFER_WRITE_ENABLER {
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
            BUFFER_WRITE_ENABLER::VALUE1 => false,
            BUFFER_WRITE_ENABLER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUFFER_WRITE_ENABLER {
        match value {
            false => BUFFER_WRITE_ENABLER::VALUE1,
            true => BUFFER_WRITE_ENABLER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BUFFER_WRITE_ENABLER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BUFFER_WRITE_ENABLER::VALUE2
    }
}
#[doc = "Possible values of the field `READ_TRANSFER_ACTIVE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READ_TRANSFER_ACTIVER {
    #[doc = "No valid data"]
    VALUE1,
    #[doc = "Transferring data"]
    VALUE2,
}
impl READ_TRANSFER_ACTIVER {
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
            READ_TRANSFER_ACTIVER::VALUE1 => false,
            READ_TRANSFER_ACTIVER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> READ_TRANSFER_ACTIVER {
        match value {
            false => READ_TRANSFER_ACTIVER::VALUE1,
            true => READ_TRANSFER_ACTIVER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == READ_TRANSFER_ACTIVER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == READ_TRANSFER_ACTIVER::VALUE2
    }
}
#[doc = "Possible values of the field `WRITE_TRANSFER_ACTIVE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRITE_TRANSFER_ACTIVER {
    #[doc = "No valid data"]
    VALUE1,
    #[doc = "Transferring data"]
    VALUE2,
}
impl WRITE_TRANSFER_ACTIVER {
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
            WRITE_TRANSFER_ACTIVER::VALUE1 => false,
            WRITE_TRANSFER_ACTIVER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WRITE_TRANSFER_ACTIVER {
        match value {
            false => WRITE_TRANSFER_ACTIVER::VALUE1,
            true => WRITE_TRANSFER_ACTIVER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WRITE_TRANSFER_ACTIVER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WRITE_TRANSFER_ACTIVER::VALUE2
    }
}
#[doc = "Possible values of the field `DAT_LINE_ACTIVE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAT_LINE_ACTIVER {
    #[doc = "DAT line inactive"]
    VALUE1,
    #[doc = "DAT line active"]
    VALUE2,
}
impl DAT_LINE_ACTIVER {
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
            DAT_LINE_ACTIVER::VALUE1 => false,
            DAT_LINE_ACTIVER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DAT_LINE_ACTIVER {
        match value {
            false => DAT_LINE_ACTIVER::VALUE1,
            true => DAT_LINE_ACTIVER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DAT_LINE_ACTIVER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DAT_LINE_ACTIVER::VALUE2
    }
}
#[doc = "Possible values of the field `COMMAND_INHIBIT_DAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMMAND_INHIBIT_DATR {
    #[doc = "Can issue command which uses the DAT line"]
    VALUE1,
    #[doc = "Cannot issue command which uses the DAT line"]
    VALUE2,
}
impl COMMAND_INHIBIT_DATR {
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
            COMMAND_INHIBIT_DATR::VALUE1 => false,
            COMMAND_INHIBIT_DATR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COMMAND_INHIBIT_DATR {
        match value {
            false => COMMAND_INHIBIT_DATR::VALUE1,
            true => COMMAND_INHIBIT_DATR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == COMMAND_INHIBIT_DATR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == COMMAND_INHIBIT_DATR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct COMMAND_INHIBIT_CMDR {
    bits: bool,
}
impl COMMAND_INHIBIT_CMDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 25:28 - Line Signal Level"]
    #[inline]
    pub fn dat_7_4_pin_level(&self) -> DAT_7_4_PIN_LEVELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DAT_7_4_PIN_LEVELR { bits }
    }
    #[doc = "Bit 24 - CMD Line Signal Level"]
    #[inline]
    pub fn cmd_line_level(&self) -> CMD_LINE_LEVELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CMD_LINE_LEVELR { bits }
    }
    #[doc = "Bits 20:23 - Line Signal Level"]
    #[inline]
    pub fn dat_3_0_pin_level(&self) -> DAT_3_0_PIN_LEVELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DAT_3_0_PIN_LEVELR { bits }
    }
    #[doc = "Bit 19 - Write Protect Switch Pin Level"]
    #[inline]
    pub fn write_protect_pin_level(&self) -> WRITE_PROTECT_PIN_LEVELR {
        WRITE_PROTECT_PIN_LEVELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Card Detect Pin Level"]
    #[inline]
    pub fn card_detect_pin_level(&self) -> CARD_DETECT_PIN_LEVELR {
        CARD_DETECT_PIN_LEVELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Card State Stable"]
    #[inline]
    pub fn card_state_stable(&self) -> CARD_STATE_STABLER {
        CARD_STATE_STABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Card Inserted"]
    #[inline]
    pub fn card_inserted(&self) -> CARD_INSERTEDR {
        CARD_INSERTEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Buffer Read Enable"]
    #[inline]
    pub fn buffer_read_enable(&self) -> BUFFER_READ_ENABLER {
        BUFFER_READ_ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Buffer Write Enable"]
    #[inline]
    pub fn buffer_write_enable(&self) -> BUFFER_WRITE_ENABLER {
        BUFFER_WRITE_ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Read Transfer Active"]
    #[inline]
    pub fn read_transfer_active(&self) -> READ_TRANSFER_ACTIVER {
        READ_TRANSFER_ACTIVER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Write Transfer Active"]
    #[inline]
    pub fn write_transfer_active(&self) -> WRITE_TRANSFER_ACTIVER {
        WRITE_TRANSFER_ACTIVER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - DAT Line Active"]
    #[inline]
    pub fn dat_line_active(&self) -> DAT_LINE_ACTIVER {
        DAT_LINE_ACTIVER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Command Inhibit (DAT)"]
    #[inline]
    pub fn command_inhibit_dat(&self) -> COMMAND_INHIBIT_DATR {
        COMMAND_INHIBIT_DATR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Command Inhibit (CMD)"]
    #[inline]
    pub fn command_inhibit_cmd(&self) -> COMMAND_INHIBIT_CMDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        COMMAND_INHIBIT_CMDR { bits }
    }
}
