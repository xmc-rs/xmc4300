#[doc = "Writer for register TRBSCR"]
pub type W = crate::W<u32, super::TRBSCR>;
#[doc = "Register TRBSCR `reset()`'s with value 0"]
impl crate::ResetValue for super::TRBSCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clear Standard Receive Buffer Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSRBI_AW {
    #[doc = "0: No effect."]
    VALUE1 = 0,
    #[doc = "1: Clear TRBSR.SRBI."]
    VALUE2 = 1,
}
impl From<CSRBI_AW> for bool {
    #[inline(always)]
    fn from(variant: CSRBI_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CSRBI`"]
pub struct CSRBI_W<'a> {
    w: &'a mut W,
}
impl<'a> CSRBI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSRBI_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CSRBI_AW::VALUE1)
    }
    #[doc = "Clear TRBSR.SRBI."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CSRBI_AW::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Clear Receive Buffer Error Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRBERI_AW {
    #[doc = "0: No effect."]
    VALUE1 = 0,
    #[doc = "1: Clear TRBSR.RBERI."]
    VALUE2 = 1,
}
impl From<CRBERI_AW> for bool {
    #[inline(always)]
    fn from(variant: CRBERI_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CRBERI`"]
pub struct CRBERI_W<'a> {
    w: &'a mut W,
}
impl<'a> CRBERI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRBERI_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CRBERI_AW::VALUE1)
    }
    #[doc = "Clear TRBSR.RBERI."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CRBERI_AW::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Clear Alternative Receive Buffer Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARBI_AW {
    #[doc = "0: No effect."]
    VALUE1 = 0,
    #[doc = "1: Clear TRBSR.ARBI."]
    VALUE2 = 1,
}
impl From<CARBI_AW> for bool {
    #[inline(always)]
    fn from(variant: CARBI_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CARBI`"]
pub struct CARBI_W<'a> {
    w: &'a mut W,
}
impl<'a> CARBI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CARBI_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CARBI_AW::VALUE1)
    }
    #[doc = "Clear TRBSR.ARBI."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CARBI_AW::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Clear Standard Transmit Buffer Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSTBI_AW {
    #[doc = "0: No effect."]
    VALUE1 = 0,
    #[doc = "1: Clear TRBSR.STBI."]
    VALUE2 = 1,
}
impl From<CSTBI_AW> for bool {
    #[inline(always)]
    fn from(variant: CSTBI_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CSTBI`"]
pub struct CSTBI_W<'a> {
    w: &'a mut W,
}
impl<'a> CSTBI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSTBI_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CSTBI_AW::VALUE1)
    }
    #[doc = "Clear TRBSR.STBI."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CSTBI_AW::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Clear Transmit Buffer Error Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTBERI_AW {
    #[doc = "0: No effect."]
    VALUE1 = 0,
    #[doc = "1: Clear TRBSR.TBERI."]
    VALUE2 = 1,
}
impl From<CTBERI_AW> for bool {
    #[inline(always)]
    fn from(variant: CTBERI_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CTBERI`"]
pub struct CTBERI_W<'a> {
    w: &'a mut W,
}
impl<'a> CTBERI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTBERI_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CTBERI_AW::VALUE1)
    }
    #[doc = "Clear TRBSR.TBERI."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CTBERI_AW::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Clear Bypass Data Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CBDV_AW {
    #[doc = "0: No effect."]
    VALUE1 = 0,
    #[doc = "1: Clear BYPCR.BDV."]
    VALUE2 = 1,
}
impl From<CBDV_AW> for bool {
    #[inline(always)]
    fn from(variant: CBDV_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CBDV`"]
pub struct CBDV_W<'a> {
    w: &'a mut W,
}
impl<'a> CBDV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CBDV_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CBDV_AW::VALUE1)
    }
    #[doc = "Clear BYPCR.BDV."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CBDV_AW::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Flush Receive Buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLUSHRB_AW {
    #[doc = "0: No effect."]
    VALUE1 = 0,
    #[doc = "1: The receive FIFO buffer is cleared (filling level is cleared and output pointer is set to input pointer value). Should only be used while the FIFO buffer is not taking part in data traffic."]
    VALUE2 = 1,
}
impl From<FLUSHRB_AW> for bool {
    #[inline(always)]
    fn from(variant: FLUSHRB_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `FLUSHRB`"]
pub struct FLUSHRB_W<'a> {
    w: &'a mut W,
}
impl<'a> FLUSHRB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLUSHRB_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FLUSHRB_AW::VALUE1)
    }
    #[doc = "The receive FIFO buffer is cleared (filling level is cleared and output pointer is set to input pointer value). Should only be used while the FIFO buffer is not taking part in data traffic."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FLUSHRB_AW::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Flush Transmit Buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLUSHTB_AW {
    #[doc = "0: No effect."]
    VALUE1 = 0,
    #[doc = "1: The transmit FIFO buffer is cleared (filling level is cleared and output pointer is set to input pointer value). Should only be used while the FIFO buffer is not taking part in data traffic."]
    VALUE2 = 1,
}
impl From<FLUSHTB_AW> for bool {
    #[inline(always)]
    fn from(variant: FLUSHTB_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `FLUSHTB`"]
pub struct FLUSHTB_W<'a> {
    w: &'a mut W,
}
impl<'a> FLUSHTB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLUSHTB_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FLUSHTB_AW::VALUE1)
    }
    #[doc = "The transmit FIFO buffer is cleared (filling level is cleared and output pointer is set to input pointer value). Should only be used while the FIFO buffer is not taking part in data traffic."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FLUSHTB_AW::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Clear Standard Receive Buffer Event"]
    #[inline(always)]
    pub fn csrbi(&mut self) -> CSRBI_W {
        CSRBI_W { w: self }
    }
    #[doc = "Bit 1 - Clear Receive Buffer Error Event"]
    #[inline(always)]
    pub fn crberi(&mut self) -> CRBERI_W {
        CRBERI_W { w: self }
    }
    #[doc = "Bit 2 - Clear Alternative Receive Buffer Event"]
    #[inline(always)]
    pub fn carbi(&mut self) -> CARBI_W {
        CARBI_W { w: self }
    }
    #[doc = "Bit 8 - Clear Standard Transmit Buffer Event"]
    #[inline(always)]
    pub fn cstbi(&mut self) -> CSTBI_W {
        CSTBI_W { w: self }
    }
    #[doc = "Bit 9 - Clear Transmit Buffer Error Event"]
    #[inline(always)]
    pub fn ctberi(&mut self) -> CTBERI_W {
        CTBERI_W { w: self }
    }
    #[doc = "Bit 10 - Clear Bypass Data Valid"]
    #[inline(always)]
    pub fn cbdv(&mut self) -> CBDV_W {
        CBDV_W { w: self }
    }
    #[doc = "Bit 14 - Flush Receive Buffer"]
    #[inline(always)]
    pub fn flushrb(&mut self) -> FLUSHRB_W {
        FLUSHRB_W { w: self }
    }
    #[doc = "Bit 15 - Flush Transmit Buffer"]
    #[inline(always)]
    pub fn flushtb(&mut self) -> FLUSHTB_W {
        FLUSHTB_W { w: self }
    }
}
