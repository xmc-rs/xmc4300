#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::NSR {
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
pub struct LECR {
    bits: u8,
}
impl LECR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `TXOK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXOKR {
    #[doc = "No successful transmission since last (most recent) flag reset."]
    VALUE1,
    #[doc = "A message has been transmitted successfully (error-free and acknowledged by at least another node)."]
    VALUE2,
}
impl TXOKR {
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
            TXOKR::VALUE1 => false,
            TXOKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXOKR {
        match value {
            false => TXOKR::VALUE1,
            true => TXOKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TXOKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TXOKR::VALUE2
    }
}
#[doc = "Possible values of the field `RXOK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXOKR {
    #[doc = "No successful reception since last (most recent) flag reset."]
    VALUE1,
    #[doc = "A message has been received successfully."]
    VALUE2,
}
impl RXOKR {
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
            RXOKR::VALUE1 => false,
            RXOKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXOKR {
        match value {
            false => RXOKR::VALUE1,
            true => RXOKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RXOKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RXOKR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct ALERTR {
    bits: bool,
}
impl ALERTR {
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
#[doc = "Possible values of the field `EWRN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWRNR {
    #[doc = "No warning limit exceeded."]
    VALUE1,
    #[doc = "One of the error counters REC or TEC reached the warning limit EWRNLVL."]
    VALUE2,
}
impl EWRNR {
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
            EWRNR::VALUE1 => false,
            EWRNR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EWRNR {
        match value {
            false => EWRNR::VALUE1,
            true => EWRNR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EWRNR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EWRNR::VALUE2
    }
}
#[doc = "Possible values of the field `BOFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOFFR {
    #[doc = "CAN controller is not in the bus-off state."]
    VALUE1,
    #[doc = "CAN controller is in the bus-off state."]
    VALUE2,
}
impl BOFFR {
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
            BOFFR::VALUE1 => false,
            BOFFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BOFFR {
        match value {
            false => BOFFR::VALUE1,
            true => BOFFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BOFFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BOFFR::VALUE2
    }
}
#[doc = "Possible values of the field `LLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLER {
    #[doc = "No List Length Error since last (most recent) flag reset."]
    VALUE1,
    #[doc = "A List Length Error has been detected during message acceptance filtering. The number of elements in the list that belongs to this CAN node differs from the list SIZE given in the list termination pointer."]
    VALUE2,
}
impl LLER {
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
            LLER::VALUE1 => false,
            LLER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LLER {
        match value {
            false => LLER::VALUE1,
            true => LLER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LLER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LLER::VALUE2
    }
}
#[doc = "Possible values of the field `LOE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOER {
    #[doc = "No List Object Error since last (most recent) flag reset."]
    VALUE1,
    #[doc = "A List Object Error has been detected during message acceptance filtering. A message object with wrong LIST index entry in the Message Object Status Register has been detected."]
    VALUE2,
}
impl LOER {
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
            LOER::VALUE1 => false,
            LOER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOER {
        match value {
            false => LOER::VALUE1,
            true => LOER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LOER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LOER::VALUE2
    }
}
#[doc = r" Proxy"]
pub struct _LECW<'a> {
    w: &'a mut W,
}
impl<'a> _LECW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXOK`"]
pub enum TXOKW {
    #[doc = "No successful transmission since last (most recent) flag reset."]
    VALUE1,
    #[doc = "A message has been transmitted successfully (error-free and acknowledged by at least another node)."]
    VALUE2,
}
impl TXOKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXOKW::VALUE1 => false,
            TXOKW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXOKW<'a> {
    w: &'a mut W,
}
impl<'a> _TXOKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXOKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No successful transmission since last (most recent) flag reset."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TXOKW::VALUE1)
    }
    #[doc = "A message has been transmitted successfully (error-free and acknowledged by at least another node)."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TXOKW::VALUE2)
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
#[doc = "Values that can be written to the field `RXOK`"]
pub enum RXOKW {
    #[doc = "No successful reception since last (most recent) flag reset."]
    VALUE1,
    #[doc = "A message has been received successfully."]
    VALUE2,
}
impl RXOKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXOKW::VALUE1 => false,
            RXOKW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXOKW<'a> {
    w: &'a mut W,
}
impl<'a> _RXOKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXOKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No successful reception since last (most recent) flag reset."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXOKW::VALUE1)
    }
    #[doc = "A message has been received successfully."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RXOKW::VALUE2)
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
#[doc = r" Proxy"]
pub struct _ALERTW<'a> {
    w: &'a mut W,
}
impl<'a> _ALERTW<'a> {
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
#[doc = "Values that can be written to the field `LLE`"]
pub enum LLEW {
    #[doc = "No List Length Error since last (most recent) flag reset."]
    VALUE1,
    #[doc = "A List Length Error has been detected during message acceptance filtering. The number of elements in the list that belongs to this CAN node differs from the list SIZE given in the list termination pointer."]
    VALUE2,
}
impl LLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LLEW::VALUE1 => false,
            LLEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LLEW<'a> {
    w: &'a mut W,
}
impl<'a> _LLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No List Length Error since last (most recent) flag reset."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LLEW::VALUE1)
    }
    #[doc = "A List Length Error has been detected during message acceptance filtering. The number of elements in the list that belongs to this CAN node differs from the list SIZE given in the list termination pointer."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LLEW::VALUE2)
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
#[doc = "Values that can be written to the field `LOE`"]
pub enum LOEW {
    #[doc = "No List Object Error since last (most recent) flag reset."]
    VALUE1,
    #[doc = "A List Object Error has been detected during message acceptance filtering. A message object with wrong LIST index entry in the Message Object Status Register has been detected."]
    VALUE2,
}
impl LOEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOEW::VALUE1 => false,
            LOEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOEW<'a> {
    w: &'a mut W,
}
impl<'a> _LOEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No List Object Error since last (most recent) flag reset."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LOEW::VALUE1)
    }
    #[doc = "A List Object Error has been detected during message acceptance filtering. A message object with wrong LIST index entry in the Message Object Status Register has been detected."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LOEW::VALUE2)
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
    #[doc = "Bits 0:2 - Last Error Code"]
    #[inline]
    pub fn lec(&self) -> LECR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LECR { bits }
    }
    #[doc = "Bit 3 - Message Transmitted Successfully"]
    #[inline]
    pub fn txok(&self) -> TXOKR {
        TXOKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Message Received Successfully"]
    #[inline]
    pub fn rxok(&self) -> RXOKR {
        RXOKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Alert Warning"]
    #[inline]
    pub fn alert(&self) -> ALERTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ALERTR { bits }
    }
    #[doc = "Bit 6 - Error Warning Status"]
    #[inline]
    pub fn ewrn(&self) -> EWRNR {
        EWRNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Bus-off Status"]
    #[inline]
    pub fn boff(&self) -> BOFFR {
        BOFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - List Length Error"]
    #[inline]
    pub fn lle(&self) -> LLER {
        LLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - List Object Error"]
    #[inline]
    pub fn loe(&self) -> LOER {
        LOER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
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
    #[doc = "Bits 0:2 - Last Error Code"]
    #[inline]
    pub fn lec(&mut self) -> _LECW {
        _LECW { w: self }
    }
    #[doc = "Bit 3 - Message Transmitted Successfully"]
    #[inline]
    pub fn txok(&mut self) -> _TXOKW {
        _TXOKW { w: self }
    }
    #[doc = "Bit 4 - Message Received Successfully"]
    #[inline]
    pub fn rxok(&mut self) -> _RXOKW {
        _RXOKW { w: self }
    }
    #[doc = "Bit 5 - Alert Warning"]
    #[inline]
    pub fn alert(&mut self) -> _ALERTW {
        _ALERTW { w: self }
    }
    #[doc = "Bit 8 - List Length Error"]
    #[inline]
    pub fn lle(&mut self) -> _LLEW {
        _LLEW { w: self }
    }
    #[doc = "Bit 9 - List Object Error"]
    #[inline]
    pub fn loe(&mut self) -> _LOEW {
        _LOEW { w: self }
    }
}
