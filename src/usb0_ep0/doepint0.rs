#[doc = "Register `DOEPINT0` reader"]
pub struct R(crate::R<DOEPINT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPINT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPINT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPINT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPINT0` writer"]
pub struct W(crate::W<DOEPINT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPINT0_SPEC>;
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
impl From<crate::W<DOEPINT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPINT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XferCompl` reader - Transfer Completed Interrupt"]
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
#[doc = "Field `XferCompl` writer - Transfer Completed Interrupt"]
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
#[doc = "Field `EPDisbld` reader - Endpoint Disabled Interrupt"]
pub struct EPDISBLD_R(crate::FieldReader<bool, bool>);
impl EPDISBLD_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPDISBLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPDISBLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPDisbld` writer - Endpoint Disabled Interrupt"]
pub struct EPDISBLD_W<'a> {
    w: &'a mut W,
}
impl<'a> EPDISBLD_W<'a> {
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
#[doc = "Field `SetUp` reader - SETUP Phase Done"]
pub struct SETUP_R(crate::FieldReader<bool, bool>);
impl SETUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SetUp` writer - SETUP Phase Done"]
pub struct SETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> SETUP_W<'a> {
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
#[doc = "Field `OUTTknEPdis` reader - OUT Token Received When Endpoint Disabled"]
pub struct OUTTKNEPDIS_R(crate::FieldReader<bool, bool>);
impl OUTTKNEPDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        OUTTKNEPDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTTKNEPDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTTknEPdis` writer - OUT Token Received When Endpoint Disabled"]
pub struct OUTTKNEPDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTKNEPDIS_W<'a> {
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
#[doc = "Field `StsPhseRcvd` reader - Status Phase Received For Control Write"]
pub struct STSPHSERCVD_R(crate::FieldReader<bool, bool>);
impl STSPHSERCVD_R {
    pub(crate) fn new(bits: bool) -> Self {
        STSPHSERCVD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STSPHSERCVD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `StsPhseRcvd` writer - Status Phase Received For Control Write"]
pub struct STSPHSERCVD_W<'a> {
    w: &'a mut W,
}
impl<'a> STSPHSERCVD_W<'a> {
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
#[doc = "Field `Back2BackSETup` reader - Back-to-Back SETUP Packets Received"]
pub struct BACK2BACKSETUP_R(crate::FieldReader<bool, bool>);
impl BACK2BACKSETUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        BACK2BACKSETUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACK2BACKSETUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Back2BackSETup` writer - Back-to-Back SETUP Packets Received"]
pub struct BACK2BACKSETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> BACK2BACKSETUP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `PktDrpSts` reader - Packet Dropped Status"]
pub struct PKTDRPSTS_R(crate::FieldReader<bool, bool>);
impl PKTDRPSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PKTDRPSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKTDRPSTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PktDrpSts` writer - Packet Dropped Status"]
pub struct PKTDRPSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> PKTDRPSTS_W<'a> {
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
#[doc = "Field `BbleErrIntrpt` reader - BbleErr (Babble Error) interrupt"]
pub struct BBLEERRINTRPT_R(crate::FieldReader<bool, bool>);
impl BBLEERRINTRPT_R {
    pub(crate) fn new(bits: bool) -> Self {
        BBLEERRINTRPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BBLEERRINTRPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BbleErrIntrpt` writer - BbleErr (Babble Error) interrupt"]
pub struct BBLEERRINTRPT_W<'a> {
    w: &'a mut W,
}
impl<'a> BBLEERRINTRPT_W<'a> {
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
#[doc = "Field `NAKIntrpt` reader - NAK interrupt"]
pub struct NAKINTRPT_R(crate::FieldReader<bool, bool>);
impl NAKINTRPT_R {
    pub(crate) fn new(bits: bool) -> Self {
        NAKINTRPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NAKINTRPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NAKIntrpt` writer - NAK interrupt"]
pub struct NAKINTRPT_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKINTRPT_W<'a> {
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
#[doc = "Field `NYETIntrpt` reader - NYET interrupt"]
pub struct NYETINTRPT_R(crate::FieldReader<bool, bool>);
impl NYETINTRPT_R {
    pub(crate) fn new(bits: bool) -> Self {
        NYETINTRPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NYETINTRPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NYETIntrpt` writer - NYET interrupt"]
pub struct NYETINTRPT_W<'a> {
    w: &'a mut W,
}
impl<'a> NYETINTRPT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Transfer Completed Interrupt"]
    #[inline(always)]
    pub fn xfer_compl(&self) -> XFERCOMPL_R {
        XFERCOMPL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt"]
    #[inline(always)]
    pub fn epdisbld(&self) -> EPDISBLD_R {
        EPDISBLD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    pub fn ahberr(&self) -> AHBERR_R {
        AHBERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SETUP Phase Done"]
    #[inline(always)]
    pub fn set_up(&self) -> SETUP_R {
        SETUP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - OUT Token Received When Endpoint Disabled"]
    #[inline(always)]
    pub fn outtkn_epdis(&self) -> OUTTKNEPDIS_R {
        OUTTKNEPDIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Status Phase Received For Control Write"]
    #[inline(always)]
    pub fn sts_phse_rcvd(&self) -> STSPHSERCVD_R {
        STSPHSERCVD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Back-to-Back SETUP Packets Received"]
    #[inline(always)]
    pub fn back2back_setup(&self) -> BACK2BACKSETUP_R {
        BACK2BACKSETUP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 9 - BNA (Buffer Not Available) Interrupt"]
    #[inline(always)]
    pub fn bnaintr(&self) -> BNAINTR_R {
        BNAINTR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Packet Dropped Status"]
    #[inline(always)]
    pub fn pkt_drp_sts(&self) -> PKTDRPSTS_R {
        PKTDRPSTS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - BbleErr (Babble Error) interrupt"]
    #[inline(always)]
    pub fn bble_err_intrpt(&self) -> BBLEERRINTRPT_R {
        BBLEERRINTRPT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - NAK interrupt"]
    #[inline(always)]
    pub fn nakintrpt(&self) -> NAKINTRPT_R {
        NAKINTRPT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - NYET interrupt"]
    #[inline(always)]
    pub fn nyetintrpt(&self) -> NYETINTRPT_R {
        NYETINTRPT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Completed Interrupt"]
    #[inline(always)]
    pub fn xfer_compl(&mut self) -> XFERCOMPL_W {
        XFERCOMPL_W { w: self }
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt"]
    #[inline(always)]
    pub fn epdisbld(&mut self) -> EPDISBLD_W {
        EPDISBLD_W { w: self }
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    pub fn ahberr(&mut self) -> AHBERR_W {
        AHBERR_W { w: self }
    }
    #[doc = "Bit 3 - SETUP Phase Done"]
    #[inline(always)]
    pub fn set_up(&mut self) -> SETUP_W {
        SETUP_W { w: self }
    }
    #[doc = "Bit 4 - OUT Token Received When Endpoint Disabled"]
    #[inline(always)]
    pub fn outtkn_epdis(&mut self) -> OUTTKNEPDIS_W {
        OUTTKNEPDIS_W { w: self }
    }
    #[doc = "Bit 5 - Status Phase Received For Control Write"]
    #[inline(always)]
    pub fn sts_phse_rcvd(&mut self) -> STSPHSERCVD_W {
        STSPHSERCVD_W { w: self }
    }
    #[doc = "Bit 6 - Back-to-Back SETUP Packets Received"]
    #[inline(always)]
    pub fn back2back_setup(&mut self) -> BACK2BACKSETUP_W {
        BACK2BACKSETUP_W { w: self }
    }
    #[doc = "Bit 9 - BNA (Buffer Not Available) Interrupt"]
    #[inline(always)]
    pub fn bnaintr(&mut self) -> BNAINTR_W {
        BNAINTR_W { w: self }
    }
    #[doc = "Bit 11 - Packet Dropped Status"]
    #[inline(always)]
    pub fn pkt_drp_sts(&mut self) -> PKTDRPSTS_W {
        PKTDRPSTS_W { w: self }
    }
    #[doc = "Bit 12 - BbleErr (Babble Error) interrupt"]
    #[inline(always)]
    pub fn bble_err_intrpt(&mut self) -> BBLEERRINTRPT_W {
        BBLEERRINTRPT_W { w: self }
    }
    #[doc = "Bit 13 - NAK interrupt"]
    #[inline(always)]
    pub fn nakintrpt(&mut self) -> NAKINTRPT_W {
        NAKINTRPT_W { w: self }
    }
    #[doc = "Bit 14 - NYET interrupt"]
    #[inline(always)]
    pub fn nyetintrpt(&mut self) -> NYETINTRPT_W {
        NYETINTRPT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Endpoint Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepint0](index.html) module"]
pub struct DOEPINT0_SPEC;
impl crate::RegisterSpec for DOEPINT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doepint0::R](R) reader structure"]
impl crate::Readable for DOEPINT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doepint0::W](W) writer structure"]
impl crate::Writable for DOEPINT0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DOEPINT0 to value 0x80"]
impl crate::Resettable for DOEPINT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
