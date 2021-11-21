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
#[doc = "Deactivate SyncManager\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `DEACT` reader - Deactivate SyncManager"]
pub struct DEACT_R(crate::FieldReader<bool, DEACT_A>);
impl DEACT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEACT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == DEACT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DEACT_A::VALUE2
    }
}
impl core::ops::Deref for DEACT_R {
    type Target = crate::FieldReader<bool, DEACT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEACT` writer - Deactivate SyncManager"]
pub struct DEACT_W<'a> {
    w: &'a mut W,
}
impl<'a> DEACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEACT_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "Field `REP_ACK` reader - Repeat Ack"]
pub struct REP_ACK_R(crate::FieldReader<bool, bool>);
impl REP_ACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        REP_ACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REP_ACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REP_ACK` writer - Repeat Ack"]
pub struct REP_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> REP_ACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Deactivate SyncManager"]
    #[inline(always)]
    pub fn deact(&self) -> DEACT_R {
        DEACT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Repeat Ack"]
    #[inline(always)]
    pub fn rep_ack(&self) -> REP_ACK_R {
        REP_ACK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Deactivate SyncManager"]
    #[inline(always)]
    pub fn deact(&mut self) -> DEACT_W {
        DEACT_W { w: self }
    }
    #[doc = "Bit 1 - Repeat Ack"]
    #[inline(always)]
    pub fn rep_ack(&mut self) -> REP_ACK_W {
        REP_ACK_W { w: self }
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
}
#[doc = "`reset()` method sets SM_PDI_CTR to value 0"]
impl crate::Resettable for SM_PDI_CTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
