#[doc = "Register `HCINTMSK` reader"]
pub struct R(crate::R<HCINTMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCINTMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCINTMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCINTMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCINTMSK` writer"]
pub struct W(crate::W<HCINTMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCINTMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<HCINTMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCINTMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XferComplMsk` reader - Transfer Completed Mask"]
pub struct XFERCOMPLMSK_R(crate::FieldReader<bool, bool>);
impl XFERCOMPLMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        XFERCOMPLMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XFERCOMPLMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XferComplMsk` writer - Transfer Completed Mask"]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `ChHltdMsk` reader - Channel Halted Mask"]
pub struct CHHLTDMSK_R(crate::FieldReader<bool, bool>);
impl CHHLTDMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHHLTDMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHHLTDMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ChHltdMsk` writer - Channel Halted Mask"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `AHBErrMsk` reader - AHB Error Mask"]
pub struct AHBERRMSK_R(crate::FieldReader<bool, bool>);
impl AHBERRMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        AHBERRMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHBERRMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHBErrMsk` writer - AHB Error Mask"]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `StallMsk` reader - STALL Response Received Interrupt Mask"]
pub struct STALLMSK_R(crate::FieldReader<bool, bool>);
impl STALLMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        STALLMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STALLMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `StallMsk` writer - STALL Response Received Interrupt Mask"]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `NakMsk` reader - NAK Response Received Interrupt Mask"]
pub struct NAKMSK_R(crate::FieldReader<bool, bool>);
impl NAKMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        NAKMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NAKMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NakMsk` writer - NAK Response Received Interrupt Mask"]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `AckMsk` reader - ACK Response Received/Transmitted Interrupt Mask"]
pub struct ACKMSK_R(crate::FieldReader<bool, bool>);
impl ACKMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACKMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACKMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AckMsk` writer - ACK Response Received/Transmitted Interrupt Mask"]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `NyetMsk` reader - NYET Response Received Interrupt Mask"]
pub struct NYETMSK_R(crate::FieldReader<bool, bool>);
impl NYETMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        NYETMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NYETMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NyetMsk` writer - NYET Response Received Interrupt Mask"]
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `XactErrMsk` reader - Transaction Error Mask"]
pub struct XACTERRMSK_R(crate::FieldReader<bool, bool>);
impl XACTERRMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        XACTERRMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XACTERRMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XactErrMsk` writer - Transaction Error Mask"]
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `BblErrMsk` reader - Babble Error Mask"]
pub struct BBLERRMSK_R(crate::FieldReader<bool, bool>);
impl BBLERRMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        BBLERRMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BBLERRMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BblErrMsk` writer - Babble Error Mask"]
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `FrmOvrunMsk` reader - Frame Overrun Mask"]
pub struct FRMOVRUNMSK_R(crate::FieldReader<bool, bool>);
impl FRMOVRUNMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRMOVRUNMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRMOVRUNMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FrmOvrunMsk` writer - Frame Overrun Mask"]
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `DataTglErrMsk` reader - Data Toggle Error Mask"]
pub struct DATATGLERRMSK_R(crate::FieldReader<bool, bool>);
impl DATATGLERRMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATATGLERRMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATATGLERRMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DataTglErrMsk` writer - Data Toggle Error Mask"]
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `BNAIntrMsk` reader - BNA (Buffer Not Available) Interrupt mask register"]
pub struct BNAINTRMSK_R(crate::FieldReader<bool, bool>);
impl BNAINTRMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        BNAINTRMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BNAINTRMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BNAIntrMsk` writer - BNA (Buffer Not Available) Interrupt mask register"]
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `DESC_LST_ROLLIntrMsk` reader - Descriptor rollover interrupt Mask register"]
pub struct DESC_LST_ROLLINTRMSK_R(crate::FieldReader<bool, bool>);
impl DESC_LST_ROLLINTRMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        DESC_LST_ROLLINTRMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DESC_LST_ROLLINTRMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DESC_LST_ROLLIntrMsk` writer - Descriptor rollover interrupt Mask register"]
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Channel Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcintmsk](index.html) module"]
pub struct HCINTMSK_SPEC;
impl crate::RegisterSpec for HCINTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcintmsk::R](R) reader structure"]
impl crate::Readable for HCINTMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcintmsk::W](W) writer structure"]
impl crate::Writable for HCINTMSK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCINTMSK to value 0"]
impl crate::Resettable for HCINTMSK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
