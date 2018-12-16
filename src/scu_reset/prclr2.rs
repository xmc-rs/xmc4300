#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PRCLR2 {
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
}
#[doc = "Values that can be written to the field `WDTRS`"]
pub enum WDTRSW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "De-assert reset"]
    CONST_1,
}
impl WDTRSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WDTRSW::CONST_0 => false,
            WDTRSW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WDTRSW<'a> {
    w: &'a mut W,
}
impl<'a> _WDTRSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDTRSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(WDTRSW::CONST_0)
    }
    #[doc = "De-assert reset"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(WDTRSW::CONST_1)
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
#[doc = "Values that can be written to the field `ETH0RS`"]
pub enum ETH0RSW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "De-assert reset"]
    CONST_1,
}
impl ETH0RSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ETH0RSW::CONST_0 => false,
            ETH0RSW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ETH0RSW<'a> {
    w: &'a mut W,
}
impl<'a> _ETH0RSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ETH0RSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ETH0RSW::CONST_0)
    }
    #[doc = "De-assert reset"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ETH0RSW::CONST_1)
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
#[doc = "Values that can be written to the field `DMA0RS`"]
pub enum DMA0RSW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "De-assert reset"]
    CONST_1,
}
impl DMA0RSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMA0RSW::CONST_0 => false,
            DMA0RSW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMA0RSW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA0RSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA0RSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(DMA0RSW::CONST_0)
    }
    #[doc = "De-assert reset"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(DMA0RSW::CONST_1)
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
#[doc = "Values that can be written to the field `FCERS`"]
pub enum FCERSW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "De-assert reset"]
    CONST_1,
}
impl FCERSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FCERSW::CONST_0 => false,
            FCERSW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FCERSW<'a> {
    w: &'a mut W,
}
impl<'a> _FCERSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FCERSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(FCERSW::CONST_0)
    }
    #[doc = "De-assert reset"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(FCERSW::CONST_1)
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
#[doc = "Values that can be written to the field `USBRS`"]
pub enum USBRSW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "De-assert reset"]
    CONST_1,
}
impl USBRSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USBRSW::CONST_0 => false,
            USBRSW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USBRSW<'a> {
    w: &'a mut W,
}
impl<'a> _USBRSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBRSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(USBRSW::CONST_0)
    }
    #[doc = "De-assert reset"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(USBRSW::CONST_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ECAT0RS`"]
pub enum ECAT0RSW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "De-assert reset"]
    CONST_1,
}
impl ECAT0RSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ECAT0RSW::CONST_0 => false,
            ECAT0RSW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ECAT0RSW<'a> {
    w: &'a mut W,
}
impl<'a> _ECAT0RSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ECAT0RSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ECAT0RSW::CONST_0)
    }
    #[doc = "De-assert reset"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ECAT0RSW::CONST_1)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[doc = "Bit 1 - WDT Reset Clear"]
    #[inline]
    pub fn wdtrs(&mut self) -> _WDTRSW {
        _WDTRSW { w: self }
    }
    #[doc = "Bit 2 - ETH0 Reset Clear"]
    #[inline]
    pub fn eth0rs(&mut self) -> _ETH0RSW {
        _ETH0RSW { w: self }
    }
    #[doc = "Bit 4 - DMA0 Reset Clear"]
    #[inline]
    pub fn dma0rs(&mut self) -> _DMA0RSW {
        _DMA0RSW { w: self }
    }
    #[doc = "Bit 6 - FCE Reset Clear"]
    #[inline]
    pub fn fcers(&mut self) -> _FCERSW {
        _FCERSW { w: self }
    }
    #[doc = "Bit 7 - USB Reset Clear"]
    #[inline]
    pub fn usbrs(&mut self) -> _USBRSW {
        _USBRSW { w: self }
    }
    #[doc = "Bit 10 - ECAT0 Reset Clear"]
    #[inline]
    pub fn ecat0rs(&mut self) -> _ECAT0RSW {
        _ECAT0RSW { w: self }
    }
}
