#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLKSET {
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
#[doc = "Values that can be written to the field `USBCEN`"]
pub enum USBCENW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Enable"]
    CONST_1,
}
impl USBCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USBCENW::CONST_0 => false,
            USBCENW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USBCENW<'a> {
    w: &'a mut W,
}
impl<'a> _USBCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(USBCENW::CONST_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(USBCENW::CONST_1)
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
#[doc = "Values that can be written to the field `MMCCEN`"]
pub enum MMCCENW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Enable"]
    CONST_1,
}
impl MMCCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MMCCENW::CONST_0 => false,
            MMCCENW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MMCCENW<'a> {
    w: &'a mut W,
}
impl<'a> _MMCCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MMCCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(MMCCENW::CONST_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(MMCCENW::CONST_1)
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
#[doc = "Values that can be written to the field `ETH0CEN`"]
pub enum ETH0CENW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Enable"]
    CONST_1,
}
impl ETH0CENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ETH0CENW::CONST_0 => false,
            ETH0CENW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ETH0CENW<'a> {
    w: &'a mut W,
}
impl<'a> _ETH0CENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ETH0CENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ETH0CENW::CONST_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ETH0CENW::CONST_1)
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
#[doc = "Values that can be written to the field `CCUCEN`"]
pub enum CCUCENW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Enable"]
    CONST_1,
}
impl CCUCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCUCENW::CONST_0 => false,
            CCUCENW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCUCENW<'a> {
    w: &'a mut W,
}
impl<'a> _CCUCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCUCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(CCUCENW::CONST_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(CCUCENW::CONST_1)
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
#[doc = "Values that can be written to the field `WDTCEN`"]
pub enum WDTCENW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Enable"]
    CONST_1,
}
impl WDTCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WDTCENW::CONST_0 => false,
            WDTCENW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WDTCENW<'a> {
    w: &'a mut W,
}
impl<'a> _WDTCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDTCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(WDTCENW::CONST_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(WDTCENW::CONST_1)
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
    #[doc = "Bit 0 - USB Clock Enable"]
    #[inline]
    pub fn usbcen(&mut self) -> _USBCENW {
        _USBCENW { w: self }
    }
    #[doc = "Bit 1 - MMC Clock Enable"]
    #[inline]
    pub fn mmccen(&mut self) -> _MMCCENW {
        _MMCCENW { w: self }
    }
    #[doc = "Bit 2 - Ethernet Clock Enable"]
    #[inline]
    pub fn eth0cen(&mut self) -> _ETH0CENW {
        _ETH0CENW { w: self }
    }
    #[doc = "Bit 4 - CCU Clock Enable"]
    #[inline]
    pub fn ccucen(&mut self) -> _CCUCENW {
        _CCUCENW { w: self }
    }
    #[doc = "Bit 5 - WDT Clock Enable"]
    #[inline]
    pub fn wdtcen(&mut self) -> _WDTCENW {
        _WDTCENW { w: self }
    }
}
