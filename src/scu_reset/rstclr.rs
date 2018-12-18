#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RSTCLR {
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
#[doc = "Values that can be written to the field `RSCLR`"]
pub enum RSCLRW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Clears field RSTSTAT.RSTSTAT"]
    CONST_1,
}
impl RSCLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSCLRW::CONST_0 => false,
            RSCLRW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _RSCLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSCLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(RSCLRW::CONST_0)
    }
    #[doc = "Clears field RSTSTAT.RSTSTAT"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(RSCLRW::CONST_1)
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
#[doc = "Values that can be written to the field `HIBWK`"]
pub enum HIBWKW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "De-assert reset status bit"]
    CONST_1,
}
impl HIBWKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HIBWKW::CONST_0 => false,
            HIBWKW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HIBWKW<'a> {
    w: &'a mut W,
}
impl<'a> _HIBWKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HIBWKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(HIBWKW::CONST_0)
    }
    #[doc = "De-assert reset status bit"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(HIBWKW::CONST_1)
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
#[doc = "Values that can be written to the field `HIBRS`"]
pub enum HIBRSW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "De-assert reset"]
    CONST_1,
}
impl HIBRSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HIBRSW::CONST_0 => false,
            HIBRSW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HIBRSW<'a> {
    w: &'a mut W,
}
impl<'a> _HIBRSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HIBRSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(HIBRSW::CONST_0)
    }
    #[doc = "De-assert reset"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(HIBRSW::CONST_1)
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
#[doc = "Values that can be written to the field `LCKEN`"]
pub enum LCKENW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Disable reset when Lockup gets asserted"]
    CONST_1,
}
impl LCKENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LCKENW::CONST_0 => false,
            LCKENW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LCKENW<'a> {
    w: &'a mut W,
}
impl<'a> _LCKENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LCKENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(LCKENW::CONST_0)
    }
    #[doc = "Disable reset when Lockup gets asserted"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(LCKENW::CONST_1)
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
#[doc = "Values that can be written to the field `ECAT0RS`"]
pub enum ECAT0RSW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "De-assert reset status"]
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
    #[doc = "De-assert reset status"]
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
        const OFFSET: u8 = 12;
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
    #[doc = "Bit 0 - Clear Reset Status"]
    #[inline]
    pub fn rsclr(&mut self) -> _RSCLRW {
        _RSCLRW { w: self }
    }
    #[doc = "Bit 8 - Clear Hibernate Wake-up Reset Status"]
    #[inline]
    pub fn hibwk(&mut self) -> _HIBWKW {
        _HIBWKW { w: self }
    }
    #[doc = "Bit 9 - Clear Hibernate Reset"]
    #[inline]
    pub fn hibrs(&mut self) -> _HIBRSW {
        _HIBRSW { w: self }
    }
    #[doc = "Bit 10 - Enable Lockup Reset"]
    #[inline]
    pub fn lcken(&mut self) -> _LCKENW {
        _LCKENW { w: self }
    }
    #[doc = "Bit 12 - ECAT0 Reset Status Information"]
    #[inline]
    pub fn ecat0rs(&mut self) -> _ECAT0RSW {
        _ECAT0RSW { w: self }
    }
}
