#[doc = "Register `CTR` reader"]
pub struct R(crate::R<CTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTR` writer"]
pub struct W(crate::W<CTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTR_SPEC>;
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
impl From<crate::W<CTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENB` reader - Enable"]
pub type ENB_R = crate::BitReader<bool>;
#[doc = "Field `ENB` writer - Enable"]
pub type ENB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTR_SPEC, bool, O>;
#[doc = "Field `PRE` reader - Pre-warning"]
pub type PRE_R = crate::BitReader<bool>;
#[doc = "Field `PRE` writer - Pre-warning"]
pub type PRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTR_SPEC, bool, O>;
#[doc = "Field `DSP` reader - Debug Suspend"]
pub type DSP_R = crate::BitReader<bool>;
#[doc = "Field `DSP` writer - Debug Suspend"]
pub type DSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTR_SPEC, bool, O>;
#[doc = "Field `SPW` reader - Service Indication Pulse Width"]
pub type SPW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPW` writer - Service Indication Pulse Width"]
pub type SPW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn enb(&self) -> ENB_R {
        ENB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pre-warning"]
    #[inline(always)]
    pub fn pre(&self) -> PRE_R {
        PRE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Debug Suspend"]
    #[inline(always)]
    pub fn dsp(&self) -> DSP_R {
        DSP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Service Indication Pulse Width"]
    #[inline(always)]
    pub fn spw(&self) -> SPW_R {
        SPW_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enb(&mut self) -> ENB_W<0> {
        ENB_W::new(self)
    }
    #[doc = "Bit 1 - Pre-warning"]
    #[inline(always)]
    #[must_use]
    pub fn pre(&mut self) -> PRE_W<1> {
        PRE_W::new(self)
    }
    #[doc = "Bit 4 - Debug Suspend"]
    #[inline(always)]
    #[must_use]
    pub fn dsp(&mut self) -> DSP_W<4> {
        DSP_W::new(self)
    }
    #[doc = "Bits 8:15 - Service Indication Pulse Width"]
    #[inline(always)]
    #[must_use]
    pub fn spw(&mut self) -> SPW_W<8> {
        SPW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WDT Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctr](index.html) module"]
pub struct CTR_SPEC;
impl crate::RegisterSpec for CTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctr::R](R) reader structure"]
impl crate::Readable for CTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctr::W](W) writer structure"]
impl crate::Writable for CTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTR to value 0"]
impl crate::Resettable for CTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
