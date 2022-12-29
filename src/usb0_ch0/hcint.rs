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
pub type XFER_COMPL_R = crate::BitReader<bool>;
#[doc = "Field `XferCompl` writer - Transfer Completed"]
pub type XFER_COMPL_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINT_SPEC, bool, O>;
#[doc = "Field `ChHltd` reader - Channel Halted"]
pub type CH_HLTD_R = crate::BitReader<bool>;
#[doc = "Field `ChHltd` writer - Channel Halted"]
pub type CH_HLTD_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINT_SPEC, bool, O>;
#[doc = "Field `AHBErr` reader - AHB Error"]
pub type AHBERR_R = crate::BitReader<bool>;
#[doc = "Field `AHBErr` writer - AHB Error"]
pub type AHBERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINT_SPEC, bool, O>;
#[doc = "Field `STALL` reader - STALL Response Received Interrupt"]
pub type STALL_R = crate::BitReader<bool>;
#[doc = "Field `STALL` writer - STALL Response Received Interrupt"]
pub type STALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINT_SPEC, bool, O>;
#[doc = "Field `NAK` reader - NAK Response Received Interrupt"]
pub type NAK_R = crate::BitReader<bool>;
#[doc = "Field `NAK` writer - NAK Response Received Interrupt"]
pub type NAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINT_SPEC, bool, O>;
#[doc = "Field `ACK` reader - ACK Response Received/Transmitted Interrupt"]
pub type ACK_R = crate::BitReader<bool>;
#[doc = "Field `ACK` writer - ACK Response Received/Transmitted Interrupt"]
pub type ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINT_SPEC, bool, O>;
#[doc = "Field `NYET` reader - NYET Response Received Interrupt"]
pub type NYET_R = crate::BitReader<bool>;
#[doc = "Field `NYET` writer - NYET Response Received Interrupt"]
pub type NYET_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINT_SPEC, bool, O>;
#[doc = "Field `XactErr` reader - Transaction Error"]
pub type XACT_ERR_R = crate::BitReader<bool>;
#[doc = "Field `XactErr` writer - Transaction Error"]
pub type XACT_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINT_SPEC, bool, O>;
#[doc = "Field `BblErr` reader - Babble Error"]
pub type BBL_ERR_R = crate::BitReader<bool>;
#[doc = "Field `BblErr` writer - Babble Error"]
pub type BBL_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINT_SPEC, bool, O>;
#[doc = "Field `FrmOvrun` reader - Frame Overrun"]
pub type FRM_OVRUN_R = crate::BitReader<bool>;
#[doc = "Field `FrmOvrun` writer - Frame Overrun"]
pub type FRM_OVRUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINT_SPEC, bool, O>;
#[doc = "Field `DataTglErr` reader - Data Toggle Error"]
pub type DATA_TGL_ERR_R = crate::BitReader<bool>;
#[doc = "Field `DataTglErr` writer - Data Toggle Error"]
pub type DATA_TGL_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINT_SPEC, bool, O>;
#[doc = "Field `BNAIntr` reader - BNA (Buffer Not Available) Interrupt"]
pub type BNAINTR_R = crate::BitReader<bool>;
#[doc = "Field `BNAIntr` writer - BNA (Buffer Not Available) Interrupt"]
pub type BNAINTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINT_SPEC, bool, O>;
#[doc = "Field `XCS_XACT_ERR` reader - Excessive Transaction Error"]
pub type XCS_XACT_ERR_R = crate::BitReader<bool>;
#[doc = "Field `XCS_XACT_ERR` writer - Excessive Transaction Error"]
pub type XCS_XACT_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINT_SPEC, bool, O>;
#[doc = "Field `DESC_LST_ROLLIntr` reader - Descriptor rollover interrupt"]
pub type DESC_LST_ROLLINTR_R = crate::BitReader<bool>;
#[doc = "Field `DESC_LST_ROLLIntr` writer - Descriptor rollover interrupt"]
pub type DESC_LST_ROLLINTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Transfer Completed"]
    #[inline(always)]
    pub fn xfer_compl(&self) -> XFER_COMPL_R {
        XFER_COMPL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Halted"]
    #[inline(always)]
    pub fn ch_hltd(&self) -> CH_HLTD_R {
        CH_HLTD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    pub fn ahberr(&self) -> AHBERR_R {
        AHBERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STALL Response Received Interrupt"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAK Response Received Interrupt"]
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ACK Response Received/Transmitted Interrupt"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NYET Response Received Interrupt"]
    #[inline(always)]
    pub fn nyet(&self) -> NYET_R {
        NYET_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transaction Error"]
    #[inline(always)]
    pub fn xact_err(&self) -> XACT_ERR_R {
        XACT_ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Babble Error"]
    #[inline(always)]
    pub fn bbl_err(&self) -> BBL_ERR_R {
        BBL_ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Frame Overrun"]
    #[inline(always)]
    pub fn frm_ovrun(&self) -> FRM_OVRUN_R {
        FRM_OVRUN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data Toggle Error"]
    #[inline(always)]
    pub fn data_tgl_err(&self) -> DATA_TGL_ERR_R {
        DATA_TGL_ERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BNA (Buffer Not Available) Interrupt"]
    #[inline(always)]
    pub fn bnaintr(&self) -> BNAINTR_R {
        BNAINTR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Excessive Transaction Error"]
    #[inline(always)]
    pub fn xcs_xact_err(&self) -> XCS_XACT_ERR_R {
        XCS_XACT_ERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Descriptor rollover interrupt"]
    #[inline(always)]
    pub fn desc_lst_rollintr(&self) -> DESC_LST_ROLLINTR_R {
        DESC_LST_ROLLINTR_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Completed"]
    #[inline(always)]
    #[must_use]
    pub fn xfer_compl(&mut self) -> XFER_COMPL_W<0> {
        XFER_COMPL_W::new(self)
    }
    #[doc = "Bit 1 - Channel Halted"]
    #[inline(always)]
    #[must_use]
    pub fn ch_hltd(&mut self) -> CH_HLTD_W<1> {
        CH_HLTD_W::new(self)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    #[must_use]
    pub fn ahberr(&mut self) -> AHBERR_W<2> {
        AHBERR_W::new(self)
    }
    #[doc = "Bit 3 - STALL Response Received Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<3> {
        STALL_W::new(self)
    }
    #[doc = "Bit 4 - NAK Response Received Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nak(&mut self) -> NAK_W<4> {
        NAK_W::new(self)
    }
    #[doc = "Bit 5 - ACK Response Received/Transmitted Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> ACK_W<5> {
        ACK_W::new(self)
    }
    #[doc = "Bit 6 - NYET Response Received Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nyet(&mut self) -> NYET_W<6> {
        NYET_W::new(self)
    }
    #[doc = "Bit 7 - Transaction Error"]
    #[inline(always)]
    #[must_use]
    pub fn xact_err(&mut self) -> XACT_ERR_W<7> {
        XACT_ERR_W::new(self)
    }
    #[doc = "Bit 8 - Babble Error"]
    #[inline(always)]
    #[must_use]
    pub fn bbl_err(&mut self) -> BBL_ERR_W<8> {
        BBL_ERR_W::new(self)
    }
    #[doc = "Bit 9 - Frame Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn frm_ovrun(&mut self) -> FRM_OVRUN_W<9> {
        FRM_OVRUN_W::new(self)
    }
    #[doc = "Bit 10 - Data Toggle Error"]
    #[inline(always)]
    #[must_use]
    pub fn data_tgl_err(&mut self) -> DATA_TGL_ERR_W<10> {
        DATA_TGL_ERR_W::new(self)
    }
    #[doc = "Bit 11 - BNA (Buffer Not Available) Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn bnaintr(&mut self) -> BNAINTR_W<11> {
        BNAINTR_W::new(self)
    }
    #[doc = "Bit 12 - Excessive Transaction Error"]
    #[inline(always)]
    #[must_use]
    pub fn xcs_xact_err(&mut self) -> XCS_XACT_ERR_W<12> {
        XCS_XACT_ERR_W::new(self)
    }
    #[doc = "Bit 13 - Descriptor rollover interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn desc_lst_rollintr(&mut self) -> DESC_LST_ROLLINTR_W<13> {
        DESC_LST_ROLLINTR_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCINT to value 0"]
impl crate::Resettable for HCINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
