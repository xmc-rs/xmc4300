#[doc = "Register `TRANSMIT_POLL_DEMAND` reader"]
pub struct R(crate::R<TRANSMIT_POLL_DEMAND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRANSMIT_POLL_DEMAND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRANSMIT_POLL_DEMAND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRANSMIT_POLL_DEMAND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRANSMIT_POLL_DEMAND` writer"]
pub struct W(crate::W<TRANSMIT_POLL_DEMAND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRANSMIT_POLL_DEMAND_SPEC>;
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
impl From<crate::W<TRANSMIT_POLL_DEMAND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRANSMIT_POLL_DEMAND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TPD` reader - Transmit Poll Demand"]
pub type TPD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TPD` writer - Transmit Poll Demand"]
pub type TPD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRANSMIT_POLL_DEMAND_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Transmit Poll Demand"]
    #[inline(always)]
    pub fn tpd(&self) -> TPD_R {
        TPD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit Poll Demand"]
    #[inline(always)]
    #[must_use]
    pub fn tpd(&mut self) -> TPD_W<0> {
        TPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Poll Demand Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [transmit_poll_demand](index.html) module"]
pub struct TRANSMIT_POLL_DEMAND_SPEC;
impl crate::RegisterSpec for TRANSMIT_POLL_DEMAND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [transmit_poll_demand::R](R) reader structure"]
impl crate::Readable for TRANSMIT_POLL_DEMAND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [transmit_poll_demand::W](W) writer structure"]
impl crate::Writable for TRANSMIT_POLL_DEMAND_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRANSMIT_POLL_DEMAND to value 0"]
impl crate::Resettable for TRANSMIT_POLL_DEMAND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
