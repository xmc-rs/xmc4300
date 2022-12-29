#[doc = "Register `DSTAT` reader"]
pub struct R(crate::R<DSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSTAT` writer"]
pub struct W(crate::W<DSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSTAT_SPEC>;
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
impl From<crate::W<DSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSTAT` reader - Destination Status"]
pub type DSTAT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DSTAT` writer - Destination Status"]
pub type DSTAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DSTAT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Destination Status"]
    #[inline(always)]
    pub fn dstat(&self) -> DSTAT_R {
        DSTAT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Destination Status"]
    #[inline(always)]
    #[must_use]
    pub fn dstat(&mut self) -> DSTAT_W<0> {
        DSTAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Destination Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dstat](index.html) module"]
pub struct DSTAT_SPEC;
impl crate::RegisterSpec for DSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dstat::R](R) reader structure"]
impl crate::Readable for DSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dstat::W](W) writer structure"]
impl crate::Writable for DSTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSTAT to value 0"]
impl crate::Resettable for DSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
