#[doc = "Register `SSTATAR` reader"]
pub struct R(crate::R<SSTATAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSTATAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSTATAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSTATAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSTATAR` writer"]
pub struct W(crate::W<SSTATAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSTATAR_SPEC>;
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
impl From<crate::W<SSTATAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSTATAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSTATAR` reader - Source Status Address"]
pub struct SSTATAR_R(crate::FieldReader<u32, u32>);
impl SSTATAR_R {
    pub(crate) fn new(bits: u32) -> Self {
        SSTATAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSTATAR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSTATAR` writer - Source Status Address"]
pub struct SSTATAR_W<'a> {
    w: &'a mut W,
}
impl<'a> SSTATAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Source Status Address"]
    #[inline(always)]
    pub fn sstatar(&self) -> SSTATAR_R {
        SSTATAR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Source Status Address"]
    #[inline(always)]
    pub fn sstatar(&mut self) -> SSTATAR_W {
        SSTATAR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source Status Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sstatar](index.html) module"]
pub struct SSTATAR_SPEC;
impl crate::RegisterSpec for SSTATAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sstatar::R](R) reader structure"]
impl crate::Readable for SSTATAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sstatar::W](W) writer structure"]
impl crate::Writable for SSTATAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSTATAR to value 0"]
impl crate::Resettable for SSTATAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
