#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::NCR {
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
#[doc = "Possible values of the field `INIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INITR {
    #[doc = "Resetting bit INIT enables the participation of the node in the CAN traffic. If the CAN node is in the bus-off state, the ongoing bus-off recovery (which does not depend on the INIT bit) is continued. With the end of the bus-off recovery sequence the CAN node is allowed to take part in the CAN traffic. If the CAN node is not in the bus-off state, a sequence of 11 consecutive recessive bits must be detected before the node is allowed to take part in the CAN traffic."]
    VALUE1,
    #[doc = "Setting this bit terminates the participation of this node in the CAN traffic. Any ongoing frame transfer is cancelled and the transmit line goes recessive. If the CAN node is in the bus-off state, then the running bus-off recovery sequence is continued. If the INIT bit is still set after the successful completion of the bus-off recovery sequence, i.e. after detecting 128 sequences of 11 consecutive recessive bits (11 1), then the CAN node leaves the bus-off state but remains inactive as long as INIT remains set."]
    VALUE2,
}
impl INITR {
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
            INITR::VALUE1 => false,
            INITR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INITR {
        match value {
            false => INITR::VALUE1,
            true => INITR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == INITR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == INITR::VALUE2
    }
}
#[doc = "Possible values of the field `TRIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIER {
    #[doc = "Transfer interrupt is disabled."]
    VALUE1,
    #[doc = "Transfer interrupt is enabled."]
    VALUE2,
}
impl TRIER {
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
            TRIER::VALUE1 => false,
            TRIER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIER {
        match value {
            false => TRIER::VALUE1,
            true => TRIER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TRIER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TRIER::VALUE2
    }
}
#[doc = "Possible values of the field `LECIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LECIER {
    #[doc = "Last error code interrupt is disabled."]
    VALUE1,
    #[doc = "Last error code interrupt is enabled."]
    VALUE2,
}
impl LECIER {
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
            LECIER::VALUE1 => false,
            LECIER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LECIER {
        match value {
            false => LECIER::VALUE1,
            true => LECIER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LECIER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LECIER::VALUE2
    }
}
#[doc = "Possible values of the field `ALIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALIER {
    #[doc = "Alert interrupt is disabled."]
    VALUE1,
    #[doc = "Alert interrupt is enabled."]
    VALUE2,
}
impl ALIER {
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
            ALIER::VALUE1 => false,
            ALIER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ALIER {
        match value {
            false => ALIER::VALUE1,
            true => ALIER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ALIER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ALIER::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct CANDISR {
    bits: bool,
}
impl CANDISR {
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
pub struct TXDISR {
    bits: bool,
}
impl TXDISR {
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
#[doc = "Possible values of the field `CCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCER {
    #[doc = "The Bit Timing Register, the Port Control Register, Error Counter Register may only be read. All attempts to modify them are ignored."]
    VALUE1,
    #[doc = "The Bit Timing Register, the Port Control Register, Error Counter Register may be read and written."]
    VALUE2,
}
impl CCER {
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
            CCER::VALUE1 => false,
            CCER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCER {
        match value {
            false => CCER::VALUE1,
            true => CCER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CCER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CCER::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct CALMR {
    bits: bool,
}
impl CALMR {
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
#[doc = "Values that can be written to the field `INIT`"]
pub enum INITW {
    #[doc = "Resetting bit INIT enables the participation of the node in the CAN traffic. If the CAN node is in the bus-off state, the ongoing bus-off recovery (which does not depend on the INIT bit) is continued. With the end of the bus-off recovery sequence the CAN node is allowed to take part in the CAN traffic. If the CAN node is not in the bus-off state, a sequence of 11 consecutive recessive bits must be detected before the node is allowed to take part in the CAN traffic."]
    VALUE1,
    #[doc = "Setting this bit terminates the participation of this node in the CAN traffic. Any ongoing frame transfer is cancelled and the transmit line goes recessive. If the CAN node is in the bus-off state, then the running bus-off recovery sequence is continued. If the INIT bit is still set after the successful completion of the bus-off recovery sequence, i.e. after detecting 128 sequences of 11 consecutive recessive bits (11 1), then the CAN node leaves the bus-off state but remains inactive as long as INIT remains set."]
    VALUE2,
}
impl INITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INITW::VALUE1 => false,
            INITW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INITW<'a> {
    w: &'a mut W,
}
impl<'a> _INITW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INITW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Resetting bit INIT enables the participation of the node in the CAN traffic. If the CAN node is in the bus-off state, the ongoing bus-off recovery (which does not depend on the INIT bit) is continued. With the end of the bus-off recovery sequence the CAN node is allowed to take part in the CAN traffic. If the CAN node is not in the bus-off state, a sequence of 11 consecutive recessive bits must be detected before the node is allowed to take part in the CAN traffic."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(INITW::VALUE1)
    }
    #[doc = "Setting this bit terminates the participation of this node in the CAN traffic. Any ongoing frame transfer is cancelled and the transmit line goes recessive. If the CAN node is in the bus-off state, then the running bus-off recovery sequence is continued. If the INIT bit is still set after the successful completion of the bus-off recovery sequence, i.e. after detecting 128 sequences of 11 consecutive recessive bits (11 1), then the CAN node leaves the bus-off state but remains inactive as long as INIT remains set."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(INITW::VALUE2)
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
#[doc = "Values that can be written to the field `TRIE`"]
pub enum TRIEW {
    #[doc = "Transfer interrupt is disabled."]
    VALUE1,
    #[doc = "Transfer interrupt is enabled."]
    VALUE2,
}
impl TRIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRIEW::VALUE1 => false,
            TRIEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transfer interrupt is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TRIEW::VALUE1)
    }
    #[doc = "Transfer interrupt is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TRIEW::VALUE2)
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
#[doc = "Values that can be written to the field `LECIE`"]
pub enum LECIEW {
    #[doc = "Last error code interrupt is disabled."]
    VALUE1,
    #[doc = "Last error code interrupt is enabled."]
    VALUE2,
}
impl LECIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LECIEW::VALUE1 => false,
            LECIEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LECIEW<'a> {
    w: &'a mut W,
}
impl<'a> _LECIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LECIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Last error code interrupt is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LECIEW::VALUE1)
    }
    #[doc = "Last error code interrupt is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LECIEW::VALUE2)
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
#[doc = "Values that can be written to the field `ALIE`"]
pub enum ALIEW {
    #[doc = "Alert interrupt is disabled."]
    VALUE1,
    #[doc = "Alert interrupt is enabled."]
    VALUE2,
}
impl ALIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ALIEW::VALUE1 => false,
            ALIEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALIEW<'a> {
    w: &'a mut W,
}
impl<'a> _ALIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Alert interrupt is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALIEW::VALUE1)
    }
    #[doc = "Alert interrupt is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALIEW::VALUE2)
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
#[doc = r" Proxy"]
pub struct _CANDISW<'a> {
    w: &'a mut W,
}
impl<'a> _CANDISW<'a> {
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
#[doc = r" Proxy"]
pub struct _TXDISW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDISW<'a> {
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
#[doc = "Values that can be written to the field `CCE`"]
pub enum CCEW {
    #[doc = "The Bit Timing Register, the Port Control Register, Error Counter Register may only be read. All attempts to modify them are ignored."]
    VALUE1,
    #[doc = "The Bit Timing Register, the Port Control Register, Error Counter Register may be read and written."]
    VALUE2,
}
impl CCEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCEW::VALUE1 => false,
            CCEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCEW<'a> {
    w: &'a mut W,
}
impl<'a> _CCEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The Bit Timing Register, the Port Control Register, Error Counter Register may only be read. All attempts to modify them are ignored."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCEW::VALUE1)
    }
    #[doc = "The Bit Timing Register, the Port Control Register, Error Counter Register may be read and written."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCEW::VALUE2)
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
#[doc = r" Proxy"]
pub struct _CALMW<'a> {
    w: &'a mut W,
}
impl<'a> _CALMW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Node Initialization"]
    #[inline]
    pub fn init(&self) -> INITR {
        INITR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Transfer Interrupt Enable"]
    #[inline]
    pub fn trie(&self) -> TRIER {
        TRIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - LEC Indicated Error Interrupt Enable"]
    #[inline]
    pub fn lecie(&self) -> LECIER {
        LECIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Alert Interrupt Enable"]
    #[inline]
    pub fn alie(&self) -> ALIER {
        ALIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - CAN Disable"]
    #[inline]
    pub fn candis(&self) -> CANDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CANDISR { bits }
    }
    #[doc = "Bit 5 - Transmit Disable"]
    #[inline]
    pub fn txdis(&self) -> TXDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXDISR { bits }
    }
    #[doc = "Bit 6 - Configuration Change Enable"]
    #[inline]
    pub fn cce(&self) -> CCER {
        CCER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - CAN Analyzer Mode"]
    #[inline]
    pub fn calm(&self) -> CALMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CALMR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 65 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Node Initialization"]
    #[inline]
    pub fn init(&mut self) -> _INITW {
        _INITW { w: self }
    }
    #[doc = "Bit 1 - Transfer Interrupt Enable"]
    #[inline]
    pub fn trie(&mut self) -> _TRIEW {
        _TRIEW { w: self }
    }
    #[doc = "Bit 2 - LEC Indicated Error Interrupt Enable"]
    #[inline]
    pub fn lecie(&mut self) -> _LECIEW {
        _LECIEW { w: self }
    }
    #[doc = "Bit 3 - Alert Interrupt Enable"]
    #[inline]
    pub fn alie(&mut self) -> _ALIEW {
        _ALIEW { w: self }
    }
    #[doc = "Bit 4 - CAN Disable"]
    #[inline]
    pub fn candis(&mut self) -> _CANDISW {
        _CANDISW { w: self }
    }
    #[doc = "Bit 5 - Transmit Disable"]
    #[inline]
    pub fn txdis(&mut self) -> _TXDISW {
        _TXDISW { w: self }
    }
    #[doc = "Bit 6 - Configuration Change Enable"]
    #[inline]
    pub fn cce(&mut self) -> _CCEW {
        _CCEW { w: self }
    }
    #[doc = "Bit 7 - CAN Analyzer Mode"]
    #[inline]
    pub fn calm(&mut self) -> _CALMW {
        _CALMW { w: self }
    }
}
