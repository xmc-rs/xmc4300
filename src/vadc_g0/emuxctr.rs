#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EMUXCTR {
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
pub struct EMUXSETR {
    bits: u8,
}
impl EMUXSETR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EMUXACTR {
    bits: u8,
}
impl EMUXACTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EMUXCHR {
    bits: u16,
}
impl EMUXCHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `EMUXMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMUXMODER {
    #[doc = "Software control (no hardware action)"]
    VALUE1,
    #[doc = "Steady mode (use EMUXSET value)"]
    VALUE2,
    #[doc = "Single-step mode"]
    VALUE3,
    #[doc = "Sequence mode"]
    VALUE4,
}
impl EMUXMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EMUXMODER::VALUE1 => 0,
            EMUXMODER::VALUE2 => 1,
            EMUXMODER::VALUE3 => 2,
            EMUXMODER::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EMUXMODER {
        match value {
            0 => EMUXMODER::VALUE1,
            1 => EMUXMODER::VALUE2,
            2 => EMUXMODER::VALUE3,
            3 => EMUXMODER::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EMUXMODER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EMUXMODER::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == EMUXMODER::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == EMUXMODER::VALUE4
    }
}
#[doc = "Possible values of the field `EMXCOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMXCODR {
    #[doc = "Output the channel number in binary code"]
    VALUE1,
    #[doc = "Output the channel number in Gray code"]
    VALUE2,
}
impl EMXCODR {
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
            EMXCODR::VALUE1 => false,
            EMXCODR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EMXCODR {
        match value {
            false => EMXCODR::VALUE1,
            true => EMXCODR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EMXCODR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EMXCODR::VALUE2
    }
}
#[doc = "Possible values of the field `EMXST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMXSTR {
    #[doc = "Use STCE whenever the setting changes"]
    VALUE1,
    #[doc = "Use STCE for each conversion of an external channel"]
    VALUE2,
}
impl EMXSTR {
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
            EMXSTR::VALUE1 => false,
            EMXSTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EMXSTR {
        match value {
            false => EMXSTR::VALUE1,
            true => EMXSTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EMXSTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EMXSTR::VALUE2
    }
}
#[doc = "Possible values of the field `EMXCSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMXCSSR {
    #[doc = "Channel number: Bitfield EMUXCH selects an arbitrary channel"]
    VALUE1,
    #[doc = "Channel enable: Each bit of bitfield EMUXCH selects the associated channel for EMUX control"]
    VALUE2,
}
impl EMXCSSR {
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
            EMXCSSR::VALUE1 => false,
            EMXCSSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EMXCSSR {
        match value {
            false => EMXCSSR::VALUE1,
            true => EMXCSSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EMXCSSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EMXCSSR::VALUE2
    }
}
#[doc = r" Proxy"]
pub struct _EMUXSETW<'a> {
    w: &'a mut W,
}
impl<'a> _EMUXSETW<'a> {
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
#[doc = r" Proxy"]
pub struct _EMUXCHW<'a> {
    w: &'a mut W,
}
impl<'a> _EMUXCHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EMUXMODE`"]
pub enum EMUXMODEW {
    #[doc = "Software control (no hardware action)"]
    VALUE1,
    #[doc = "Steady mode (use EMUXSET value)"]
    VALUE2,
    #[doc = "Single-step mode"]
    VALUE3,
    #[doc = "Sequence mode"]
    VALUE4,
}
impl EMUXMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EMUXMODEW::VALUE1 => 0,
            EMUXMODEW::VALUE2 => 1,
            EMUXMODEW::VALUE3 => 2,
            EMUXMODEW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EMUXMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _EMUXMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EMUXMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Software control (no hardware action)"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EMUXMODEW::VALUE1)
    }
    #[doc = "Steady mode (use EMUXSET value)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EMUXMODEW::VALUE2)
    }
    #[doc = "Single-step mode"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(EMUXMODEW::VALUE3)
    }
    #[doc = "Sequence mode"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(EMUXMODEW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EMXCOD`"]
