#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PRCLR0 {
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
#[doc = "Values that can be written to the field `VADCRS`"]
pub enum VADCRSW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "De-assert reset"]
    CONST_1,
}
impl VADCRSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VADCRSW::CONST_0 => false,
            VADCRSW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VADCRSW<'a> {
    w: &'a mut W,
}
impl<'a> _VADCRSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VADCRSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(VADCRSW::CONST_0)
    }
    #[doc = "De-assert reset"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(VADCRSW::CONST_1)
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
#[doc = "Values that can be written to the field `CCU40RS`"]
pub enum CCU40RSW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "De-assert reset"]
    CONST_1,
}
impl CCU40RSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCU40RSW::CONST_0 => false,
            CCU40RSW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCU40RSW<'a> {
    w: &'a mut W,
}
impl<'a> _CCU40RSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCU40RSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(CCU40RSW::CONST_0)
    }
    #[doc = "De-assert reset"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(CCU40RSW::CONST_1)
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
#[doc = "Values that can be written to the field `CCU41RS`"]
pub enum CCU41RSW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "De-assert reset"]
    CONST_1,
}
impl CCU41RSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCU41RSW::CONST_0 => false,
            CCU41RSW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCU41RSW<'a> {
    w: &'a mut W,
}
impl<'a> _CCU41RSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCU41RSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(CCU41RSW::CONST_0)
    }
    #[doc = "De-assert reset"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(CCU41RSW::CONST_1)
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
#[doc = "Values that can be written to the field `CCU80RS`"]
pub enum CCU80RSW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "De-assert reset"]
    CONST_1,
}
impl CCU80RSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCU80RSW::CONST_0 => false,
            CCU80RSW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCU80RSW<'a> {
    w: &'a mut W,
}
impl<'a> _CCU80RSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCU80RSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(CCU80RSW::CONST_0)
    }
    #[doc = "De-assert reset"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(CCU80RSW::CONST_1)
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
#[doc = "Values that can be written to the field `USIC0RS`"]
pub enum USIC0RSW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "De-assert reset"]
    CONST_1,
}
impl USIC0RSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USIC0RSW::CONST_0 => false,
            USIC0RSW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USIC0RSW<'a> {
    w: &'a mut W,
}
impl<'a> _USIC0RSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USIC0RSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(USIC0RSW::CONST_0)
    }
    #[doc = "De-assert reset"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(USIC0RSW::CONST_1)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ERU1RS`"]
pub enum ERU1RSW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "De-assert reset"]
    CONST_1,
}
impl ERU1RSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERU1RSW::CONST_0 => false,
            ERU1RSW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERU1RSW<'a> {
    w: &'a mut W,
}
impl<'a> _ERU1RSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERU1RSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ERU1RSW::CONST_0)
    }
    #[doc = "De-assert reset"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ERU1RSW::CONST_1)
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
    #[doc = "Bit 0 - VADC Reset Clear"]
    #[inline]
    pub fn vadcrs(&mut self) -> _VADCRSW {
        _VADCRSW { w: self }
    }
    #[doc = "Bit 2 - CCU40 Reset Clear"]
    #[inline]
    pub fn ccu40rs(&mut self) -> _CCU40RSW {
        _CCU40RSW { w: self }
    }
    #[doc = "Bit 3 - CCU41 Reset Clear"]
    #[inline]
    pub fn ccu41rs(&mut self) -> _CCU41RSW {
        _CCU41RSW { w: self }
    }
    #[doc = "Bit 7 - CCU80 Reset Clear"]
    #[inline]
    pub fn ccu80rs(&mut self) -> _CCU80RSW {
        _CCU80RSW { w: self }
    }
    #[doc = "Bit 11 - USIC0 Reset Clear"]
    #[inline]
    pub fn usic0rs(&mut self) -> _USIC0RSW {
        _USIC0RSW { w: self }
    }
    #[doc = "Bit 16 - ERU1 Reset Clear"]
    #[inline]
    pub fn eru1rs(&mut self) -> _ERU1RSW {
        _ERU1RSW { w: self }
    }
}
