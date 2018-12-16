#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GRSTCTL {
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
pub struct CSFTRSTR {
    bits: bool,
}
impl CSFTRSTR {
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
pub struct FRMCNTRRSTR {
    bits: bool,
}
impl FRMCNTRRSTR {
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
pub struct RXFFLSHR {
    bits: bool,
}
impl RXFFLSHR {
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
pub struct TXFFLSHR {
    bits: bool,
}
impl TXFFLSHR {
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
#[doc = "Possible values of the field `TxFNum`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFNUMR {
    #[doc = "Non-periodic TxFIFO flush in Host mode or Tx FIFO 0 flush in device mode"]
    VALUE1,
    #[doc = "Periodic TxFIFO flush in Host mode or Tx FIFO 1 flush in device mode"]
    VALUE2,
    #[doc = "Tx FIFO 2 flush in device mode"]
    VALUE3,
    #[doc = "Tx FIFO 15 flush in device mode"]
    VALUE4,
    #[doc = "Flush all the transmit FIFOs in device or host mode."]
    VALUE5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TXFNUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TXFNUMR::VALUE1 => 0,
            TXFNUMR::VALUE2 => 1,
            TXFNUMR::VALUE3 => 2,
            TXFNUMR::VALUE4 => 15,
            TXFNUMR::VALUE5 => 16,
            TXFNUMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TXFNUMR {
        match value {
            0 => TXFNUMR::VALUE1,
            1 => TXFNUMR::VALUE2,
            2 => TXFNUMR::VALUE3,
            15 => TXFNUMR::VALUE4,
            16 => TXFNUMR::VALUE5,
            i => TXFNUMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TXFNUMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TXFNUMR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == TXFNUMR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == TXFNUMR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == TXFNUMR::VALUE5
    }
}
#[doc = r" Value of the field"]
pub struct DMAREQR {
    bits: bool,
}
impl DMAREQR {
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
pub struct AHBIDLER {
    bits: bool,
}
impl AHBIDLER {
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
#[doc = r" Proxy"]
pub struct _CSFTRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _CSFTRSTW<'a> {
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
#[doc = r" Proxy"]
pub struct _FRMCNTRRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _FRMCNTRRSTW<'a> {
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
#[doc = r" Proxy"]
pub struct _RXFFLSHW<'a> {
    w: &'a mut W,
}
impl<'a> _RXFFLSHW<'a> {
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
pub struct _TXFFLSHW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFFLSHW<'a> {
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
#[doc = "Values that can be written to the field `TxFNum`"]
pub enum TXFNUMW {
    #[doc = "Non-periodic TxFIFO flush in Host mode or Tx FIFO 0 flush in device mode"]
    VALUE1,
    #[doc = "Periodic TxFIFO flush in Host mode or Tx FIFO 1 flush in device mode"]
    VALUE2,
    #[doc = "Tx FIFO 2 flush in device mode"]
    VALUE3,
    #[doc = "Tx FIFO 15 flush in device mode"]
    VALUE4,
    #[doc = "Flush all the transmit FIFOs in device or host mode."]
    VALUE5,
}
impl TXFNUMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TXFNUMW::VALUE1 => 0,
            TXFNUMW::VALUE2 => 1,
            TXFNUMW::VALUE3 => 2,
            TXFNUMW::VALUE4 => 15,
            TXFNUMW::VALUE5 => 16,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXFNUMW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFNUMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXFNUMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Non-periodic TxFIFO flush in Host mode or Tx FIFO 0 flush in device mode"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TXFNUMW::VALUE1)
    }
    #[doc = "Periodic TxFIFO flush in Host mode or Tx FIFO 1 flush in device mode"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TXFNUMW::VALUE2)
    }
    #[doc = "Tx FIFO 2 flush in device mode"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(TXFNUMW::VALUE3)
    }
    #[doc = "Tx FIFO 15 flush in device mode"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(TXFNUMW::VALUE4)
    }
    #[doc = "Flush all the transmit FIFOs in device or host mode."]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(TXFNUMW::VALUE5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 6;
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
    #[doc = "Bit 0 - Core Soft Reset"]
    #[inline]
    pub fn csft_rst(&self) -> CSFTRSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CSFTRSTR { bits }
    }
    #[doc = "Bit 2 - Host Frame Counter Reset"]
    #[inline]
    pub fn frm_cntr_rst(&self) -> FRMCNTRRSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FRMCNTRRSTR { bits }
    }
    #[doc = "Bit 4 - RxFIFO Flush"]
    #[inline]
    pub fn rx_fflsh(&self) -> RXFFLSHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXFFLSHR { bits }
    }
    #[doc = "Bit 5 - TxFIFO Flush"]
    #[inline]
    pub fn tx_fflsh(&self) -> TXFFLSHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXFFLSHR { bits }
    }
    #[doc = "Bits 6:10 - TxFIFO Number"]
    #[inline]
    pub fn tx_fnum(&self) -> TXFNUMR {
        TXFNUMR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 30 - DMA Request Signal"]
    #[inline]
    pub fn dmareq(&self) -> DMAREQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMAREQR { bits }
    }
    #[doc = "Bit 31 - AHB Master Idle"]
    #[inline]
    pub fn ahbidle(&self) -> AHBIDLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AHBIDLER { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 268435456 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Core Soft Reset"]
    #[inline]
    pub fn csft_rst(&mut self) -> _CSFTRSTW {
        _CSFTRSTW { w: self }
    }
    #[doc = "Bit 2 - Host Frame Counter Reset"]
    #[inline]
    pub fn frm_cntr_rst(&mut self) -> _FRMCNTRRSTW {
        _FRMCNTRRSTW { w: self }
    }
    #[doc = "Bit 4 - RxFIFO Flush"]
    #[inline]
    pub fn rx_fflsh(&mut self) -> _RXFFLSHW {
        _RXFFLSHW { w: self }
    }
    #[doc = "Bit 5 - TxFIFO Flush"]
    #[inline]
    pub fn tx_fflsh(&mut self) -> _TXFFLSHW {
        _TXFFLSHW { w: self }
    }
    #[doc = "Bits 6:10 - TxFIFO Number"]
    #[inline]
    pub fn tx_fnum(&mut self) -> _TXFNUMW {
        _TXFNUMW { w: self }
    }
}
