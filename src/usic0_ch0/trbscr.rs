#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TRBSCR {
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
#[doc = "Values that can be written to the field `CSRBI`"]
pub enum CSRBIW {
    #[doc = "No effect."]
    VALUE1,
    #[doc = "Clear TRBSR.SRBI."]
    VALUE2,
}
impl CSRBIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CSRBIW::VALUE1 => false,
            CSRBIW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSRBIW<'a> {
    w: &'a mut W,
}
impl<'a> _CSRBIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSRBIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CSRBIW::VALUE1)
    }
    #[doc = "Clear TRBSR.SRBI."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CSRBIW::VALUE2)
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
#[doc = "Values that can be written to the field `CRBERI`"]
pub enum CRBERIW {
    #[doc = "No effect."]
    VALUE1,
    #[doc = "Clear TRBSR.RBERI."]
    VALUE2,
}
impl CRBERIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRBERIW::VALUE1 => false,
            CRBERIW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRBERIW<'a> {
    w: &'a mut W,
}
impl<'a> _CRBERIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRBERIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CRBERIW::VALUE1)
    }
    #[doc = "Clear TRBSR.RBERI."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CRBERIW::VALUE2)
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
#[doc = "Values that can be written to the field `CARBI`"]
pub enum CARBIW {
    #[doc = "No effect."]
    VALUE1,
    #[doc = "Clear TRBSR.ARBI."]
    VALUE2,
}
impl CARBIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CARBIW::VALUE1 => false,
            CARBIW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CARBIW<'a> {
    w: &'a mut W,
}
impl<'a> _CARBIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CARBIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CARBIW::VALUE1)
    }
    #[doc = "Clear TRBSR.ARBI."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CARBIW::VALUE2)
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
#[doc = "Values that can be written to the field `CSTBI`"]
pub enum CSTBIW {
    #[doc = "No effect."]
    VALUE1,
    #[doc = "Clear TRBSR.STBI."]
    VALUE2,
}
impl CSTBIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CSTBIW::VALUE1 => false,
            CSTBIW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSTBIW<'a> {
    w: &'a mut W,
}
impl<'a> _CSTBIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSTBIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CSTBIW::VALUE1)
    }
    #[doc = "Clear TRBSR.STBI."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CSTBIW::VALUE2)
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
#[doc = "Values that can be written to the field `CTBERI`"]
pub enum CTBERIW {
    #[doc = "No effect."]
    VALUE1,
    #[doc = "Clear TRBSR.TBERI."]
    VALUE2,
}
impl CTBERIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTBERIW::VALUE1 => false,
            CTBERIW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTBERIW<'a> {
    w: &'a mut W,
}
impl<'a> _CTBERIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTBERIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CTBERIW::VALUE1)
    }
    #[doc = "Clear TRBSR.TBERI."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CTBERIW::VALUE2)
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
#[doc = "Values that can be written to the field `CBDV`"]
pub enum CBDVW {
    #[doc = "No effect."]
    VALUE1,
    #[doc = "Clear BYPCR.BDV."]
    VALUE2,
}
impl CBDVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CBDVW::VALUE1 => false,
            CBDVW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CBDVW<'a> {
    w: &'a mut W,
}
impl<'a> _CBDVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CBDVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CBDVW::VALUE1)
    }
    #[doc = "Clear BYPCR.BDV."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CBDVW::VALUE2)
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
#[doc = "Values that can be written to the field `FLUSHRB`"]
pub enum FLUSHRBW {
    #[doc = "No effect."]
    VALUE1,
    #[doc = "The receive FIFO buffer is cleared (filling level is cleared and output pointer is set to input pointer value). Should only be used while the FIFO buffer is not taking part in data traffic."]
    VALUE2,
}
impl FLUSHRBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLUSHRBW::VALUE1 => false,
            FLUSHRBW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLUSHRBW<'a> {
    w: &'a mut W,
}
impl<'a> _FLUSHRBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLUSHRBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FLUSHRBW::VALUE1)
    }
    #[doc = "The receive FIFO buffer is cleared (filling level is cleared and output pointer is set to input pointer value). Should only be used while the FIFO buffer is not taking part in data traffic."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FLUSHRBW::VALUE2)
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
#[doc = "Values that can be written to the field `FLUSHTB`"]
pub enum FLUSHTBW {
    #[doc = "No effect."]
    VALUE1,
    #[doc = "The transmit FIFO buffer is cleared (filling level is cleared and output pointer is set to input pointer value). Should only be used while the FIFO buffer is not taking part in data traffic."]
    VALUE2,
}
impl FLUSHTBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLUSHTBW::VALUE1 => false,
            FLUSHTBW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLUSHTBW<'a> {
    w: &'a mut W,
}
impl<'a> _FLUSHTBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLUSHTBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FLUSHTBW::VALUE1)
    }
    #[doc = "The transmit FIFO buffer is cleared (filling level is cleared and output pointer is set to input pointer value). Should only be used while the FIFO buffer is not taking part in data traffic."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FLUSHTBW::VALUE2)
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
    #[doc = "Bit 0 - Clear Standard Receive Buffer Event"]
    #[inline]
    pub fn csrbi(&mut self) -> _CSRBIW {
        _CSRBIW { w: self }
    }
    #[doc = "Bit 1 - Clear Receive Buffer Error Event"]
    #[inline]
    pub fn crberi(&mut self) -> _CRBERIW {
        _CRBERIW { w: self }
    }
    #[doc = "Bit 2 - Clear Alternative Receive Buffer Event"]
    #[inline]
    pub fn carbi(&mut self) -> _CARBIW {
        _CARBIW { w: self }
    }
    #[doc = "Bit 8 - Clear Standard Transmit Buffer Event"]
    #[inline]
    pub fn cstbi(&mut self) -> _CSTBIW {
        _CSTBIW { w: self }
    }
    #[doc = "Bit 9 - Clear Transmit Buffer Error Event"]
    #[inline]
    pub fn ctberi(&mut self) -> _CTBERIW {
        _CTBERIW { w: self }
    }
    #[doc = "Bit 10 - Clear Bypass Data Valid"]
    #[inline]
    pub fn cbdv(&mut self) -> _CBDVW {
        _CBDVW { w: self }
    }
    #[doc = "Bit 14 - Flush Receive Buffer"]
    #[inline]
    pub fn flushrb(&mut self) -> _FLUSHRBW {
        _FLUSHRBW { w: self }
    }
    #[doc = "Bit 15 - Flush Transmit Buffer"]
    #[inline]
    pub fn flushtb(&mut self) -> _FLUSHTBW {
        _FLUSHTBW { w: self }
    }
}
