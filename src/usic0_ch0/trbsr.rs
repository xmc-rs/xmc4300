#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TRBSR {
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
#[doc = "Possible values of the field `SRBI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRBIR {
    #[doc = "A standard receive buffer event has not been detected."]
    VALUE1,
    #[doc = "A standard receive buffer event has been detected."]
    VALUE2,
}
impl SRBIR {
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
            SRBIR::VALUE1 => false,
            SRBIR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRBIR {
        match value {
            false => SRBIR::VALUE1,
            true => SRBIR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SRBIR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SRBIR::VALUE2
    }
}
#[doc = "Possible values of the field `RBERI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBERIR {
    #[doc = "A receive buffer error event has not been detected."]
    VALUE1,
    #[doc = "A receive buffer error event has been detected."]
    VALUE2,
}
impl RBERIR {
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
            RBERIR::VALUE1 => false,
            RBERIR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RBERIR {
        match value {
            false => RBERIR::VALUE1,
            true => RBERIR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RBERIR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RBERIR::VALUE2
    }
}
#[doc = "Possible values of the field `ARBI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARBIR {
    #[doc = "An alternative receive buffer event has not been detected."]
    VALUE1,
    #[doc = "An alternative receive buffer event has been detected."]
    VALUE2,
}
impl ARBIR {
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
            ARBIR::VALUE1 => false,
            ARBIR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ARBIR {
        match value {
            false => ARBIR::VALUE1,
            true => ARBIR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ARBIR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ARBIR::VALUE2
    }
}
#[doc = "Possible values of the field `REMPTY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REMPTYR {
    #[doc = "The receive buffer is not empty."]
    VALUE1,
    #[doc = "The receive buffer is empty."]
    VALUE2,
}
impl REMPTYR {
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
            REMPTYR::VALUE1 => false,
            REMPTYR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REMPTYR {
        match value {
            false => REMPTYR::VALUE1,
            true => REMPTYR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REMPTYR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == REMPTYR::VALUE2
    }
}
#[doc = "Possible values of the field `RFULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFULLR {
    #[doc = "The receive buffer is not full."]
    VALUE1,
    #[doc = "The receive buffer is full."]
    VALUE2,
}
impl RFULLR {
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
            RFULLR::VALUE1 => false,
            RFULLR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RFULLR {
        match value {
            false => RFULLR::VALUE1,
            true => RFULLR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RFULLR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RFULLR::VALUE2
    }
}
#[doc = "Possible values of the field `RBUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBUSR {
    #[doc = "The receive buffer information has been completely updated."]
    VALUE1,
    #[doc = "The OUTR update from the FIFO memory is ongoing. A read from OUTR will be delayed. FIFO pointers from the previous read are not yet updated."]
    VALUE2,
}
impl RBUSR {
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
            RBUSR::VALUE1 => false,
            RBUSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RBUSR {
        match value {
            false => RBUSR::VALUE1,
            true => RBUSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RBUSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RBUSR::VALUE2
    }
}
#[doc = "Possible values of the field `SRBT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRBTR {
    #[doc = "A standard receive buffer event is not triggered using this bit."]
    VALUE1,
    #[doc = "A standard receive buffer event is triggered using this bit."]
    VALUE2,
}
impl SRBTR {
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
            SRBTR::VALUE1 => false,
            SRBTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRBTR {
        match value {
            false => SRBTR::VALUE1,
            true => SRBTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SRBTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SRBTR::VALUE2
    }
}
#[doc = "Possible values of the field `STBI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STBIR {
    #[doc = "A standard transmit buffer event has not been detected."]
    VALUE1,
    #[doc = "A standard transmit buffer event has been detected."]
    VALUE2,
}
impl STBIR {
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
            STBIR::VALUE1 => false,
            STBIR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STBIR {
        match value {
            false => STBIR::VALUE1,
            true => STBIR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == STBIR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == STBIR::VALUE2
    }
}
#[doc = "Possible values of the field `TBERI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBERIR {
    #[doc = "A transmit buffer error event has not been detected."]
    VALUE1,
    #[doc = "A transmit buffer error event has been detected."]
    VALUE2,
}
impl TBERIR {
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
            TBERIR::VALUE1 => false,
            TBERIR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TBERIR {
        match value {
            false => TBERIR::VALUE1,
            true => TBERIR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TBERIR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TBERIR::VALUE2
    }
}
#[doc = "Possible values of the field `TEMPTY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEMPTYR {
    #[doc = "The transmit buffer is not empty."]
    VALUE1,
    #[doc = "The transmit buffer is empty."]
    VALUE2,
}
impl TEMPTYR {
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
            TEMPTYR::VALUE1 => false,
            TEMPTYR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TEMPTYR {
        match value {
            false => TEMPTYR::VALUE1,
            true => TEMPTYR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TEMPTYR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TEMPTYR::VALUE2
    }
}
#[doc = "Possible values of the field `TFULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFULLR {
    #[doc = "The transmit buffer is not full."]
    VALUE1,
    #[doc = "The transmit buffer is full."]
    VALUE2,
}
impl TFULLR {
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
            TFULLR::VALUE1 => false,
            TFULLR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TFULLR {
        match value {
            false => TFULLR::VALUE1,
            true => TFULLR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TFULLR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TFULLR::VALUE2
    }
}
#[doc = "Possible values of the field `TBUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBUSR {
    #[doc = "The transmit buffer information has been completely updated."]
    VALUE1,
    #[doc = "The FIFO memory update after write to INx is ongoing. A write to INx will be delayed. FIFO pointers from the previous INx write are not yet updated."]
    VALUE2,
}
impl TBUSR {
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
            TBUSR::VALUE1 => false,
            TBUSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TBUSR {
        match value {
            false => TBUSR::VALUE1,
            true => TBUSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TBUSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TBUSR::VALUE2
    }
}
#[doc = "Possible values of the field `STBT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STBTR {
    #[doc = "A standard transmit buffer event is not triggered using this bit."]
    VALUE1,
    #[doc = "A standard transmit buffer event is triggered using this bit."]
    VALUE2,
}
impl STBTR {
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
            STBTR::VALUE1 => false,
            STBTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STBTR {
        match value {
            false => STBTR::VALUE1,
            true => STBTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == STBTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == STBTR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct RBFLVLR {
    bits: u8,
}
impl RBFLVLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TBFLVLR {
    bits: u8,
}
impl TBFLVLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `SRBI`"]
pub enum SRBIW {
    #[doc = "A standard receive buffer event has not been detected."]
    VALUE1,
    #[doc = "A standard receive buffer event has been detected."]
    VALUE2,
}
impl SRBIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRBIW::VALUE1 => false,
            SRBIW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRBIW<'a> {
    w: &'a mut W,
}
impl<'a> _SRBIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRBIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A standard receive buffer event has not been detected."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRBIW::VALUE1)
    }
    #[doc = "A standard receive buffer event has been detected."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SRBIW::VALUE2)
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
#[doc = "Values that can be written to the field `RBERI`"]
pub enum RBERIW {
    #[doc = "A receive buffer error event has not been detected."]
    VALUE1,
    #[doc = "A receive buffer error event has been detected."]
    VALUE2,
}
impl RBERIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RBERIW::VALUE1 => false,
            RBERIW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RBERIW<'a> {
    w: &'a mut W,
}
impl<'a> _RBERIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RBERIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A receive buffer error event has not been detected."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RBERIW::VALUE1)
    }
    #[doc = "A receive buffer error event has been detected."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RBERIW::VALUE2)
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
#[doc = "Values that can be written to the field `ARBI`"]
pub enum ARBIW {
    #[doc = "An alternative receive buffer event has not been detected."]
    VALUE1,
    #[doc = "An alternative receive buffer event has been detected."]
    VALUE2,
}
impl ARBIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ARBIW::VALUE1 => false,
            ARBIW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ARBIW<'a> {
    w: &'a mut W,
}
impl<'a> _ARBIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ARBIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An alternative receive buffer event has not been detected."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ARBIW::VALUE1)
    }
    #[doc = "An alternative receive buffer event has been detected."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ARBIW::VALUE2)
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
#[doc = "Values that can be written to the field `STBI`"]
pub enum STBIW {
    #[doc = "A standard transmit buffer event has not been detected."]
    VALUE1,
    #[doc = "A standard transmit buffer event has been detected."]
    VALUE2,
}
impl STBIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STBIW::VALUE1 => false,
            STBIW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STBIW<'a> {
    w: &'a mut W,
}
impl<'a> _STBIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STBIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A standard transmit buffer event has not been detected."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(STBIW::VALUE1)
    }
    #[doc = "A standard transmit buffer event has been detected."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(STBIW::VALUE2)
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
#[doc = "Values that can be written to the field `TBERI`"]
pub enum TBERIW {
    #[doc = "A transmit buffer error event has not been detected."]
    VALUE1,
    #[doc = "A transmit buffer error event has been detected."]
    VALUE2,
}
impl TBERIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TBERIW::VALUE1 => false,
            TBERIW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TBERIW<'a> {
    w: &'a mut W,
}
impl<'a> _TBERIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TBERIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A transmit buffer error event has not been detected."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TBERIW::VALUE1)
    }
    #[doc = "A transmit buffer error event has been detected."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TBERIW::VALUE2)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Standard Receive Buffer Event"]
    #[inline]
    pub fn srbi(&self) -> SRBIR {
        SRBIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Receive Buffer Error Event"]
    #[inline]
    pub fn rberi(&self) -> RBERIR {
        RBERIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Alternative Receive Buffer Event"]
    #[inline]
    pub fn arbi(&self) -> ARBIR {
        ARBIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Receive Buffer Empty"]
    #[inline]
    pub fn rempty(&self) -> REMPTYR {
        REMPTYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Receive Buffer Full"]
    #[inline]
    pub fn rfull(&self) -> RFULLR {
        RFULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Receive Buffer Busy"]
    #[inline]
    pub fn rbus(&self) -> RBUSR {
        RBUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Standard Receive Buffer Event Trigger"]
    #[inline]
    pub fn srbt(&self) -> SRBTR {
        SRBTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Standard Transmit Buffer Event"]
    #[inline]
    pub fn stbi(&self) -> STBIR {
        STBIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Transmit Buffer Error Event"]
    #[inline]
    pub fn tberi(&self) -> TBERIR {
        TBERIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Transmit Buffer Empty"]
    #[inline]
    pub fn tempty(&self) -> TEMPTYR {
        TEMPTYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Transmit Buffer Full"]
    #[inline]
    pub fn tfull(&self) -> TFULLR {
        TFULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Transmit Buffer Busy"]
    #[inline]
    pub fn tbus(&self) -> TBUSR {
        TBUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Standard Transmit Buffer Event Trigger"]
    #[inline]
    pub fn stbt(&self) -> STBTR {
        STBTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:22 - Receive Buffer Filling Level"]
    #[inline]
    pub fn rbflvl(&self) -> RBFLVLR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RBFLVLR { bits }
    }
    #[doc = "Bits 24:30 - Transmit Buffer Filling Level"]
    #[inline]
    pub fn tbflvl(&self) -> TBFLVLR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TBFLVLR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2056 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Standard Receive Buffer Event"]
    #[inline]
    pub fn srbi(&mut self) -> _SRBIW {
        _SRBIW { w: self }
    }
    #[doc = "Bit 1 - Receive Buffer Error Event"]
    #[inline]
    pub fn rberi(&mut self) -> _RBERIW {
        _RBERIW { w: self }
    }
    #[doc = "Bit 2 - Alternative Receive Buffer Event"]
    #[inline]
    pub fn arbi(&mut self) -> _ARBIW {
        _ARBIW { w: self }
    }
    #[doc = "Bit 8 - Standard Transmit Buffer Event"]
    #[inline]
    pub fn stbi(&mut self) -> _STBIW {
        _STBIW { w: self }
    }
    #[doc = "Bit 9 - Transmit Buffer Error Event"]
    #[inline]
    pub fn tberi(&mut self) -> _TBERIW {
        _TBERIW { w: self }
    }
}
