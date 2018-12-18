#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFGH {
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
pub struct DEST_PERR {
    bits: u8,
}
impl DEST_PERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SRC_PERR {
    bits: u8,
}
impl SRC_PERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PROTCTLR {
    bits: u8,
}
impl PROTCTLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `FIFO_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFO_MODER {
    #[doc = "Space/data available for single AHB transfer of the specified transfer width."]
    VALUE1,
    #[doc = "Data available is greater than or equal to half the FIFO depth for destination transfers and space available is greater than half the fifo depth for source transfers. The exceptions are at the end of a burst transaction request or at the end of a block transfer."]
    VALUE2,
}
impl FIFO_MODER {
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
            FIFO_MODER::VALUE1 => false,
            FIFO_MODER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FIFO_MODER {
        match value {
            false => FIFO_MODER::VALUE1,
            true => FIFO_MODER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == FIFO_MODER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == FIFO_MODER::VALUE2
    }
}
#[doc = "Possible values of the field `FCMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCMODER {
    #[doc = "Source transaction requests are serviced when they occur. Data pre-fetching is enabled."]
    VALUE1,
    #[doc = "Source transaction requests are not serviced until a destination transaction request occurs. In this mode, the amount of data transferred from the source is limited so that it is guaranteed to be transferred to the destination prior to block termination by the destination. Data pre-fetching is disabled."]
    VALUE2,
}
impl FCMODER {
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
            FCMODER::VALUE1 => false,
            FCMODER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FCMODER {
        match value {
            false => FCMODER::VALUE1,
            true => FCMODER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == FCMODER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == FCMODER::VALUE2
    }
}
#[doc = r" Proxy"]
pub struct _DEST_PERW<'a> {
    w: &'a mut W,
}
impl<'a> _DEST_PERW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SRC_PERW<'a> {
    w: &'a mut W,
}
impl<'a> _SRC_PERW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PROTCTLW<'a> {
    w: &'a mut W,
}
impl<'a> _PROTCTLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FIFO_MODE`"]
pub enum FIFO_MODEW {
    #[doc = "Space/data available for single AHB transfer of the specified transfer width."]
    VALUE1,
    #[doc = "Data available is greater than or equal to half the FIFO depth for destination transfers and space available is greater than half the fifo depth for source transfers. The exceptions are at the end of a burst transaction request or at the end of a block transfer."]
    VALUE2,
}
impl FIFO_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FIFO_MODEW::VALUE1 => false,
            FIFO_MODEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FIFO_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _FIFO_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FIFO_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Space/data available for single AHB transfer of the specified transfer width."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FIFO_MODEW::VALUE1)
    }
    #[doc = "Data available is greater than or equal to half the FIFO depth for destination transfers and space available is greater than half the fifo depth for source transfers. The exceptions are at the end of a burst transaction request or at the end of a block transfer."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FIFO_MODEW::VALUE2)
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
#[doc = "Values that can be written to the field `FCMODE`"]
pub enum FCMODEW {
    #[doc = "Source transaction requests are serviced when they occur. Data pre-fetching is enabled."]
    VALUE1,
    #[doc = "Source transaction requests are not serviced until a destination transaction request occurs. In this mode, the amount of data transferred from the source is limited so that it is guaranteed to be transferred to the destination prior to block termination by the destination. Data pre-fetching is disabled."]
    VALUE2,
}
impl FCMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FCMODEW::VALUE1 => false,
            FCMODEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FCMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _FCMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FCMODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Source transaction requests are serviced when they occur. Data pre-fetching is enabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FCMODEW::VALUE1)
    }
    #[doc = "Source transaction requests are not serviced until a destination transaction request occurs. In this mode, the amount of data transferred from the source is limited so that it is guaranteed to be transferred to the destination prior to block termination by the destination. Data pre-fetching is disabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FCMODEW::VALUE2)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 11:14 - Destination Peripheral"]
    #[inline]
    pub fn dest_per(&self) -> DEST_PERR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DEST_PERR { bits }
    }
    #[doc = "Bits 7:10 - Source Peripheral"]
    #[inline]
    pub fn src_per(&self) -> SRC_PERR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SRC_PERR { bits }
    }
    #[doc = "Bits 2:4 - Protection Control"]
    #[inline]
    pub fn protctl(&self) -> PROTCTLR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PROTCTLR { bits }
    }
    #[doc = "Bit 1 - FIFO Mode Select"]
    #[inline]
    pub fn fifo_mode(&self) -> FIFO_MODER {
        FIFO_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Flow Control Mode"]
    #[inline]
    pub fn fcmode(&self) -> FCMODER {
        FCMODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 11:14 - Destination Peripheral"]
    #[inline]
    pub fn dest_per(&mut self) -> _DEST_PERW {
        _DEST_PERW { w: self }
    }
    #[doc = "Bits 7:10 - Source Peripheral"]
    #[inline]
    pub fn src_per(&mut self) -> _SRC_PERW {
        _SRC_PERW { w: self }
    }
    #[doc = "Bits 2:4 - Protection Control"]
    #[inline]
    pub fn protctl(&mut self) -> _PROTCTLW {
        _PROTCTLW { w: self }
    }
    #[doc = "Bit 1 - FIFO Mode Select"]
    #[inline]
    pub fn fifo_mode(&mut self) -> _FIFO_MODEW {
        _FIFO_MODEW { w: self }
    }
    #[doc = "Bit 0 - Flow Control Mode"]
    #[inline]
    pub fn fcmode(&mut self) -> _FCMODEW {
        _FCMODEW { w: self }
    }
}
