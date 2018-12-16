#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PSR_ASCMODE {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `TXIDLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXIDLER {
    #[doc = "The transmitter line has not yet been idle."]
    VALUE1,
    #[doc = "The transmitter line has been idle and frame transmission is possible."]
    VALUE2,
}
impl TXIDLER {
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
            TXIDLER::VALUE1 => false,
            TXIDLER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXIDLER {
        match value {
            false => TXIDLER::VALUE1,
            true => TXIDLER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TXIDLER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TXIDLER::VALUE2
    }
}
#[doc = "Possible values of the field `RXIDLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXIDLER {
    #[doc = "The receiver line has not yet been idle."]
    VALUE1,
    #[doc = "The receiver line has been idle and frame reception is possible."]
    VALUE2,
}
impl RXIDLER {
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
            RXIDLER::VALUE1 => false,
            RXIDLER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXIDLER {
        match value {
            false => RXIDLER::VALUE1,
            true => RXIDLER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RXIDLER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RXIDLER::VALUE2
    }
}
#[doc = "Possible values of the field `SBD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBDR {
    #[doc = "A synchronization break has not yet been detected."]
    VALUE1,
    #[doc = "A synchronization break has been detected."]
    VALUE2,
}
impl SBDR {
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
            SBDR::VALUE1 => false,
            SBDR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SBDR {
        match value {
            false => SBDR::VALUE1,
            true => SBDR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SBDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SBDR::VALUE2
    }
}
#[doc = "Possible values of the field `COL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COLR {
    #[doc = "A collision has not yet been detected and frame transmission is possible."]
    VALUE1,
    #[doc = "A collision has been detected and frame transmission is not possible."]
    VALUE2,
}
impl COLR {
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
            COLR::VALUE1 => false,
            COLR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COLR {
        match value {
            false => COLR::VALUE1,
            true => COLR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == COLR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == COLR::VALUE2
    }
}
#[doc = "Possible values of the field `RNS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RNSR {
    #[doc = "Receiver noise has not been detected."]
    VALUE1,
    #[doc = "Receiver noise has been detected."]
    VALUE2,
}
impl RNSR {
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
            RNSR::VALUE1 => false,
            RNSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RNSR {
        match value {
            false => RNSR::VALUE1,
            true => RNSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RNSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RNSR::VALUE2
    }
}
#[doc = "Possible values of the field `FER0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FER0R {
    #[doc = "A format error 0 has not been detected."]
    VALUE1,
    #[doc = "A format error 0 has been detected."]
    VALUE2,
}
impl FER0R {
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
            FER0R::VALUE1 => false,
            FER0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FER0R {
        match value {
            false => FER0R::VALUE1,
            true => FER0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == FER0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == FER0R::VALUE2
    }
}
#[doc = "Possible values of the field `FER1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FER1R {
    #[doc = "A format error 1 has not been detected."]
    VALUE1,
    #[doc = "A format error 1 has been detected."]
    VALUE2,
}
impl FER1R {
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
            FER1R::VALUE1 => false,
            FER1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FER1R {
        match value {
            false => FER1R::VALUE1,
            true => FER1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == FER1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == FER1R::VALUE2
    }
}
#[doc = "Possible values of the field `RFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFFR {
    #[doc = "The received frame is not yet finished."]
    VALUE1,
    #[doc = "The received frame is finished."]
    VALUE2,
}
impl RFFR {
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
            RFFR::VALUE1 => false,
            RFFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RFFR {
        match value {
            false => RFFR::VALUE1,
            true => RFFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RFFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RFFR::VALUE2
    }
}
#[doc = "Possible values of the field `TFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFFR {
    #[doc = "The transmitter frame is not yet finished."]
    VALUE1,
    #[doc = "The transmitter frame is finished."]
    VALUE2,
}
impl TFFR {
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
            TFFR::VALUE1 => false,
            TFFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TFFR {
        match value {
            false => TFFR::VALUE1,
            true => TFFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TFFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TFFR::VALUE2
    }
}
#[doc = "Possible values of the field `BUSY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSYR {
    #[doc = "A data transfer does not take place."]
    VALUE1,
    #[doc = "A data transfer currently takes place."]
    VALUE2,
}
impl BUSYR {
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
            BUSYR::VALUE1 => false,
            BUSYR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUSYR {
        match value {
            false => BUSYR::VALUE1,
            true => BUSYR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BUSYR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BUSYR::VALUE2
    }
}
#[doc = "Possible values of the field `RSIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSIFR {
    #[doc = "A receiver start event has not occurred."]
    VALUE1,
    #[doc = "A receiver start event has occurred."]
    VALUE2,
}
impl RSIFR {
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
            RSIFR::VALUE1 => false,
            RSIFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RSIFR {
        match value {
            false => RSIFR::VALUE1,
            true => RSIFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RSIFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RSIFR::VALUE2
    }
}
#[doc = "Possible values of the field `DLIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLIFR {
    #[doc = "A data lost event has not occurred."]
    VALUE1,
    #[doc = "A data lost event has occurred."]
    VALUE2,
}
impl DLIFR {
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
            DLIFR::VALUE1 => false,
            DLIFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DLIFR {
        match value {
            false => DLIFR::VALUE1,
            true => DLIFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DLIFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DLIFR::VALUE2
    }
}
#[doc = "Possible values of the field `TSIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSIFR {
    #[doc = "A transmit shift event has not occurred."]
    VALUE1,
    #[doc = "A transmit shift event has occurred."]
    VALUE2,
}
impl TSIFR {
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
            TSIFR::VALUE1 => false,
            TSIFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSIFR {
        match value {
            false => TSIFR::VALUE1,
            true => TSIFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TSIFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TSIFR::VALUE2
    }
}
#[doc = "Possible values of the field `TBIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBIFR {
    #[doc = "A transmit buffer event has not occurred."]
    VALUE1,
    #[doc = "A transmit buffer event has occurred."]
    VALUE2,
}
impl TBIFR {
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
            TBIFR::VALUE1 => false,
            TBIFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TBIFR {
        match value {
            false => TBIFR::VALUE1,
            true => TBIFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TBIFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TBIFR::VALUE2
    }
}
#[doc = "Possible values of the field `RIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RIFR {
    #[doc = "A receive event has not occurred."]
    VALUE1,
    #[doc = "A receive event has occurred."]
    VALUE2,
}
impl RIFR {
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
            RIFR::VALUE1 => false,
            RIFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RIFR {
        match value {
            false => RIFR::VALUE1,
            true => RIFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RIFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RIFR::VALUE2
    }
}
#[doc = "Possible values of the field `AIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AIFR {
    #[doc = "An alternative receive event has not occurred."]
    VALUE1,
    #[doc = "An alternative receive event has occurred."]
    VALUE2,
}
impl AIFR {
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
            AIFR::VALUE1 => false,
            AIFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AIFR {
        match value {
            false => AIFR::VALUE1,
            true => AIFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AIFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == AIFR::VALUE2
    }
}
#[doc = "Possible values of the field `BRGIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRGIFR {
    #[doc = "A baud rate generator event has not occurred."]
    VALUE1,
    #[doc = "A baud rate generator event has occurred."]
    VALUE2,
}
impl BRGIFR {
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
            BRGIFR::VALUE1 => false,
            BRGIFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BRGIFR {
        match value {
            false => BRGIFR::VALUE1,
            true => BRGIFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BRGIFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BRGIFR::VALUE2
    }
}
#[doc = "Values that can be written to the field `TXIDLE`"]
pub enum TXIDLEW {
    #[doc = "The transmitter line has not yet been idle."]
    VALUE1,
    #[doc = "The transmitter line has been idle and frame transmission is possible."]
    VALUE2,
}
impl TXIDLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXIDLEW::VALUE1 => false,
            TXIDLEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXIDLEW<'a> {
    w: &'a mut W,
}
impl<'a> _TXIDLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXIDLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The transmitter line has not yet been idle."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TXIDLEW::VALUE1)
    }
    #[doc = "The transmitter line has been idle and frame transmission is possible."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TXIDLEW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXIDLE`"]
