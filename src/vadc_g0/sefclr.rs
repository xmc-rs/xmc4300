#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SEFCLR {
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
#[doc = "Values that can be written to the field `SEV0`"]
pub enum SEV0W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Clear the source event flag in GxSEFLAG"]
    VALUE2,
}
impl SEV0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEV0W::VALUE1 => false,
            SEV0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEV0W<'a> {
    w: &'a mut W,
}
impl<'a> _SEV0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEV0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SEV0W::VALUE1)
    }
    #[doc = "Clear the source event flag in GxSEFLAG"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SEV0W::VALUE2)
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
#[doc = "Values that can be written to the field `SEV1`"]
pub enum SEV1W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Clear the source event flag in GxSEFLAG"]
    VALUE2,
}
impl SEV1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEV1W::VALUE1 => false,
            SEV1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEV1W<'a> {
    w: &'a mut W,
}
impl<'a> _SEV1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEV1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SEV1W::VALUE1)
    }
    #[doc = "Clear the source event flag in GxSEFLAG"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SEV1W::VALUE2)
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
    #[doc = "Bit 0 - Clear Source Event 0/1"]
    #[inline]
    pub fn sev0(&mut self) -> _SEV0W {
        _SEV0W { w: self }
    }
    #[doc = "Bit 1 - Clear Source Event 0/1"]
    #[inline]
    pub fn sev1(&mut self) -> _SEV1W {
        _SEV1W { w: self }
    }
}
