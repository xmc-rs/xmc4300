#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::EN_INT_STATUS_NORM {
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
#[doc = r" Value of the field"]
pub struct FIXED_TO_0R {
    bits: bool,
}
impl FIXED_TO_0R {
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
#[doc = "Possible values of the field `CARD_INT_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARD_INT_ENR {
    #[doc = "Masked"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl CARD_INT_ENR {
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
            CARD_INT_ENR::VALUE1 => false,
            CARD_INT_ENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CARD_INT_ENR {
        match value {
            false => CARD_INT_ENR::VALUE1,
            true => CARD_INT_ENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CARD_INT_ENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CARD_INT_ENR::VALUE2
    }
}
#[doc = "Possible values of the field `CARD_REMOVAL_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARD_REMOVAL_ENR {
    #[doc = "Masked"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl CARD_REMOVAL_ENR {
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
            CARD_REMOVAL_ENR::VALUE1 => false,
            CARD_REMOVAL_ENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CARD_REMOVAL_ENR {
        match value {
            false => CARD_REMOVAL_ENR::VALUE1,
            true => CARD_REMOVAL_ENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CARD_REMOVAL_ENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CARD_REMOVAL_ENR::VALUE2
    }
}
#[doc = "Possible values of the field `CARD_INS_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARD_INS_ENR {
    #[doc = "Masked"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl CARD_INS_ENR {
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
            CARD_INS_ENR::VALUE1 => false,
            CARD_INS_ENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CARD_INS_ENR {
        match value {
            false => CARD_INS_ENR::VALUE1,
            true => CARD_INS_ENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CARD_INS_ENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CARD_INS_ENR::VALUE2
    }
}
#[doc = "Possible values of the field `BUFF_READ_READY_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFF_READ_READY_ENR {
    #[doc = "Masked"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl BUFF_READ_READY_ENR {
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
            BUFF_READ_READY_ENR::VALUE1 => false,
            BUFF_READ_READY_ENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUFF_READ_READY_ENR {
        match value {
            false => BUFF_READ_READY_ENR::VALUE1,
            true => BUFF_READ_READY_ENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BUFF_READ_READY_ENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BUFF_READ_READY_ENR::VALUE2
    }
}
#[doc = "Possible values of the field `BUFF_WRITE_READY_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFF_WRITE_READY_ENR {
    #[doc = "Masked"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl BUFF_WRITE_READY_ENR {
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
            BUFF_WRITE_READY_ENR::VALUE1 => false,
            BUFF_WRITE_READY_ENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUFF_WRITE_READY_ENR {
        match value {
            false => BUFF_WRITE_READY_ENR::VALUE1,
            true => BUFF_WRITE_READY_ENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BUFF_WRITE_READY_ENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BUFF_WRITE_READY_ENR::VALUE2
    }
}
#[doc = "Possible values of the field `BLOCK_GAP_EVENT_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLOCK_GAP_EVENT_ENR {
    #[doc = "Masked"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl BLOCK_GAP_EVENT_ENR {
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
            BLOCK_GAP_EVENT_ENR::VALUE1 => false,
            BLOCK_GAP_EVENT_ENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BLOCK_GAP_EVENT_ENR {
        match value {
            false => BLOCK_GAP_EVENT_ENR::VALUE1,
            true => BLOCK_GAP_EVENT_ENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BLOCK_GAP_EVENT_ENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BLOCK_GAP_EVENT_ENR::VALUE2
    }
}
#[doc = "Possible values of the field `TX_COMPLETE_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_COMPLETE_ENR {
    #[doc = "Masked"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl TX_COMPLETE_ENR {
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
            TX_COMPLETE_ENR::VALUE1 => false,
            TX_COMPLETE_ENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TX_COMPLETE_ENR {
        match value {
            false => TX_COMPLETE_ENR::VALUE1,
            true => TX_COMPLETE_ENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TX_COMPLETE_ENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TX_COMPLETE_ENR::VALUE2
    }
}
#[doc = "Possible values of the field `CMD_COMPLETE_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_COMPLETE_ENR {
    #[doc = "Masked"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl CMD_COMPLETE_ENR {
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
            CMD_COMPLETE_ENR::VALUE1 => false,
            CMD_COMPLETE_ENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMD_COMPLETE_ENR {
        match value {
            false => CMD_COMPLETE_ENR::VALUE1,
            true => CMD_COMPLETE_ENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CMD_COMPLETE_ENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CMD_COMPLETE_ENR::VALUE2
    }
}
#[doc = "Values that can be written to the field `CARD_INT_EN`"]
pub enum CARD_INT_ENW {
    #[doc = "Masked"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl CARD_INT_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CARD_INT_ENW::VALUE1 => false,
            CARD_INT_ENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CARD_INT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CARD_INT_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CARD_INT_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CARD_INT_ENW::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CARD_INT_ENW::VALUE2)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CARD_REMOVAL_EN`"]
pub enum CARD_REMOVAL_ENW {
    #[doc = "Masked"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl CARD_REMOVAL_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CARD_REMOVAL_ENW::VALUE1 => false,
            CARD_REMOVAL_ENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CARD_REMOVAL_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CARD_REMOVAL_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CARD_REMOVAL_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CARD_REMOVAL_ENW::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CARD_REMOVAL_ENW::VALUE2)
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
#[doc = "Values that can be written to the field `CARD_INS_EN`"]
pub enum CARD_INS_ENW {
    #[doc = "Masked"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl CARD_INS_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CARD_INS_ENW::VALUE1 => false,
            CARD_INS_ENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CARD_INS_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CARD_INS_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CARD_INS_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CARD_INS_ENW::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CARD_INS_ENW::VALUE2)
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
#[doc = "Values that can be written to the field `BUFF_READ_READY_EN`"]
pub enum BUFF_READ_READY_ENW {
    #[doc = "Masked"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl BUFF_READ_READY_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUFF_READ_READY_ENW::VALUE1 => false,
            BUFF_READ_READY_ENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUFF_READ_READY_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _BUFF_READ_READY_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUFF_READ_READY_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BUFF_READ_READY_ENW::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BUFF_READ_READY_ENW::VALUE2)
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
#[doc = "Values that can be written to the field `BUFF_WRITE_READY_EN`"]
pub enum BUFF_WRITE_READY_ENW {
    #[doc = "Masked"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl BUFF_WRITE_READY_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUFF_WRITE_READY_ENW::VALUE1 => false,
            BUFF_WRITE_READY_ENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUFF_WRITE_READY_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _BUFF_WRITE_READY_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUFF_WRITE_READY_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BUFF_WRITE_READY_ENW::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BUFF_WRITE_READY_ENW::VALUE2)
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
#[doc = "Values that can be written to the field `BLOCK_GAP_EVENT_EN`"]
pub enum BLOCK_GAP_EVENT_ENW {
    #[doc = "Masked"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl BLOCK_GAP_EVENT_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BLOCK_GAP_EVENT_ENW::VALUE1 => false,
            BLOCK_GAP_EVENT_ENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BLOCK_GAP_EVENT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _BLOCK_GAP_EVENT_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BLOCK_GAP_EVENT_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BLOCK_GAP_EVENT_ENW::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BLOCK_GAP_EVENT_ENW::VALUE2)
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
#[doc = "Values that can be written to the field `TX_COMPLETE_EN`"]
pub enum TX_COMPLETE_ENW {
    #[doc = "Masked"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl TX_COMPLETE_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TX_COMPLETE_ENW::VALUE1 => false,
            TX_COMPLETE_ENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TX_COMPLETE_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_COMPLETE_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_COMPLETE_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TX_COMPLETE_ENW::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TX_COMPLETE_ENW::VALUE2)
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
#[doc = "Values that can be written to the field `CMD_COMPLETE_EN`"]
pub enum CMD_COMPLETE_ENW {
    #[doc = "Masked"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl CMD_COMPLETE_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMD_COMPLETE_ENW::VALUE1 => false,
            CMD_COMPLETE_ENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMD_COMPLETE_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CMD_COMPLETE_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMD_COMPLETE_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMD_COMPLETE_ENW::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMD_COMPLETE_ENW::VALUE2)
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
    #[doc = "Bit 15 - Fixed to 0"]
    #[inline]
    pub fn fixed_to_0(&self) -> FIXED_TO_0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        FIXED_TO_0R { bits }
    }
    #[doc = "Bit 8 - Card Interrupt Status Enable"]
    #[inline]
    pub fn card_int_en(&self) -> CARD_INT_ENR {
        CARD_INT_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 7 - Card Removal Status Enable"]
    #[inline]
    pub fn card_removal_en(&self) -> CARD_REMOVAL_ENR {
        CARD_REMOVAL_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 6 - Card Insertion Status Enable"]
    #[inline]
    pub fn card_ins_en(&self) -> CARD_INS_ENR {
        CARD_INS_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 5 - Buffer Read Ready Status Enable"]
    #[inline]
    pub fn buff_read_ready_en(&self) -> BUFF_READ_READY_ENR {
        BUFF_READ_READY_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - Buffer Write Ready Status Enable"]
    #[inline]
    pub fn buff_write_ready_en(&self) -> BUFF_WRITE_READY_ENR {
        BUFF_WRITE_READY_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 2 - Block Gap Event Status Enable"]
    #[inline]
    pub fn block_gap_event_en(&self) -> BLOCK_GAP_EVENT_ENR {
        BLOCK_GAP_EVENT_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - Transfer Complete Status Enable"]
    #[inline]
    pub fn tx_complete_en(&self) -> TX_COMPLETE_ENR {
        TX_COMPLETE_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 0 - Command Complete Status Enable"]
    #[inline]
    pub fn cmd_complete_en(&self) -> CMD_COMPLETE_ENR {
        CMD_COMPLETE_ENR::_from({
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
    #[doc = "Bit 8 - Card Interrupt Status Enable"]
    #[inline]
    pub fn card_int_en(&mut self) -> _CARD_INT_ENW {
        _CARD_INT_ENW { w: self }
    }
    #[doc = "Bit 7 - Card Removal Status Enable"]
    #[inline]
    pub fn card_removal_en(&mut self) -> _CARD_REMOVAL_ENW {
        _CARD_REMOVAL_ENW { w: self }
    }
    #[doc = "Bit 6 - Card Insertion Status Enable"]
    #[inline]
    pub fn card_ins_en(&mut self) -> _CARD_INS_ENW {
        _CARD_INS_ENW { w: self }
    }
    #[doc = "Bit 5 - Buffer Read Ready Status Enable"]
    #[inline]
    pub fn buff_read_ready_en(&mut self) -> _BUFF_READ_READY_ENW {
        _BUFF_READ_READY_ENW { w: self }
    }
    #[doc = "Bit 4 - Buffer Write Ready Status Enable"]
    #[inline]
    pub fn buff_write_ready_en(&mut self) -> _BUFF_WRITE_READY_ENW {
        _BUFF_WRITE_READY_ENW { w: self }
    }
    #[doc = "Bit 2 - Block Gap Event Status Enable"]
    #[inline]
    pub fn block_gap_event_en(&mut self) -> _BLOCK_GAP_EVENT_ENW {
        _BLOCK_GAP_EVENT_ENW { w: self }
    }
    #[doc = "Bit 1 - Transfer Complete Status Enable"]
    #[inline]
    pub fn tx_complete_en(&mut self) -> _TX_COMPLETE_ENW {
        _TX_COMPLETE_ENW { w: self }
    }
    #[doc = "Bit 0 - Command Complete Status Enable"]
    #[inline]
    pub fn cmd_complete_en(&mut self) -> _CMD_COMPLETE_ENW {
        _CMD_COMPLETE_ENW { w: self }
    }
}
