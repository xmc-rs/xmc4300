#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MOSTAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `RXPND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXPNDR {
    #[doc = "No CAN message has been received."]
    VALUE1,
    #[doc = "A CAN message has been received by the message object n, either directly or via gateway copy action."]
    VALUE2,
}
impl RXPNDR {
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
            RXPNDR::VALUE1 => false,
            RXPNDR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXPNDR {
        match value {
            false => RXPNDR::VALUE1,
            true => RXPNDR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RXPNDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RXPNDR::VALUE2
    }
}
#[doc = "Possible values of the field `TXPND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXPNDR {
    #[doc = "No CAN message has been transmitted."]
    VALUE1,
    #[doc = "A CAN message from message object n has been transmitted successfully over the CAN bus."]
    VALUE2,
}
impl TXPNDR {
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
            TXPNDR::VALUE1 => false,
            TXPNDR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXPNDR {
        match value {
            false => TXPNDR::VALUE1,
            true => TXPNDR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TXPNDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TXPNDR::VALUE2
    }
}
#[doc = "Possible values of the field `RXUPD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXUPDR {
    #[doc = "No receive update ongoing."]
    VALUE1,
    #[doc = "Message identifier, DLC, and data of the message object are currently updated."]
    VALUE2,
}
impl RXUPDR {
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
            RXUPDR::VALUE1 => false,
            RXUPDR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXUPDR {
        match value {
            false => RXUPDR::VALUE1,
            true => RXUPDR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RXUPDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RXUPDR::VALUE2
    }
}
#[doc = "Possible values of the field `NEWDAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NEWDATR {
    #[doc = "No update of the message object n since last flag reset."]
    VALUE1,
    #[doc = "Message object n has been updated."]
    VALUE2,
}
impl NEWDATR {
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
            NEWDATR::VALUE1 => false,
            NEWDATR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NEWDATR {
        match value {
            false => NEWDATR::VALUE1,
            true => NEWDATR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == NEWDATR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == NEWDATR::VALUE2
    }
}
#[doc = "Possible values of the field `MSGLST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSGLSTR {
    #[doc = "No CAN message is lost."]
    VALUE1,
    #[doc = "A CAN message is lost because NEWDAT has become set again when it has already been set."]
    VALUE2,
}
impl MSGLSTR {
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
            MSGLSTR::VALUE1 => false,
            MSGLSTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSGLSTR {
        match value {
            false => MSGLSTR::VALUE1,
            true => MSGLSTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MSGLSTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MSGLSTR::VALUE2
    }
}
#[doc = "Possible values of the field `MSGVAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSGVALR {
    #[doc = "Message object n is not valid."]
    VALUE1,
    #[doc = "Message object n is valid."]
    VALUE2,
}
impl MSGVALR {
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
            MSGVALR::VALUE1 => false,
            MSGVALR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSGVALR {
        match value {
            false => MSGVALR::VALUE1,
            true => MSGVALR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MSGVALR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MSGVALR::VALUE2
    }
}
#[doc = "Possible values of the field `RTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTSELR {
    #[doc = "Message object n is not selected for receive or transmit operation."]
    VALUE1,
    #[doc = "Message object n is selected for receive or transmit operation."]
    VALUE2,
}
impl RTSELR {
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
            RTSELR::VALUE1 => false,
            RTSELR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTSELR {
        match value {
            false => RTSELR::VALUE1,
            true => RTSELR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RTSELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RTSELR::VALUE2
    }
}
#[doc = "Possible values of the field `RXEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXENR {
    #[doc = "Message object n is not enabled for frame reception."]
    VALUE1,
    #[doc = "Message object n is enabled for frame reception."]
    VALUE2,
}
impl RXENR {
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
            RXENR::VALUE1 => false,
            RXENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXENR {
        match value {
            false => RXENR::VALUE1,
            true => RXENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RXENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RXENR::VALUE2
    }
}
#[doc = "Possible values of the field `TXRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRQR {
    #[doc = "No transmission of message object n is requested."]
    VALUE1,
    #[doc = "Transmission of message object n on the CAN bus is requested."]
    VALUE2,
}
impl TXRQR {
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
            TXRQR::VALUE1 => false,
            TXRQR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXRQR {
        match value {
            false => TXRQR::VALUE1,
            true => TXRQR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TXRQR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TXRQR::VALUE2
    }
}
#[doc = "Possible values of the field `TXEN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEN0R {
    #[doc = "Message object n is not enabled for frame transmission."]
    VALUE1,
    #[doc = "Message object n is enabled for frame transmission."]
    VALUE2,
}
impl TXEN0R {
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
            TXEN0R::VALUE1 => false,
            TXEN0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXEN0R {
        match value {
            false => TXEN0R::VALUE1,
            true => TXEN0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TXEN0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TXEN0R::VALUE2
    }
}
#[doc = "Possible values of the field `TXEN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEN1R {
    #[doc = "Message object n is not enabled for frame transmission."]
    VALUE1,
    #[doc = "Message object n is enabled for frame transmission."]
    VALUE2,
}
impl TXEN1R {
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
            TXEN1R::VALUE1 => false,
            TXEN1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXEN1R {
        match value {
            false => TXEN1R::VALUE1,
            true => TXEN1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TXEN1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TXEN1R::VALUE2
    }
}
#[doc = "Possible values of the field `DIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRR {
    #[doc = "Receive Object selected: With TXRQ = 1, a Remote Frame with the identifier of message object n is scheduled for transmission. On reception of a Data Frame with matching identifier, the message is stored in message object n."]
    VALUE1,
    #[doc = "Transmit Object selected: If TXRQ = 1, message object n is scheduled for transmission of a Data Frame. On reception of a Remote Frame with matching identifier, bit TXRQ is set."]
    VALUE2,
}
impl DIRR {
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
            DIRR::VALUE1 => false,
            DIRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIRR {
        match value {
            false => DIRR::VALUE1,
            true => DIRR::VALUE2,
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
#[doc = r" Value of the field"]
pub struct LISTR {
    bits: u8,
}
impl LISTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PPREVR {
    bits: u8,
}
impl PPREVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PNEXTR {
    bits: u8,
}
impl PNEXTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Receive Pending"]
    #[inline]
    pub fn rxpnd(&self) -> RXPNDR {
        RXPNDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Transmit Pending"]
    #[inline]
    pub fn txpnd(&self) -> TXPNDR {
        TXPNDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Receive Updating"]
    #[inline]
    pub fn rxupd(&self) -> RXUPDR {
        RXUPDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - New Data"]
    #[inline]
    pub fn newdat(&self) -> NEWDATR {
        NEWDATR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Message Lost"]
    #[inline]
    pub fn msglst(&self) -> MSGLSTR {
        MSGLSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Message Valid"]
    #[inline]
    pub fn msgval(&self) -> MSGVALR {
        MSGVALR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Receive/Transmit Selected"]
    #[inline]
    pub fn rtsel(&self) -> RTSELR {
        RTSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Receive Enable"]
    #[inline]
    pub fn rxen(&self) -> RXENR {
        RXENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Transmit Request"]
    #[inline]
    pub fn txrq(&self) -> TXRQR {
        TXRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Transmit Enable 0"]
    #[inline]
    pub fn txen0(&self) -> TXEN0R {
        TXEN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Transmit Enable 1"]
    #[inline]
    pub fn txen1(&self) -> TXEN1R {
        TXEN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Message Direction"]
    #[inline]
    pub fn dir(&self) -> DIRR {
        DIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 12:15 - List Allocation"]
    #[inline]
    pub fn list(&self) -> LISTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LISTR { bits }
    }
    #[doc = "Bits 16:23 - Pointer to Previous Message Object"]
    #[inline]
    pub fn pprev(&self) -> PPREVR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PPREVR { bits }
    }
    #[doc = "Bits 24:31 - Pointer to Next Message Object"]
    #[inline]
    pub fn pnext(&self) -> PNEXTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PNEXTR { bits }
    }
}
