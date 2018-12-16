#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MOAR {
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
pub struct IDR {
    bits: u32,
}
impl IDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Possible values of the field `IDE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDER {
    #[doc = "Message object n handles standard frames with 11-bit identifier."]
    VALUE1,
    #[doc = "Message object n handles extended frames with 29-bit identifier."]
    VALUE2,
}
impl IDER {
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
            IDER::VALUE1 => false,
            IDER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IDER {
        match value {
            false => IDER::VALUE1,
            true => IDER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == IDER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == IDER::VALUE2
    }
}
#[doc = "Possible values of the field `PRI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRIR {
    #[doc = "Transmit acceptance filtering is based on the list order. This means that message object n is considered for transmission only if there is no other message object with valid transmit request (MSGVAL & TXEN0 & TXEN1 = 1) somewhere before this object in the list."]
    VALUE2,
    #[doc = "Transmit acceptance filtering is based on the CAN identifier. This means, message object n is considered for transmission only if there is no other message object with higher priority identifier + IDE + DIR (with respect to CAN arbitration rules) somewhere in the list (see )."]
    VALUE3,
    #[doc = "Transmit acceptance filtering is based on the list order (as PRI = 01B)."]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PRIR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRIR::VALUE2 => 1,
            PRIR::VALUE3 => 2,
            PRIR::VALUE4 => 3,
            PRIR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRIR {
        match value {
            1 => PRIR::VALUE2,
            2 => PRIR::VALUE3,
            3 => PRIR::VALUE4,
            i => PRIR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PRIR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PRIR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == PRIR::VALUE4
    }
}
#[doc = r" Proxy"]
pub struct _IDW<'a> {
    w: &'a mut W,
}
impl<'a> _IDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 536870911;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IDE`"]
pub enum IDEW {
    #[doc = "Message object n handles standard frames with 11-bit identifier."]
    VALUE1,
    #[doc = "Message object n handles extended frames with 29-bit identifier."]
    VALUE2,
}
impl IDEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IDEW::VALUE1 => false,
            IDEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDEW<'a> {
    w: &'a mut W,
}
impl<'a> _IDEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Message object n handles standard frames with 11-bit identifier."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(IDEW::VALUE1)
    }
    #[doc = "Message object n handles extended frames with 29-bit identifier."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(IDEW::VALUE2)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRI`"]
pub enum PRIW {
    #[doc = "Transmit acceptance filtering is based on the list order. This means that message object n is considered for transmission only if there is no other message object with valid transmit request (MSGVAL & TXEN0 & TXEN1 = 1) somewhere before this object in the list."]
    VALUE2,
    #[doc = "Transmit acceptance filtering is based on the CAN identifier. This means, message object n is considered for transmission only if there is no other message object with higher priority identifier + IDE + DIR (with respect to CAN arbitration rules) somewhere in the list (see )."]
    VALUE3,
    #[doc = "Transmit acceptance filtering is based on the list order (as PRI = 01B)."]
    VALUE4,
}
impl PRIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRIW::VALUE2 => 1,
            PRIW::VALUE3 => 2,
            PRIW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRIW<'a> {
    w: &'a mut W,
}
impl<'a> _PRIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRIW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Transmit acceptance filtering is based on the list order. This means that message object n is considered for transmission only if there is no other message object with valid transmit request (MSGVAL & TXEN0 & TXEN1 = 1) somewhere before this object in the list."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRIW::VALUE2)
    }
    #[doc = "Transmit acceptance filtering is based on the CAN identifier. This means, message object n is considered for transmission only if there is no other message object with higher priority identifier + IDE + DIR (with respect to CAN arbitration rules) somewhere in the list (see )."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(PRIW::VALUE3)
    }
    #[doc = "Transmit acceptance filtering is based on the list order (as PRI = 01B)."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(PRIW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:28 - CAN Identifier of Message Object n"]
    #[inline]
    pub fn id(&self) -> IDR {
        let bits = {
            const MASK: u32 = 536870911;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        IDR { bits }
    }
    #[doc = "Bit 29 - Identifier Extension Bit of Message Object n"]
    #[inline]
    pub fn ide(&self) -> IDER {
        IDER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 30:31 - Priority Class"]
    #[inline]
    pub fn pri(&self) -> PRIR {
        PRIR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 0:28 - CAN Identifier of Message Object n"]
    #[inline]
    pub fn id(&mut self) -> _IDW {
        _IDW { w: self }
    }
    #[doc = "Bit 29 - Identifier Extension Bit of Message Object n"]
    #[inline]
    pub fn ide(&mut self) -> _IDEW {
        _IDEW { w: self }
    }
    #[doc = "Bits 30:31 - Priority Class"]
    #[inline]
    pub fn pri(&mut self) -> _PRIW {
        _PRIW { w: self }
    }
}
