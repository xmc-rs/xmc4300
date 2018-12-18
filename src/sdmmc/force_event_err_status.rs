#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::FORCE_EVENT_ERR_STATUS {
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
#[doc = "Values that can be written to the field `FE_CEATA_ERR`"]
pub enum FE_CEATA_ERRW {
    #[doc = "No interrupt"]
    VALUE1,
    #[doc = "Interrupt is generated"]
    VALUE2,
}
impl FE_CEATA_ERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FE_CEATA_ERRW::VALUE1 => false,
            FE_CEATA_ERRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FE_CEATA_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _FE_CEATA_ERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FE_CEATA_ERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_CEATA_ERRW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_CEATA_ERRW::VALUE2)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FE_TARGET_RESPONSE_ERR`"]
pub enum FE_TARGET_RESPONSE_ERRW {
    #[doc = "No interrupt"]
    VALUE1,
    #[doc = "Interrupt is generated"]
    VALUE2,
}
impl FE_TARGET_RESPONSE_ERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FE_TARGET_RESPONSE_ERRW::VALUE1 => false,
            FE_TARGET_RESPONSE_ERRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FE_TARGET_RESPONSE_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _FE_TARGET_RESPONSE_ERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FE_TARGET_RESPONSE_ERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_TARGET_RESPONSE_ERRW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_TARGET_RESPONSE_ERRW::VALUE2)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FE_ACMD12_ERR`"]
pub enum FE_ACMD12_ERRW {
    #[doc = "No interrupt"]
    VALUE1,
    #[doc = "Interrupt is generated"]
    VALUE2,
}
impl FE_ACMD12_ERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FE_ACMD12_ERRW::VALUE1 => false,
            FE_ACMD12_ERRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FE_ACMD12_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _FE_ACMD12_ERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FE_ACMD12_ERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_ACMD12_ERRW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_ACMD12_ERRW::VALUE2)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FE_CURRENT_LIMIT_ERR`"]
pub enum FE_CURRENT_LIMIT_ERRW {
    #[doc = "No interrupt"]
    VALUE1,
    #[doc = "Interrupt is generated"]
    VALUE2,
}
impl FE_CURRENT_LIMIT_ERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FE_CURRENT_LIMIT_ERRW::VALUE1 => false,
            FE_CURRENT_LIMIT_ERRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FE_CURRENT_LIMIT_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _FE_CURRENT_LIMIT_ERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FE_CURRENT_LIMIT_ERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_CURRENT_LIMIT_ERRW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_CURRENT_LIMIT_ERRW::VALUE2)
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
#[doc = "Values that can be written to the field `FE_DATA_END_BIT_ERR`"]
pub enum FE_DATA_END_BIT_ERRW {
    #[doc = "No interrupt"]
    VALUE1,
    #[doc = "Interrupt is generated"]
    VALUE2,
}
impl FE_DATA_END_BIT_ERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FE_DATA_END_BIT_ERRW::VALUE1 => false,
            FE_DATA_END_BIT_ERRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FE_DATA_END_BIT_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _FE_DATA_END_BIT_ERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FE_DATA_END_BIT_ERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_DATA_END_BIT_ERRW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_DATA_END_BIT_ERRW::VALUE2)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FE_DATA_CRC_ERR`"]
