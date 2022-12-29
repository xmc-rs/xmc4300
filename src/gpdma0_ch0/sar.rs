#[doc = "Register `SAR` reader"]
pub struct R(crate::R<SAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR` writer"]
pub struct W(crate::W<SAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_SPEC>;
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
impl From<crate::W<SAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR` reader - Current Source Address of DMA transfer"]
pub type SAR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SAR` writer - Current Source Address of DMA transfer"]
pub type SAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Current Source Address of DMA transfer"]
    #[inline(always)]
    pub fn sar(&self) -> SAR_R {
        SAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Current Source Address of DMA transfer"]
    #[inline(always)]
    #[must_use]
    pub fn sar(&mut self) -> SAR_W<0> {
        SAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar](index.html) module"]
pub struct SAR_SPEC;
impl crate::RegisterSpec for SAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar::R](R) reader structure"]
impl crate::Readable for SAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar::W](W) writer structure"]
impl crate::Writable for SAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR to value 0"]
impl crate::Resettable for SAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
