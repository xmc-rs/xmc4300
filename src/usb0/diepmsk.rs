#[doc = "Reader of register DIEPMSK"]
pub type R = crate::R<u32, super::DIEPMSK>;
#[doc = "Writer for register DIEPMSK"]
pub type W = crate::W<u32, super::DIEPMSK>;
#[doc = "Register DIEPMSK `reset()`'s with value 0"]
impl crate::ResetValue for super::DIEPMSK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XferComplMsk`"]
pub type XFERCOMPLMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XferComplMsk`"]
pub struct XFERCOMPLMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> XFERCOMPLMSK_W<'a> {
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
#[doc = "Reader of field `EPDisbldMsk`"]
pub type EPDISBLDMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPDisbldMsk`"]
pub struct EPDISBLDMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> EPDISBLDMSK_W<'a> {
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
#[doc = "Reader of field `AHBErrMsk`"]
pub type AHBERRMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AHBErrMsk`"]
pub struct AHBERRMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBERRMSK_W<'a> {
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
#[doc = "Reader of field `TimeOUTMsk`"]
pub type TIMEOUTMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TimeOUTMsk`"]
pub struct TIMEOUTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUTMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `INTknTXFEmpMsk`"]
pub type INTKNTXFEMPMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTknTXFEmpMsk`"]
pub struct INTKNTXFEMPMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> INTKNTXFEMPMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `INEPNakEffMsk`"]
pub type INEPNAKEFFMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INEPNakEffMsk`"]
pub struct INEPNAKEFFMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> INEPNAKEFFMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `TxfifoUndrnMsk`"]
pub type TXFIFOUNDRNMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TxfifoUndrnMsk`"]
pub struct TXFIFOUNDRNMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFOUNDRNMSK_W<'a> {
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
#[doc = "Reader of field `BNAInIntrMsk`"]
pub type BNAININTRMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BNAInIntrMsk`"]
pub struct BNAININTRMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> BNAININTRMSK_W<'a> {
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
#[doc = "Reader of field `NAKMsk`"]
pub type NAKMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NAKMsk`"]
pub struct NAKMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Transfer Completed Interrupt Mask"]
    #[inline(always)]
    pub fn xfer_compl_msk(&self) -> XFERCOMPLMSK_R {
        XFERCOMPLMSK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt Mask"]
    #[inline(always)]
    pub fn epdisbld_msk(&self) -> EPDISBLDMSK_R {
        EPDISBLDMSK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AHB Error Mask"]
    #[inline(always)]
    pub fn ahberr_msk(&self) -> AHBERRMSK_R {
        AHBERRMSK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Timeout Condition Mask"]
    #[inline(always)]
    pub fn time_outmsk(&self) -> TIMEOUTMSK_R {
        TIMEOUTMSK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - IN Token Received When TxFIFO Empty Mask"]
    #[inline(always)]
    pub fn intkn_txfemp_msk(&self) -> INTKNTXFEMPMSK_R {
        INTKNTXFEMPMSK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - IN Endpoint NAK Effective Mask"]
    #[inline(always)]
    pub fn inepnak_eff_msk(&self) -> INEPNAKEFFMSK_R {
        INEPNAKEFFMSK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Fifo Underrun Mask"]
    #[inline(always)]
    pub fn txfifo_undrn_msk(&self) -> TXFIFOUNDRNMSK_R {
        TXFIFOUNDRNMSK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - BNA Interrupt Mask"]
    #[inline(always)]
    pub fn bnain_intr_msk(&self) -> BNAININTRMSK_R {
        BNAININTRMSK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 13 - NAK interrupt Mask"]
    #[inline(always)]
    pub fn nakmsk(&self) -> NAKMSK_R {
        NAKMSK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Completed Interrupt Mask"]
    #[inline(always)]
    pub fn xfer_compl_msk(&mut self) -> XFERCOMPLMSK_W {
        XFERCOMPLMSK_W { w: self }
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt Mask"]
    #[inline(always)]
    pub fn epdisbld_msk(&mut self) -> EPDISBLDMSK_W {
        EPDISBLDMSK_W { w: self }
    }
    #[doc = "Bit 2 - AHB Error Mask"]
    #[inline(always)]
    pub fn ahberr_msk(&mut self) -> AHBERRMSK_W {
        AHBERRMSK_W { w: self }
    }
    #[doc = "Bit 3 - Timeout Condition Mask"]
    #[inline(always)]
    pub fn time_outmsk(&mut self) -> TIMEOUTMSK_W {
        TIMEOUTMSK_W { w: self }
    }
    #[doc = "Bit 4 - IN Token Received When TxFIFO Empty Mask"]
    #[inline(always)]
    pub fn intkn_txfemp_msk(&mut self) -> INTKNTXFEMPMSK_W {
        INTKNTXFEMPMSK_W { w: self }
    }
    #[doc = "Bit 6 - IN Endpoint NAK Effective Mask"]
    #[inline(always)]
    pub fn inepnak_eff_msk(&mut self) -> INEPNAKEFFMSK_W {
        INEPNAKEFFMSK_W { w: self }
    }
    #[doc = "Bit 8 - Fifo Underrun Mask"]
    #[inline(always)]
    pub fn txfifo_undrn_msk(&mut self) -> TXFIFOUNDRNMSK_W {
        TXFIFOUNDRNMSK_W { w: self }
    }
    #[doc = "Bit 9 - BNA Interrupt Mask"]
    #[inline(always)]
    pub fn bnain_intr_msk(&mut self) -> BNAININTRMSK_W {
        BNAININTRMSK_W { w: self }
    }
    #[doc = "Bit 13 - NAK interrupt Mask"]
    #[inline(always)]
    pub fn nakmsk(&mut self) -> NAKMSK_W {
        NAKMSK_W { w: self }
    }
}
