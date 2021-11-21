#[doc = "Register `DSTATAR` reader"]
pub struct R(crate::R<DSTATAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSTATAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSTATAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSTATAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSTATAR` writer"]
pub struct W(crate::W<DSTATAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSTATAR_SPEC>;
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
impl From<crate::W<DSTATAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSTATAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSTATAR` reader - Destination Status Address"]
pub struct DSTATAR_R(crate::FieldReader<u32, u32>);
impl DSTATAR_R {
    pub(crate) fn new(bits: u32) -> Self {
        DSTATAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSTATAR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSTATAR` writer - Destination Status Address"]
pub struct DSTATAR_W<'a> {
    w: &'a mut W,
}
impl<'a> DSTATAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Destination Status Address"]
    #[inline(always)]
    pub fn dstatar(&self) -> DSTATAR_R {
        DSTATAR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Destination Status Address"]
    #[inline(always)]
    pub fn dstatar(&mut self) -> DSTATAR_W {
        DSTATAR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Destination Status Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dstatar](index.html) module"]
pub struct DSTATAR_SPEC;
impl crate::RegisterSpec for DSTATAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dstatar::R](R) reader structure"]
impl crate::Readable for DSTATAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dstatar::W](W) writer structure"]
impl crate::Writable for DSTATAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DSTATAR to value 0"]
impl crate::Resettable for DSTATAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