pub enum RXIDLEW {
    #[doc = "The receiver line has not yet been idle."]
    VALUE1,
    #[doc = "The receiver line has been idle and frame reception is possible."]
    VALUE2,
}
impl RXIDLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXIDLEW::VALUE1 => false,
            RXIDLEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXIDLEW<'a> {
    w: &'a mut W,
}
impl<'a> _RXIDLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXIDLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The receiver line has not yet been idle."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXIDLEW::VALUE1)
    }
    #[doc = "The receiver line has been idle and frame reception is possible."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RXIDLEW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SBD`"]
pub enum SBDW {
    #[doc = "A synchronization break has not yet been detected."]
    VALUE1,
    #[doc = "A synchronization break has been detected."]
    VALUE2,
}
impl SBDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SBDW::VALUE1 => false,
            SBDW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SBDW<'a> {
    w: &'a mut W,
}
impl<'a> _SBDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SBDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A synchronization break has not yet been detected."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SBDW::VALUE1)
    }
    #[doc = "A synchronization break has been detected."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SBDW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `COL`"]
pub enum COLW {
    #[doc = "A collision has not yet been detected and frame transmission is possible."]
    VALUE1,
    #[doc = "A collision has been detected and frame transmission is not possible."]
    VALUE2,
}
impl COLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COLW::VALUE1 => false,
            COLW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COLW<'a> {
    w: &'a mut W,
}
impl<'a> _COLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A collision has not yet been detected and frame transmission is possible."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(COLW::VALUE1)
    }
    #[doc = "A collision has been detected and frame transmission is not possible."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(COLW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RNS`"]
