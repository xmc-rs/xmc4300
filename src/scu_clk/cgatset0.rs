#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CGATSET0 {
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
#[doc = "Values that can be written to the field `VADC`"]
pub enum VADCW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Enable gating"]
    CONST_1,
}
impl VADCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VADCW::CONST_0 => false,
            VADCW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VADCW<'a> {
    w: &'a mut W,
}
impl<'a> _VADCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VADCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(VADCW::CONST_0)
    }
    #[doc = "Enable gating"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(VADCW::CONST_1)
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
#[doc = "Values that can be written to the field `CCU40`"]
pub enum CCU40W {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Enable gating"]
    CONST_1,
}
impl CCU40W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCU40W::CONST_0 => false,
            CCU40W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCU40W<'a> {
    w: &'a mut W,
}
impl<'a> _CCU40W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCU40W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(CCU40W::CONST_0)
    }
    #[doc = "Enable gating"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(CCU40W::CONST_1)
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
#[doc = "Values that can be written to the field `CCU41`"]
pub enum CCU41W {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Enable gating"]
    CONST_1,
}
impl CCU41W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCU41W::CONST_0 => false,
            CCU41W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCU41W<'a> {
    w: &'a mut W,
}
impl<'a> _CCU41W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCU41W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(CCU41W::CONST_0)
    }
    #[doc = "Enable gating"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(CCU41W::CONST_1)
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
#[doc = "Values that can be written to the field `CCU80`"]
pub enum CCU80W {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Enable gating"]
    CONST_1,
}
impl CCU80W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCU80W::CONST_0 => false,
            CCU80W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCU80W<'a> {
    w: &'a mut W,
}
impl<'a> _CCU80W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCU80W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(CCU80W::CONST_0)
    }
    #[doc = "Enable gating"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(CCU80W::CONST_1)
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
#[doc = "Values that can be written to the field `POSIF0`"]
pub enum POSIF0W {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Enable gating"]
    CONST_1,
}
impl POSIF0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            POSIF0W::CONST_0 => false,
            POSIF0W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POSIF0W<'a> {
    w: &'a mut W,
}
impl<'a> _POSIF0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POSIF0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(POSIF0W::CONST_0)
    }
    #[doc = "Enable gating"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(POSIF0W::CONST_1)
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
#[doc = "Values that can be written to the field `USIC0`"]
pub enum USIC0W {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Enable gating"]
    CONST_1,
}
impl USIC0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USIC0W::CONST_0 => false,
            USIC0W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USIC0W<'a> {
    w: &'a mut W,
}
impl<'a> _USIC0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USIC0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(USIC0W::CONST_0)
    }
    #[doc = "Enable gating"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(USIC0W::CONST_1)
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
#[doc = "Values that can be written to the field `ERU1`"]
pub enum ERU1W {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Enable gating"]
    CONST_1,
}
impl ERU1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERU1W::CONST_0 => false,
            ERU1W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERU1W<'a> {
    w: &'a mut W,
}
impl<'a> _ERU1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERU1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ERU1W::CONST_0)
    }
    #[doc = "Enable gating"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ERU1W::CONST_1)
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
    #[doc = "Bit 0 - VADC Gating Set"]
    #[inline]
    pub fn vadc(&mut self) -> _VADCW {
        _VADCW { w: self }
    }
    #[doc = "Bit 2 - CCU40 Gating Set"]
    #[inline]
    pub fn ccu40(&mut self) -> _CCU40W {
        _CCU40W { w: self }
    }
    #[doc = "Bit 3 - CCU41 Gating Set"]
    #[inline]
    pub fn ccu41(&mut self) -> _CCU41W {
        _CCU41W { w: self }
    }
    #[doc = "Bit 7 - CCU80 Gating Set"]
    #[inline]
    pub fn ccu80(&mut self) -> _CCU80W {
        _CCU80W { w: self }
    }
    #[doc = "Bit 9 - POSIF0 Gating Set"]
    #[inline]
    pub fn posif0(&mut self) -> _POSIF0W {
        _POSIF0W { w: self }
    }
    #[doc = "Bit 11 - USIC0 Gating Set"]
    #[inline]
    pub fn usic0(&mut self) -> _USIC0W {
        _USIC0W { w: self }
    }
    #[doc = "Bit 16 - ERU1 Gating Set"]
    #[inline]
    pub fn eru1(&mut self) -> _ERU1W {
        _ERU1W { w: self }
    }
}
