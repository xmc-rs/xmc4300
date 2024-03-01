#[doc = "Register `GOTGINT` reader"]
pub type R = crate::R<GotgintSpec>;
#[doc = "Register `GOTGINT` writer"]
pub type W = crate::W<GotgintSpec>;
#[doc = "Field `SesEndDet` reader - Session End Detected"]
pub type SesEndDetR = crate::BitReader;
#[doc = "Field `SesEndDet` writer - Session End Detected"]
pub type SesEndDetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SesReqSucStsChng` reader - Session Request Success Status Change"]
pub type SesReqSucStsChngR = crate::BitReader;
#[doc = "Field `SesReqSucStsChng` writer - Session Request Success Status Change"]
pub type SesReqSucStsChngW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HstNegSucStsChng` reader - Host Negotiation Success Status Change"]
pub type HstNegSucStsChngR = crate::BitReader;
#[doc = "Field `HstNegSucStsChng` writer - Host Negotiation Success Status Change"]
pub type HstNegSucStsChngW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HstNegDet` reader - Host Negotiation Detected"]
pub type HstNegDetR = crate::BitReader;
#[doc = "Field `HstNegDet` writer - Host Negotiation Detected"]
pub type HstNegDetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADevTOUTChg` reader - A-Device Timeout Change"]
pub type AdevToutchgR = crate::BitReader;
#[doc = "Field `ADevTOUTChg` writer - A-Device Timeout Change"]
pub type AdevToutchgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DbnceDone` reader - Debounce Done"]
pub type DbnceDoneR = crate::BitReader;
#[doc = "Field `DbnceDone` writer - Debounce Done"]
pub type DbnceDoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Session End Detected"]
    #[inline(always)]
    pub fn ses_end_det(&self) -> SesEndDetR {
        SesEndDetR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Session Request Success Status Change"]
    #[inline(always)]
    pub fn ses_req_suc_sts_chng(&self) -> SesReqSucStsChngR {
        SesReqSucStsChngR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Host Negotiation Success Status Change"]
    #[inline(always)]
    pub fn hst_neg_suc_sts_chng(&self) -> HstNegSucStsChngR {
        HstNegSucStsChngR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 17 - Host Negotiation Detected"]
    #[inline(always)]
    pub fn hst_neg_det(&self) -> HstNegDetR {
        HstNegDetR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - A-Device Timeout Change"]
    #[inline(always)]
    pub fn adev_toutchg(&self) -> AdevToutchgR {
        AdevToutchgR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Debounce Done"]
    #[inline(always)]
    pub fn dbnce_done(&self) -> DbnceDoneR {
        DbnceDoneR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Session End Detected"]
    #[inline(always)]
    #[must_use]
    pub fn ses_end_det(&mut self) -> SesEndDetW<GotgintSpec> {
        SesEndDetW::new(self, 2)
    }
    #[doc = "Bit 8 - Session Request Success Status Change"]
    #[inline(always)]
    #[must_use]
    pub fn ses_req_suc_sts_chng(&mut self) -> SesReqSucStsChngW<GotgintSpec> {
        SesReqSucStsChngW::new(self, 8)
    }
    #[doc = "Bit 9 - Host Negotiation Success Status Change"]
    #[inline(always)]
    #[must_use]
    pub fn hst_neg_suc_sts_chng(&mut self) -> HstNegSucStsChngW<GotgintSpec> {
        HstNegSucStsChngW::new(self, 9)
    }
    #[doc = "Bit 17 - Host Negotiation Detected"]
    #[inline(always)]
    #[must_use]
    pub fn hst_neg_det(&mut self) -> HstNegDetW<GotgintSpec> {
        HstNegDetW::new(self, 17)
    }
    #[doc = "Bit 18 - A-Device Timeout Change"]
    #[inline(always)]
    #[must_use]
    pub fn adev_toutchg(&mut self) -> AdevToutchgW<GotgintSpec> {
        AdevToutchgW::new(self, 18)
    }
    #[doc = "Bit 19 - Debounce Done"]
    #[inline(always)]
    #[must_use]
    pub fn dbnce_done(&mut self) -> DbnceDoneW<GotgintSpec> {
        DbnceDoneW::new(self, 19)
    }
}
#[doc = "OTG Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gotgint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gotgint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GotgintSpec;
impl crate::RegisterSpec for GotgintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gotgint::R`](R) reader structure"]
impl crate::Readable for GotgintSpec {}
#[doc = "`write(|w| ..)` method takes [`gotgint::W`](W) writer structure"]
impl crate::Writable for GotgintSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GOTGINT to value 0"]
impl crate::Resettable for GotgintSpec {
    const RESET_VALUE: u32 = 0;
}
