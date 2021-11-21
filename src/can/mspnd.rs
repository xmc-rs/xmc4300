#[doc = "Register `MSPND[%s]` reader"]
pub struct R(crate::R<MSPND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSPND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSPND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSPND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSPND[%s]` writer"]
pub struct W(crate::W<MSPND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSPND_SPEC>;
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
impl From<crate::W<MSPND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSPND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PND` reader - Message Pending"]
pub struct PND_R(crate::FieldReader<u32, u32>);
impl PND_R {
    pub(crate) fn new(bits: u32) -> Self {
        PND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PND_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PND` writer - Message Pending"]
pub struct PND_W<'a> {
    w: &'a mut W,
}
impl<'a> PND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Message Pending"]
    #[inline(always)]
    pub fn pnd(&self) -> PND_R {
        PND_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Message Pending"]
    #[inline(always)]
    pub fn pnd(&mut self) -> PND_W {
        PND_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Message Pending Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mspnd](index.html) module"]
pub struct MSPND_SPEC;
impl crate::RegisterSpec for MSPND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mspnd::R](R) reader structure"]
impl crate::Readable for MSPND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mspnd::W](W) writer structure"]
impl crate::Writable for MSPND_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MSPND[%s]
to value 0"]
impl crate::Resettable for MSPND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
