#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::INT_STATUS_NORM {
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
#[doc = "Possible values of the field `ERR_INT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR_INTR {
    #[doc = "No Error."]
    VALUE1,
    #[doc = "Error."]
    VALUE2,
}
impl ERR_INTR {
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
            ERR_INTR::VALUE1 => false,
            ERR_INTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERR_INTR {
        match value {
            false => ERR_INTR::VALUE1,
            true => ERR_INTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ERR_INTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ERR_INTR::VALUE2
    }
}
#[doc = "Possible values of the field `CARD_INT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARD_INTR {
    #[doc = "No Card Interrupt"]
    VALUE1,
    #[doc = "Generate Card Interrupt"]
    VALUE2,
}
impl CARD_INTR {
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
            CARD_INTR::VALUE1 => false,
            CARD_INTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CARD_INTR {
        match value {
            false => CARD_INTR::VALUE1,
            true => CARD_INTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CARD_INTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CARD_INTR::VALUE2
    }
}
#[doc = "Possible values of the field `CARD_REMOVAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARD_REMOVALR {
    #[doc = "Card State Stable or Debouncing"]
    VALUE1,
    #[doc = "Card Removed"]
    VALUE2,
}
impl CARD_REMOVALR {
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
            CARD_REMOVALR::VALUE1 => false,
            CARD_REMOVALR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CARD_REMOVALR {
        match value {
            false => CARD_REMOVALR::VALUE1,
            true => CARD_REMOVALR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CARD_REMOVALR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CARD_REMOVALR::VALUE2
    }
}
#[doc = "Possible values of the field `CARD_INS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARD_INSR {
    #[doc = "Card State Stable or Debouncing"]
    VALUE1,
    #[doc = "Card Inserted"]
    VALUE2,
}
impl CARD_INSR {
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
            CARD_INSR::VALUE1 => false,
            CARD_INSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CARD_INSR {
        match value {
            false => CARD_INSR::VALUE1,
            true => CARD_INSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CARD_INSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CARD_INSR::VALUE2
    }
}
#[doc = "Possible values of the field `BUFF_READ_READY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFF_READ_READYR {
    #[doc = "Not Ready to read Buffer."]
    VALUE1,
    #[doc = "Ready to read Buffer."]
    VALUE2,
}
impl BUFF_READ_READYR {
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
            BUFF_READ_READYR::VALUE1 => false,
            BUFF_READ_READYR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUFF_READ_READYR {
        match value {
            false => BUFF_READ_READYR::VALUE1,
            true => BUFF_READ_READYR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BUFF_READ_READYR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BUFF_READ_READYR::VALUE2
    }
}
#[doc = "Possible values of the field `BUFF_WRITE_READY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFF_WRITE_READYR {
    #[doc = "Not Ready to Write Buffer."]
    VALUE1,
    #[doc = "Ready to Write Buffer."]
    VALUE2,
}
impl BUFF_WRITE_READYR {
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
            BUFF_WRITE_READYR::VALUE1 => false,
            BUFF_WRITE_READYR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUFF_WRITE_READYR {
        match value {
            false => BUFF_WRITE_READYR::VALUE1,
            true => BUFF_WRITE_READYR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BUFF_WRITE_READYR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BUFF_WRITE_READYR::VALUE2
    }
}
#[doc = "Possible values of the field `BLOCK_GAP_EVENT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLOCK_GAP_EVENTR {
    #[doc = "No Block Gap Event"]
    VALUE1,
    #[doc = "Transaction stopped at Block Gap"]
    VALUE2,
}
impl BLOCK_GAP_EVENTR {
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
            BLOCK_GAP_EVENTR::VALUE1 => false,
            BLOCK_GAP_EVENTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BLOCK_GAP_EVENTR {
        match value {
            false => BLOCK_GAP_EVENTR::VALUE1,
            true => BLOCK_GAP_EVENTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BLOCK_GAP_EVENTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BLOCK_GAP_EVENTR::VALUE2
    }
}
#[doc = "Possible values of the field `TX_COMPLETE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_COMPLETER {
    #[doc = "No Data Transfer Complete"]
    VALUE1,
    #[doc = "Data Transfer Complete"]
    VALUE2,
}
impl TX_COMPLETER {
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
            TX_COMPLETER::VALUE1 => false,
            TX_COMPLETER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TX_COMPLETER {
        match value {
            false => TX_COMPLETER::VALUE1,
            true => TX_COMPLETER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TX_COMPLETER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TX_COMPLETER::VALUE2
    }
}
#[doc = "Possible values of the field `CMD_COMPLETE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_COMPLETER {
    #[doc = "No Command Complete"]
    VALUE1,
    #[doc = "Command Complete"]
    VALUE2,
}
impl CMD_COMPLETER {
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
            CMD_COMPLETER::VALUE1 => false,
            CMD_COMPLETER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMD_COMPLETER {
        match value {
            false => CMD_COMPLETER::VALUE1,
            true => CMD_COMPLETER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CMD_COMPLETER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CMD_COMPLETER::VALUE2
    }
}
#[doc = "Values that can be written to the field `CARD_REMOVAL`"]
pub enum CARD_REMOVALW {
    #[doc = "Card State Stable or Debouncing"]
    VALUE1,
    #[doc = "Card Removed"]
    VALUE2,
}
impl CARD_REMOVALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CARD_REMOVALW::VALUE1 => false,
            CARD_REMOVALW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CARD_REMOVALW<'a> {
    w: &'a mut W,
}
impl<'a> _CARD_REMOVALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CARD_REMOVALW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Card State Stable or Debouncing"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CARD_REMOVALW::VALUE1)
    }
    #[doc = "Card Removed"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CARD_REMOVALW::VALUE2)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CARD_INS`"]
