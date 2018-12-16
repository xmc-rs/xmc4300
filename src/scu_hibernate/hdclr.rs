#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HDCLR {
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
#[doc = "Values that can be written to the field `EPEV`"]
pub enum EPEVW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Clear wake-up event"]
    CONST_1,
}
impl EPEVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPEVW::CONST_0 => false,
            EPEVW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPEVW<'a> {
    w: &'a mut W,
}
impl<'a> _EPEVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPEVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(EPEVW::CONST_0)
    }
    #[doc = "Clear wake-up event"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(EPEVW::CONST_1)
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
#[doc = "Values that can be written to the field `ENEV`"]
pub enum ENEVW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Clear wake-up event"]
    CONST_1,
}
impl ENEVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENEVW::CONST_0 => false,
            ENEVW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENEVW<'a> {
    w: &'a mut W,
}
impl<'a> _ENEVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENEVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ENEVW::CONST_0)
    }
    #[doc = "Clear wake-up event"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ENEVW::CONST_1)
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
#[doc = "Values that can be written to the field `RTCEV`"]
pub enum RTCEVW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Clear wake-up event"]
    CONST_1,
}
impl RTCEVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTCEVW::CONST_0 => false,
            RTCEVW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTCEVW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCEVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTCEVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(RTCEVW::CONST_0)
    }
    #[doc = "Clear wake-up event"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(RTCEVW::CONST_1)
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
#[doc = "Values that can be written to the field `ULPWDG`"]
pub enum ULPWDGW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Clear watchdog alarm"]
    CONST_1,
}
impl ULPWDGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ULPWDGW::CONST_0 => false,
            ULPWDGW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ULPWDGW<'a> {
    w: &'a mut W,
}
impl<'a> _ULPWDGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ULPWDGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ULPWDGW::CONST_0)
    }
    #[doc = "Clear watchdog alarm"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ULPWDGW::CONST_1)
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
    #[doc = "Bit 0 - Wake-up Pin Event Positive Edge Clear"]
    #[inline]
    pub fn epev(&mut self) -> _EPEVW {
        _EPEVW { w: self }
    }
    #[doc = "Bit 1 - Wake-up Pin Event Negative Edge Clear"]
    #[inline]
    pub fn enev(&mut self) -> _ENEVW {
        _ENEVW { w: self }
    }
    #[doc = "Bit 2 - RTC Event Clear"]
    #[inline]
    pub fn rtcev(&mut self) -> _RTCEVW {
        _RTCEVW { w: self }
    }
    #[doc = "Bit 3 - ULP WDG Alarm Clear"]
    #[inline]
    pub fn ulpwdg(&mut self) -> _ULPWDGW {
        _ULPWDGW { w: self }
    }
}
