#[doc = "Register `DAR` reader"]
pub struct R(crate::R<DAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAR` writer"]
pub struct W(crate::W<DAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAR_SPEC>;
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
impl From<crate::W<DAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAR` reader - Current Destination address of DMA transfer"]
pub type DAR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DAR` writer - Current Destination address of DMA transfer"]
pub type DAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Current Destination address of DMA transfer"]
    #[inline(always)]
    pub fn dar(&self) -> DAR_R {
        DAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Current Destination address of DMA transfer"]
    #[inline(always)]
    #[must_use]
    pub fn dar(&mut self) -> DAR_W<0> {
        DAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Destination Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dar](index.html) module"]
pub struct DAR_SPEC;
impl crate::RegisterSpec for DAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dar::R](R) reader structure"]
impl crate::Readable for DAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dar::W](W) writer structure"]
impl crate::Writable for DAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAR to value 0"]
impl crate::Resettable for DAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
