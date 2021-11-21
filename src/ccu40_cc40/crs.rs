#[doc = "Register `CRS` reader"]
pub struct R(crate::R<CRS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRS` writer"]
pub struct W(crate::W<CRS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRS_SPEC>;
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
impl From<crate::W<CRS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRS` reader - Compare Register"]
pub struct CRS_R(crate::FieldReader<u16, u16>);
impl CRS_R {
    pub(crate) fn new(bits: u16) -> Self {
        CRS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRS` writer - Compare Register"]
pub struct CRS_W<'a> {
    w: &'a mut W,
}
impl<'a> CRS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Compare Register"]
    #[inline(always)]
    pub fn crs(&self) -> CRS_R {
        CRS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Compare Register"]
    #[inline(always)]
    pub fn crs(&mut self) -> CRS_W {
        CRS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Shadow Compare Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crs](index.html) module"]
pub struct CRS_SPEC;
impl crate::RegisterSpec for CRS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crs::R](R) reader structure"]
impl crate::Readable for CRS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crs::W](W) writer structure"]
impl crate::Writable for CRS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRS to value 0"]
impl crate::Resettable for CRS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
