#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SRACT {
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
#[doc = "Values that can be written to the field `AGSR0`"]
pub enum AGSR0W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Activate the associated service request line"]
    VALUE2,
}
impl AGSR0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AGSR0W::VALUE1 => false,
            AGSR0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AGSR0W<'a> {
    w: &'a mut W,
}
impl<'a> _AGSR0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AGSR0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(AGSR0W::VALUE1)
    }
    #[doc = "Activate the associated service request line"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(AGSR0W::VALUE2)
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
#[doc = "Values that can be written to the field `AGSR1`"]
pub enum AGSR1W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Activate the associated service request line"]
    VALUE2,
}
impl AGSR1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AGSR1W::VALUE1 => false,
            AGSR1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AGSR1W<'a> {
    w: &'a mut W,
}
impl<'a> _AGSR1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AGSR1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(AGSR1W::VALUE1)
    }
    #[doc = "Activate the associated service request line"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(AGSR1W::VALUE2)
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
#[doc = "Values that can be written to the field `AGSR2`"]
pub enum AGSR2W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Activate the associated service request line"]
    VALUE2,
}
impl AGSR2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AGSR2W::VALUE1 => false,
            AGSR2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AGSR2W<'a> {
    w: &'a mut W,
}
impl<'a> _AGSR2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AGSR2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(AGSR2W::VALUE1)
    }
    #[doc = "Activate the associated service request line"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(AGSR2W::VALUE2)
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
#[doc = "Values that can be written to the field `AGSR3`"]
pub enum AGSR3W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Activate the associated service request line"]
    VALUE2,
}
impl AGSR3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AGSR3W::VALUE1 => false,
            AGSR3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AGSR3W<'a> {
    w: &'a mut W,
}
impl<'a> _AGSR3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AGSR3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(AGSR3W::VALUE1)
    }
    #[doc = "Activate the associated service request line"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(AGSR3W::VALUE2)
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
#[doc = "Values that can be written to the field `ASSR0`"]
pub enum ASSR0W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Activate the associated service request line"]
    VALUE2,
}
impl ASSR0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ASSR0W::VALUE1 => false,
            ASSR0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASSR0W<'a> {
    w: &'a mut W,
}
impl<'a> _ASSR0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASSR0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSR0W::VALUE1)
    }
    #[doc = "Activate the associated service request line"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSR0W::VALUE2)
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
#[doc = "Values that can be written to the field `ASSR1`"]
pub enum ASSR1W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Activate the associated service request line"]
    VALUE2,
}
impl ASSR1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ASSR1W::VALUE1 => false,
            ASSR1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASSR1W<'a> {
    w: &'a mut W,
}
impl<'a> _ASSR1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASSR1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSR1W::VALUE1)
    }
    #[doc = "Activate the associated service request line"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSR1W::VALUE2)
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
#[doc = "Values that can be written to the field `ASSR2`"]
pub enum ASSR2W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Activate the associated service request line"]
    VALUE2,
}
impl ASSR2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ASSR2W::VALUE1 => false,
            ASSR2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASSR2W<'a> {
    w: &'a mut W,
}
impl<'a> _ASSR2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASSR2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSR2W::VALUE1)
    }
    #[doc = "Activate the associated service request line"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSR2W::VALUE2)
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
#[doc = "Values that can be written to the field `ASSR3`"]
pub enum ASSR3W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Activate the associated service request line"]
    VALUE2,
}
impl ASSR3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ASSR3W::VALUE1 => false,
            ASSR3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASSR3W<'a> {
    w: &'a mut W,
}
impl<'a> _ASSR3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASSR3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSR3W::VALUE1)
    }
    #[doc = "Activate the associated service request line"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSR3W::VALUE2)
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
    #[doc = "Bit 0 - Activate Group Service Request Node 0"]
    #[inline]
    pub fn agsr0(&mut self) -> _AGSR0W {
        _AGSR0W { w: self }
    }
    #[doc = "Bit 1 - Activate Group Service Request Node 1"]
    #[inline]
    pub fn agsr1(&mut self) -> _AGSR1W {
        _AGSR1W { w: self }
    }
    #[doc = "Bit 2 - Activate Group Service Request Node 2"]
    #[inline]
    pub fn agsr2(&mut self) -> _AGSR2W {
        _AGSR2W { w: self }
    }
    #[doc = "Bit 3 - Activate Group Service Request Node 3"]
    #[inline]
    pub fn agsr3(&mut self) -> _AGSR3W {
        _AGSR3W { w: self }
    }
    #[doc = "Bit 8 - Activate Shared Service Request Node 0"]
    #[inline]
    pub fn assr0(&mut self) -> _ASSR0W {
        _ASSR0W { w: self }
    }
    #[doc = "Bit 9 - Activate Shared Service Request Node 1"]
    #[inline]
    pub fn assr1(&mut self) -> _ASSR1W {
        _ASSR1W { w: self }
    }
    #[doc = "Bit 10 - Activate Shared Service Request Node 2"]
    #[inline]
    pub fn assr2(&mut self) -> _ASSR2W {
        _ASSR2W { w: self }
    }
    #[doc = "Bit 11 - Activate Shared Service Request Node 3"]
    #[inline]
    pub fn assr3(&mut self) -> _ASSR3W {
        _ASSR3W { w: self }
    }
}
