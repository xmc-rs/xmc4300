#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PWRSET {
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
#[doc = "Values that can be written to the field `HIB`"]
pub enum HIBW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Enable Hibernate domain"]
    CONST_1,
}
impl HIBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HIBW::CONST_0 => false,
            HIBW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HIBW<'a> {
    w: &'a mut W,
}
impl<'a> _HIBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HIBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(HIBW::CONST_0)
    }
    #[doc = "Enable Hibernate domain"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(HIBW::CONST_1)
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
#[doc = "Values that can be written to the field `USBPHYPDQ`"]
pub enum USBPHYPDQW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Active"]
    CONST_1,
}
impl USBPHYPDQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USBPHYPDQW::CONST_0 => false,
            USBPHYPDQW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USBPHYPDQW<'a> {
    w: &'a mut W,
}
impl<'a> _USBPHYPDQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBPHYPDQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(USBPHYPDQW::CONST_0)
    }
    #[doc = "Active"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(USBPHYPDQW::CONST_1)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USBOTGEN`"]
pub enum USBOTGENW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Active"]
    CONST_1,
}
impl USBOTGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USBOTGENW::CONST_0 => false,
            USBOTGENW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USBOTGENW<'a> {
    w: &'a mut W,
}
impl<'a> _USBOTGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBOTGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(USBOTGENW::CONST_0)
    }
    #[doc = "Active"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(USBOTGENW::CONST_1)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USBPUWQ`"]
pub enum USBPUWQW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Pull-up not active"]
    CONST_1,
}
impl USBPUWQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USBPUWQW::CONST_0 => false,
            USBPUWQW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USBPUWQW<'a> {
    w: &'a mut W,
}
impl<'a> _USBPUWQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBPUWQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(USBPUWQW::CONST_0)
    }
    #[doc = "Pull-up not active"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(USBPUWQW::CONST_1)
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
        const OFFSET: u8 = 18;
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
    #[doc = "Bit 0 - Set Hibernate Domain Enable"]
    #[inline]
    pub fn hib(&mut self) -> _HIBW {
        _HIBW { w: self }
    }
    #[doc = "Bit 16 - Set USB PHY Transceiver Disable"]
    #[inline]
    pub fn usbphypdq(&mut self) -> _USBPHYPDQW {
        _USBPHYPDQW { w: self }
    }
    #[doc = "Bit 17 - Set USB On-The-Go Comparators Enable"]
    #[inline]
    pub fn usbotgen(&mut self) -> _USBOTGENW {
        _USBOTGENW { w: self }
    }
    #[doc = "Bit 18 - Set USB Weak Pull-Up at PADN Enable"]
    #[inline]
    pub fn usbpuwq(&mut self) -> _USBPUWQW {
        _USBPUWQW { w: self }
    }
}
