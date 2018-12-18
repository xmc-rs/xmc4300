#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FMR {
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
#[doc = "Values that can be written to the field `MTDV`"]
pub enum MTDVW {
    #[doc = "No action."]
    VALUE1,
    #[doc = "Bit TDV is set, TE is unchanged."]
    VALUE2,
    #[doc = "Bits TDV and TE are cleared."]
    VALUE3,
}
impl MTDVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MTDVW::VALUE1 => 0,
            MTDVW::VALUE2 => 1,
            MTDVW::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTDVW<'a> {
    w: &'a mut W,
}
impl<'a> _MTDVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTDVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No action."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MTDVW::VALUE1)
    }
    #[doc = "Bit TDV is set, TE is unchanged."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MTDVW::VALUE2)
    }
    #[doc = "Bits TDV and TE are cleared."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(MTDVW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ATVC`"]
pub enum ATVCW {
    #[doc = "No action."]
    VALUE1,
    #[doc = "Bit TCSR.TVC is set."]
    VALUE2,
}
impl ATVCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ATVCW::VALUE1 => false,
            ATVCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ATVCW<'a> {
    w: &'a mut W,
}
impl<'a> _ATVCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ATVCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ATVCW::VALUE1)
    }
    #[doc = "Bit TCSR.TVC is set."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ATVCW::VALUE2)
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
#[doc = "Values that can be written to the field `CRDV0`"]
pub enum CRDV0W {
    #[doc = "No action."]
    VALUE1,
    #[doc = "Bits RBUF01SR.RDV00 and RBUF01SR.RDV10 are cleared."]
    VALUE2,
}
impl CRDV0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRDV0W::VALUE1 => false,
            CRDV0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRDV0W<'a> {
    w: &'a mut W,
}
impl<'a> _CRDV0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRDV0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CRDV0W::VALUE1)
    }
    #[doc = "Bits RBUF01SR.RDV00 and RBUF01SR.RDV10 are cleared."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CRDV0W::VALUE2)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CRDV1`"]
pub enum CRDV1W {
    #[doc = "No action."]
    VALUE1,
    #[doc = "Bits RBUF01SR.RDV01 and RBUF01SR.RDV11 are cleared."]
    VALUE2,
}
impl CRDV1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRDV1W::VALUE1 => false,
            CRDV1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRDV1W<'a> {
    w: &'a mut W,
}
impl<'a> _CRDV1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRDV1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CRDV1W::VALUE1)
    }
    #[doc = "Bits RBUF01SR.RDV01 and RBUF01SR.RDV11 are cleared."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CRDV1W::VALUE2)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SIO0`"]
pub enum SIO0W {
    #[doc = "No action."]
    VALUE1,
    #[doc = "The service request output SRx is activated."]
    VALUE2,
}
impl SIO0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SIO0W::VALUE1 => false,
            SIO0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SIO0W<'a> {
    w: &'a mut W,
}
impl<'a> _SIO0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SIO0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SIO0W::VALUE1)
    }
    #[doc = "The service request output SRx is activated."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SIO0W::VALUE2)
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
#[doc = "Values that can be written to the field `SIO1`"]
pub enum SIO1W {
    #[doc = "No action."]
    VALUE1,
    #[doc = "The service request output SRx is activated."]
    VALUE2,
}
impl SIO1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SIO1W::VALUE1 => false,
            SIO1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SIO1W<'a> {
    w: &'a mut W,
}
impl<'a> _SIO1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SIO1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SIO1W::VALUE1)
    }
    #[doc = "The service request output SRx is activated."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SIO1W::VALUE2)
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
#[doc = "Values that can be written to the field `SIO2`"]
pub enum SIO2W {
    #[doc = "No action."]
    VALUE1,
    #[doc = "The service request output SRx is activated."]
    VALUE2,
}
impl SIO2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SIO2W::VALUE1 => false,
            SIO2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SIO2W<'a> {
    w: &'a mut W,
}
impl<'a> _SIO2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SIO2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SIO2W::VALUE1)
    }
    #[doc = "The service request output SRx is activated."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SIO2W::VALUE2)
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
#[doc = "Values that can be written to the field `SIO3`"]
pub enum SIO3W {
    #[doc = "No action."]
    VALUE1,
    #[doc = "The service request output SRx is activated."]
    VALUE2,
}
impl SIO3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SIO3W::VALUE1 => false,
            SIO3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SIO3W<'a> {
    w: &'a mut W,
}
impl<'a> _SIO3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SIO3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SIO3W::VALUE1)
    }
    #[doc = "The service request output SRx is activated."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SIO3W::VALUE2)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SIO4`"]
pub enum SIO4W {
    #[doc = "No action."]
    VALUE1,
    #[doc = "The service request output SRx is activated."]
    VALUE2,
}
impl SIO4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SIO4W::VALUE1 => false,
            SIO4W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SIO4W<'a> {
    w: &'a mut W,
}
impl<'a> _SIO4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SIO4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SIO4W::VALUE1)
    }
    #[doc = "The service request output SRx is activated."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SIO4W::VALUE2)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SIO5`"]
pub enum SIO5W {
    #[doc = "No action."]
    VALUE1,
    #[doc = "The service request output SRx is activated."]
    VALUE2,
}
impl SIO5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SIO5W::VALUE1 => false,
            SIO5W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SIO5W<'a> {
    w: &'a mut W,
}
impl<'a> _SIO5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SIO5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SIO5W::VALUE1)
    }
    #[doc = "The service request output SRx is activated."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SIO5W::VALUE2)
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
        const OFFSET: u8 = 21;
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
    #[doc = "Bits 0:1 - Modify Transmit Data Valid"]
    #[inline]
    pub fn mtdv(&mut self) -> _MTDVW {
        _MTDVW { w: self }
    }
    #[doc = "Bit 4 - Activate Bit TVC"]
    #[inline]
    pub fn atvc(&mut self) -> _ATVCW {
        _ATVCW { w: self }
    }
    #[doc = "Bit 14 - Clear Bits RDV for RBUF0"]
    #[inline]
    pub fn crdv0(&mut self) -> _CRDV0W {
        _CRDV0W { w: self }
    }
    #[doc = "Bit 15 - Clear Bit RDV for RBUF1"]
    #[inline]
    pub fn crdv1(&mut self) -> _CRDV1W {
        _CRDV1W { w: self }
    }
    #[doc = "Bit 16 - Set Interrupt Output SRx"]
    #[inline]
    pub fn sio0(&mut self) -> _SIO0W {
        _SIO0W { w: self }
    }
    #[doc = "Bit 17 - Set Interrupt Output SRx"]
    #[inline]
    pub fn sio1(&mut self) -> _SIO1W {
        _SIO1W { w: self }
    }
    #[doc = "Bit 18 - Set Interrupt Output SRx"]
    #[inline]
    pub fn sio2(&mut self) -> _SIO2W {
        _SIO2W { w: self }
    }
    #[doc = "Bit 19 - Set Interrupt Output SRx"]
    #[inline]
    pub fn sio3(&mut self) -> _SIO3W {
        _SIO3W { w: self }
    }
    #[doc = "Bit 20 - Set Interrupt Output SRx"]
    #[inline]
    pub fn sio4(&mut self) -> _SIO4W {
        _SIO4W { w: self }
    }
    #[doc = "Bit 21 - Set Interrupt Output SRx"]
    #[inline]
    pub fn sio5(&mut self) -> _SIO5W {
        _SIO5W { w: self }
    }
}
