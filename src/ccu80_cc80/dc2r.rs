#[doc = "Register `DC2R` reader"]
pub struct R(crate::R<DC2R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DC2R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DC2R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DC2R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DC2R` writer"]
pub struct W(crate::W<DC2R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DC2R_SPEC>;
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
impl From<crate::W<DC2R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DC2R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DT2R` reader - Rise Value for Dead Time of Channel 2"]
pub struct DT2R_R(crate::FieldReader<u8, u8>);
impl DT2R_R {
    pub(crate) fn new(bits: u8) -> Self {
        DT2R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DT2R_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DT2R` writer - Rise Value for Dead Time of Channel 2"]
pub struct DT2R_W<'a> {
    w: &'a mut W,
}
impl<'a> DT2R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `DT2F` reader - Fall Value for Dead Time of Channel 2"]
pub struct DT2F_R(crate::FieldReader<u8, u8>);
impl DT2F_R {
    pub(crate) fn new(bits: u8) -> Self {
        DT2F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DT2F_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DT2F` writer - Fall Value for Dead Time of Channel 2"]
pub struct DT2F_W<'a> {
    w: &'a mut W,
}
impl<'a> DT2F_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Rise Value for Dead Time of Channel 2"]
    #[inline(always)]
    pub fn dt2r(&self) -> DT2R_R {
        DT2R_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Fall Value for Dead Time of Channel 2"]
    #[inline(always)]
    pub fn dt2f(&self) -> DT2F_R {
        DT2F_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Rise Value for Dead Time of Channel 2"]
    #[inline(always)]
    pub fn dt2r(&mut self) -> DT2R_W {
        DT2R_W { w: self }
    }
    #[doc = "Bits 8:15 - Fall Value for Dead Time of Channel 2"]
    #[inline(always)]
    pub fn dt2f(&mut self) -> DT2F_W {
        DT2F_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel 2 Dead Time Values\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc2r](index.html) module"]
pub struct DC2R_SPEC;
impl crate::RegisterSpec for DC2R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dc2r::R](R) reader structure"]
impl crate::Readable for DC2R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dc2r::W](W) writer structure"]
impl crate::Writable for DC2R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DC2R to value 0"]
impl crate::Resettable for DC2R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
