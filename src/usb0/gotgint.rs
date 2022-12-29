#[doc = "Register `GOTGINT` reader"]
pub struct R(crate::R<GOTGINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GOTGINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GOTGINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GOTGINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GOTGINT` writer"]
pub struct W(crate::W<GOTGINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GOTGINT_SPEC>;
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
impl From<crate::W<GOTGINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GOTGINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SesEndDet` reader - Session End Detected"]
pub type SES_END_DET_R = crate::BitReader<bool>;
#[doc = "Field `SesEndDet` writer - Session End Detected"]
pub type SES_END_DET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGINT_SPEC, bool, O>;
#[doc = "Field `SesReqSucStsChng` reader - Session Request Success Status Change"]
pub type SES_REQ_SUC_STS_CHNG_R = crate::BitReader<bool>;
#[doc = "Field `SesReqSucStsChng` writer - Session Request Success Status Change"]
pub type SES_REQ_SUC_STS_CHNG_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGINT_SPEC, bool, O>;
#[doc = "Field `HstNegSucStsChng` reader - Host Negotiation Success Status Change"]
pub type HST_NEG_SUC_STS_CHNG_R = crate::BitReader<bool>;
#[doc = "Field `HstNegSucStsChng` writer - Host Negotiation Success Status Change"]
pub type HST_NEG_SUC_STS_CHNG_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGINT_SPEC, bool, O>;
#[doc = "Field `HstNegDet` reader - Host Negotiation Detected"]
pub type HST_NEG_DET_R = crate::BitReader<bool>;
#[doc = "Field `HstNegDet` writer - Host Negotiation Detected"]
pub type HST_NEG_DET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGINT_SPEC, bool, O>;
#[doc = "Field `ADevTOUTChg` reader - A-Device Timeout Change"]
pub type ADEV_TOUTCHG_R = crate::BitReader<bool>;
#[doc = "Field `ADevTOUTChg` writer - A-Device Timeout Change"]
pub type ADEV_TOUTCHG_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGINT_SPEC, bool, O>;
#[doc = "Field `DbnceDone` reader - Debounce Done"]
pub type DBNCE_DONE_R = crate::BitReader<bool>;
#[doc = "Field `DbnceDone` writer - Debounce Done"]
pub type DBNCE_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGINT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - Session End Detected"]
    #[inline(always)]
    pub fn ses_end_det(&self) -> SES_END_DET_R {
        SES_END_DET_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Session Request Success Status Change"]
    #[inline(always)]
    pub fn ses_req_suc_sts_chng(&self) -> SES_REQ_SUC_STS_CHNG_R {
        SES_REQ_SUC_STS_CHNG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Host Negotiation Success Status Change"]
    #[inline(always)]
    pub fn hst_neg_suc_sts_chng(&self) -> HST_NEG_SUC_STS_CHNG_R {
        HST_NEG_SUC_STS_CHNG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 17 - Host Negotiation Detected"]
    #[inline(always)]
    pub fn hst_neg_det(&self) -> HST_NEG_DET_R {
        HST_NEG_DET_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - A-Device Timeout Change"]
    #[inline(always)]
    pub fn adev_toutchg(&self) -> ADEV_TOUTCHG_R {
        ADEV_TOUTCHG_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Debounce Done"]
    #[inline(always)]
    pub fn dbnce_done(&self) -> DBNCE_DONE_R {
        DBNCE_DONE_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Session End Detected"]
    #[inline(always)]
    #[must_use]
    pub fn ses_end_det(&mut self) -> SES_END_DET_W<2> {
        SES_END_DET_W::new(self)
    }
    #[doc = "Bit 8 - Session Request Success Status Change"]
    #[inline(always)]
    #[must_use]
    pub fn ses_req_suc_sts_chng(&mut self) -> SES_REQ_SUC_STS_CHNG_W<8> {
        SES_REQ_SUC_STS_CHNG_W::new(self)
    }
    #[doc = "Bit 9 - Host Negotiation Success Status Change"]
    #[inline(always)]
    #[must_use]
    pub fn hst_neg_suc_sts_chng(&mut self) -> HST_NEG_SUC_STS_CHNG_W<9> {
        HST_NEG_SUC_STS_CHNG_W::new(self)
    }
    #[doc = "Bit 17 - Host Negotiation Detected"]
    #[inline(always)]
    #[must_use]
    pub fn hst_neg_det(&mut self) -> HST_NEG_DET_W<17> {
        HST_NEG_DET_W::new(self)
    }
    #[doc = "Bit 18 - A-Device Timeout Change"]
    #[inline(always)]
    #[must_use]
    pub fn adev_toutchg(&mut self) -> ADEV_TOUTCHG_W<18> {
        ADEV_TOUTCHG_W::new(self)
    }
    #[doc = "Bit 19 - Debounce Done"]
    #[inline(always)]
    #[must_use]
    pub fn dbnce_done(&mut self) -> DBNCE_DONE_W<19> {
        DBNCE_DONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gotgint](index.html) module"]
pub struct GOTGINT_SPEC;
impl crate::RegisterSpec for GOTGINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gotgint::R](R) reader structure"]
impl crate::Readable for GOTGINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gotgint::W](W) writer structure"]
impl crate::Writable for GOTGINT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GOTGINT to value 0"]
impl crate::Resettable for GOTGINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