pub enum RNSW {
    #[doc = "Receiver noise has not been detected."]
    VALUE1,
    #[doc = "Receiver noise has been detected."]
    VALUE2,
}
impl RNSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RNSW::VALUE1 => false,
            RNSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RNSW<'a> {
    w: &'a mut W,
}
impl<'a> _RNSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RNSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receiver noise has not been detected."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RNSW::VALUE1)
    }
    #[doc = "Receiver noise has been detected."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RNSW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FER0`"]
pub enum FER0W {
    #[doc = "A format error 0 has not been detected."]
    VALUE1,
    #[doc = "A format error 0 has been detected."]
    VALUE2,
}
impl FER0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FER0W::VALUE1 => false,
            FER0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FER0W<'a> {
    w: &'a mut W,
}
impl<'a> _FER0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FER0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A format error 0 has not been detected."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FER0W::VALUE1)
    }
    #[doc = "A format error 0 has been detected."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FER0W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FER1`"]
pub enum FER1W {
    #[doc = "A format error 1 has not been detected."]
    VALUE1,
    #[doc = "A format error 1 has been detected."]
    VALUE2,
}
impl FER1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FER1W::VALUE1 => false,
            FER1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FER1W<'a> {
    w: &'a mut W,
}
impl<'a> _FER1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FER1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A format error 1 has not been detected."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FER1W::VALUE1)
    }
    #[doc = "A format error 1 has been detected."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FER1W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RFF`"]
pub enum RFFW {
    #[doc = "The received frame is not yet finished."]
    VALUE1,
    #[doc = "The received frame is finished."]
    VALUE2,
}
impl RFFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RFFW::VALUE1 => false,
            RFFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RFFW<'a> {
    w: &'a mut W,
}
impl<'a> _RFFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RFFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The received frame is not yet finished."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RFFW::VALUE1)
    }
    #[doc = "The received frame is finished."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RFFW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TFF`"]
pub enum TFFW {
    #[doc = "The transmitter frame is not yet finished."]
    VALUE1,
    #[doc = "The transmitter frame is finished."]
    VALUE2,
}
impl TFFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TFFW::VALUE1 => false,
            TFFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TFFW<'a> {
    w: &'a mut W,
}
impl<'a> _TFFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TFFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The transmitter frame is not yet finished."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TFFW::VALUE1)
    }
    #[doc = "The transmitter frame is finished."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TFFW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RSIF`"]
