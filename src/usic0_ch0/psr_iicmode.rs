#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PSR_IICMODE {
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
#[doc = "Possible values of the field `SLSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLSELR {
    #[doc = "The device is not selected as slave."]
    VALUE1,
    #[doc = "The device is selected as slave."]
    VALUE2,
}
impl SLSELR {
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
            SLSELR::VALUE1 => false,
            SLSELR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLSELR {
        match value {
            false => SLSELR::VALUE1,
            true => SLSELR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SLSELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SLSELR::VALUE2
    }
}
#[doc = "Possible values of the field `WTDF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WTDFR {
    #[doc = "A wrong TDF code has not been found."]
    VALUE1,
    #[doc = "A wrong TDF code has been found."]
    VALUE2,
}
impl WTDFR {
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
            WTDFR::VALUE1 => false,
            WTDFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WTDFR {
        match value {
            false => WTDFR::VALUE1,
            true => WTDFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WTDFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WTDFR::VALUE2
    }
}
#[doc = "Possible values of the field `SCR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCRR {
    #[doc = "A start condition has not yet been detected."]
    VALUE1,
    #[doc = "A start condition has been detected."]
    VALUE2,
}
impl SCRR {
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
            SCRR::VALUE1 => false,
            SCRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCRR {
        match value {
            false => SCRR::VALUE1,
            true => SCRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SCRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SCRR::VALUE2
    }
}
#[doc = "Possible values of the field `RSCR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSCRR {
    #[doc = "A repeated start condition has not yet been detected."]
    VALUE1,
    #[doc = "A repeated start condition has been detected."]
    VALUE2,
}
impl RSCRR {
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
            RSCRR::VALUE1 => false,
            RSCRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RSCRR {
        match value {
            false => RSCRR::VALUE1,
            true => RSCRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RSCRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RSCRR::VALUE2
    }
}
#[doc = "Possible values of the field `PCR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCRR {
    #[doc = "A stop condition has not yet been detected."]
    VALUE1,
    #[doc = "A stop condition has been detected."]
    VALUE2,
}
impl PCRR {
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
            PCRR::VALUE1 => false,
            PCRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PCRR {
        match value {
            false => PCRR::VALUE1,
            true => PCRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PCRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PCRR::VALUE2
    }
}
#[doc = "Possible values of the field `NACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACKR {
    #[doc = "A non-acknowledge has not been received."]
    VALUE1,
    #[doc = "A non-acknowledge has been received."]
    VALUE2,
}
impl NACKR {
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
            NACKR::VALUE1 => false,
            NACKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NACKR {
        match value {
            false => NACKR::VALUE1,
            true => NACKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == NACKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == NACKR::VALUE2
    }
}
#[doc = "Possible values of the field `ARL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARLR {
    #[doc = "An arbitration has not been lost."]
    VALUE1,
    #[doc = "An arbitration has been lost."]
    VALUE2,
}
impl ARLR {
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
            ARLR::VALUE1 => false,
            ARLR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ARLR {
        match value {
            false => ARLR::VALUE1,
            true => ARLR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ARLR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ARLR::VALUE2
    }
}
#[doc = "Possible values of the field `SRR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRRR {
    #[doc = "A slave read request has not been detected."]
    VALUE1,
    #[doc = "A slave read request has been detected."]
    VALUE2,
}
impl SRRR {
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
            SRRR::VALUE1 => false,
            SRRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRRR {
        match value {
            false => SRRR::VALUE1,
            true => SRRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SRRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SRRR::VALUE2
    }
}
#[doc = "Possible values of the field `ERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRR {
    #[doc = "An IIC error has not been detected."]
    VALUE1,
    #[doc = "An IIC error has been detected."]
    VALUE2,
}
impl ERRR {
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
            ERRR::VALUE1 => false,
            ERRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERRR {
        match value {
            false => ERRR::VALUE1,
            true => ERRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ERRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ERRR::VALUE2
    }
}
#[doc = "Possible values of the field `ACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACKR {
    #[doc = "An acknowledge has not been received."]
    VALUE1,
    #[doc = "An acknowledge has been received."]
    VALUE2,
}
impl ACKR {
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
            ACKR::VALUE1 => false,
            ACKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACKR {
        match value {
            false => ACKR::VALUE1,
            true => ACKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ACKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ACKR::VALUE2
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
#[doc = "Values that can be written to the field `SLSEL`"]
pub enum SLSELW {
    #[doc = "The device is not selected as slave."]
    VALUE1,
    #[doc = "The device is selected as slave."]
    VALUE2,
}
impl SLSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLSELW::VALUE1 => false,
            SLSELW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLSELW<'a> {
    w: &'a mut W,
}
impl<'a> _SLSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The device is not selected as slave."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SLSELW::VALUE1)
    }
    #[doc = "The device is selected as slave."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SLSELW::VALUE2)
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
#[doc = "Values that can be written to the field `WTDF`"]
pub enum WTDFW {
    #[doc = "A wrong TDF code has not been found."]
    VALUE1,
    #[doc = "A wrong TDF code has been found."]
    VALUE2,
}
impl WTDFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WTDFW::VALUE1 => false,
            WTDFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WTDFW<'a> {
    w: &'a mut W,
}
impl<'a> _WTDFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WTDFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A wrong TDF code has not been found."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WTDFW::VALUE1)
    }
    #[doc = "A wrong TDF code has been found."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WTDFW::VALUE2)
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
#[doc = "Values that can be written to the field `SCR`"]
pub enum SCRW {
    #[doc = "A start condition has not yet been detected."]
    VALUE1,
    #[doc = "A start condition has been detected."]
    VALUE2,
}
impl SCRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SCRW::VALUE1 => false,
            SCRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCRW<'a> {
    w: &'a mut W,
}
impl<'a> _SCRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A start condition has not yet been detected."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SCRW::VALUE1)
    }
    #[doc = "A start condition has been detected."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SCRW::VALUE2)
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
#[doc = "Values that can be written to the field `RSCR`"]
pub enum RSCRW {
    #[doc = "A repeated start condition has not yet been detected."]
    VALUE1,
    #[doc = "A repeated start condition has been detected."]
    VALUE2,
}
impl RSCRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSCRW::VALUE1 => false,
            RSCRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSCRW<'a> {
    w: &'a mut W,
}
impl<'a> _RSCRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSCRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A repeated start condition has not yet been detected."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RSCRW::VALUE1)
    }
    #[doc = "A repeated start condition has been detected."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RSCRW::VALUE2)
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
#[doc = "Values that can be written to the field `PCR`"]
pub enum PCRW {
    #[doc = "A stop condition has not yet been detected."]
    VALUE1,
    #[doc = "A stop condition has been detected."]
    VALUE2,
}
impl PCRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PCRW::VALUE1 => false,
            PCRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCRW<'a> {
    w: &'a mut W,
}
impl<'a> _PCRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A stop condition has not yet been detected."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PCRW::VALUE1)
    }
    #[doc = "A stop condition has been detected."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PCRW::VALUE2)
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
#[doc = "Values that can be written to the field `NACK`"]
pub enum NACKW {
    #[doc = "A non-acknowledge has not been received."]
    VALUE1,
    #[doc = "A non-acknowledge has been received."]
    VALUE2,
}
impl NACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NACKW::VALUE1 => false,
            NACKW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NACKW<'a> {
    w: &'a mut W,
}
impl<'a> _NACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A non-acknowledge has not been received."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(NACKW::VALUE1)
    }
    #[doc = "A non-acknowledge has been received."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(NACKW::VALUE2)
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
#[doc = "Values that can be written to the field `ARL`"]
pub enum ARLW {
    #[doc = "An arbitration has not been lost."]
    VALUE1,
    #[doc = "An arbitration has been lost."]
    VALUE2,
}
impl ARLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ARLW::VALUE1 => false,
            ARLW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ARLW<'a> {
    w: &'a mut W,
}
impl<'a> _ARLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ARLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An arbitration has not been lost."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ARLW::VALUE1)
    }
    #[doc = "An arbitration has been lost."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ARLW::VALUE2)
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
#[doc = "Values that can be written to the field `SRR`"]
pub enum SRRW {
    #[doc = "A slave read request has not been detected."]
    VALUE1,
    #[doc = "A slave read request has been detected."]
    VALUE2,
}
impl SRRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRRW::VALUE1 => false,
            SRRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRRW<'a> {
    w: &'a mut W,
}
impl<'a> _SRRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A slave read request has not been detected."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRRW::VALUE1)
    }
    #[doc = "A slave read request has been detected."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SRRW::VALUE2)
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
#[doc = "Values that can be written to the field `ERR`"]
pub enum ERRW {
    #[doc = "An IIC error has not been detected."]
    VALUE1,
    #[doc = "An IIC error has been detected."]
    VALUE2,
}
impl ERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERRW::VALUE1 => false,
            ERRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _ERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An IIC error has not been detected."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ERRW::VALUE1)
    }
    #[doc = "An IIC error has been detected."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ERRW::VALUE2)
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
#[doc = "Values that can be written to the field `ACK`"]
pub enum ACKW {
    #[doc = "An acknowledge has not been received."]
    VALUE1,
    #[doc = "An acknowledge has been received."]
    VALUE2,
}
impl ACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACKW::VALUE1 => false,
            ACKW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _ACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An acknowledge has not been received."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ACKW::VALUE1)
    }
    #[doc = "An acknowledge has been received."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ACKW::VALUE2)
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
        const OFFSET: u8 = 9;
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
    #[doc = "Bit 0 - Slave Select"]
    #[inline]
    pub fn slsel(&self) -> SLSELR {
        SLSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Wrong TDF Code Found"]
    #[inline]
    pub fn wtdf(&self) -> WTDFR {
        WTDFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Start Condition Received"]
    #[inline]
    pub fn scr(&self) -> SCRR {
        SCRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Repeated Start Condition Received"]
    #[inline]
    pub fn rscr(&self) -> RSCRR {
        RSCRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Stop Condition Received"]
    #[inline]
    pub fn pcr(&self) -> PCRR {
        PCRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Non-Acknowledge Received"]
    #[inline]
    pub fn nack(&self) -> NACKR {
        NACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Arbitration Lost"]
    #[inline]
    pub fn arl(&self) -> ARLR {
        ARLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Slave Read Request"]
    #[inline]
    pub fn srr(&self) -> SRRR {
        SRRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Error"]
    #[inline]
    pub fn err(&self) -> ERRR {
        ERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Acknowledge Received"]
    #[inline]
    pub fn ack(&self) -> ACKR {
        ACKR::_from({
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
    #[doc = "Bit 0 - Slave Select"]
    #[inline]
    pub fn slsel(&mut self) -> _SLSELW {
        _SLSELW { w: self }
    }
    #[doc = "Bit 1 - Wrong TDF Code Found"]
    #[inline]
    pub fn wtdf(&mut self) -> _WTDFW {
        _WTDFW { w: self }
    }
    #[doc = "Bit 2 - Start Condition Received"]
    #[inline]
    pub fn scr(&mut self) -> _SCRW {
        _SCRW { w: self }
    }
    #[doc = "Bit 3 - Repeated Start Condition Received"]
    #[inline]
    pub fn rscr(&mut self) -> _RSCRW {
        _RSCRW { w: self }
    }
    #[doc = "Bit 4 - Stop Condition Received"]
    #[inline]
    pub fn pcr(&mut self) -> _PCRW {
        _PCRW { w: self }
    }
    #[doc = "Bit 5 - Non-Acknowledge Received"]
    #[inline]
    pub fn nack(&mut self) -> _NACKW {
        _NACKW { w: self }
    }
    #[doc = "Bit 6 - Arbitration Lost"]
    #[inline]
    pub fn arl(&mut self) -> _ARLW {
        _ARLW { w: self }
    }
    #[doc = "Bit 7 - Slave Read Request"]
    #[inline]
    pub fn srr(&mut self) -> _SRRW {
        _SRRW { w: self }
    }
    #[doc = "Bit 8 - Error"]
    #[inline]
    pub fn err(&mut self) -> _ERRW {
        _ERRW { w: self }
    }
    #[doc = "Bit 9 - Acknowledge Received"]
    #[inline]
    pub fn ack(&mut self) -> _ACKW {
        _ACKW { w: self }
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
