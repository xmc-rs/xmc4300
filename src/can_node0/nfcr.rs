#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::NFCR {
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
pub struct CFCR {
    bits: u16,
}
impl CFCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CFSELR {
    bits: u8,
}
impl CFSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CFMOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFMODR {
    #[doc = "Frame Count Mode: The frame counter is incremented upon the reception and transmission of frames."]
    VALUE1,
    #[doc = "Time Stamp Mode: The frame counter is used to count bit times."]
    VALUE2,
    #[doc = "Bit Timing Mode: The frame counter is used for analysis of the bit timing."]
    VALUE3,
    #[doc = "Error Count Mode: The frame counter is used for counting when an error frame is received or an error is detected by the node."]
    VALUE4,
}
impl CFMODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CFMODR::VALUE1 => 0,
            CFMODR::VALUE2 => 1,
            CFMODR::VALUE3 => 2,
            CFMODR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CFMODR {
        match value {
            0 => CFMODR::VALUE1,
            1 => CFMODR::VALUE2,
            2 => CFMODR::VALUE3,
            3 => CFMODR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CFMODR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CFMODR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CFMODR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == CFMODR::VALUE4
    }
}
#[doc = "Possible values of the field `CFCIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFCIER {
    #[doc = "CAN frame counter overflow interrupt is disabled."]
    VALUE1,
    #[doc = "CAN frame counter overflow interrupt is enabled."]
    VALUE2,
}
impl CFCIER {
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
            CFCIER::VALUE1 => false,
            CFCIER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CFCIER {
        match value {
            false => CFCIER::VALUE1,
            true => CFCIER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CFCIER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CFCIER::VALUE2
    }
}
#[doc = "Possible values of the field `CFCOV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFCOVR {
    #[doc = "No overflow has occurred since last flag reset."]
    VALUE1,
    #[doc = "An overflow has occurred since last flag reset."]
    VALUE2,
}
impl CFCOVR {
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
            CFCOVR::VALUE1 => false,
            CFCOVR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CFCOVR {
        match value {
            false => CFCOVR::VALUE1,
            true => CFCOVR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CFCOVR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CFCOVR::VALUE2
    }
}
#[doc = r" Proxy"]
pub struct _CFCW<'a> {
    w: &'a mut W,
}
impl<'a> _CFCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CFSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CFSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CFMOD`"]
pub enum CFMODW {
    #[doc = "Frame Count Mode: The frame counter is incremented upon the reception and transmission of frames."]
    VALUE1,
    #[doc = "Time Stamp Mode: The frame counter is used to count bit times."]
    VALUE2,
    #[doc = "Bit Timing Mode: The frame counter is used for analysis of the bit timing."]
    VALUE3,
    #[doc = "Error Count Mode: The frame counter is used for counting when an error frame is received or an error is detected by the node."]
    VALUE4,
}
impl CFMODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFMODW::VALUE1 => 0,
            CFMODW::VALUE2 => 1,
            CFMODW::VALUE3 => 2,
            CFMODW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFMODW<'a> {
    w: &'a mut W,
}
impl<'a> _CFMODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFMODW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Frame Count Mode: The frame counter is incremented upon the reception and transmission of frames."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CFMODW::VALUE1)
    }
    #[doc = "Time Stamp Mode: The frame counter is used to count bit times."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CFMODW::VALUE2)
    }
    #[doc = "Bit Timing Mode: The frame counter is used for analysis of the bit timing."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CFMODW::VALUE3)
    }
    #[doc = "Error Count Mode: The frame counter is used for counting when an error frame is received or an error is detected by the node."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(CFMODW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CFCIE`"]
pub enum CFCIEW {
    #[doc = "CAN frame counter overflow interrupt is disabled."]
    VALUE1,
    #[doc = "CAN frame counter overflow interrupt is enabled."]
    VALUE2,
}
impl CFCIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CFCIEW::VALUE1 => false,
            CFCIEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFCIEW<'a> {
    w: &'a mut W,
}
impl<'a> _CFCIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFCIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CAN frame counter overflow interrupt is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CFCIEW::VALUE1)
    }
    #[doc = "CAN frame counter overflow interrupt is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CFCIEW::VALUE2)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CFCOV`"]
pub enum CFCOVW {
    #[doc = "No overflow has occurred since last flag reset."]
    VALUE1,
    #[doc = "An overflow has occurred since last flag reset."]
    VALUE2,
}
impl CFCOVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CFCOVW::VALUE1 => false,
            CFCOVW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFCOVW<'a> {
    w: &'a mut W,
}
impl<'a> _CFCOVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFCOVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No overflow has occurred since last flag reset."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CFCOVW::VALUE1)
    }
    #[doc = "An overflow has occurred since last flag reset."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CFCOVW::VALUE2)
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
        const OFFSET: u8 = 23;
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
    #[doc = "Bits 0:15 - CAN Frame Counter"]
    #[inline]
    pub fn cfc(&self) -> CFCR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        CFCR { bits }
    }
    #[doc = "Bits 16:18 - CAN Frame Count Selection"]
    #[inline]
    pub fn cfsel(&self) -> CFSELR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CFSELR { bits }
    }
    #[doc = "Bits 19:20 - CAN Frame Counter Mode"]
    #[inline]
    pub fn cfmod(&self) -> CFMODR {
        CFMODR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 22 - CAN Frame Count Interrupt Enable"]
    #[inline]
    pub fn cfcie(&self) -> CFCIER {
        CFCIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - CAN Frame Counter Overflow Flag"]
    #[inline]
    pub fn cfcov(&self) -> CFCOVR {
        CFCOVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
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
    #[doc = "Bits 0:15 - CAN Frame Counter"]
    #[inline]
    pub fn cfc(&mut self) -> _CFCW {
        _CFCW { w: self }
    }
    #[doc = "Bits 16:18 - CAN Frame Count Selection"]
    #[inline]
    pub fn cfsel(&mut self) -> _CFSELW {
        _CFSELW { w: self }
    }
    #[doc = "Bits 19:20 - CAN Frame Counter Mode"]
    #[inline]
    pub fn cfmod(&mut self) -> _CFMODW {
        _CFMODW { w: self }
    }
    #[doc = "Bit 22 - CAN Frame Count Interrupt Enable"]
    #[inline]
    pub fn cfcie(&mut self) -> _CFCIEW {
        _CFCIEW { w: self }
    }
    #[doc = "Bit 23 - CAN Frame Counter Overflow Flag"]
    #[inline]
    pub fn cfcov(&mut self) -> _CFCOVW {
        _CFCOVW { w: self }
    }
}