pub enum RSIFW {
    #[doc = "A receiver start event has not occurred."]
    VALUE1,
    #[doc = "A receiver start event has occurred."]
    VALUE2,
}
impl RSIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSIFW::VALUE1 => false,
            RSIFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSIFW<'a> {
    w: &'a mut W,
}
impl<'a> _RSIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A receiver start event has not occurred."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RSIFW::VALUE1)
    }
    #[doc = "A receiver start event has occurred."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RSIFW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DLIF`"]
pub enum DLIFW {
    #[doc = "A data lost event has not occurred."]
    VALUE1,
    #[doc = "A data lost event has occurred."]
    VALUE2,
}
impl DLIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DLIFW::VALUE1 => false,
            DLIFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DLIFW<'a> {
    w: &'a mut W,
}
impl<'a> _DLIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DLIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A data lost event has not occurred."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DLIFW::VALUE1)
    }
    #[doc = "A data lost event has occurred."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DLIFW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TSIF`"]
pub enum TSIFW {
    #[doc = "A transmit shift event has not occurred."]
    VALUE1,
    #[doc = "A transmit shift event has occurred."]
    VALUE2,
}
impl TSIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSIFW::VALUE1 => false,
            TSIFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSIFW<'a> {
    w: &'a mut W,
}
impl<'a> _TSIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A transmit shift event has not occurred."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TSIFW::VALUE1)
    }
    #[doc = "A transmit shift event has occurred."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TSIFW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TBIF`"]
pub enum TBIFW {
    #[doc = "A transmit buffer event has not occurred."]
    VALUE1,
    #[doc = "A transmit buffer event has occurred."]
    VALUE2,
}
impl TBIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TBIFW::VALUE1 => false,
            TBIFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TBIFW<'a> {
    w: &'a mut W,
}
impl<'a> _TBIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TBIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A transmit buffer event has not occurred."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TBIFW::VALUE1)
    }
    #[doc = "A transmit buffer event has occurred."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TBIFW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RIF`"]
pub enum RIFW {
    #[doc = "A receive event has not occurred."]
    VALUE1,
    #[doc = "A receive event has occurred."]
    VALUE2,
}
impl RIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RIFW::VALUE1 => false,
            RIFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RIFW<'a> {
    w: &'a mut W,
}
impl<'a> _RIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A receive event has not occurred."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RIFW::VALUE1)
    }
    #[doc = "A receive event has occurred."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RIFW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AIF`"]
pub enum AIFW {
    #[doc = "An alternative receive event has not occurred."]
    VALUE1,
    #[doc = "An alternative receive event has occurred."]
    VALUE2,
}
impl AIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AIFW::VALUE1 => false,
            AIFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AIFW<'a> {
    w: &'a mut W,
}
impl<'a> _AIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An alternative receive event has not occurred."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(AIFW::VALUE1)
    }
    #[doc = "An alternative receive event has occurred."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(AIFW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BRGIF`"]
