#[doc = "Register `SM_PDI_CTR` reader"]
pub struct R(crate::R<SM_PDI_CTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SM_PDI_CTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SM_PDI_CTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SM_PDI_CTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SM_PDI_CTR` writer"]
pub struct W(crate::W<SM_PDI_CTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SM_PDI_CTR_SPEC>;
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
impl From<crate::W<SM_PDI_CTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SM_PDI_CTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEACT` reader - Deactivate SyncManager"]
pub type DEACT_R = crate::BitReader<DEACT_A>;
#[doc = "Deactivate SyncManager\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEACT_A {
    #[doc = "0: Read 0 for Normal operation, SyncManager activated, write 0 for Activate SyncManager"]
    VALUE1 = 0,
    #[doc = "1: Read 1 for SyncManager deactivated and reset SyncManager locks access to Memory area, write 1 for Request SyncManager deactivation"]
    VALUE2 = 1,
}
impl From<DEACT_A> for bool {
    #[inline(always)]
    fn from(variant: DEACT_A) -> Self {
        variant as u8 != 0
    }
}
impl DEACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEACT_A {
        match self.bits {
            false => DEACT_A::VALUE1,
            true => DEACT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DEACT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DEACT_A::VALUE2
    }
}
#[doc = "Field `DEACT` writer - Deactivate SyncManager"]
pub type DEACT_W<'a, const O: u8> = crate::BitWriter<'a, u8, SM_PDI_CTR_SPEC, DEACT_A, O>;
impl<'a, const O: u8> DEACT_W<'a, O> {
    #[doc = "Read 0 for Normal operation, SyncManager activated, write 0 for Activate SyncManager"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DEACT_A::VALUE1)
    }
    #[doc = "Read 1 for SyncManager deactivated and reset SyncManager locks access to Memory area, write 1 for Request SyncManager deactivation"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DEACT_A::VALUE2)
    }
}
#[doc = "Field `REP_ACK` reader - Repeat Ack"]
pub type REP_ACK_R = crate::BitReader<bool>;
#[doc = "Field `REP_ACK` writer - Repeat Ack"]
pub type REP_ACK_W<'a, const O: u8> = crate::BitWriter<'a, u8, SM_PDI_CTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Deactivate SyncManager"]
    #[inline(always)]
    pub fn deact(&self) -> DEACT_R {
        DEACT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Repeat Ack"]
    #[inline(always)]
    pub fn rep_ack(&self) -> REP_ACK_R {
        REP_ACK_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Deactivate SyncManager"]
    #[inline(always)]
    #[must_use]
    pub fn deact(&mut self) -> DEACT_W<0> {
        DEACT_W::new(self)
    }
    #[doc = "Bit 1 - Repeat Ack"]
    #[inline(always)]
    #[must_use]
    pub fn rep_ack(&mut self) -> REP_ACK_W<1> {
        REP_ACK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDI Control SyncManager 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm_pdi_ctr](index.html) module"]
pub struct SM_PDI_CTR_SPEC;
impl crate::RegisterSpec for SM_PDI_CTR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sm_pdi_ctr::R](R) reader structure"]
impl crate::Readable for SM_PDI_CTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sm_pdi_ctr::W](W) writer structure"]
impl crate::Writable for SM_PDI_CTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SM_PDI_CTR to value 0"]
impl crate::Resettable for SM_PDI_CTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