pub enum FE_DATA_CRC_ERRW {
    #[doc = "No interrupt"]
    VALUE1,
    #[doc = "Interrupt is generated"]
    VALUE2,
}
impl FE_DATA_CRC_ERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FE_DATA_CRC_ERRW::VALUE1 => false,
            FE_DATA_CRC_ERRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FE_DATA_CRC_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _FE_DATA_CRC_ERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FE_DATA_CRC_ERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_DATA_CRC_ERRW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_DATA_CRC_ERRW::VALUE2)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FE_DATA_TIMEOUT_ERR`"]
pub enum FE_DATA_TIMEOUT_ERRW {
    #[doc = "No interrupt"]
    VALUE1,
    #[doc = "Interrupt is generated"]
    VALUE2,
}
impl FE_DATA_TIMEOUT_ERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FE_DATA_TIMEOUT_ERRW::VALUE1 => false,
            FE_DATA_TIMEOUT_ERRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FE_DATA_TIMEOUT_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _FE_DATA_TIMEOUT_ERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FE_DATA_TIMEOUT_ERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_DATA_TIMEOUT_ERRW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_DATA_TIMEOUT_ERRW::VALUE2)
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
#[doc = "Values that can be written to the field `FE_CMD_IND_ERR`"]
pub enum FE_CMD_IND_ERRW {
    #[doc = "No interrupt"]
    VALUE1,
    #[doc = "Interrupt is generated"]
    VALUE2,
}
impl FE_CMD_IND_ERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FE_CMD_IND_ERRW::VALUE1 => false,
            FE_CMD_IND_ERRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FE_CMD_IND_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _FE_CMD_IND_ERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FE_CMD_IND_ERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_CMD_IND_ERRW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_CMD_IND_ERRW::VALUE2)
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
#[doc = "Values that can be written to the field `FE_CMD_END_BIT_ERR`"]
pub enum FE_CMD_END_BIT_ERRW {
    #[doc = "No interrupt"]
    VALUE1,
    #[doc = "Interrupt is generated"]
    VALUE2,
}
impl FE_CMD_END_BIT_ERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FE_CMD_END_BIT_ERRW::VALUE1 => false,
            FE_CMD_END_BIT_ERRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FE_CMD_END_BIT_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _FE_CMD_END_BIT_ERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FE_CMD_END_BIT_ERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_CMD_END_BIT_ERRW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_CMD_END_BIT_ERRW::VALUE2)
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
#[doc = "Values that can be written to the field `FE_CMD_CRC_ERR`"]
pub enum FE_CMD_CRC_ERRW {
    #[doc = "No interrupt"]
    VALUE1,
    #[doc = "Interrupt is generated"]
    VALUE2,
}
impl FE_CMD_CRC_ERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FE_CMD_CRC_ERRW::VALUE1 => false,
            FE_CMD_CRC_ERRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FE_CMD_CRC_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _FE_CMD_CRC_ERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FE_CMD_CRC_ERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_CMD_CRC_ERRW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_CMD_CRC_ERRW::VALUE2)
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
#[doc = "Values that can be written to the field `FE_CMD_TIMEOUT_ERR`"]
pub enum FE_CMD_TIMEOUT_ERRW {
    #[doc = "No interrupt"]
    VALUE1,
    #[doc = "Interrupt is generated"]
    VALUE2,
}
impl FE_CMD_TIMEOUT_ERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FE_CMD_TIMEOUT_ERRW::VALUE1 => false,
            FE_CMD_TIMEOUT_ERRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FE_CMD_TIMEOUT_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _FE_CMD_TIMEOUT_ERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FE_CMD_TIMEOUT_ERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_CMD_TIMEOUT_ERRW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_CMD_TIMEOUT_ERRW::VALUE2)
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
    #[doc = "Bit 13 - Force Event for Ceata Error"]
    #[inline]
    pub fn fe_ceata_err(&mut self) -> _FE_CEATA_ERRW {
        _FE_CEATA_ERRW { w: self }
    }
    #[doc = "Bit 12 - Force event for Target Response Error"]
    #[inline]
    pub fn fe_target_response_err(&mut self) -> _FE_TARGET_RESPONSE_ERRW {
        _FE_TARGET_RESPONSE_ERRW { w: self }
    }
    #[doc = "Bit 8 - Force Event for Auto CMD Error"]
    #[inline]
    pub fn fe_acmd12_err(&mut self) -> _FE_ACMD12_ERRW {
        _FE_ACMD12_ERRW { w: self }
    }
    #[doc = "Bit 7 - Force Event for Current Limit Error"]
    #[inline]
    pub fn fe_current_limit_err(&mut self) -> _FE_CURRENT_LIMIT_ERRW {
        _FE_CURRENT_LIMIT_ERRW { w: self }
    }
    #[doc = "Bit 6 - Force Event for Data End Bit Error"]
    #[inline]
    pub fn fe_data_end_bit_err(&mut self) -> _FE_DATA_END_BIT_ERRW {
        _FE_DATA_END_BIT_ERRW { w: self }
    }
    #[doc = "Bit 5 - Force Event for Data CRC Error"]
    #[inline]
    pub fn fe_data_crc_err(&mut self) -> _FE_DATA_CRC_ERRW {
        _FE_DATA_CRC_ERRW { w: self }
    }
    #[doc = "Bit 4 - Force Event for Data Timeout Error"]
    #[inline]
    pub fn fe_data_timeout_err(&mut self) -> _FE_DATA_TIMEOUT_ERRW {
        _FE_DATA_TIMEOUT_ERRW { w: self }
    }
    #[doc = "Bit 3 - Force Event for Command Index Error"]
    #[inline]
    pub fn fe_cmd_ind_err(&mut self) -> _FE_CMD_IND_ERRW {
        _FE_CMD_IND_ERRW { w: self }
    }
    #[doc = "Bit 2 - Force Event for Command End Bit Error"]
    #[inline]
    pub fn fe_cmd_end_bit_err(&mut self) -> _FE_CMD_END_BIT_ERRW {
        _FE_CMD_END_BIT_ERRW { w: self }
    }
    #[doc = "Bit 1 - Force Event for Command CRC Error"]
    #[inline]
    pub fn fe_cmd_crc_err(&mut self) -> _FE_CMD_CRC_ERRW {
        _FE_CMD_CRC_ERRW { w: self }
    }
    #[doc = "Bit 0 - Force Event for Command Timeout Error"]
    #[inline]
    pub fn fe_cmd_timeout_err(&mut self) -> _FE_CMD_TIMEOUT_ERRW {
        _FE_CMD_TIMEOUT_ERRW { w: self }
    }
}