pub enum BRGIFW {
    #[doc = "A baud rate generator event has not occurred."]
    VALUE1,
    #[doc = "A baud rate generator event has occurred."]
    VALUE2,
}
impl BRGIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BRGIFW::VALUE1 => false,
            BRGIFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BRGIFW<'a> {
    w: &'a mut W,
}
impl<'a> _BRGIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BRGIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A baud rate generator event has not occurred."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BRGIFW::VALUE1)
    }
    #[doc = "A baud rate generator event has occurred."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BRGIFW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Transmission Idle"]
    #[inline]
    pub fn txidle(&self) -> TXIDLER {
        TXIDLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Reception Idle"]
    #[inline]
    pub fn rxidle(&self) -> RXIDLER {
        RXIDLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Synchronization Break Detected"]
    #[inline]
    pub fn sbd(&self) -> SBDR {
        SBDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Collision Detected"]
    #[inline]
    pub fn col(&self) -> COLR {
        COLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Receiver Noise Detected"]
    #[inline]
    pub fn rns(&self) -> RNSR {
        RNSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Format Error in Stop Bit 0"]
    #[inline]
    pub fn fer0(&self) -> FER0R {
        FER0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Format Error in Stop Bit 1"]
    #[inline]
    pub fn fer1(&self) -> FER1R {
        FER1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Receive Frame Finished"]
    #[inline]
    pub fn rff(&self) -> RFFR {
        RFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Transmitter Frame Finished"]
    #[inline]
    pub fn tff(&self) -> TFFR {
        TFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Transfer Status BUSY"]
    #[inline]
    pub fn busy(&self) -> BUSYR {
        BUSYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Receiver Start Indication Flag"]
    #[inline]
    pub fn rsif(&self) -> RSIFR {
        RSIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Data Lost Indication Flag"]
    #[inline]
    pub fn dlif(&self) -> DLIFR {
        DLIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Transmit Shift Indication Flag"]
    #[inline]
    pub fn tsif(&self) -> TSIFR {
        TSIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Transmit Buffer Indication Flag"]
    #[inline]
    pub fn tbif(&self) -> TBIFR {
        TBIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Receive Indication Flag"]
    #[inline]
    pub fn rif(&self) -> RIFR {
        RIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Alternative Receive Indication Flag"]
    #[inline]
    pub fn aif(&self) -> AIFR {
        AIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Baud Rate Generator Indication Flag"]
    #[inline]
    pub fn brgif(&self) -> BRGIFR {
        BRGIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Transmission Idle"]
    #[inline]
    pub fn txidle(&mut self) -> _TXIDLEW {
        _TXIDLEW { w: self }
    }
    #[doc = "Bit 1 - Reception Idle"]
    #[inline]
    pub fn rxidle(&mut self) -> _RXIDLEW {
        _RXIDLEW { w: self }
    }
    #[doc = "Bit 2 - Synchronization Break Detected"]
    #[inline]
    pub fn sbd(&mut self) -> _SBDW {
        _SBDW { w: self }
    }
    #[doc = "Bit 3 - Collision Detected"]
    #[inline]
    pub fn col(&mut self) -> _COLW {
        _COLW { w: self }
    }
    #[doc = "Bit 4 - Receiver Noise Detected"]
    #[inline]
    pub fn rns(&mut self) -> _RNSW {
        _RNSW { w: self }
    }
    #[doc = "Bit 5 - Format Error in Stop Bit 0"]
    #[inline]
    pub fn fer0(&mut self) -> _FER0W {
        _FER0W { w: self }
    }
    #[doc = "Bit 6 - Format Error in Stop Bit 1"]
    #[inline]
    pub fn fer1(&mut self) -> _FER1W {
        _FER1W { w: self }
    }
    #[doc = "Bit 7 - Receive Frame Finished"]
    #[inline]
    pub fn rff(&mut self) -> _RFFW {
        _RFFW { w: self }
    }
    #[doc = "Bit 8 - Transmitter Frame Finished"]
    #[inline]
    pub fn tff(&mut self) -> _TFFW {
        _TFFW { w: self }
    }
    #[doc = "Bit 10 - Receiver Start Indication Flag"]
    #[inline]
    pub fn rsif(&mut self) -> _RSIFW {
        _RSIFW { w: self }
    }
    #[doc = "Bit 11 - Data Lost Indication Flag"]
    #[inline]
    pub fn dlif(&mut self) -> _DLIFW {
        _DLIFW { w: self }
    }
    #[doc = "Bit 12 - Transmit Shift Indication Flag"]
    #[inline]
    pub fn tsif(&mut self) -> _TSIFW {
        _TSIFW { w: self }
    }
    #[doc = "Bit 13 - Transmit Buffer Indication Flag"]
    #[inline]
    pub fn tbif(&mut self) -> _TBIFW {
        _TBIFW { w: self }
    }
    #[doc = "Bit 14 - Receive Indication Flag"]
    #[inline]
    pub fn rif(&mut self) -> _RIFW {
        _RIFW { w: self }
    }
    #[doc = "Bit 15 - Alternative Receive Indication Flag"]
    #[inline]
    pub fn aif(&mut self) -> _AIFW {
        _AIFW { w: self }
    }
    #[doc = "Bit 16 - Baud Rate Generator Indication Flag"]
    #[inline]
    pub fn brgif(&mut self) -> _BRGIFW {
        _BRGIFW { w: self }
    }
}
