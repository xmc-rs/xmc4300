#[doc = "Register `HCINT` reader"]
pub struct R(crate::R<HCINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCINT` writer"]
pub struct W(crate::W<HCINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCINT_SPEC>;
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
impl From<crate::W<HCINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XferCompl` reader - Transfer Completed"]
pub struct XFERCOMPL_R(crate::FieldReader<bool, bool>);
impl XFERCOMPL_R {
    pub(crate) fn new(bits: bool) -> Self {
        XFERCOMPL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XFERCOMPL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XferCompl` writer - Transfer Completed"]
pub struct XFERCOMPL_W<'a> {
    w: &'a mut W,
}
impl<'a> XFERCOMPL_W<'a> {
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
#[doc = "Field `ChHltd` reader - Channel Halted"]
pub struct CHHLTD_R(crate::FieldReader<bool, bool>);
impl CHHLTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHHLTD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHHLTD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ChHltd` writer - Channel Halted"]
pub struct CHHLTD_W<'a> {
    w: &'a mut W,
}
impl<'a> CHHLTD_W<'a> {
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
#[doc = "Field `AHBErr` reader - AHB Error"]
pub struct AHBERR_R(crate::FieldReader<bool, bool>);
impl AHBERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        AHBERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHBERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHBErr` writer - AHB Error"]
pub struct AHBERR_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBERR_W<'a> {
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
#[doc = "Field `STALL` reader - STALL Response Received Interrupt"]
pub struct STALL_R(crate::FieldReader<bool, bool>);
impl STALL_R {
    pub(crate) fn new(bits: bool) -> Self {
        STALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STALL` writer - STALL Response Received Interrupt"]
pub struct STALL_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_W<'a> {
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
#[doc = "Field `NAK` reader - NAK Response Received Interrupt"]
pub struct NAK_R(crate::FieldReader<bool, bool>);
impl NAK_R {
    pub(crate) fn new(bits: bool) -> Self {
        NAK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NAK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NAK` writer - NAK Response Received Interrupt"]
pub struct NAK_W<'a> {
    w: &'a mut W,
}
impl<'a> NAK_W<'a> {
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
#[doc = "Field `ACK` reader - ACK Response Received/Transmitted Interrupt"]
pub struct ACK_R(crate::FieldReader<bool, bool>);
impl ACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACK` writer - ACK Response Received/Transmitted Interrupt"]
pub struct ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> ACK_W<'a> {
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
#[doc = "Field `NYET` reader - NYET Response Received Interrupt"]
pub struct NYET_R(crate::FieldReader<bool, bool>);
impl NYET_R {
    pub(crate) fn new(bits: bool) -> Self {
        NYET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NYET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NYET` writer - NYET Response Received Interrupt"]
pub struct NYET_W<'a> {
    w: &'a mut W,
}
impl<'a> NYET_W<'a> {
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
#[doc = "Field `XactErr` reader - Transaction Error"]
pub struct XACTERR_R(crate::FieldReader<bool, bool>);
impl XACTERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        XACTERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XACTERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XactErr` writer - Transaction Error"]
pub struct XACTERR_W<'a> {
    w: &'a mut W,
}
impl<'a> XACTERR_W<'a> {
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
#[doc = "Field `BblErr` reader - Babble Error"]
pub struct BBLERR_R(crate::FieldReader<bool, bool>);
impl BBLERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        BBLERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BBLERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BblErr` writer - Babble Error"]
pub struct BBLERR_W<'a> {
    w: &'a mut W,
}
impl<'a> BBLERR_W<'a> {
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
#[doc = "Field `FrmOvrun` reader - Frame Overrun"]
pub struct FRMOVRUN_R(crate::FieldReader<bool, bool>);
impl FRMOVRUN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRMOVRUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRMOVRUN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FrmOvrun` writer - Frame Overrun"]
pub struct FRMOVRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRMOVRUN_W<'a> {
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
#[doc = "Field `DataTglErr` reader - Data Toggle Error"]
pub struct DATATGLERR_R(crate::FieldReader<bool, bool>);
impl DATATGLERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATATGLERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATATGLERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DataTglErr` writer - Data Toggle Error"]
pub struct DATATGLERR_W<'a> {
    w: &'a mut W,
}
impl<'a> DATATGLERR_W<'a> {
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
#[doc = "Field `BNAIntr` reader - BNA (Buffer Not Available) Interrupt"]
pub struct BNAINTR_R(crate::FieldReader<bool, bool>);
impl BNAINTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        BNAINTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BNAINTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BNAIntr` writer - BNA (Buffer Not Available) Interrupt"]
pub struct BNAINTR_W<'a> {
    w: &'a mut W,
}
impl<'a> BNAINTR_W<'a> {
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
#[doc = "Field `XCS_XACT_ERR` reader - Excessive Transaction Error"]
pub struct XCS_XACT_ERR_R(crate::FieldReader<bool, bool>);
impl XCS_XACT_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        XCS_XACT_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XCS_XACT_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XCS_XACT_ERR` writer - Excessive Transaction Error"]
pub struct XCS_XACT_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> XCS_XACT_ERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `DESC_LST_ROLLIntr` reader - Descriptor rollover interrupt"]
pub struct DESC_LST_ROLLINTR_R(crate::FieldReader<bool, bool>);
impl DESC_LST_ROLLINTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DESC_LST_ROLLINTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DESC_LST_ROLLINTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DESC_LST_ROLLIntr` writer - Descriptor rollover interrupt"]
pub struct DESC_LST_ROLLINTR_W<'a> {
    w: &'a mut W,
}
impl<'a> DESC_LST_ROLLINTR_W<'a> {
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
    #[doc = "Bit 0 - Transfer Completed"]
    #[inline(always)]
    pub fn xfer_compl(&self) -> XFERCOMPL_R {
        XFERCOMPL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel Halted"]
    #[inline(always)]
    pub fn ch_hltd(&self) -> CHHLTD_R {
        CHHLTD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    pub fn ahberr(&self) -> AHBERR_R {
        AHBERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - STALL Response Received Interrupt"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - NAK Response Received Interrupt"]
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ACK Response Received/Transmitted Interrupt"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - NYET Response Received Interrupt"]
    #[inline(always)]
    pub fn nyet(&self) -> NYET_R {
        NYET_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transaction Error"]
    #[inline(always)]
    pub fn xact_err(&self) -> XACTERR_R {
        XACTERR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Babble Error"]
    #[inline(always)]
    pub fn bbl_err(&self) -> BBLERR_R {
        BBLERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Frame Overrun"]
    #[inline(always)]
    pub fn frm_ovrun(&self) -> FRMOVRUN_R {
        FRMOVRUN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Data Toggle Error"]
    #[inline(always)]
    pub fn data_tgl_err(&self) -> DATATGLERR_R {
        DATATGLERR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - BNA (Buffer Not Available) Interrupt"]
    #[inline(always)]
    pub fn bnaintr(&self) -> BNAINTR_R {
        BNAINTR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Excessive Transaction Error"]
    #[inline(always)]
    pub fn xcs_xact_err(&self) -> XCS_XACT_ERR_R {
        XCS_XACT_ERR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Descriptor rollover interrupt"]
    #[inline(always)]
    pub fn desc_lst_rollintr(&self) -> DESC_LST_ROLLINTR_R {
        DESC_LST_ROLLINTR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Completed"]
    #[inline(always)]
    pub fn xfer_compl(&mut self) -> XFERCOMPL_W {
        XFERCOMPL_W { w: self }
    }
    #[doc = "Bit 1 - Channel Halted"]
    #[inline(always)]
    pub fn ch_hltd(&mut self) -> CHHLTD_W {
        CHHLTD_W { w: self }
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    pub fn ahberr(&mut self) -> AHBERR_W {
        AHBERR_W { w: self }
    }
    #[doc = "Bit 3 - STALL Response Received Interrupt"]
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W {
        STALL_W { w: self }
    }
    #[doc = "Bit 4 - NAK Response Received Interrupt"]
    #[inline(always)]
    pub fn nak(&mut self) -> NAK_W {
        NAK_W { w: self }
    }
    #[doc = "Bit 5 - ACK Response Received/Transmitted Interrupt"]
    #[inline(always)]
    pub fn ack(&mut self) -> ACK_W {
        ACK_W { w: self }
    }
    #[doc = "Bit 6 - NYET Response Received Interrupt"]
    #[inline(always)]
    pub fn nyet(&mut self) -> NYET_W {
        NYET_W { w: self }
    }
    #[doc = "Bit 7 - Transaction Error"]
    #[inline(always)]
    pub fn xact_err(&mut self) -> XACTERR_W {
        XACTERR_W { w: self }
    }
    #[doc = "Bit 8 - Babble Error"]
    #[inline(always)]
    pub fn bbl_err(&mut self) -> BBLERR_W {
        BBLERR_W { w: self }
    }
    #[doc = "Bit 9 - Frame Overrun"]
    #[inline(always)]
    pub fn frm_ovrun(&mut self) -> FRMOVRUN_W {
        FRMOVRUN_W { w: self }
    }
    #[doc = "Bit 10 - Data Toggle Error"]
    #[inline(always)]
    pub fn data_tgl_err(&mut self) -> DATATGLERR_W {
        DATATGLERR_W { w: self }
    }
    #[doc = "Bit 11 - BNA (Buffer Not Available) Interrupt"]
    #[inline(always)]
    pub fn bnaintr(&mut self) -> BNAINTR_W {
        BNAINTR_W { w: self }
    }
    #[doc = "Bit 12 - Excessive Transaction Error"]
    #[inline(always)]
    pub fn xcs_xact_err(&mut self) -> XCS_XACT_ERR_W {
        XCS_XACT_ERR_W { w: self }
    }
    #[doc = "Bit 13 - Descriptor rollover interrupt"]
    #[inline(always)]
    pub fn desc_lst_rollintr(&mut self) -> DESC_LST_ROLLINTR_W {
        DESC_LST_ROLLINTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Channel Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcint](index.html) module"]
pub struct HCINT_SPEC;
impl crate::RegisterSpec for HCINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcint::R](R) reader structure"]
impl crate::Readable for HCINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcint::W](W) writer structure"]
impl crate::Writable for HCINT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCINT to value 0"]
impl crate::Resettable for HCINT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