pub enum CARD_INSW {
    #[doc = "Card State Stable or Debouncing"]
    VALUE1,
    #[doc = "Card Inserted"]
    VALUE2,
}
impl CARD_INSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CARD_INSW::VALUE1 => false,
            CARD_INSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CARD_INSW<'a> {
    w: &'a mut W,
}
impl<'a> _CARD_INSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CARD_INSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Card State Stable or Debouncing"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CARD_INSW::VALUE1)
    }
    #[doc = "Card Inserted"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CARD_INSW::VALUE2)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BUFF_READ_READY`"]
pub enum BUFF_READ_READYW {
    #[doc = "Not Ready to read Buffer."]
    VALUE1,
    #[doc = "Ready to read Buffer."]
    VALUE2,
}
impl BUFF_READ_READYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUFF_READ_READYW::VALUE1 => false,
            BUFF_READ_READYW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUFF_READ_READYW<'a> {
    w: &'a mut W,
}
impl<'a> _BUFF_READ_READYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUFF_READ_READYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not Ready to read Buffer."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BUFF_READ_READYW::VALUE1)
    }
    #[doc = "Ready to read Buffer."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BUFF_READ_READYW::VALUE2)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BUFF_WRITE_READY`"]
pub enum BUFF_WRITE_READYW {
    #[doc = "Not Ready to Write Buffer."]
    VALUE1,
    #[doc = "Ready to Write Buffer."]
    VALUE2,
}
impl BUFF_WRITE_READYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUFF_WRITE_READYW::VALUE1 => false,
            BUFF_WRITE_READYW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUFF_WRITE_READYW<'a> {
    w: &'a mut W,
}
impl<'a> _BUFF_WRITE_READYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUFF_WRITE_READYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not Ready to Write Buffer."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BUFF_WRITE_READYW::VALUE1)
    }
    #[doc = "Ready to Write Buffer."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BUFF_WRITE_READYW::VALUE2)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BLOCK_GAP_EVENT`"]
pub enum BLOCK_GAP_EVENTW {
    #[doc = "No Block Gap Event"]
    VALUE1,
    #[doc = "Transaction stopped at Block Gap"]
    VALUE2,
}
impl BLOCK_GAP_EVENTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BLOCK_GAP_EVENTW::VALUE1 => false,
            BLOCK_GAP_EVENTW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BLOCK_GAP_EVENTW<'a> {
    w: &'a mut W,
}
impl<'a> _BLOCK_GAP_EVENTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BLOCK_GAP_EVENTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Block Gap Event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BLOCK_GAP_EVENTW::VALUE1)
    }
    #[doc = "Transaction stopped at Block Gap"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BLOCK_GAP_EVENTW::VALUE2)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TX_COMPLETE`"]
