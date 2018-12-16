#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::QINR0 {
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
#[doc = r" Proxy"]
pub struct _REQCHNRW<'a> {
    w: &'a mut W,
}
impl<'a> _REQCHNRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RF`"]
pub enum RFW {
    #[doc = "No refill: this queue entry is converted once and then invalidated"]
    VALUE1,
    #[doc = "Automatic refill: this queue entry is automatically reloaded into QINRx when the related conversion is started"]
    VALUE2,
}
impl RFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RFW::VALUE1 => false,
            RFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RFW<'a> {
    w: &'a mut W,
}
impl<'a> _RFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No refill: this queue entry is converted once and then invalidated"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RFW::VALUE1)
    }
    #[doc = "Automatic refill: this queue entry is automatically reloaded into QINRx when the related conversion is started"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RFW::VALUE2)
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
#[doc = "Values that can be written to the field `ENSI`"]
pub enum ENSIW {
    #[doc = "No request source interrupt"]
    VALUE1,
    #[doc = "A request source event interrupt is generated upon a request source event (related conversion is finished)"]
    VALUE2,
}
impl ENSIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENSIW::VALUE1 => false,
            ENSIW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENSIW<'a> {
    w: &'a mut W,
}
impl<'a> _ENSIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENSIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No request source interrupt"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENSIW::VALUE1)
    }
    #[doc = "A request source event interrupt is generated upon a request source event (related conversion is finished)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENSIW::VALUE2)
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
#[doc = "Values that can be written to the field `EXTR`"]
pub enum EXTRW {
    #[doc = "A valid queue entry immediately leads to a conversion request."]
    VALUE1,
    #[doc = "A valid queue entry waits for a trigger event to occur before issuing a conversion request."]
    VALUE2,
}
impl EXTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EXTRW::VALUE1 => false,
            EXTRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTRW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A valid queue entry immediately leads to a conversion request."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EXTRW::VALUE1)
    }
    #[doc = "A valid queue entry waits for a trigger event to occur before issuing a conversion request."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EXTRW::VALUE2)
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
    #[doc = "Bits 0:4 - Request Channel Number"]
    #[inline]
    pub fn reqchnr(&mut self) -> _REQCHNRW {
        _REQCHNRW { w: self }
    }
    #[doc = "Bit 5 - Refill"]
    #[inline]
    pub fn rf(&mut self) -> _RFW {
        _RFW { w: self }
    }
    #[doc = "Bit 6 - Enable Source Interrupt"]
    #[inline]
    pub fn ensi(&mut self) -> _ENSIW {
        _ENSIW { w: self }
    }
    #[doc = "Bit 7 - External Trigger"]
    #[inline]
    pub fn extr(&mut self) -> _EXTRW {
        _EXTRW { w: self }
    }
}
