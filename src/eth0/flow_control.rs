#[doc = "Register `FLOW_CONTROL` reader"]
pub struct R(crate::R<FLOW_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLOW_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLOW_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLOW_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLOW_CONTROL` writer"]
pub struct W(crate::W<FLOW_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLOW_CONTROL_SPEC>;
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
impl From<crate::W<FLOW_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLOW_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FCA_BPA` reader - Flow Control Busy or Backpressure Activate"]
pub type FCA_BPA_R = crate::BitReader<bool>;
#[doc = "Field `FCA_BPA` writer - Flow Control Busy or Backpressure Activate"]
pub type FCA_BPA_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLOW_CONTROL_SPEC, bool, O>;
#[doc = "Field `TFE` reader - Transmit Flow Control Enable"]
pub type TFE_R = crate::BitReader<bool>;
#[doc = "Field `TFE` writer - Transmit Flow Control Enable"]
pub type TFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLOW_CONTROL_SPEC, bool, O>;
#[doc = "Field `RFE` reader - Receive Flow Control Enable"]
pub type RFE_R = crate::BitReader<bool>;
#[doc = "Field `RFE` writer - Receive Flow Control Enable"]
pub type RFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLOW_CONTROL_SPEC, bool, O>;
#[doc = "Field `UP` reader - Unicast Pause Frame Detect"]
pub type UP_R = crate::BitReader<bool>;
#[doc = "Field `UP` writer - Unicast Pause Frame Detect"]
pub type UP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLOW_CONTROL_SPEC, bool, O>;
#[doc = "Field `PLT` reader - Pause Low Threshold"]
pub type PLT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLT` writer - Pause Low Threshold"]
pub type PLT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLOW_CONTROL_SPEC, u8, u8, 2, O>;
#[doc = "Field `DZPQ` reader - Disable Zero-Quanta Pause"]
pub type DZPQ_R = crate::BitReader<bool>;
#[doc = "Field `DZPQ` writer - Disable Zero-Quanta Pause"]
pub type DZPQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLOW_CONTROL_SPEC, bool, O>;
#[doc = "Field `PT` reader - Pause Time"]
pub type PT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PT` writer - Pause Time"]
pub type PT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLOW_CONTROL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - Flow Control Busy or Backpressure Activate"]
    #[inline(always)]
    pub fn fca_bpa(&self) -> FCA_BPA_R {
        FCA_BPA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Flow Control Enable"]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive Flow Control Enable"]
    #[inline(always)]
    pub fn rfe(&self) -> RFE_R {
        RFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Unicast Pause Frame Detect"]
    #[inline(always)]
    pub fn up(&self) -> UP_R {
        UP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Pause Low Threshold"]
    #[inline(always)]
    pub fn plt(&self) -> PLT_R {
        PLT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Disable Zero-Quanta Pause"]
    #[inline(always)]
    pub fn dzpq(&self) -> DZPQ_R {
        DZPQ_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Pause Time"]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Flow Control Busy or Backpressure Activate"]
    #[inline(always)]
    #[must_use]
    pub fn fca_bpa(&mut self) -> FCA_BPA_W<0> {
        FCA_BPA_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Flow Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tfe(&mut self) -> TFE_W<1> {
        TFE_W::new(self)
    }
    #[doc = "Bit 2 - Receive Flow Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfe(&mut self) -> RFE_W<2> {
        RFE_W::new(self)
    }
    #[doc = "Bit 3 - Unicast Pause Frame Detect"]
    #[inline(always)]
    #[must_use]
    pub fn up(&mut self) -> UP_W<3> {
        UP_W::new(self)
    }
    #[doc = "Bits 4:5 - Pause Low Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn plt(&mut self) -> PLT_W<4> {
        PLT_W::new(self)
    }
    #[doc = "Bit 7 - Disable Zero-Quanta Pause"]
    #[inline(always)]
    #[must_use]
    pub fn dzpq(&mut self) -> DZPQ_W<7> {
        DZPQ_W::new(self)
    }
    #[doc = "Bits 16:31 - Pause Time"]
    #[inline(always)]
    #[must_use]
    pub fn pt(&mut self) -> PT_W<16> {
        PT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flow Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flow_control](index.html) module"]
pub struct FLOW_CONTROL_SPEC;
impl crate::RegisterSpec for FLOW_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flow_control::R](R) reader structure"]
impl crate::Readable for FLOW_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flow_control::W](W) writer structure"]
impl crate::Writable for FLOW_CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLOW_CONTROL to value 0"]
impl crate::Resettable for FLOW_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