pub enum TX_COMPLETEW {
    #[doc = "No Data Transfer Complete"]
    VALUE1,
    #[doc = "Data Transfer Complete"]
    VALUE2,
}
impl TX_COMPLETEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TX_COMPLETEW::VALUE1 => false,
            TX_COMPLETEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TX_COMPLETEW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_COMPLETEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_COMPLETEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Data Transfer Complete"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TX_COMPLETEW::VALUE1)
    }
    #[doc = "Data Transfer Complete"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TX_COMPLETEW::VALUE2)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CMD_COMPLETE`"]
pub enum CMD_COMPLETEW {
    #[doc = "No Command Complete"]
    VALUE1,
    #[doc = "Command Complete"]
    VALUE2,
}
impl CMD_COMPLETEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMD_COMPLETEW::VALUE1 => false,
            CMD_COMPLETEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMD_COMPLETEW<'a> {
    w: &'a mut W,
}
impl<'a> _CMD_COMPLETEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMD_COMPLETEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Command Complete"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMD_COMPLETEW::VALUE1)
    }
    #[doc = "Command Complete"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMD_COMPLETEW::VALUE2)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 15 - Error Interrupt"]
    #[inline]
    pub fn err_int(&self) -> ERR_INTR {
        ERR_INTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 8 - Card Interrupt"]
    #[inline]
    pub fn card_int(&self) -> CARD_INTR {
        CARD_INTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 7 - Card Removal"]
    #[inline]
    pub fn card_removal(&self) -> CARD_REMOVALR {
        CARD_REMOVALR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 6 - Card Insertion"]
    #[inline]
    pub fn card_ins(&self) -> CARD_INSR {
        CARD_INSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 5 - Buffer Read Ready"]
    #[inline]
    pub fn buff_read_ready(&self) -> BUFF_READ_READYR {
        BUFF_READ_READYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - Buffer Write Ready"]
    #[inline]
    pub fn buff_write_ready(&self) -> BUFF_WRITE_READYR {
        BUFF_WRITE_READYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 2 - Block Gap Event"]
    #[inline]
    pub fn block_gap_event(&self) -> BLOCK_GAP_EVENTR {
        BLOCK_GAP_EVENTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - Transfer Complete"]
    #[inline]
    pub fn tx_complete(&self) -> TX_COMPLETER {
        TX_COMPLETER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 0 - Command Complete"]
    #[inline]
    pub fn cmd_complete(&self) -> CMD_COMPLETER {
        CMD_COMPLETER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
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
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 7 - Card Removal"]
    #[inline]
    pub fn card_removal(&mut self) -> _CARD_REMOVALW {
        _CARD_REMOVALW { w: self }
    }
    #[doc = "Bit 6 - Card Insertion"]
    #[inline]
    pub fn card_ins(&mut self) -> _CARD_INSW {
        _CARD_INSW { w: self }
    }
    #[doc = "Bit 5 - Buffer Read Ready"]
    #[inline]
    pub fn buff_read_ready(&mut self) -> _BUFF_READ_READYW {
        _BUFF_READ_READYW { w: self }
    }
    #[doc = "Bit 4 - Buffer Write Ready"]
    #[inline]
    pub fn buff_write_ready(&mut self) -> _BUFF_WRITE_READYW {
        _BUFF_WRITE_READYW { w: self }
    }
    #[doc = "Bit 2 - Block Gap Event"]
    #[inline]
    pub fn block_gap_event(&mut self) -> _BLOCK_GAP_EVENTW {
        _BLOCK_GAP_EVENTW { w: self }
    }
    #[doc = "Bit 1 - Transfer Complete"]
    #[inline]
    pub fn tx_complete(&mut self) -> _TX_COMPLETEW {
        _TX_COMPLETEW { w: self }
    }
    #[doc = "Bit 0 - Command Complete"]
    #[inline]
    pub fn cmd_complete(&mut self) -> _CMD_COMPLETEW {
        _CMD_COMPLETEW { w: self }
    }
}