pub enum EMXCODW {
    #[doc = "Output the channel number in binary code"]
    VALUE1,
    #[doc = "Output the channel number in Gray code"]
    VALUE2,
}
impl EMXCODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EMXCODW::VALUE1 => false,
            EMXCODW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EMXCODW<'a> {
    w: &'a mut W,
}
impl<'a> _EMXCODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EMXCODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Output the channel number in binary code"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EMXCODW::VALUE1)
    }
    #[doc = "Output the channel number in Gray code"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EMXCODW::VALUE2)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EMXST`"]
pub enum EMXSTW {
    #[doc = "Use STCE whenever the setting changes"]
    VALUE1,
    #[doc = "Use STCE for each conversion of an external channel"]
    VALUE2,
}
impl EMXSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EMXSTW::VALUE1 => false,
            EMXSTW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EMXSTW<'a> {
    w: &'a mut W,
}
impl<'a> _EMXSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EMXSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use STCE whenever the setting changes"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EMXSTW::VALUE1)
    }
    #[doc = "Use STCE for each conversion of an external channel"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EMXSTW::VALUE2)
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
#[doc = "Values that can be written to the field `EMXWC`"]
pub enum EMXWCW {
    #[doc = "No write access to EMUX cfg."]
    VALUE1,
    #[doc = "Bitfields EMXMODE, EMXCOD, EMXST, EMXCSS can be written"]
    VALUE2,
}
impl EMXWCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EMXWCW::VALUE1 => false,
            EMXWCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EMXWCW<'a> {
    w: &'a mut W,
}
impl<'a> _EMXWCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EMXWCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No write access to EMUX cfg."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EMXWCW::VALUE1)
    }
    #[doc = "Bitfields EMXMODE, EMXCOD, EMXST, EMXCSS can be written"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EMXWCW::VALUE2)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:2 - External Multiplexer Start Selection"]
    #[inline]
    pub fn emuxset(&self) -> EMUXSETR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EMUXSETR { bits }
    }
    #[doc = "Bits 8:10 - External Multiplexer Actual Selection"]
    #[inline]
    pub fn emuxact(&self) -> EMUXACTR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EMUXACTR { bits }
    }
    #[doc = "Bits 16:25 - External Multiplexer Channel Select"]
    #[inline]
    pub fn emuxch(&self) -> EMUXCHR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        EMUXCHR { bits }
    }
    #[doc = "Bits 26:27 - External Multiplexer Mode"]
    #[inline]
    pub fn emuxmode(&self) -> EMUXMODER {
        EMUXMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 28 - External Multiplexer Coding Scheme"]
    #[inline]
    pub fn emxcod(&self) -> EMXCODR {
        EMXCODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - External Multiplexer Sample Time Control"]
    #[inline]
    pub fn emxst(&self) -> EMXSTR {
        EMXSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - External Multiplexer Channel Selection Style"]
    #[inline]
    pub fn emxcss(&self) -> EMXCSSR {
        EMXCSSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:2 - External Multiplexer Start Selection"]
    #[inline]
    pub fn emuxset(&mut self) -> _EMUXSETW {
        _EMUXSETW { w: self }
    }
    #[doc = "Bits 16:25 - External Multiplexer Channel Select"]
    #[inline]
    pub fn emuxch(&mut self) -> _EMUXCHW {
        _EMUXCHW { w: self }
    }
    #[doc = "Bits 26:27 - External Multiplexer Mode"]
    #[inline]
    pub fn emuxmode(&mut self) -> _EMUXMODEW {
        _EMUXMODEW { w: self }
    }
    #[doc = "Bit 28 - External Multiplexer Coding Scheme"]
    #[inline]
    pub fn emxcod(&mut self) -> _EMXCODW {
        _EMXCODW { w: self }
    }
    #[doc = "Bit 29 - External Multiplexer Sample Time Control"]
    #[inline]
    pub fn emxst(&mut self) -> _EMXSTW {
        _EMXSTW { w: self }
    }
    #[doc = "Bit 31 - Write Control for EMUX Configuration"]
    #[inline]
    pub fn emxwc(&mut self) -> _EMXWCW {
        _EMXWCW { w: self }
    }
}
