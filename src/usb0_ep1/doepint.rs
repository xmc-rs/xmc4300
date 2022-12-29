#[doc = "Register `DOEPINT` reader"]
pub struct R(crate::R<DOEPINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPINT` writer"]
pub struct W(crate::W<DOEPINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPINT_SPEC>;
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
impl From<crate::W<DOEPINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XferCompl` reader - Transfer Completed Interrupt"]
pub type XFER_COMPL_R = crate::BitReader<bool>;
#[doc = "Field `XferCompl` writer - Transfer Completed Interrupt"]
pub type XFER_COMPL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT_SPEC, bool, O>;
#[doc = "Field `EPDisbld` reader - Endpoint Disabled Interrupt"]
pub type EPDISBLD_R = crate::BitReader<bool>;
#[doc = "Field `EPDisbld` writer - Endpoint Disabled Interrupt"]
pub type EPDISBLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT_SPEC, bool, O>;
#[doc = "Field `AHBErr` reader - AHB Error"]
pub type AHBERR_R = crate::BitReader<bool>;
#[doc = "Field `AHBErr` writer - AHB Error"]
pub type AHBERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT_SPEC, bool, O>;
#[doc = "Field `SetUp` reader - SETUP Phase Done"]
pub type SET_UP_R = crate::BitReader<bool>;
#[doc = "Field `SetUp` writer - SETUP Phase Done"]
pub type SET_UP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT_SPEC, bool, O>;
#[doc = "Field `OUTTknEPdis` reader - OUT Token Received When Endpoint Disabled"]
pub type OUTTKN_EPDIS_R = crate::BitReader<bool>;
#[doc = "Field `OUTTknEPdis` writer - OUT Token Received When Endpoint Disabled"]
pub type OUTTKN_EPDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT_SPEC, bool, O>;
#[doc = "Field `StsPhseRcvd` reader - Status Phase Received For Control Write"]
pub type STS_PHSE_RCVD_R = crate::BitReader<bool>;
#[doc = "Field `StsPhseRcvd` writer - Status Phase Received For Control Write"]
pub type STS_PHSE_RCVD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT_SPEC, bool, O>;
#[doc = "Field `Back2BackSETup` reader - Back-to-Back SETUP Packets Received"]
pub type BACK2BACK_SETUP_R = crate::BitReader<bool>;
#[doc = "Field `Back2BackSETup` writer - Back-to-Back SETUP Packets Received"]
pub type BACK2BACK_SETUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT_SPEC, bool, O>;
#[doc = "Field `BNAIntr` reader - BNA (Buffer Not Available) Interrupt"]
pub type BNAINTR_R = crate::BitReader<bool>;
#[doc = "Field `BNAIntr` writer - BNA (Buffer Not Available) Interrupt"]
pub type BNAINTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT_SPEC, bool, O>;
#[doc = "Field `PktDrpSts` reader - Packet Dropped Status"]
pub type PKT_DRP_STS_R = crate::BitReader<bool>;
#[doc = "Field `PktDrpSts` writer - Packet Dropped Status"]
pub type PKT_DRP_STS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT_SPEC, bool, O>;
#[doc = "Field `BbleErrIntrpt` reader - BbleErr (Babble Error) interrupt"]
pub type BBLE_ERR_INTRPT_R = crate::BitReader<bool>;
#[doc = "Field `BbleErrIntrpt` writer - BbleErr (Babble Error) interrupt"]
pub type BBLE_ERR_INTRPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT_SPEC, bool, O>;
#[doc = "Field `NAKIntrpt` reader - NAK interrupt"]
pub type NAKINTRPT_R = crate::BitReader<bool>;
#[doc = "Field `NAKIntrpt` writer - NAK interrupt"]
pub type NAKINTRPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT_SPEC, bool, O>;
#[doc = "Field `NYETIntrpt` reader - NYET interrupt"]
pub type NYETINTRPT_R = crate::BitReader<bool>;
#[doc = "Field `NYETIntrpt` writer - NYET interrupt"]
pub type NYETINTRPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Transfer Completed Interrupt"]
    #[inline(always)]
    pub fn xfer_compl(&self) -> XFER_COMPL_R {
        XFER_COMPL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt"]
    #[inline(always)]
    pub fn epdisbld(&self) -> EPDISBLD_R {
        EPDISBLD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    pub fn ahberr(&self) -> AHBERR_R {
        AHBERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SETUP Phase Done"]
    #[inline(always)]
    pub fn set_up(&self) -> SET_UP_R {
        SET_UP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OUT Token Received When Endpoint Disabled"]
    #[inline(always)]
    pub fn outtkn_epdis(&self) -> OUTTKN_EPDIS_R {
        OUTTKN_EPDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Status Phase Received For Control Write"]
    #[inline(always)]
    pub fn sts_phse_rcvd(&self) -> STS_PHSE_RCVD_R {
        STS_PHSE_RCVD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Back-to-Back SETUP Packets Received"]
    #[inline(always)]
    pub fn back2back_setup(&self) -> BACK2BACK_SETUP_R {
        BACK2BACK_SETUP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - BNA (Buffer Not Available) Interrupt"]
    #[inline(always)]
    pub fn bnaintr(&self) -> BNAINTR_R {
        BNAINTR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Packet Dropped Status"]
    #[inline(always)]
    pub fn pkt_drp_sts(&self) -> PKT_DRP_STS_R {
        PKT_DRP_STS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - BbleErr (Babble Error) interrupt"]
    #[inline(always)]
    pub fn bble_err_intrpt(&self) -> BBLE_ERR_INTRPT_R {
        BBLE_ERR_INTRPT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NAK interrupt"]
    #[inline(always)]
    pub fn nakintrpt(&self) -> NAKINTRPT_R {
        NAKINTRPT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NYET interrupt"]
    #[inline(always)]
    pub fn nyetintrpt(&self) -> NYETINTRPT_R {
        NYETINTRPT_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Completed Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn xfer_compl(&mut self) -> XFER_COMPL_W<0> {
        XFER_COMPL_W::new(self)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn epdisbld(&mut self) -> EPDISBLD_W<1> {
        EPDISBLD_W::new(self)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    #[must_use]
    pub fn ahberr(&mut self) -> AHBERR_W<2> {
        AHBERR_W::new(self)
    }
    #[doc = "Bit 3 - SETUP Phase Done"]
    #[inline(always)]
    #[must_use]
    pub fn set_up(&mut self) -> SET_UP_W<3> {
        SET_UP_W::new(self)
    }
    #[doc = "Bit 4 - OUT Token Received When Endpoint Disabled"]
    #[inline(always)]
    #[must_use]
    pub fn outtkn_epdis(&mut self) -> OUTTKN_EPDIS_W<4> {
        OUTTKN_EPDIS_W::new(self)
    }
    #[doc = "Bit 5 - Status Phase Received For Control Write"]
    #[inline(always)]
    #[must_use]
    pub fn sts_phse_rcvd(&mut self) -> STS_PHSE_RCVD_W<5> {
        STS_PHSE_RCVD_W::new(self)
    }
    #[doc = "Bit 6 - Back-to-Back SETUP Packets Received"]
    #[inline(always)]
    #[must_use]
    pub fn back2back_setup(&mut self) -> BACK2BACK_SETUP_W<6> {
        BACK2BACK_SETUP_W::new(self)
    }
    #[doc = "Bit 9 - BNA (Buffer Not Available) Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn bnaintr(&mut self) -> BNAINTR_W<9> {
        BNAINTR_W::new(self)
    }
    #[doc = "Bit 11 - Packet Dropped Status"]
    #[inline(always)]
    #[must_use]
    pub fn pkt_drp_sts(&mut self) -> PKT_DRP_STS_W<11> {
        PKT_DRP_STS_W::new(self)
    }
    #[doc = "Bit 12 - BbleErr (Babble Error) interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn bble_err_intrpt(&mut self) -> BBLE_ERR_INTRPT_W<12> {
        BBLE_ERR_INTRPT_W::new(self)
    }
    #[doc = "Bit 13 - NAK interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nakintrpt(&mut self) -> NAKINTRPT_W<13> {
        NAKINTRPT_W::new(self)
    }
    #[doc = "Bit 14 - NYET interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nyetintrpt(&mut self) -> NYETINTRPT_W<14> {
        NYETINTRPT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Endpoint Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepint](index.html) module"]
pub struct DOEPINT_SPEC;
impl crate::RegisterSpec for DOEPINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doepint::R](R) reader structure"]
impl crate::Readable for DOEPINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doepint::W](W) writer structure"]
impl crate::Writable for DOEPINT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPINT to value 0x80"]
impl crate::Resettable for DOEPINT_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
