#[doc = "Reader of register DOEPINT0"]
pub type R = crate::R<u32, super::DOEPINT0>;
#[doc = "Writer for register DOEPINT0"]
pub type W = crate::W<u32, super::DOEPINT0>;
#[doc = "Register DOEPINT0 `reset()`'s with value 0x80"]
impl crate::ResetValue for super::DOEPINT0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x80
    }
}
#[doc = "Reader of field `XferCompl`"]
pub type XFERCOMPL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XferCompl`"]
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `EPDisbld`"]
pub type EPDISBLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPDisbld`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `AHBErr`"]
pub type AHBERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AHBErr`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `SetUp`"]
pub type SETUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SetUp`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `OUTTknEPdis`"]
pub type OUTTKNEPDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTTknEPdis`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `StsPhseRcvd`"]
pub type STSPHSERCVD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `StsPhseRcvd`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `Back2BackSETup`"]
pub type BACK2BACKSETUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Back2BackSETup`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `BNAIntr`"]
pub type BNAINTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BNAIntr`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `PktDrpSts`"]
pub type PKTDRPSTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PktDrpSts`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `BbleErrIntrpt`"]
pub type BBLEERRINTRPT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BbleErrIntrpt`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `NAKIntrpt`"]
pub type NAKINTRPT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NAKIntrpt`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `NYETIntrpt`"]
pub type NYETINTRPT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NYETIntrpt`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
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
}
