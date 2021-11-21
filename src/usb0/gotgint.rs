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
pub struct SESENDDET_R(crate::FieldReader<bool, bool>);
impl SESENDDET_R {
    pub(crate) fn new(bits: bool) -> Self {
        SESENDDET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SESENDDET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SesEndDet` writer - Session End Detected"]
pub struct SESENDDET_W<'a> {
    w: &'a mut W,
}
impl<'a> SESENDDET_W<'a> {
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
#[doc = "Field `SesReqSucStsChng` reader - Session Request Success Status Change"]
pub struct SESREQSUCSTSCHNG_R(crate::FieldReader<bool, bool>);
impl SESREQSUCSTSCHNG_R {
    pub(crate) fn new(bits: bool) -> Self {
        SESREQSUCSTSCHNG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SESREQSUCSTSCHNG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SesReqSucStsChng` writer - Session Request Success Status Change"]
pub struct SESREQSUCSTSCHNG_W<'a> {
    w: &'a mut W,
}
impl<'a> SESREQSUCSTSCHNG_W<'a> {
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
#[doc = "Field `HstNegSucStsChng` reader - Host Negotiation Success Status Change"]
pub struct HSTNEGSUCSTSCHNG_R(crate::FieldReader<bool, bool>);
impl HSTNEGSUCSTSCHNG_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSTNEGSUCSTSCHNG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSTNEGSUCSTSCHNG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HstNegSucStsChng` writer - Host Negotiation Success Status Change"]
pub struct HSTNEGSUCSTSCHNG_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTNEGSUCSTSCHNG_W<'a> {
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
#[doc = "Field `HstNegDet` reader - Host Negotiation Detected"]
pub struct HSTNEGDET_R(crate::FieldReader<bool, bool>);
impl HSTNEGDET_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSTNEGDET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSTNEGDET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HstNegDet` writer - Host Negotiation Detected"]
pub struct HSTNEGDET_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTNEGDET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `ADevTOUTChg` reader - A-Device Timeout Change"]
pub struct ADEVTOUTCHG_R(crate::FieldReader<bool, bool>);
impl ADEVTOUTCHG_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADEVTOUTCHG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADEVTOUTCHG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADevTOUTChg` writer - A-Device Timeout Change"]
pub struct ADEVTOUTCHG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADEVTOUTCHG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `DbnceDone` reader - Debounce Done"]
pub struct DBNCEDONE_R(crate::FieldReader<bool, bool>);
impl DBNCEDONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBNCEDONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBNCEDONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DbnceDone` writer - Debounce Done"]
pub struct DBNCEDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DBNCEDONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Session End Detected"]
    #[inline(always)]
    pub fn ses_end_det(&self) -> SESENDDET_R {
        SESENDDET_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Session Request Success Status Change"]
    #[inline(always)]
    pub fn ses_req_suc_sts_chng(&self) -> SESREQSUCSTSCHNG_R {
        SESREQSUCSTSCHNG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Host Negotiation Success Status Change"]
    #[inline(always)]
    pub fn hst_neg_suc_sts_chng(&self) -> HSTNEGSUCSTSCHNG_R {
        HSTNEGSUCSTSCHNG_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Host Negotiation Detected"]
    #[inline(always)]
    pub fn hst_neg_det(&self) -> HSTNEGDET_R {
        HSTNEGDET_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - A-Device Timeout Change"]
    #[inline(always)]
    pub fn adev_toutchg(&self) -> ADEVTOUTCHG_R {
        ADEVTOUTCHG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Debounce Done"]
    #[inline(always)]
    pub fn dbnce_done(&self) -> DBNCEDONE_R {
        DBNCEDONE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Session End Detected"]
    #[inline(always)]
    pub fn ses_end_det(&mut self) -> SESENDDET_W {
        SESENDDET_W { w: self }
    }
    #[doc = "Bit 8 - Session Request Success Status Change"]
    #[inline(always)]
    pub fn ses_req_suc_sts_chng(&mut self) -> SESREQSUCSTSCHNG_W {
        SESREQSUCSTSCHNG_W { w: self }
    }
    #[doc = "Bit 9 - Host Negotiation Success Status Change"]
    #[inline(always)]
    pub fn hst_neg_suc_sts_chng(&mut self) -> HSTNEGSUCSTSCHNG_W {
        HSTNEGSUCSTSCHNG_W { w: self }
    }
    #[doc = "Bit 17 - Host Negotiation Detected"]
    #[inline(always)]
    pub fn hst_neg_det(&mut self) -> HSTNEGDET_W {
        HSTNEGDET_W { w: self }
    }
    #[doc = "Bit 18 - A-Device Timeout Change"]
    #[inline(always)]
    pub fn adev_toutchg(&mut self) -> ADEVTOUTCHG_W {
        ADEVTOUTCHG_W { w: self }
    }
    #[doc = "Bit 19 - Debounce Done"]
    #[inline(always)]
    pub fn dbnce_done(&mut self) -> DBNCEDONE_W {
        DBNCEDONE_W { w: self }
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
}
#[doc = "`reset()` method sets GOTGINT to value 0"]
impl crate::Resettable for GOTGINT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
