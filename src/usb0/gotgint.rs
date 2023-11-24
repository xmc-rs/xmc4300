#[doc = "Register `GOTGINT` reader"]
pub type R = crate::R<GOTGINT_SPEC>;
#[doc = "Register `GOTGINT` writer"]
pub type W = crate::W<GOTGINT_SPEC>;
#[doc = "Field `SesEndDet` reader - Session End Detected"]
pub type SES_END_DET_R = crate::BitReader;
#[doc = "Field `SesEndDet` writer - Session End Detected"]
pub type SES_END_DET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SesReqSucStsChng` reader - Session Request Success Status Change"]
pub type SES_REQ_SUC_STS_CHNG_R = crate::BitReader;
#[doc = "Field `SesReqSucStsChng` writer - Session Request Success Status Change"]
pub type SES_REQ_SUC_STS_CHNG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HstNegSucStsChng` reader - Host Negotiation Success Status Change"]
pub type HST_NEG_SUC_STS_CHNG_R = crate::BitReader;
#[doc = "Field `HstNegSucStsChng` writer - Host Negotiation Success Status Change"]
pub type HST_NEG_SUC_STS_CHNG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HstNegDet` reader - Host Negotiation Detected"]
pub type HST_NEG_DET_R = crate::BitReader;
#[doc = "Field `HstNegDet` writer - Host Negotiation Detected"]
pub type HST_NEG_DET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADevTOUTChg` reader - A-Device Timeout Change"]
pub type ADEV_TOUTCHG_R = crate::BitReader;
#[doc = "Field `ADevTOUTChg` writer - A-Device Timeout Change"]
pub type ADEV_TOUTCHG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DbnceDone` reader - Debounce Done"]
pub type DBNCE_DONE_R = crate::BitReader;
#[doc = "Field `DbnceDone` writer - Debounce Done"]
pub type DBNCE_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn ses_end_det(&mut self) -> SES_END_DET_W<GOTGINT_SPEC> {
        SES_END_DET_W::new(self, 2)
    }
    #[doc = "Bit 8 - Session Request Success Status Change"]
    #[inline(always)]
    #[must_use]
    pub fn ses_req_suc_sts_chng(&mut self) -> SES_REQ_SUC_STS_CHNG_W<GOTGINT_SPEC> {
        SES_REQ_SUC_STS_CHNG_W::new(self, 8)
    }
    #[doc = "Bit 9 - Host Negotiation Success Status Change"]
    #[inline(always)]
    #[must_use]
    pub fn hst_neg_suc_sts_chng(&mut self) -> HST_NEG_SUC_STS_CHNG_W<GOTGINT_SPEC> {
        HST_NEG_SUC_STS_CHNG_W::new(self, 9)
    }
    #[doc = "Bit 17 - Host Negotiation Detected"]
    #[inline(always)]
    #[must_use]
    pub fn hst_neg_det(&mut self) -> HST_NEG_DET_W<GOTGINT_SPEC> {
        HST_NEG_DET_W::new(self, 17)
    }
    #[doc = "Bit 18 - A-Device Timeout Change"]
    #[inline(always)]
    #[must_use]
    pub fn adev_toutchg(&mut self) -> ADEV_TOUTCHG_W<GOTGINT_SPEC> {
        ADEV_TOUTCHG_W::new(self, 18)
    }
    #[doc = "Bit 19 - Debounce Done"]
    #[inline(always)]
    #[must_use]
    pub fn dbnce_done(&mut self) -> DBNCE_DONE_W<GOTGINT_SPEC> {
        DBNCE_DONE_W::new(self, 19)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "OTG Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gotgint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gotgint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GOTGINT_SPEC;
impl crate::RegisterSpec for GOTGINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gotgint::R`](R) reader structure"]
impl crate::Readable for GOTGINT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gotgint::W`](W) writer structure"]
impl crate::Writable for GOTGINT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GOTGINT to value 0"]
impl crate::Resettable for GOTGINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
