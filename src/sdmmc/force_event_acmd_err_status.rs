#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::FORCE_EVENT_ACMD_ERR_STATUS {
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
#[doc = "Values that can be written to the field `FE_CMD_NOT_ISSUED_ACMD12_ERR`"]
pub enum FE_CMD_NOT_ISSUED_ACMD12_ERRW {
    #[doc = "No interrupt"]
    VALUE1,
    #[doc = "Interrupt is generated"]
    VALUE2,
}
impl FE_CMD_NOT_ISSUED_ACMD12_ERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FE_CMD_NOT_ISSUED_ACMD12_ERRW::VALUE1 => false,
            FE_CMD_NOT_ISSUED_ACMD12_ERRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FE_CMD_NOT_ISSUED_ACMD12_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _FE_CMD_NOT_ISSUED_ACMD12_ERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FE_CMD_NOT_ISSUED_ACMD12_ERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_CMD_NOT_ISSUED_ACMD12_ERRW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_CMD_NOT_ISSUED_ACMD12_ERRW::VALUE2)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FE_ACMD_IND_ERR`"]
pub enum FE_ACMD_IND_ERRW {
    #[doc = "No interrupt"]
    VALUE1,
    #[doc = "Interrupt is generated"]
    VALUE2,
}
impl FE_ACMD_IND_ERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FE_ACMD_IND_ERRW::VALUE1 => false,
            FE_ACMD_IND_ERRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FE_ACMD_IND_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _FE_ACMD_IND_ERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FE_ACMD_IND_ERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_ACMD_IND_ERRW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_ACMD_IND_ERRW::VALUE2)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FE_ACMD_END_BIT_ERR`"]
pub enum FE_ACMD_END_BIT_ERRW {
    #[doc = "No interrupt"]
    VALUE1,
    #[doc = "Interrupt is generated"]
    VALUE2,
}
impl FE_ACMD_END_BIT_ERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FE_ACMD_END_BIT_ERRW::VALUE1 => false,
            FE_ACMD_END_BIT_ERRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FE_ACMD_END_BIT_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _FE_ACMD_END_BIT_ERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FE_ACMD_END_BIT_ERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_ACMD_END_BIT_ERRW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_ACMD_END_BIT_ERRW::VALUE2)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FE_ACMD_CRC_ERR`"]
pub enum FE_ACMD_CRC_ERRW {
    #[doc = "No interrupt"]
    VALUE1,
    #[doc = "Interrupt is generated"]
    VALUE2,
}
impl FE_ACMD_CRC_ERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FE_ACMD_CRC_ERRW::VALUE1 => false,
            FE_ACMD_CRC_ERRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FE_ACMD_CRC_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _FE_ACMD_CRC_ERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FE_ACMD_CRC_ERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_ACMD_CRC_ERRW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_ACMD_CRC_ERRW::VALUE2)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FE_ACMD_TIMEOUT_ERR`"]
pub enum FE_ACMD_TIMEOUT_ERRW {
    #[doc = "No interrupt"]
    VALUE1,
    #[doc = "Interrupt is generated"]
    VALUE2,
}
impl FE_ACMD_TIMEOUT_ERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FE_ACMD_TIMEOUT_ERRW::VALUE1 => false,
            FE_ACMD_TIMEOUT_ERRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FE_ACMD_TIMEOUT_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _FE_ACMD_TIMEOUT_ERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FE_ACMD_TIMEOUT_ERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_ACMD_TIMEOUT_ERRW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_ACMD_TIMEOUT_ERRW::VALUE2)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FE_ACMD_NOT_EXEC`"]
pub enum FE_ACMD_NOT_EXECW {
    #[doc = "No interrupt"]
    VALUE1,
    #[doc = "Interrupt is generated"]
    VALUE2,
}
impl FE_ACMD_NOT_EXECW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FE_ACMD_NOT_EXECW::VALUE1 => false,
            FE_ACMD_NOT_EXECW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FE_ACMD_NOT_EXECW<'a> {
    w: &'a mut W,
}
impl<'a> _FE_ACMD_NOT_EXECW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FE_ACMD_NOT_EXECW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_ACMD_NOT_EXECW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_ACMD_NOT_EXECW::VALUE2)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
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
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 7 - Force Event for CMD not issued by Auto CMD12 Error"]
    #[inline]
    pub fn fe_cmd_not_issued_acmd12_err(&mut self) -> _FE_CMD_NOT_ISSUED_ACMD12_ERRW {
        _FE_CMD_NOT_ISSUED_ACMD12_ERRW { w: self }
    }
    #[doc = "Bit 4 - Force Event for Auto CMD Index Error"]
    #[inline]
    pub fn fe_acmd_ind_err(&mut self) -> _FE_ACMD_IND_ERRW {
        _FE_ACMD_IND_ERRW { w: self }
    }
    #[doc = "Bit 3 - Force Event for Auto CMD End bit Error"]
    #[inline]
    pub fn fe_acmd_end_bit_err(&mut self) -> _FE_ACMD_END_BIT_ERRW {
        _FE_ACMD_END_BIT_ERRW { w: self }
    }
    #[doc = "Bit 2 - Force Event for Auto CMD CRC Error"]
    #[inline]
    pub fn fe_acmd_crc_err(&mut self) -> _FE_ACMD_CRC_ERRW {
        _FE_ACMD_CRC_ERRW { w: self }
    }
    #[doc = "Bit 1 - Force Event for Auto CMD timeout Error"]
    #[inline]
    pub fn fe_acmd_timeout_err(&mut self) -> _FE_ACMD_TIMEOUT_ERRW {
        _FE_ACMD_TIMEOUT_ERRW { w: self }
    }
    #[doc = "Bit 0 - Force Event for Auto CMD12 NOT Executed"]
    #[inline]
    pub fn fe_acmd_not_exec(&mut self) -> _FE_ACMD_NOT_EXECW {
        _FE_ACMD_NOT_EXECW { w: self }
    }
}
