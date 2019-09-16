#[doc = "Reader of register HCINTMSK"]
pub type R = crate::R<u32, super::HCINTMSK>;
#[doc = "Writer for register HCINTMSK"]
pub type W = crate::W<u32, super::HCINTMSK>;
#[doc = "Register HCINTMSK `reset()`'s with value 0"]
impl crate::ResetValue for super::HCINTMSK {
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
#[doc = "Reader of field `ChHltdMsk`"]
pub type CHHLTDMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ChHltdMsk`"]
pub struct CHHLTDMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> CHHLTDMSK_W<'a> {
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
#[doc = "Reader of field `StallMsk`"]
pub type STALLMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `StallMsk`"]
pub struct STALLMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> STALLMSK_W<'a> {
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
#[doc = "Reader of field `NakMsk`"]
pub type NAKMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NakMsk`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `AckMsk`"]
pub type ACKMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AckMsk`"]
pub struct ACKMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> ACKMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `NyetMsk`"]
pub type NYETMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NyetMsk`"]
pub struct NYETMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> NYETMSK_W<'a> {
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
#[doc = "Reader of field `XactErrMsk`"]
pub type XACTERRMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XactErrMsk`"]
pub struct XACTERRMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> XACTERRMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `BblErrMsk`"]
pub type BBLERRMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BblErrMsk`"]
pub struct BBLERRMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> BBLERRMSK_W<'a> {
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
#[doc = "Reader of field `FrmOvrunMsk`"]
pub type FRMOVRUNMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FrmOvrunMsk`"]
pub struct FRMOVRUNMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> FRMOVRUNMSK_W<'a> {
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
#[doc = "Reader of field `DataTglErrMsk`"]
pub type DATATGLERRMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DataTglErrMsk`"]
pub struct DATATGLERRMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> DATATGLERRMSK_W<'a> {
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
#[doc = "Reader of field `BNAIntrMsk`"]
pub type BNAINTRMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BNAIntrMsk`"]
pub struct BNAINTRMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> BNAINTRMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `DESC_LST_ROLLIntrMsk`"]
pub type DESC_LST_ROLLINTRMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DESC_LST_ROLLIntrMsk`"]
pub struct DESC_LST_ROLLINTRMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> DESC_LST_ROLLINTRMSK_W<'a> {
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
    #[doc = "Bit 0 - Transfer Completed Mask"]
    #[inline(always)]
    pub fn xfer_compl_msk(&self) -> XFERCOMPLMSK_R {
        XFERCOMPLMSK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel Halted Mask"]
    #[inline(always)]
    pub fn ch_hltd_msk(&self) -> CHHLTDMSK_R {
        CHHLTDMSK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AHB Error Mask"]
    #[inline(always)]
    pub fn ahberr_msk(&self) -> AHBERRMSK_R {
        AHBERRMSK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - STALL Response Received Interrupt Mask"]
    #[inline(always)]
    pub fn stall_msk(&self) -> STALLMSK_R {
        STALLMSK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - NAK Response Received Interrupt Mask"]
    #[inline(always)]
    pub fn nak_msk(&self) -> NAKMSK_R {
        NAKMSK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ACK Response Received/Transmitted Interrupt Mask"]
    #[inline(always)]
    pub fn ack_msk(&self) -> ACKMSK_R {
        ACKMSK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - NYET Response Received Interrupt Mask"]
    #[inline(always)]
    pub fn nyet_msk(&self) -> NYETMSK_R {
        NYETMSK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transaction Error Mask"]
    #[inline(always)]
    pub fn xact_err_msk(&self) -> XACTERRMSK_R {
        XACTERRMSK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Babble Error Mask"]
    #[inline(always)]
    pub fn bbl_err_msk(&self) -> BBLERRMSK_R {
        BBLERRMSK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Frame Overrun Mask"]
    #[inline(always)]
    pub fn frm_ovrun_msk(&self) -> FRMOVRUNMSK_R {
        FRMOVRUNMSK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Data Toggle Error Mask"]
    #[inline(always)]
    pub fn data_tgl_err_msk(&self) -> DATATGLERRMSK_R {
        DATATGLERRMSK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - BNA (Buffer Not Available) Interrupt mask register"]
    #[inline(always)]
    pub fn bnaintr_msk(&self) -> BNAINTRMSK_R {
        BNAINTRMSK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Descriptor rollover interrupt Mask register"]
    #[inline(always)]
    pub fn desc_lst_rollintr_msk(&self) -> DESC_LST_ROLLINTRMSK_R {
        DESC_LST_ROLLINTRMSK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Completed Mask"]
    #[inline(always)]
    pub fn xfer_compl_msk(&mut self) -> XFERCOMPLMSK_W {
        XFERCOMPLMSK_W { w: self }
    }
    #[doc = "Bit 1 - Channel Halted Mask"]
    #[inline(always)]
    pub fn ch_hltd_msk(&mut self) -> CHHLTDMSK_W {
        CHHLTDMSK_W { w: self }
    }
    #[doc = "Bit 2 - AHB Error Mask"]
    #[inline(always)]
    pub fn ahberr_msk(&mut self) -> AHBERRMSK_W {
        AHBERRMSK_W { w: self }
    }
    #[doc = "Bit 3 - STALL Response Received Interrupt Mask"]
    #[inline(always)]
    pub fn stall_msk(&mut self) -> STALLMSK_W {
        STALLMSK_W { w: self }
    }
    #[doc = "Bit 4 - NAK Response Received Interrupt Mask"]
    #[inline(always)]
    pub fn nak_msk(&mut self) -> NAKMSK_W {
        NAKMSK_W { w: self }
    }
    #[doc = "Bit 5 - ACK Response Received/Transmitted Interrupt Mask"]
    #[inline(always)]
    pub fn ack_msk(&mut self) -> ACKMSK_W {
        ACKMSK_W { w: self }
    }
    #[doc = "Bit 6 - NYET Response Received Interrupt Mask"]
    #[inline(always)]
    pub fn nyet_msk(&mut self) -> NYETMSK_W {
        NYETMSK_W { w: self }
    }
    #[doc = "Bit 7 - Transaction Error Mask"]
    #[inline(always)]
    pub fn xact_err_msk(&mut self) -> XACTERRMSK_W {
        XACTERRMSK_W { w: self }
    }
    #[doc = "Bit 8 - Babble Error Mask"]
    #[inline(always)]
    pub fn bbl_err_msk(&mut self) -> BBLERRMSK_W {
        BBLERRMSK_W { w: self }
    }
    #[doc = "Bit 9 - Frame Overrun Mask"]
    #[inline(always)]
    pub fn frm_ovrun_msk(&mut self) -> FRMOVRUNMSK_W {
        FRMOVRUNMSK_W { w: self }
    }
    #[doc = "Bit 10 - Data Toggle Error Mask"]
    #[inline(always)]
    pub fn data_tgl_err_msk(&mut self) -> DATATGLERRMSK_W {
        DATATGLERRMSK_W { w: self }
    }
    #[doc = "Bit 11 - BNA (Buffer Not Available) Interrupt mask register"]
    #[inline(always)]
    pub fn bnaintr_msk(&mut self) -> BNAINTRMSK_W {
        BNAINTRMSK_W { w: self }
    }
    #[doc = "Bit 13 - Descriptor rollover interrupt Mask register"]
    #[inline(always)]
    pub fn desc_lst_rollintr_msk(&mut self) -> DESC_LST_ROLLINTRMSK_W {
        DESC_LST_ROLLINTRMSK_W { w: self }
    }
}
